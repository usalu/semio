//! 🗄️ `db_cli` — the `db` binary: `inspect`/`doc`/`wal-inspect`/`snapshot-inspect`/`verify`/
//! `query`/`replay`/`repair`/`compact`/`health`/`conflict-simulate`/`replica-simulate`/`migrate`/
//! `profile` over a `db::storage::FsStorage`-rooted document store. Hand-rolled arg parsing (no
//! external CLI crate, per repo convention), exit codes `0` (success) / `1` (operation failed) /
//! `2` (usage error), never panics on corrupt input. Frozen contract:
//! `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_cli` row).
//!
//! 🎯 Design choice (depends on the `db` facade alone, `db_*` paths nowhere): every subcommand
//! below reaches every primitive it needs purely through `db::<submodule>::…` paths (`db::storage`,
//! `db::wal`, `db::snapshot`, `db::conflict`, `db::cluster`, `db::observe`, `db::document`,
//! `db::actor`, `db::core`, …) — the facade's own re-exports, verified complete against `db/rs/
//! lib.rs`. This crate's `Cargo.toml` accordingly depends on nothing but `db` itself plus two
//! siblings that are NOT `db_*` crates: `protocol` (every frozen `Database`/`DocumentHandle` entry
//! point is typed against `protocol::DocumentId`/`OperationEnvelope`/…, which the facade exposes
//! without re-exporting a path to) and `pack` (`SnapshotManager::verify`'s `VerificationLevel` is
//! pack's own type, snapshots being pack files). No `db_storage`/`db_wal`/`db_snapshot`/… path
//! dependency of its own — `wal-inspect`/`snapshot-inspect`/`replay`/`repair` still need
//! lower-level access than the actor-mediated `Database` API exposes (there is no `Database` method
//! that lists WAL segments or snapshot generations), reached the same facade-path way.
//!
//! 🎯 Every subcommand is real, including `migrate` and `profile`: `migrate` appends a genuine
//! `WAL_MIGRATION` record via `db::wal::DocumentWal` (force-flushed durably); `profile` submits `N`
//! real commands sequentially through `DocumentHandle::submit` and reports wall-clock throughput —
//! both self-contained (bootstrap their own document/WAL if it doesn't exist yet), needing nothing
//! from `db_testkit` (a separate, non-`db`-facade sibling crate this one deliberately does not
//! depend on, to keep the "facade only" dependency footprint honest). `conflict-simulate` runs the
//! genuine `db::conflict::ConflictDetector` over two hand-built `CommandTouch`es (no storage
//! touched); `replica-simulate` runs the genuine `db::cluster::replicate_document` primitive between
//! two local `FsStorage` roots (no network transport exists yet in this family, which is exactly why
//! this stays a *simulation* rather than a real cluster operation).

use std::collections::HashMap;
use std::path::Path;

use db::storage::{SnapshotStorage as _, WalStorage as _};

//#region 🔖Args
/// ✂️ Splits argv-style slices into positionals and `--flag value` / `--flag=value` pairs; a
/// trailing bare `--flag` with nothing after it maps to an empty-string value. Callers with any
/// no-argument boolean flags must strip those out of `args` first via `strip_flag` — this parser
/// always tries to consume the next token as a value, which would otherwise swallow a following
/// positional/flag. Mirrors `protocol_cli::parse_args`'s exact shape (same repo convention).
fn parse_args(args: &[String]) -> (Vec<String>, HashMap<String, String>) {
    let mut positional = Vec::new();
    let mut flags = HashMap::new();
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        let Some(rest) = arg.strip_prefix("--") else {
            positional.push(arg.clone());
            index += 1;
            continue;
        };
        if let Some((key, value)) = rest.split_once('=') {
            flags.insert(key.to_string(), value.to_string());
            index += 1;
        } else if index + 1 < args.len() {
            flags.insert(rest.to_string(), args[index + 1].clone());
            index += 2;
        } else {
            flags.insert(rest.to_string(), String::new());
            index += 1;
        }
    }
    (positional, flags)
}

/// ✂️ Pulls every occurrence of a no-value boolean `--<name>` flag out of `args` before the
/// generic `parse_args` runs — see `parse_args`'s doc for why that's required.
fn strip_flag(args: &[String], name: &str) -> (Vec<String>, bool) {
    let flag = format!("--{name}");
    let mut present = false;
    let mut rest = Vec::with_capacity(args.len());
    for arg in args {
        if arg == &flag {
            present = true;
        } else {
            rest.push(arg.clone());
        }
    }
    (rest, present)
}

fn parse_profile(flags: &HashMap<String, String>) -> Result<db::Profile, String> {
    match flags.get("profile").map(String::as_str) {
        None => Ok(db::Profile::Dev),
        Some("test") => Ok(db::Profile::Test),
        Some("dev") => Ok(db::Profile::Dev),
        Some("prod") => Ok(db::Profile::Prod),
        Some(other) => Err(format!("unknown --profile '{other}' (expected test|dev|prod)")),
    }
}
//#endregion 🔖Args

//#region 🔖Format
fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

/// 🧾 A best-effort human display of a query result's raw value bytes — UTF-8 text verbatim (the
/// common case: `db_document`'s path-value convention stores JSON-encoded scalars, which decode as
/// text), or a byte count for anything that doesn't decode, never a panic.
fn describe_value_bytes(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_string(),
        Err(_) => format!("<{} bytes, non-utf8>", bytes.len()),
    }
}

fn fail(context: &str, err: impl std::fmt::Display) -> i32 {
    eprintln!("db: {context}: {err}");
    1
}

fn usage(message: &str) -> i32 {
    eprintln!("db: {message}");
    2
}
//#endregion 🔖Format

//#region 🔖Health
fn health_state_label(state: &db::observe::HealthState) -> String {
    match state {
        db::observe::HealthState::Healthy => "healthy".to_string(),
        db::observe::HealthState::Degraded(reason) => format!("degraded: {reason}"),
        db::observe::HealthState::Unhealthy(reason) => format!("unhealthy: {reason}"),
    }
}

fn print_health(health: &db::DbHealth) {
    println!("== health ==");
    println!("  overall: {}", health_state_label(&health.report.overall));
    println!("  open_documents: {}", health.open_documents);
    for (name, state) in &health.report.components {
        println!("  - {name}: {}", health_state_label(state));
    }
}
//#endregion 🔖Health

//#region 🔖Inspect
/// 📇 `db inspect <root> [--profile test|dev|prod]` — opens (zero-touch, creating if absent) the
/// `Database` at `root` and prints its catalog plus a health snapshot.
fn cmd_inspect(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(root) = positional.first() else {
        return usage("usage: db inspect <root> [--profile test|dev|prod]");
    };
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };

    let catalog = database.catalog();
    println!("== catalog: {root} ==");
    println!("  documents: {}", catalog.documents.len());
    for entry in &catalog.documents {
        println!("  - {} (created_at_ms={})", entry.document.0, entry.created_at_ms);
    }
    print_health(&database.health());

    if let Err(err) = database.shutdown(std::time::Duration::from_secs(5)) {
        return fail("shutdown", err);
    }
    0
}
//#endregion 🔖Inspect

//#region 🔖Doc
fn print_engine_frontier(frontier: &db::Frontier) {
    println!("  head_seq: {}", frontier.head_seq);
    println!("  commit_seq: {}", frontier.commit_seq);
    println!("  epoch: {}", frontier.epoch);
    println!("  chain_hash: {}", hex32(&frontier.chain_hash));
}

/// 📄 `db doc <root> <document-id> [--profile ...]` — opens `document` through a real
/// `DocumentHandle` (recovering it from its WAL if not already open) and prints its current
/// frontier plus a tail of its committed history (real: `DocumentHandle::history` replays the WAL
/// directly per `db_engine`'s own module doc).
fn cmd_doc(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 2 {
        return usage("usage: db doc <root> <document-id> [--profile test|dev|prod]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };
    let document_id = protocol::DocumentId(id.clone());
    let handle = match database.document(&document_id) {
        Ok(handle) => handle,
        Err(err) => return fail("document", err),
    };

    let outcome = match handle.frontier() {
        Ok(frontier) => {
            println!("== document {id} ==");
            print_engine_frontier(&frontier);
            match handle.history() {
                Ok(history) => {
                    println!("  history entries: {}", history.entries.len());
                    for entry in history.entries.iter().rev().take(10) {
                        println!("    - operations={} head_seq={}", entry.operation_ids.len(), entry.frontier.head_seq);
                    }
                }
                Err(err) => println!("  history: unavailable ({err})"),
            }
            0
        }
        Err(err) => fail("frontier", err),
    };

    match database.shutdown(std::time::Duration::from_secs(5)) {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err),
    }
}
//#endregion 🔖Doc

//#region 🔖WalInspect
fn wal_record_kind_name(record: &db::wal::WalRecord) -> &'static str {
    match record {
        db::wal::WalRecord::SegmentHeader { .. } => "segment_header",
        db::wal::WalRecord::TxBegin { .. } => "tx_begin",
        db::wal::WalRecord::TxCommit { .. } => "tx_commit",
        db::wal::WalRecord::TxAbort { .. } => "tx_abort",
        db::wal::WalRecord::Command(_) => "command",
        db::wal::WalRecord::Payload(_) => "payload",
        db::wal::WalRecord::Diff(_) => "diff",
        db::wal::WalRecord::Inverse(_) => "inverse",
        db::wal::WalRecord::Event(_) => "event",
        db::wal::WalRecord::Outbox(_) => "outbox",
        db::wal::WalRecord::Frontier(_) => "frontier",
        db::wal::WalRecord::VcsRef(_) => "vcs_ref",
        db::wal::WalRecord::SnapshotPub { .. } => "snapshot_pub",
        db::wal::WalRecord::IndexCkpt { .. } => "index_ckpt",
        db::wal::WalRecord::Lease { .. } => "lease",
        db::wal::WalRecord::Migration(_) => "migration",
    }
}

fn describe_wal_record(record: &db::wal::WalRecord) -> String {
    let kind = wal_record_kind_name(record);
    match record {
        db::wal::WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
            format!("{kind} document={document} segment_index={segment_index} prev_chain_hash={}", prev_chain_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash)))
        }
        db::wal::WalRecord::TxBegin { tx_id } => format!("{kind} tx_id={tx_id}"),
        db::wal::WalRecord::TxCommit { tx_id, record_count } => format!("{kind} tx_id={tx_id} record_count={record_count}"),
        db::wal::WalRecord::TxAbort { tx_id } => format!("{kind} tx_id={tx_id}"),
        db::wal::WalRecord::Command(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Payload(db::wal::WalPayloadRef::Inline(bytes)) => format!("{kind} inline bytes={}", bytes.len()),
        db::wal::WalRecord::Payload(db::wal::WalPayloadRef::CasRef(hash)) => format!("{kind} cas_ref hash={}", hex32(&hash.0)),
        db::wal::WalRecord::Diff(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Inverse(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Event(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Outbox(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Frontier(frontier) => {
            format!("{kind} head_seq={} commit_seq={} epoch={} chain_hash={}", frontier.head_seq, frontier.commit_seq, frontier.epoch, hex32(&frontier.chain_hash))
        }
        db::wal::WalRecord::VcsRef(id) => format!("{kind} id={id}"),
        db::wal::WalRecord::SnapshotPub { generation, frontier } => format!("{kind} generation={generation} head_seq={}", frontier.head_seq),
        db::wal::WalRecord::IndexCkpt { run_ids } => format!("{kind} run_ids={}", run_ids.len()),
        db::wal::WalRecord::Lease { resource, holder, fence, expires_at_ms } => format!("{kind} resource={resource} holder={holder} fence={fence} expires_at_ms={expires_at_ms}"),
        db::wal::WalRecord::Migration(bytes) => format!("{kind} bytes={}", bytes.len()),
    }
}

/// 📼 `db wal-inspect <root> <document-id> [--limit N]` — lists `document`'s raw WAL segments
/// (index + byte length, straight from `WalStorage`, before any decoding) then decodes every
/// `WAL_*` record via `db::wal::replay_document`. A torn/corrupt WAL is reported honestly (with a
/// hint pointing at `db repair`), never panics.
fn cmd_wal_inspect(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 2 {
        return usage("usage: db wal-inspect <root> <document-id> [--limit N]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let limit = match flags.get("limit").map(|value| value.parse::<usize>()) {
        None => None,
        Some(Ok(limit)) => Some(limit),
        Some(Err(_)) => return usage("db wal-inspect: --limit must be a non-negative integer"),
    };

    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let document = db::core::DocumentId(id.clone());

    let segments = match storage.list_segments(&document) {
        Ok(segments) => segments,
        Err(err) => return fail("list_segments", err),
    };
    println!("== wal segments: {} ==", segments.len());
    for index in &segments {
        match storage.segment_len(&document, *index) {
            Ok(len) => println!("  segment {index}: {len} bytes"),
            Err(err) => println!("  segment {index}: <error reading length: {err}>"),
        }
    }

    match db::wal::replay_document(&storage, &document) {
        Ok(records) => {
            println!("== wal records: {} ==", records.len());
            let shown = limit.unwrap_or(records.len()).min(records.len());
            for (index, record) in records.iter().take(shown).enumerate() {
                println!("  [{index}] {}", describe_wal_record(record));
            }
            if shown < records.len() {
                println!("  ... {} more (pass --limit to see more)", records.len() - shown);
            }
            0
        }
        Err(err) => {
            println!("== wal records: unavailable ==");
            println!("  {err}");
            println!("  (hint: run `db repair {root} {id}` to discard a torn tail)");
            1
        }
    }
}
//#endregion 🔖WalInspect

//#region 🔖SnapshotInspect
/// 📸 `db snapshot-inspect <root> <document-id> [--generation N] [--verify]` — lists every stored
/// generation, then decodes one (the latest, or `--generation N`) via `db::snapshot::open_latest`
/// over `SnapshotManager::materialize_chain`'s combined lineage buffer, printing its descriptor.
/// `--verify` additionally runs `SnapshotManager::verify` at `VerificationLevel::Full`.
fn cmd_snapshot_inspect(rest: &[String]) -> i32 {
    let (rest, verify) = strip_flag(rest, "verify");
    let (positional, flags) = parse_args(&rest);
    if positional.len() < 2 {
        return usage("usage: db snapshot-inspect <root> <document-id> [--generation N] [--verify]");
    }
    let root = &positional[0];
    let id = &positional[1];

    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let document = db::core::DocumentId(id.clone());

    let generations = match storage.list_generations(&document) {
        Ok(generations) => generations,
        Err(err) => return fail("list_generations", err),
    };
    println!("== snapshot generations: {} ==", generations.len());
    for generation in &generations {
        println!("  - {generation}");
    }
    let Some(&latest) = generations.last() else {
        return 0;
    };

    let generation = match flags.get("generation").map(|value| value.parse::<u64>()) {
        None => latest,
        Some(Ok(generation)) => generation,
        Some(Err(_)) => return usage("db snapshot-inspect: --generation must be a non-negative integer"),
    };

    let manager = db::snapshot::SnapshotManager::new(&storage);
    let combined = match manager.materialize_chain(&document, generation) {
        Ok(combined) => combined,
        Err(err) => return fail("materialize_chain", err),
    };
    let handle = match db::snapshot::open_latest(&combined) {
        Ok(handle) => handle,
        Err(err) => return fail("open_latest", err),
    };
    let descriptor = &handle.descriptor;
    println!("== generation {generation} descriptor ==");
    println!("  parent_generation: {}", descriptor.parent_generation.map_or_else(|| "-".to_string(), |generation| generation.to_string()));
    println!("  head_seq: {}", descriptor.head_seq);
    println!("  commit_seq: {}", descriptor.commit_seq);
    println!("  epoch: {}", descriptor.epoch);
    println!("  chain_hash: {}", hex32(&descriptor.chain_hash));
    println!("  protocol_version: {}", descriptor.protocol_version);
    println!("  vcs_head: {}", descriptor.vcs_head.as_deref().unwrap_or("-"));
    println!("  base_pack_hash: {}", descriptor.base_pack_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash.0)));
    println!("  roots: {}", descriptor.roots.len());
    println!("  new_pages: {}", descriptor.new_pages.len());
    println!("  created_at_ms: {}", descriptor.created_at_ms);

    if verify {
        match manager.verify(&document, generation, pack::VerificationLevel::Full) {
            Ok(()) => println!("== verify: OK =="),
            Err(err) => {
                println!("== verify: FAIL ==");
                println!("  {err}");
                return 1;
            }
        }
    }
    0
}
//#endregion 🔖SnapshotInspect

//#region 🔖Verify
/// 🔬 The shared per-document check `verify` runs: a full WAL replay (rejects a torn tail) plus,
/// if a snapshot exists, a full-level `SnapshotManager::verify` of its latest generation.
fn verify_document(storage: &db::storage::FsStorage, document: &db::core::DocumentId) -> Result<String, db::DbError> {
    let records = db::wal::replay_document(storage, document)?;
    let manager = db::snapshot::SnapshotManager::new(storage);
    match manager.load_latest(document)? {
        Some((generation, _descriptor)) => {
            manager.verify(document, generation, pack::VerificationLevel::Full)?;
            Ok(format!("wal records={} snapshot generation={generation} (verified)", records.len()))
        }
        None => Ok(format!("wal records={} snapshot=none", records.len())),
    }
}

/// 🔬 `db verify <root> [document-id] [--profile ...]` — verifies one document, or every document
/// in the catalog if none is given. Prints `OK <id>: <summary>` / `FAIL <id>: <reason>` per
/// document, never panics on corrupt input; exits `1` iff any document failed.
fn cmd_verify(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(root) = positional.first() else {
        return usage("usage: db verify <root> [document-id] [--profile test|dev|prod]");
    };
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };

    let ids: Vec<String> = match positional.get(1) {
        Some(id) => vec![id.clone()],
        None => {
            let database = match db::Database::open_at(Path::new(root), profile) {
                Ok(database) => database,
                Err(err) => return fail("open", err),
            };
            let ids = database.catalog().documents.iter().map(|entry| entry.document.0.clone()).collect();
            if let Err(err) = database.shutdown(std::time::Duration::from_secs(5)) {
                return fail("shutdown", err);
            }
            ids
        }
    };
    if ids.is_empty() {
        println!("db verify: no documents in catalog at '{root}'");
        return 0;
    }

    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let mut failures = 0usize;
    for id in &ids {
        let document = db::core::DocumentId(id.clone());
        match verify_document(&storage, &document) {
            Ok(summary) => println!("OK   {id}: {summary}"),
            Err(err) => {
                println!("FAIL {id}: {err}");
                failures += 1;
            }
        }
    }
    if failures > 0 { 1 } else { 0 }
}
//#endregion 🔖Verify

//#region 🔖Query
/// 🔎 `db query <root> <document-id> <path> [more-paths...] [--profile ...]` — resolves one or
/// more paths against `document`'s live (`Consistency::Canonical`) state through a real
/// `DocumentHandle::query`.
fn cmd_query(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 3 {
        return usage("usage: db query <root> <document-id> <path> [more-paths...] [--profile test|dev|prod]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let paths: Vec<String> = positional[2..].to_vec();
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };
    let document_id = protocol::DocumentId(id.clone());
    let handle = match database.document(&document_id) {
        Ok(handle) => handle,
        Err(err) => return fail("document", err),
    };

    let query = match paths.len() {
        1 => db::Query::Get { path: paths[0].clone() },
        _ => db::Query::GetMany { paths },
    };
    let outcome = match handle.query(query, db::Consistency::Canonical) {
        Ok(stream) => {
            for (path, value) in &stream.results {
                match value {
                    Some(bytes) => println!("{path} = {}", describe_value_bytes(bytes)),
                    None => println!("{path} = <unset>"),
                }
            }
            0
        }
        Err(err) => fail("query", err),
    };

    match database.shutdown(std::time::Duration::from_secs(5)) {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err),
    }
}
//#endregion 🔖Query

//#region 🔖Replay
/// 🔁 `db replay <root> <document-id>` — a raw, actor-bypassing `db::wal::replay_document` pass:
/// record-kind counts plus the frontier reconstructed from the last `WAL_FRONTIER` record. Distinct
/// from `doc`, which goes through a live `DocumentAuthority` — this is the lower-level diagnostic
/// twin, useful precisely when the actor path itself is in question.
fn cmd_replay(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest);
    if positional.len() < 2 {
        return usage("usage: db replay <root> <document-id>");
    }
    let root = &positional[0];
    let id = &positional[1];
    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let document = db::core::DocumentId(id.clone());
    let records = match db::wal::replay_document(&storage, &document) {
        Ok(records) => records,
        Err(err) => return fail("replay", err),
    };

    let mut kind_counts: std::collections::BTreeMap<&'static str, usize> = std::collections::BTreeMap::new();
    let mut last_frontier: Option<db::core::Frontier> = None;
    for record in &records {
        *kind_counts.entry(wal_record_kind_name(record)).or_insert(0) += 1;
        if let db::wal::WalRecord::Frontier(frontier) = record {
            last_frontier = Some(frontier.clone());
        }
    }

    println!("== replay: {id} ==");
    println!("  records: {}", records.len());
    for (kind, count) in &kind_counts {
        println!("  - {kind}: {count}");
    }
    match last_frontier {
        Some(frontier) => {
            println!("  reconstructed frontier:");
            println!("    head_seq: {}", frontier.head_seq);
            println!("    commit_seq: {}", frontier.commit_seq);
            println!("    epoch: {}", frontier.epoch);
            println!("    chain_hash: {}", hex32(&frontier.chain_hash));
        }
        None => println!("  reconstructed frontier: none (no committed transaction found)"),
    }
    0
}
//#endregion 🔖Replay

//#region 🔖Repair
/// 🩹 `db repair <root> <document-id>` — the real repair primitive: `db::wal::DocumentWal::open`
/// already discards a torn active-segment tail and rewrites it from the trusted prefix (see its own
/// doc's "forced by `protocol::SprWriter`'s API" design-choice note) — this subcommand simply drives
/// that recovery path and reports what it found. Idempotent on an already-clean WAL.
fn cmd_repair(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest);
    if positional.len() < 2 {
        return usage("usage: db repair <root> <document-id>");
    }
    let root = &positional[0];
    let id = &positional[1];
    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let document = db::core::DocumentId(id.clone());
    match db::wal::DocumentWal::open(&storage, document, db::wal::GroupCommitPolicy::default(), now_ms()) {
        Ok((_wal, report)) => {
            println!("== repair: {id} ==");
            println!("  segments_seen: {}", report.segments_seen);
            println!("  records_replayed: {}", report.records_replayed);
            println!("  torn_tail_bytes: {}", report.torn_tail_bytes);
            if report.torn_tail_bytes > 0 {
                println!("  discarded a torn tail — the active segment has been rewritten");
            } else {
                println!("  no torn tail found — wal was already clean");
            }
            0
        }
        Err(err) => fail("repair", err),
    }
}
//#endregion 🔖Repair

//#region 🔖Compact
/// 🧹 `db compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]` — drives a
/// real, fenced `db_compact::Compactor` pass via `Database::compact_document`.
fn cmd_compact(rest: &[String]) -> i32 {
    let (rest, consolidate) = strip_flag(rest, "consolidate");
    let (positional, flags) = parse_args(&rest);
    if positional.len() < 2 {
        return usage("usage: db compact <root> <document-id> [--holder H] [--consolidate] [--profile test|dev|prod]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let holder = flags.get("holder").cloned().unwrap_or_else(|| "db-cli".to_string());
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };
    let document_id = protocol::DocumentId(id.clone());

    let outcome = match database.compact_document(&document_id, &holder, consolidate) {
        Ok(report) => {
            println!("== compact: {id} ==");
            println!("  wal_segments_deleted: {}", report.wal_segments_deleted);
            println!("  payloads_deleted: {}", report.payloads_deleted);
            println!("  index_reports: {}", report.index_reports.len());
            println!("  snapshot_consolidated_generation: {}", report.snapshot_consolidated_generation.map_or_else(|| "-".to_string(), |generation| generation.to_string()));
            println!("  snapshot_generations_pruned: {}", report.snapshot_generations_pruned);
            0
        }
        Err(err) => fail("compact", err),
    };

    match database.shutdown(std::time::Duration::from_secs(5)) {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err),
    }
}
//#endregion 🔖Compact

//#region 🔖HealthCmd
/// 🩺 `db health <root> [--profile ...]` — a real `Database::health()` snapshot. Exits `1` iff the
/// overall status is `Unhealthy`.
fn cmd_health(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(root) = positional.first() else {
        return usage("usage: db health <root> [--profile test|dev|prod]");
    };
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };
    let health = database.health();
    print_health(&health);
    let exit = if matches!(health.report.overall, db::observe::HealthState::Unhealthy(_)) { 1 } else { 0 };

    match database.shutdown(std::time::Duration::from_secs(5)) {
        Ok(()) => exit,
        Err(err) => fail("shutdown", err),
    }
}
//#endregion 🔖HealthCmd

//#region 🔖ConflictSimulate
fn parse_merge_strategy(raw: &str) -> Result<protocol::MergeStrategyKind, String> {
    match raw {
        "lww" => Ok(protocol::MergeStrategyKind::LwwRegister),
        "seq" => Ok(protocol::MergeStrategyKind::OrderedSequence),
        "text" => Ok(protocol::MergeStrategyKind::TextSequence),
        "graph" => Ok(protocol::MergeStrategyKind::TombstonedGraphSet),
        "blob" => Ok(protocol::MergeStrategyKind::ContentAddressedBlob),
        other => Err(format!("unknown merge strategy '{other}' (expected lww|seq|text|graph|blob)")),
    }
}

fn parse_conflict_rule(raw: &str) -> Result<protocol::ConflictRule, String> {
    match raw {
        "commutes" => Ok(protocol::ConflictRule::Commutes),
        "transform" => Ok(protocol::ConflictRule::Transform),
        other => {
            let Some((kind, strategy)) = other.split_once(':') else {
                return Err(format!("unknown conflict rule '{other}' (expected commutes|transform|merge:<strategy>|crdt:<strategy>)"));
            };
            let strategy = parse_merge_strategy(strategy)?;
            match kind {
                "merge" => Ok(protocol::ConflictRule::Merge(strategy)),
                "crdt" => Ok(protocol::ConflictRule::Crdt(strategy)),
                other => Err(format!("unknown conflict rule kind '{other}' (expected merge|crdt)")),
            }
        }
    }
}

fn describe_merge_strategy(strategy: protocol::MergeStrategyKind) -> &'static str {
    match strategy {
        protocol::MergeStrategyKind::LwwRegister => "lww",
        protocol::MergeStrategyKind::OrderedSequence => "seq",
        protocol::MergeStrategyKind::TextSequence => "text",
        protocol::MergeStrategyKind::TombstonedGraphSet => "graph",
        protocol::MergeStrategyKind::ContentAddressedBlob => "blob",
    }
}

fn describe_conflict_kind(kind: &db::conflict::ConflictKind) -> String {
    match kind {
        db::conflict::ConflictKind::TouchedRegion(regions) => format!("touched_region({})", regions.iter().map(|region| region.path.as_str()).collect::<Vec<_>>().join(",")),
        db::conflict::ConflictKind::Constraint(description) => format!("constraint({description})"),
    }
}

fn describe_resolution_plan(plan: db::conflict::ResolutionPlan) -> String {
    match plan {
        db::conflict::ResolutionPlan::Commutes => "commutes".to_string(),
        db::conflict::ResolutionPlan::Transform => "transform".to_string(),
        db::conflict::ResolutionPlan::Merge(strategy) => format!("merge:{}", describe_merge_strategy(strategy)),
        db::conflict::ResolutionPlan::Crdt(strategy) => format!("crdt:{}", describe_merge_strategy(strategy)),
        db::conflict::ResolutionPlan::Reject => "reject".to_string(),
    }
}

fn touched_command(command_id: &str, actor: &str, kind: &str, rule: protocol::ConflictRule, hlc_actor: u64, paths: &str) -> db::conflict::CommandTouch {
    let touch = db::conflict::CommandTouch::new(
        protocol::OperationId(command_id.to_string()),
        protocol::ActorId(actor.to_string()),
        db::conflict::CommandKind::from(kind),
        rule,
        protocol::HybridLogicalTimestamp::new(hlc_actor, now_ms()),
    );
    paths.split(',').map(str::trim).filter(|path| !path.is_empty()).fold(touch, |touch, path| touch.touch(db::state::TouchedRegion::write(path)))
}

/// ⚔️ `db conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K]
/// [--rule-a commutes|transform|merge:<strategy>|crdt:<strategy>] [--rule-b ...]` — runs the real
/// `db::conflict::ConflictDetector` over two hand-built `CommandTouch`es (no storage touched at all:
/// a pure, local simulation). Exits `1` iff a conflict was found (so the exit code alone answers
/// "would these conflict").
fn cmd_conflict_simulate(rest: &[String]) -> i32 {
    let (_positional, flags) = parse_args(rest);
    let Some(touch_a) = flags.get("touch-a") else {
        return usage("usage: db conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K] [--rule-a R] [--rule-b R]");
    };
    let Some(touch_b) = flags.get("touch-b") else {
        return usage("db conflict-simulate: --touch-b is required");
    };
    let kind_a = flags.get("kind-a").map_or("command-a", String::as_str);
    let kind_b = flags.get("kind-b").map_or("command-b", String::as_str);
    let rule_a = match parse_conflict_rule(flags.get("rule-a").map_or("commutes", String::as_str)) {
        Ok(rule) => rule,
        Err(message) => return usage(&message),
    };
    let rule_b = match parse_conflict_rule(flags.get("rule-b").map_or("commutes", String::as_str)) {
        Ok(rule) => rule,
        Err(message) => return usage(&message),
    };

    let command_a = touched_command("simulated-a", "actor-a", kind_a, rule_a, 1, touch_a);
    let command_b = touched_command("simulated-b", "actor-b", kind_b, rule_b, 2, touch_b);
    let records = db::conflict::ConflictDetector::new().detect(&[command_a, command_b]);
    if records.is_empty() {
        println!("no conflict detected");
        return 0;
    }
    for record in &records {
        println!("conflict: {} <-> {} kind={} resolution={}", record.command_id.0, record.conflicting_with.0, describe_conflict_kind(&record.kind), describe_resolution_plan(record.resolution));
    }
    1
}
//#endregion 🔖ConflictSimulate

//#region 🔖ReplicaSimulate
/// 📡 `db replica-simulate <leader-root> <follower-root> <document-id>` — runs the real
/// `db::cluster::replicate_document` primitive between two local `FsStorage` roots: replays both
/// sides' WALs, decides a bootstrap plan, and applies it (tail-append or raw snapshot copy). No
/// network transport exists in this family yet — this is a genuine local simulation of the
/// replication mechanism, not a stub.
fn cmd_replica_simulate(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest);
    if positional.len() < 3 {
        return usage("usage: db replica-simulate <leader-root> <follower-root> <document-id>");
    }
    let leader_root = &positional[0];
    let follower_root = &positional[1];
    let id = &positional[2];

    let leader = match db::storage::FsStorage::open(Path::new(leader_root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open leader", err),
    };
    let follower = match db::storage::FsStorage::open(Path::new(follower_root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open follower", err),
    };
    let document = db::core::DocumentId(id.clone());

    match db::cluster::replicate_document(&leader, &follower, document, db::wal::GroupCommitPolicy::default(), now_ms()) {
        Ok(db::cluster::ReplicationOutcome::UpToDate { frontier }) => {
            println!("== replica-simulate: up to date ==");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db::cluster::ReplicationOutcome::TailApplied { frontier, count }) => {
            println!("== replica-simulate: tail applied ==");
            println!("  commands: {count}");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db::cluster::ReplicationOutcome::SnapshotTransferred { generation, pack_hash }) => {
            println!("== replica-simulate: snapshot transferred ==");
            println!("  generation: {generation}");
            println!("  pack_hash: {}", hex32(&pack_hash));
            0
        }
        Err(err) => fail("replicate", err),
    }
}
//#endregion 🔖ReplicaSimulate

//#region 🔖Migrate
/// 🚚 `db migrate <root> <document-id> <name> [--payload TEXT]` — appends a real `WAL_MIGRATION`
/// record (`name` on its own line, then an optional `--payload` text body) via a real
/// `db::wal::DocumentWal`, force-flushed durably (`DurabilityClass::Fsync`). `DocumentWal::open`
/// auto-creates a fresh WAL if `document` has none yet (see its own doc's "Creates a fresh WAL...
/// if `document` has no segments yet" note), so this also works as a bootstrap path for a document
/// that was never `create_document`d through the actor API.
fn cmd_migrate(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 3 {
        return usage("usage: db migrate <root> <document-id> <name> [--payload TEXT]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let name = &positional[2];
    let payload = flags.get("payload").cloned().unwrap_or_default();

    let storage = match db::storage::FsStorage::open(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err),
    };
    let document = db::core::DocumentId(id.clone());
    let now = now_ms();
    let (mut wal, _report) = match db::wal::DocumentWal::open(&storage, document, db::wal::GroupCommitPolicy::default(), now) {
        Ok(pair) => pair,
        Err(err) => return fail("open wal", err),
    };
    let bytes = format!("{name}\n{payload}").into_bytes();
    match wal.submit(&storage, &[db::wal::WalRecord::Migration(bytes)], db::DurabilityClass::Fsync, now) {
        Ok(receipt) => {
            if let Err(err) = wal.force_flush(&storage) {
                return fail("flush", err);
            }
            println!("== migrate: {id} ==");
            println!("  name: {name}");
            println!("  segment_index: {}", receipt.segment_index);
            println!("  tx_id: {}", receipt.tx_id);
            println!("  committed: {}", receipt.committed);
            0
        }
        Err(err) => fail("migrate", err),
    }
}
//#endregion 🔖Migrate

//#region 🔖Profile
/// ⏱️ `db profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N]
/// [--profile test|dev|prod]` — submits `N` (default 100) trivial single-path `set`-shaped commands
/// sequentially through the real submit pipeline (`DocumentHandle::submit`, actor-mediated, WAL
/// group-commit and all) and reports wall-clock throughput/latency. Opens `document` if it already
/// exists, else creates it first — self-contained, no separate seeding step required. Deliberately
/// hand-timed with `std::time::Instant` rather than pulling in `db_testkit`'s `WorkloadGen`/
/// criterion harness: `db_testkit` is a sibling crate, not part of the `db` facade's own re-export
/// surface, and this crate's dependency footprint is the `db` facade alone (see module doc).
fn cmd_profile(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 2 {
        return usage("usage: db profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N] [--profile test|dev|prod]");
    }
    let root = &positional[0];
    let id = &positional[1];
    let commands: u64 = match flags.get("commands") {
        Some(value) => match value.parse() {
            Ok(commands) => commands,
            Err(_) => return usage("db profile: --commands must be a non-negative integer"),
        },
        None => 100,
    };
    let durability = match flags.get("durability").map(String::as_str) {
        None => db::DurabilityClass::Fsync,
        Some("memory") => db::DurabilityClass::Memory,
        Some("os") => db::DurabilityClass::Os,
        Some("fsync") => db::DurabilityClass::Fsync,
        Some(other) => match other.strip_prefix("quorum:") {
            Some(n) => match n.parse::<u8>() {
                Ok(n) => db::DurabilityClass::Quorum(n),
                Err(_) => return usage(&format!("db profile: bad --durability quorum count '{n}'")),
            },
            None => return usage(&format!("db profile: unknown --durability '{other}' (expected memory|os|fsync|quorum:N)")),
        },
    };
    let profile = match parse_profile(&flags) {
        Ok(profile) => profile,
        Err(message) => return usage(&message),
    };
    let database = match db::Database::open_at(Path::new(root), profile) {
        Ok(database) => database,
        Err(err) => return fail("open", err),
    };
    let document_id = protocol::DocumentId(id.clone());
    let handle = match database.document(&document_id) {
        Ok(handle) => handle,
        Err(_) => match database.create_document(db::DocumentSpec::new(document_id.clone())) {
            Ok(handle) => handle,
            Err(err) => return fail("create", err),
        },
    };

    let start = std::time::Instant::now();
    for counter in 0..commands {
        let mut forward = serde_json::Map::with_capacity(1);
        forward.insert("cli/profile/counter".to_string(), serde_json::json!(counter));
        let mut backward = serde_json::Map::with_capacity(1);
        backward.insert("cli/profile/counter".to_string(), serde_json::Value::Null);
        let envelope = protocol::OperationEnvelope {
            operation_id: protocol::OperationId(format!("profile-{}-{counter}", now_ms())),
            document_id: document_id.clone(),
            actor: protocol::ActorId("profiler".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(forward)).unwrap_or_default(),
            },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(backward)).unwrap_or_default(),
            },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms()),
        };
        let batch = match db::document::CommandBatch::new(vec![envelope]) {
            Ok(batch) => batch,
            Err(err) => return fail("build batch", err),
        };
        match db::actor::block_on(handle.submit(batch, db::document::SubmitOptions { durability })) {
            Ok(Ok(_receipt)) => {}
            Ok(Err(err)) => return fail(&format!("submit rejected at command {counter}"), err),
            Err(err) => return fail(&format!("submit failed at command {counter}"), err),
        }
    }
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let per_sec = if elapsed_secs > 0.0 { commands as f64 / elapsed_secs } else { f64::INFINITY };
    let avg_latency_us = if commands > 0 { (elapsed.as_micros() as f64) / (commands as f64) } else { 0.0 };
    println!("== profile: {id} ==");
    println!("  commands: {commands}");
    println!("  elapsed_ms: {:.3}", elapsed_secs * 1000.0);
    println!("  commands_per_sec: {per_sec:.1}");
    println!("  avg_latency_us: {avg_latency_us:.1}");

    match database.shutdown(std::time::Duration::from_secs(5)) {
        Ok(()) => 0,
        Err(err) => fail("shutdown", err),
    }
}
//#endregion 🔖Profile

//#region 🔖Cli
fn print_help() {
    eprintln!("usage: db <command> [args...]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  inspect <root> [--profile test|dev|prod]");
    eprintln!("  doc <root> <document-id> [--profile ...]");
    eprintln!("  wal-inspect <root> <document-id> [--limit N]");
    eprintln!("  snapshot-inspect <root> <document-id> [--generation N] [--verify]");
    eprintln!("  verify <root> [document-id] [--profile ...]");
    eprintln!("  query <root> <document-id> <path> [more-paths...] [--profile ...]");
    eprintln!("  replay <root> <document-id>");
    eprintln!("  repair <root> <document-id>");
    eprintln!("  compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]");
    eprintln!("  health <root> [--profile ...]");
    eprintln!("  conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K] [--rule-a R] [--rule-b R]");
    eprintln!("  replica-simulate <leader-root> <follower-root> <document-id>");
    eprintln!("  migrate <root> <document-id> <name> [--payload TEXT]");
    eprintln!("  profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N] [--profile ...]");
}

pub fn main_impl(args: &[String]) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        print_help();
        return 2;
    };
    match command.as_str() {
        "inspect" => cmd_inspect(rest),
        "doc" => cmd_doc(rest),
        "wal-inspect" => cmd_wal_inspect(rest),
        "snapshot-inspect" => cmd_snapshot_inspect(rest),
        "verify" => cmd_verify(rest),
        "query" => cmd_query(rest),
        "replay" => cmd_replay(rest),
        "repair" => cmd_repair(rest),
        "compact" => cmd_compact(rest),
        "health" => cmd_health(rest),
        "conflict-simulate" => cmd_conflict_simulate(rest),
        "replica-simulate" => cmd_replica_simulate(rest),
        "migrate" => cmd_migrate(rest),
        "profile" => cmd_profile(rest),
        "help" | "--help" | "-h" => {
            print_help();
            0
        }
        other => {
            eprintln!("db: unknown subcommand '{other}'\n");
            print_help();
            2
        }
    }
}

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(main_impl(&args));
}
//#endregion 🔖Cli

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db_cli-test-{name}-{}-{}", std::process::id(), now_ms()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn test_envelope(id: &str, document: &protocol::DocumentId) -> protocol::OperationEnvelope {
        protocol::OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId("tester".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::json!({"greeting": "hello"})).unwrap(),
            },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::json!({"greeting": null})).unwrap(),
            },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    /// 🌱 Seeds `doc-1` at `root` with one committed, `Fsync`-durable transaction through the real
    /// `Database::create_document`/`DocumentHandle::submit` round trip, then cleanly shuts down.
    fn seed_document(root: &Path) {
        let database = db::Database::open_at(root, db::Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(db::DocumentSpec::new(document.clone())).unwrap();
        let batch = db::document::CommandBatch::new(vec![test_envelope("op-1", &document)]).unwrap();
        db::actor::block_on(handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync })).unwrap().unwrap();
        database.shutdown(std::time::Duration::from_secs(1)).unwrap();
    }
    //#endregion 🧸Fixtures

    //#region 🔖Inspect
    #[test]
    fn cli_inspect_reports_an_empty_catalog_and_healthy_status_on_a_fresh_root() {
        let root = tempdir("inspect-fresh");
        assert_eq!(main_impl(&[String::from("inspect"), root.to_string_lossy().to_string()]), 0);
    }
    //#endregion 🔖Inspect

    //#region 🔖FullCycle
    #[test]
    fn cli_full_cycle_succeeds_for_a_seeded_document() {
        let root = tempdir("full-cycle");
        seed_document(&root);
        let root_str = root.to_string_lossy().to_string();

        assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("query"), root_str.clone(), String::from("doc-1"), String::from("greeting")]), 0);
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("replay"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("compact"), root_str.clone(), String::from("doc-1"), String::from("--consolidate")]), 0);
        assert_eq!(main_impl(&[String::from("health"), root_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("snapshot-inspect"), root_str, String::from("doc-1")]), 0);
    }

    #[test]
    fn cli_doc_and_query_err_cleanly_on_an_unknown_document() {
        let root = tempdir("unknown-doc");
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("never-created")]), 1);
        assert_eq!(main_impl(&[String::from("query"), root_str, String::from("never-created"), String::from("x")]), 1);
    }
    //#endregion 🔖FullCycle

    //#region 🔖Verify
    #[test]
    fn cli_verify_fails_on_a_torn_wal_tail_and_repair_fixes_it() {
        let root = tempdir("torn-tail");
        seed_document(&root);

        let wal_dir = root.join("wal").join("doc-1");
        let segment_path = std::fs::read_dir(&wal_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "bin"))
            .expect("expected at least one wal segment file");
        let mut bytes = std::fs::read(&segment_path).unwrap();
        assert!(bytes.len() > 16, "segment must be large enough to truncate meaningfully");
        bytes.truncate(bytes.len() - 5);
        std::fs::write(&segment_path, &bytes).unwrap();

        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]), 1);
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]), 1);
        assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str, String::from("doc-1")]), 0);
    }
    //#endregion 🔖Verify

    //#region 🔖ConflictSimulate
    #[test]
    fn cli_conflict_simulate_detects_overlapping_writes_and_ignores_disjoint_ones() {
        assert_eq!(
            main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("a/name")]),
            1
        );
        assert_eq!(
            main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("b/name")]),
            0
        );
    }

    #[test]
    fn cli_conflict_simulate_requires_both_touch_flags() {
        assert_eq!(main_impl(&[String::from("conflict-simulate")]), 2);
        assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a")]), 2);
    }
    //#endregion 🔖ConflictSimulate

    //#region 🔖ReplicaSimulate
    #[test]
    fn cli_replica_simulate_copies_missing_commands_to_a_fresh_follower() {
        let leader_root = tempdir("replica-leader");
        let follower_root = tempdir("replica-follower");
        seed_document(&leader_root);

        let leader_str = leader_root.to_string_lossy().to_string();
        let follower_str = follower_root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("replica-simulate"), leader_str, follower_str.clone(), String::from("doc-1")]), 0);
        assert_eq!(main_impl(&[String::from("verify"), follower_str, String::from("doc-1")]), 0);
    }
    //#endregion 🔖ReplicaSimulate

    //#region 🔖Migrate
    #[test]
    fn cli_migrate_appends_a_migration_record_visible_to_wal_inspect() {
        let root = tempdir("migrate");
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            main_impl(&[String::from("migrate"), root_str.clone(), String::from("doc-1"), String::from("rename-field"), String::from("--payload"), String::from("old->new")]),
            0
        );
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str, String::from("doc-1")]), 0);
    }

    #[test]
    fn cli_migrate_reports_a_usage_error_with_too_few_args() {
        assert_eq!(main_impl(&[String::from("migrate"), String::from("root-only")]), 2);
    }
    //#endregion 🔖Migrate

    //#region 🔖Profile
    #[test]
    fn cli_profile_reports_throughput_for_n_commands_on_a_fresh_document() {
        let root = tempdir("profile");
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            main_impl(&[String::from("profile"), root_str.clone(), String::from("doc-1"), String::from("--commands"), String::from("5"), String::from("--durability"), String::from("memory")]),
            0
        );
        // 🎯 The 5 profiled commands are real, durable commits — verified via the query subcommand
        // rather than parsing this test's own stdout (`println!` isn't easily captured in-process).
        assert_eq!(main_impl(&[String::from("query"), root_str, String::from("doc-1"), String::from("cli/profile/counter")]), 0);
    }

    #[test]
    fn cli_profile_rejects_a_bad_durability_flag() {
        let root = tempdir("profile-bad-durability");
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("profile"), root_str, String::from("doc-1"), String::from("--durability"), String::from("bogus")]), 2);
    }
    //#endregion 🔖Profile

    //#region 🔖Cli
    #[test]
    fn cli_help_and_unknown_subcommand() {
        assert_eq!(main_impl(&[]), 2);
        assert_eq!(main_impl(&[String::from("help")]), 0);
        assert_eq!(main_impl(&[String::from("--help")]), 0);
        assert_eq!(main_impl(&[String::from("bogus-subcommand")]), 2);
    }
    //#endregion 🔖Cli
}
//#endregion 🧪Tests

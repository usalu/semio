//! 📦 `pack_cli` — the `pack` binary: `inspect`/`verify`/`hash`/`to-dsl`/`from-dsl`/`diff` over
//! `.spk` pack files. `inspect`/`verify`/`hash` are schema-less (self-describing decode via
//! `pack_format`'s manifest/segment framing); `to-dsl`/`from-dsl`/`diff --schema` resolve a
//! `dsl_schema::RecordSpec` against a tiny built-in registry (see `//#region 🔖Registry`) — this
//! crate must not depend on any app crate or on `dsl_derive`, so full schema resolution across
//! the 49 app document kinds is explicitly out of scope here; see `pack help`.
//!
//! See the `## pack_cli` section of the wave-0 contract at
//! `.repo/🎫/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`.

use std::collections::HashMap;
use std::path::Path;

//#region 🔖Registry
/// @emoji 🧬 Demonstration-only schema: no keyword, three keyed scalar fields — mirrors `pack`
/// facade's own `sample_spec` test fixture so a pack file built by any wave-0 crate's tests
/// round-trips through this CLI unmodified.
fn sample_spec() -> dsl_schema::RecordSpec {
    dsl_schema::RecordSpec::new(
        None,
        dsl_schema::RecordLayout::Lines,
        vec![
            dsl_schema::FieldSpec::new(1, "name", dsl_schema::Shape::Text),
            dsl_schema::FieldSpec::new(2, "age", dsl_schema::Shape::UInt),
            dsl_schema::FieldSpec::new(3, "active", dsl_schema::Shape::Bool),
        ],
    )
}

/// @emoji 🧬 Demonstration-only schema exercising a keyword and a `List` shape, distinct from
/// `sample_spec` — e.g. `note title="Todo" body="write the CLI" tags=[ "wave0" "pack" ]`.
fn note_spec() -> dsl_schema::RecordSpec {
    dsl_schema::RecordSpec::new(
        Some("note"),
        dsl_schema::RecordLayout::Lines,
        vec![
            dsl_schema::FieldSpec::new(1, "title", dsl_schema::Shape::Text),
            dsl_schema::FieldSpec::new(2, "body", dsl_schema::Shape::Text).optional(),
            dsl_schema::FieldSpec::new(3, "tags", dsl_schema::Shape::List(Box::new(dsl_schema::Shape::Text))),
        ],
    )
}

/// @emoji 📇 The built-in `--schema <name>` registry. `TODO(wave2)`: app crates own the real
/// 49-kind registry; this stays a fixed 2-entry demonstration table forever in `pack_cli`.
fn schema_registry() -> HashMap<&'static str, fn() -> dsl_schema::RecordSpec> {
    let mut registry: HashMap<&'static str, fn() -> dsl_schema::RecordSpec> = HashMap::new();
    registry.insert("sample", sample_spec);
    registry.insert("note", note_spec);
    registry
}

fn registry_names() -> String {
    let mut names: Vec<&'static str> = schema_registry().keys().copied().collect();
    names.sort_unstable();
    names.join(", ")
}

fn resolve_schema(name: &str) -> Option<dsl_schema::RecordSpec> {
    schema_registry().get(name).map(|spec_fn| spec_fn())
}
//#endregion 🔖Registry

//#region 🔖Args
/// @emoji ✂️ Splits argv-style slices into positionals and `--flag value` / `--flag=value`
/// pairs; a trailing bare `--flag` with nothing after it maps to an empty-string value.
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

fn parse_level(flags: &HashMap<String, String>) -> Result<pack::VerificationLevel, String> {
    match flags.get("level").map(String::as_str) {
        None => Ok(pack::VerificationLevel::Standard),
        Some("trusted") => Ok(pack::VerificationLevel::Trusted),
        Some("standard") => Ok(pack::VerificationLevel::Standard),
        Some("full") => Ok(pack::VerificationLevel::Full),
        Some(other) => Err(format!("unknown --level '{other}' (expected trusted|standard|full)")),
    }
}
//#endregion 🔖Args

//#region 🔖Format
fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn print_manifest(manifest: &pack::Manifest, footer: &pack::Footer) {
    println!("== manifest ==");
    println!("  schema_name: {:?}", manifest.schema_name);
    println!("  schema_hash: {}", hex32(&manifest.schema_hash));
    println!("  doc_span: offset={} len={} frames={}", manifest.doc_span.offset, manifest.doc_span.len, manifest.doc_frame_count);
    println!("  symbols_span: offset={} len={}", manifest.symbols_span.offset, manifest.symbols_span.len);
    println!("  chunk_table_span: offset={} len={}", manifest.chunk_table_span.offset, manifest.chunk_table_span.len);
    println!("  field_index_span: offset={} len={}", manifest.field_index_span.offset, manifest.field_index_span.len);
    println!("  uncompressed_body_len: {}", manifest.uncompressed_body_len);
    println!("  field_count: {}", manifest.field_count);
    println!("  chunk_count: {}", manifest.chunk_count);
    println!("  symbol_count: {}", manifest.symbol_count);
    println!("== segments (from manifest spans) ==");
    println!("  manifest: offset={} len={}", footer.manifest_offset, footer.manifest_len);
    if manifest.symbols_span.len > 0 {
        println!("  symbols: offset={} len={}", manifest.symbols_span.offset, manifest.symbols_span.len);
    }
    if manifest.chunk_table_span.len > 0 {
        println!("  chunk_table: offset={} len={}", manifest.chunk_table_span.offset, manifest.chunk_table_span.len);
    }
    if manifest.field_index_span.len > 0 {
        println!("  field_index: offset={} len={}", manifest.field_index_span.offset, manifest.field_index_span.len);
    }
    if manifest.doc_span.len > 0 {
        println!("  document: offset={} len={} frames={}", manifest.doc_span.offset, manifest.doc_span.len, manifest.doc_frame_count);
    }
}
//#endregion 🔖Format

//#region 🔖Inspect
/// @emoji 🔍 `pack inspect <file>` — prints header/footer/manifest/segment-span text; never
/// panics on corrupt input, degrading to a forward-scan recovery summary if the manifest fails
/// to load.
fn cmd_inspect(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest);
    let Some(path) = positional.first() else {
        eprintln!("usage: pack inspect <file>");
        return 2;
    };
    let source = match pack::FilePackSource::open(Path::new(path)) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("pack: cannot open '{path}': {error}");
            return 1;
        }
    };
    let limits = pack::PackLimits::default();
    let recovery = pack::recover(&source, &limits);
    match pack::PackFile::open_manifest(source, &limits, pack::VerificationLevel::Standard) {
        Ok(pack_file) => {
            let superblock = pack_file.superblock();
            println!("== header ==");
            println!("  version: {}.{}", superblock.header.version_major, superblock.header.version_minor);
            println!("  required_flags: {:#010x}", superblock.header.required_flags);
            println!("  optional_flags: {:#010x}", superblock.header.optional_flags);
            println!("== footer ==");
            println!("  file_len: {}", superblock.footer.file_len);
            println!("  content_hash: {}", superblock.footer.content_hash);
            println!("  prev_footer_offset: {}", superblock.footer.prev_footer_offset);
            if let Some(manifest) = pack_file.manifest() {
                print_manifest(manifest, &superblock.footer);
            }
            for id in 0..pack_file.chunk_count() {
                if let Ok(range) = pack_file.chunk_range(pack::ChunkId(id as u32)) {
                    println!("  chunk[{id}]: offset={} stored_len={}", range.offset, range.len);
                }
            }
            if let Ok(report) = &recovery {
                println!("== forward scan ==");
                println!("  segments_recovered: {}", report.segments_recovered);
                println!("  bytes_recovered: {}", report.bytes_recovered);
            }
            0
        }
        Err(error) => {
            eprintln!("pack: failed to open manifest: {error}");
            match recovery {
                Ok(report) => {
                    println!("== forward scan (recovery) ==");
                    println!("  segments_recovered: {}", report.segments_recovered);
                    println!("  bytes_recovered: {}", report.bytes_recovered);
                    println!("  manifest_recovered: {}", report.manifest.is_some());
                    1
                }
                Err(recover_error) => {
                    eprintln!("pack: recovery scan also failed: {recover_error}");
                    1
                }
            }
        }
    }
}
//#endregion 🔖Inspect

//#region 🔖Verify
/// @emoji 🛡️ `pack verify <file> [--level=trusted|standard|full]` — opens the manifest, reads
/// the document body, and reads every chunk at the requested `VerificationLevel`; prints `OK`/
/// `FAIL: <reason>` and never panics on corrupt input.
fn cmd_verify(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(path) = positional.first() else {
        eprintln!("usage: pack verify <file> [--level=trusted|standard|full]");
        return 2;
    };
    let level = match parse_level(&flags) {
        Ok(level) => level,
        Err(error) => {
            eprintln!("pack: {error}");
            return 2;
        }
    };
    let source = match pack::FilePackSource::open(Path::new(path)) {
        Ok(source) => source,
        Err(error) => {
            println!("FAIL: cannot open '{path}': {error}");
            return 1;
        }
    };
    let limits = pack::PackLimits::default();
    let pack_file = match pack::PackFile::open_manifest(source, &limits, level) {
        Ok(pack_file) => pack_file,
        Err(error) => {
            println!("FAIL: {error}");
            return 1;
        }
    };
    if let Err(error) = pack_file.body_bytes(level) {
        println!("FAIL: {error}");
        return 1;
    }
    for id in 0..pack_file.chunk_count() {
        if let Err(error) = pack_file.read_chunk(pack::ChunkId(id as u32), level) {
            println!("FAIL: chunk {id}: {error}");
            return 1;
        }
    }
    println!("OK");
    0
}
//#endregion 🔖Verify

//#region 🔖Hash
/// @emoji #️⃣ `pack hash <file>` — prints the footer's `content_hash` hex, reading only the
/// trailing footer bytes via `pack::content_hash`.
fn cmd_hash(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest);
    let Some(path) = positional.first() else {
        eprintln!("usage: pack hash <file>");
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("pack: cannot read '{path}': {error}");
            return 1;
        }
    };
    match pack::content_hash(&bytes) {
        Ok(hash) => {
            println!("{hash}");
            0
        }
        Err(error) => {
            eprintln!("pack: {error}");
            1
        }
    }
}
//#endregion 🔖Hash

//#region 🔖ToDsl
/// @emoji 📤 `pack to-dsl <file> --schema <name>` — decodes against a registry spec and prints
/// canonical `Document`-mode DSL text to stdout.
fn cmd_to_dsl(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(path) = positional.first() else {
        eprintln!("usage: pack to-dsl <file> --schema <name>");
        return 2;
    };
    let Some(schema_name) = flags.get("schema") else {
        eprintln!("pack: to-dsl requires --schema <name> (registry: {})", registry_names());
        return 2;
    };
    let Some(spec) = resolve_schema(schema_name) else {
        eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("pack: cannot read '{path}': {error}");
            return 1;
        }
    };
    match pack::decode_document(&bytes, &spec, &pack::DecodeOptions::default()) {
        Ok((record, report)) => {
            let mut writer = dsl_schema::Writer::new();
            dsl_schema::print_record(&record, &spec, &mut writer);
            print!("{}", writer.render(dsl_schema::JoinMode::Document));
            if !report.unknown_field_ids.is_empty() {
                eprintln!("note: unknown field ids not in schema '{schema_name}': {:?}", report.unknown_field_ids);
            }
            if report.schema_drift {
                eprintln!("note: schema_hash drift between file and registry spec '{schema_name}'");
            }
            0
        }
        Err(error) => {
            eprintln!("pack: decode failed: {error}");
            1
        }
    }
}
//#endregion 🔖ToDsl

//#region 🔖FromDsl
/// @emoji 📥 `pack from-dsl <file> --schema <name> --out <file>` — parses `<file>`'s DSL text
/// against a registry spec and encodes+writes the resulting pack file atomically to `--out`.
fn cmd_from_dsl(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    let Some(path) = positional.first() else {
        eprintln!("usage: pack from-dsl <file> --schema <name> --out <file>");
        return 2;
    };
    let Some(schema_name) = flags.get("schema") else {
        eprintln!("pack: from-dsl requires --schema <name> (registry: {})", registry_names());
        return 2;
    };
    let Some(out_path) = flags.get("out") else {
        eprintln!("pack: from-dsl requires --out <file>");
        return 2;
    };
    let Some(spec) = resolve_schema(schema_name) else {
        eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());
        return 2;
    };
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("pack: cannot read '{path}': {error}");
            return 1;
        }
    };
    let record = match dsl_schema::parse(&text, &spec, &dsl_schema::ParseOptions::default()) {
        Ok(record) => record,
        Err(error) => {
            eprintln!("pack: dsl parse failed: {error}");
            return 1;
        }
    };
    let bytes = match pack::encode_document(&spec, &record, &pack::EncodeOptions::default()) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("pack: encode failed: {error}");
            return 1;
        }
    };
    match pack::write_atomic(Path::new(out_path), &bytes) {
        Ok(()) => {
            println!("wrote {out_path} ({} bytes)", bytes.len());
            0
        }
        Err(error) => {
            eprintln!("pack: write failed: {error}");
            1
        }
    }
}
//#endregion 🔖FromDsl

//#region 🔖Diff
/// @emoji 🌳 Field-by-field diff of two decoded records; `+`/`-`/`~` prefix additions,
/// removals, and changes, keyed by field id ascending.
fn diff_records(a: &dsl_schema::RecordValue, b: &dsl_schema::RecordValue) -> Vec<String> {
    let mut ids: Vec<u16> = a.fields.keys().chain(b.fields.keys()).copied().collect();
    ids.sort_unstable();
    ids.dedup();
    let mut lines = Vec::new();
    for id in ids {
        match (a.fields.get(&id), b.fields.get(&id)) {
            (Some(av), Some(bv)) if av == bv => {}
            (Some(av), Some(bv)) => lines.push(format!("~ field {id}: {av:?} -> {bv:?}")),
            (Some(av), None) => lines.push(format!("- field {id}: {av:?}")),
            (None, Some(bv)) => lines.push(format!("+ field {id}: {bv:?}")),
            (None, None) => {}
        }
    }
    lines
}

/// @emoji 🌗 `pack diff <file-a> <file-b> [--schema <name>]` — structural `RecordValue` diff
/// when `--schema` resolves, else a raw content-hash/length/first-mismatch summary. Exit code
/// `0` when identical, `1` when they differ, `2` on a usage/resolution error.
fn cmd_diff(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest);
    if positional.len() < 2 {
        eprintln!("usage: pack diff <file-a> <file-b> [--schema <name>]");
        return 2;
    }
    let path_a = &positional[0];
    let path_b = &positional[1];
    let bytes_a = match std::fs::read(path_a) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("pack: cannot read '{path_a}': {error}");
            return 1;
        }
    };
    let bytes_b = match std::fs::read(path_b) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("pack: cannot read '{path_b}': {error}");
            return 1;
        }
    };

    if let Some(schema_name) = flags.get("schema") {
        let Some(spec) = resolve_schema(schema_name) else {
            eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());
            return 2;
        };
        let options = pack::DecodeOptions::default();
        let record_a = match pack::decode_document(&bytes_a, &spec, &options) {
            Ok((record, _)) => record,
            Err(error) => {
                eprintln!("pack: decode '{path_a}' failed: {error}");
                return 1;
            }
        };
        let record_b = match pack::decode_document(&bytes_b, &spec, &options) {
            Ok((record, _)) => record,
            Err(error) => {
                eprintln!("pack: decode '{path_b}' failed: {error}");
                return 1;
            }
        };
        let diffs = diff_records(&record_a, &record_b);
        if diffs.is_empty() {
            println!("identical");
            0
        } else {
            for line in &diffs {
                println!("{line}");
            }
            1
        }
    } else {
        let hash_a = pack::content_hash(&bytes_a);
        let hash_b = pack::content_hash(&bytes_b);
        if let (Ok(hash_a), Ok(hash_b)) = (hash_a, hash_b) {
            if hash_a == hash_b && bytes_a.len() == bytes_b.len() {
                println!("identical (content_hash {hash_a}, {} bytes)", bytes_a.len());
                return 0;
            }
            println!("content_hash: {hash_a} vs {hash_b}");
        } else {
            println!("content_hash: unavailable (footer failed to parse on at least one side)");
        }
        println!("file_len: {} vs {}", bytes_a.len(), bytes_b.len());
        match bytes_a.iter().zip(bytes_b.iter()).position(|(x, y)| x != y) {
            Some(offset) => println!("first differing byte at offset {offset}"),
            None => println!("shorter file is a byte-for-byte prefix of the longer one"),
        }
        1
    }
}
//#endregion 🔖Diff

//#region 🔖Cli
fn print_help() {
    println!("pack — inspect/verify/hash/convert .spk binary document pack files\n");
    println!("USAGE:");
    println!("  pack inspect <file>");
    println!("  pack verify <file> [--level=trusted|standard|full]");
    println!("  pack hash <file>");
    println!("  pack to-dsl <file> --schema <name>");
    println!("  pack from-dsl <file> --schema <name> --out <file>");
    println!("  pack diff <file-a> <file-b> [--schema <name>]\n");
    println!("SCHEMA REGISTRY (wave 0 scope):");
    println!("  to-dsl/from-dsl/diff --schema resolve against a tiny built-in registry ({}) defined", registry_names());
    println!("  locally in this crate for demonstration only. Full schema resolution across the 49 app");
    println!("  document kinds is out of scope for pack_cli — that wiring belongs to the app crates in");
    println!("  wave 2. inspect/verify/hash never need a schema (self-describing decode).");
}

/// @emoji 🚪 The CLI's single testable entry point — `main` is a thin `std::process::exit`
/// wrapper around this. Never panics on malformed input; every subcommand handler maps errors
/// to a printed message and a non-zero exit code instead.
pub fn main_impl(args: &[String]) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        print_help();
        return 2;
    };
    match command.as_str() {
        "inspect" => cmd_inspect(rest),
        "verify" => cmd_verify(rest),
        "hash" => cmd_hash(rest),
        "to-dsl" => cmd_to_dsl(rest),
        "from-dsl" => cmd_from_dsl(rest),
        "diff" => cmd_diff(rest),
        "help" | "--help" | "-h" => {
            print_help();
            0
        }
        other => {
            eprintln!("pack: unknown subcommand '{other}'\n");
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
    use std::sync::atomic::{AtomicU64, Ordering};

    //#region 🔖Fixtures
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_path(name: &str) -> std::path::PathBuf {
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("pack_cli_test_{}_{counter}_{name}", std::process::id()))
    }

    fn sample_record(name: &str, age: u64, active: bool) -> dsl_schema::RecordValue {
        let mut fields = HashMap::new();
        fields.insert(1, dsl_schema::FieldValue::Text(name.to_string()));
        fields.insert(2, dsl_schema::FieldValue::UInt(age));
        fields.insert(3, dsl_schema::FieldValue::Bool(active));
        dsl_schema::RecordValue { fields }
    }

    fn sample_pack_bytes(name: &str, age: u64, active: bool) -> Vec<u8> {
        let spec = sample_spec();
        let record = sample_record(name, age, active);
        pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).unwrap()
    }
    //#endregion 🔖Fixtures

    //#region 🔖Inspect
    #[test]
    fn cli_inspect_verify_hash_on_valid_pack() {
        let bytes = sample_pack_bytes("Ada Lovelace", 42, true);
        let path = temp_path("valid.spk");
        std::fs::write(&path, &bytes).unwrap();
        let path_str = path.to_string_lossy().to_string();

        assert_eq!(main_impl(&[String::from("inspect"), path_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("verify"), path_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("verify"), path_str.clone(), String::from("--level=full")]), 0);
        assert_eq!(main_impl(&[String::from("hash"), path_str]), 0);

        std::fs::remove_file(&path).ok();
    }
    //#endregion 🔖Inspect

    //#region 🔖Corrupt
    #[test]
    fn cli_verify_fails_on_corrupted_pack_without_panicking() {
        let mut bytes = sample_pack_bytes("Grace Hopper", 85, false);
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0xFF;
        let path = temp_path("corrupt.spk");
        std::fs::write(&path, &bytes).unwrap();
        let path_str = path.to_string_lossy().to_string();

        assert_ne!(main_impl(&[String::from("verify"), path_str.clone(), String::from("--level=full")]), 0);
        let inspect_code = main_impl(&[String::from("inspect"), path_str.clone()]);
        assert!(inspect_code == 0 || inspect_code == 1);
        let hash_code = main_impl(&[String::from("hash"), path_str]);
        assert!(hash_code == 0 || hash_code == 1);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn cli_handles_truncated_pack_without_panicking() {
        let bytes = sample_pack_bytes("Alan Turing", 41, true);
        let truncated = &bytes[..bytes.len() / 2];
        let path = temp_path("truncated.spk");
        std::fs::write(&path, truncated).unwrap();
        let path_str = path.to_string_lossy().to_string();

        assert_eq!(main_impl(&[String::from("verify"), path_str.clone()]), 1);
        assert_eq!(main_impl(&[String::from("inspect"), path_str.clone()]), 1);
        assert_eq!(main_impl(&[String::from("hash"), path_str]), 1);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn cli_reports_missing_file_without_panicking() {
        let missing = temp_path("does-not-exist.spk").to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("inspect"), missing.clone()]), 1);
        assert_eq!(main_impl(&[String::from("verify"), missing.clone()]), 1);
        assert_eq!(main_impl(&[String::from("hash"), missing]), 1);
    }
    //#endregion 🔖Corrupt

    //#region 🔖Dsl
    #[test]
    fn cli_to_dsl_and_from_dsl_round_trip_via_registry() {
        let bytes = sample_pack_bytes("Ada Lovelace", 42, true);
        let path = temp_path("roundtrip.spk");
        std::fs::write(&path, &bytes).unwrap();
        let path_str = path.to_string_lossy().to_string();

        assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone(), String::from("--schema"), String::from("sample")]), 0);
        assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone(), String::from("--schema"), String::from("bogus")]), 2);
        assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone()]), 2);

        let dsl_path = temp_path("roundtrip.dsl");
        let spec = sample_spec();
        let record = sample_record("Grace Hopper", 7, false);
        let mut writer = dsl_schema::Writer::new();
        dsl_schema::print_record(&record, &spec, &mut writer);
        std::fs::write(&dsl_path, writer.render(dsl_schema::JoinMode::Document)).unwrap();
        let dsl_path_str = dsl_path.to_string_lossy().to_string();

        let out_path = temp_path("fromdsl.spk");
        let out_path_str = out_path.to_string_lossy().to_string();
        assert_eq!(
            main_impl(&[
                String::from("from-dsl"),
                dsl_path_str,
                String::from("--schema"),
                String::from("sample"),
                String::from("--out"),
                out_path_str.clone(),
            ]),
            0
        );
        assert!(out_path.exists());
        assert_eq!(main_impl(&[String::from("verify"), out_path_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("diff"), path_str.clone(), out_path_str.clone(), String::from("--schema"), String::from("sample")]), 1);
        assert_eq!(main_impl(&[String::from("diff"), path_str.clone(), path_str.clone()]), 0);

        std::fs::remove_file(&path).ok();
        std::fs::remove_file(&out_path).ok();
    }

    #[test]
    fn cli_from_dsl_reports_parse_failure_without_panicking() {
        let bad_dsl_path = temp_path("bad.dsl");
        std::fs::write(&bad_dsl_path, "name=").unwrap();
        let out_path = temp_path("bad-out.spk");
        assert_eq!(
            main_impl(&[
                String::from("from-dsl"),
                bad_dsl_path.to_string_lossy().to_string(),
                String::from("--schema"),
                String::from("sample"),
                String::from("--out"),
                out_path.to_string_lossy().to_string(),
            ]),
            1
        );
        assert!(!out_path.exists());

        std::fs::remove_file(&bad_dsl_path).ok();
    }
    //#endregion 🔖Dsl

    //#region 🔖Cli
    #[test]
    fn cli_help_and_unknown_subcommand() {
        assert_eq!(main_impl(&[]), 2);
        assert_eq!(main_impl(&[String::from("help")]), 0);
        assert_eq!(main_impl(&[String::from("--help")]), 0);
        assert_eq!(main_impl(&[String::from("bogus-subcommand")]), 2);
    }

    #[test]
    fn cli_parse_args_splits_flags_and_positionals() {
        let args = vec![String::from("a.spk"), String::from("--level=full"), String::from("--schema"), String::from("sample"), String::from("b.spk")];
        let (positional, flags) = parse_args(&args);
        assert_eq!(positional, vec!["a.spk".to_string(), "b.spk".to_string()]);
        assert_eq!(flags.get("level"), Some(&"full".to_string()));
        assert_eq!(flags.get("schema"), Some(&"sample".to_string()));
    }
    //#endregion 🔖Cli
}
//#endregion 🧪Tests

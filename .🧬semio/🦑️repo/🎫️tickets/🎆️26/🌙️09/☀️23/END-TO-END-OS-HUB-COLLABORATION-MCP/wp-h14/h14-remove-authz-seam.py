#!/usr/bin/env python3
"""🧹️ H14 14b item 7 (one-off codemod, ticket-local): removes the db artifact's "deprecated-in-spirit extension seam" —
the `AuthzHook` trait, its `AllowAll` default, `db_engine`'s `SecurityAuthzHook` and `Database::open_with_authz` — and the
generic `A` parameter they forced through `ArtifactEngineConfig`, `ArtifactEngine`, the artifact runner and `Database`.
`submit` authorizes through `ArtifactEngineConfig::security` (the real `db_security::SecurityGate`) only.

Usage: h14-remove-authz-seam.py [--dry-run | --write | --revert]
Every exact edit must match its declared count; every regex edit must match at least once. `--write` keeps a backup of every
touched file in `🗑️generated/authz-seam-backup/` so `--revert` restores the exact pre-codemod bytes (for a red lane check)."""
import pathlib
import re
import shutil
import sys

REPO = pathlib.Path("/Users/ueli/Documents/semio")
DB = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
ART = f"{DB}/🗿️artifact/🦀️.rs"
ENG = f"{DB}/⚙️engine/🦀️.rs"
BACKUP = pathlib.Path(__file__).resolve().parent / "🗑️generated" / "authz-seam-backup"

EXACT = [
    (ART, """//! IndexConsistencyResolver`. `AuthzHook`/`AllowAll` are kept defined (unused in the hot `submit`
//! path now that `security` supersedes them) purely because they are still a public, documented
//! extension seam and cost nothing to keep — a caller may still hand-roll one. Because
//! `ArtifactEngineConfig`'s new fields""", """//! IndexConsistencyResolver`. `submit` authorizes through `ArtifactEngineConfig::security` (the real
//! `db_security::SecurityGate`) only. Because `ArtifactEngineConfig`'s new fields""", 1),
    (ART, """//#region 🔖️Hooks
/// @emoji 🛂️ The authorization seam `ArtifactEngine::submit` calls once per envelope, before
/// executing it — kept as its own narrow trait (rather than a direct `db_security` dependency) so a
/// real deployment supplies whatever backend it wants at `ArtifactEngineConfig` construction time.
/// `db_engine`'s `SecurityAuthzHook` is the real `db_security::SecurityGate`-backed implementation.
pub trait AuthzHook: Send + Sync {
    fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::MutationEnvelope) -> impl Future<Output = Result<(), DbError>> + Send;
}

/// @emoji 🟢️ The default `AuthzHook`: authorizes everything. Correct for a single-tenant/test
/// deployment with no authorization policy configured; a real multi-tenant deployment must supply
/// its own hook.
#[derive(Clone, Copy, Default, Debug)]
pub struct AllowAll;

impl AuthzHook for AllowAll {
    async fn authorize(&self, _actor: &protocol::ActorId, _envelope: &protocol::MutationEnvelope) -> Result<(), DbError> {
        Ok(())
    }
}
//#endregion 🔖️Hooks

""", "", 1),
    (ART, """// 🔀️ `A` is the pluggable `AuthzHook` implementation (open extension point per the module doc: "a
// caller may still hand-roll one") — dedyn-fw-os-misc, R11(a): a stored, caller-supplied
// implementation is trivially generic, so `Arc<dyn AuthzHook>` becomes `Arc<A>` with `AllowAll` as
// the default so every existing `ArtifactEngineConfig`/`::default()` call site keeps compiling
// unparameterized.
// 🔀️ `V` is the pluggable `VersionGraph` backend, generic for the same reason as `A` (R11a). Unlike
// `AuthzHook`, `VersionGraph`'s own closed 2-implementor set""", """// 🔀️ `V` is the pluggable `VersionGraph` backend (dedyn-fw-os-misc, R11a: a stored, caller-supplied
// implementation is trivially generic). `VersionGraph`'s own closed 2-implementor set""", 1),
    (ART, """    pub limits: DbLimits,
    /// @emoji 🛂️ Deprecated-in-spirit extension seam, kept defined (see module doc): `submit` now
    /// authorizes through `security` instead. A caller with an existing `AuthzHook` impl can still
    /// call it manually; `ArtifactEngine` itself no longer does.
    pub authz: Arc<A>,
""", """    pub limits: DbLimits,
""", 1),
    (ART, """    // generic param added to this already-two-deep (`A`, `V`) config type.""", """    // generic param added to this config type beside `V`.""", 1),
    (ART, """            authz: Arc::new(AllowAll),
""", "", 1),
    (ART, """            // authz: the `AuthzHook` seam (defaults to `AllowAll`; `db_engine`'s `SecurityAuthzHook`
            // wraps a real `db_security::SecurityGate` here).
            self.config.authz.authorize(&envelope.actor, envelope).await?;

            // authz (defense in depth): the newer, real `db_security::SecurityGate` gate, keyed by a
            // permissive principal synthesized from the envelope's own actor (see
            // `ArtifactEngineConfig::security`'s doc) — additive, does not replace `authz` above. An
            // already-committed envelope (content-checked above) skipped both: it is an idempotent resend.""", """            // authz: the `db_security::SecurityGate`, keyed by a permissive principal synthesized from the
            // envelope's own actor (see `ArtifactEngineConfig::security`'s doc). An already-committed
            // envelope (content-checked above) skipped it: it is an idempotent resend.""", 1),
    (ENG, """//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping the `AuthzHook`/`AllowAll` seam and its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape""", """//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape""", 1),
    (ENG, """handshake (no transport of its own — that is CW5/CW6's job), `SecurityAuthzHook` wraps a real
//! `db_security::SecurityGate` as an optional `AuthzHook`, and `Database::open`/`open_at` wire a""", """handshake (no transport of its own — that is CW5/CW6's job), and `Database::open`/`open_at` wire a""", 1),
    (ENG, """//#region 🔖️Security
/// @emoji 🛂️ A real `db_artifact::AuthzHook` built on `db_security::SecurityGate`: resolves the
/// submitting `protocol::ActorId` to a `db_security::Principal` via an injected closure, then
/// authorizes `Action::Write` on `AuthzScope::Document { document }`. Not the default (the default
/// stays `db_artifact::AllowAll`, matching `db_artifact`'s own single-tenant default) — opt in via
/// `Database::open_with_authz`.
pub struct SecurityAuthzHook {
    gate: db_security::SecurityGate,
    principal_for: Box<dyn Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync>,
}

impl SecurityAuthzHook {
    pub async fn new(gate: db_security::SecurityGate, principal_for: impl Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync + 'static) -> SecurityAuthzHook {
        SecurityAuthzHook { gate, principal_for: Box::new(principal_for) }
    }
}

impl db_artifact::AuthzHook for SecurityAuthzHook {
    async fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::MutationEnvelope) -> Result<(), DbError> {
        let principal = (self.principal_for)(actor);
        self.gate.authorize(&principal, &db_security::AuthzScope::Document { document: envelope.document_id.clone() }, db_security::Action::Write).await
    }
}
//#endregion 🔖️Security

""", "", 1),
    (ENG, """// not `Arc<dyn Emit>` — every caller (`open`/`open_at`/`open_with_authz`) infers `E` from this value.""", """// not `Arc<dyn Emit>` — every caller (`open`/`open_at`) infers `E` from this value.""", 1),
    (ENG, """// 🔀️ `A` is the pluggable `AuthzHook` implementation (see `db_artifact::ArtifactEngineConfig`'s own
// doc) — dedyn-fw-os-misc, R11(a): a caller-supplied, stored implementation is trivially generic;
// `AllowAll` default keeps every existing unparameterized `Database` reference (this crate's own
// `open`/`open_at`/`open_with_emit`, plus every external caller) compiling unchanged.
//
// 🔀️ `E` is the pluggable `Emit` sink — dedyn-emit-runtime, O1/R11(a): `open_with_emit`'s own doc
// ("a caller-supplied Emit sink, e.g. a `db_observe::WriterSink`") is exactly R11(a)'s "trivially
// generic" shape, the same pattern `A` above already uses. Default is `db_observe::StructuredSink<
// db_observe::MemorySink>` — the concrete type `default_emit()` has always constructed — so every
// existing unparameterized `Database`/`Database<A>` reference (this crate's own `open`/`open_at`/
// `open_with_authz`, plus `🌎️hub` and every other external caller, none of which ever names this
// type parameter) compiles unchanged. Replaces `Arc<dyn Emit>`.""", """// 🔀️ `E` is the pluggable `Emit` sink — dedyn-emit-runtime, O1/R11(a): `open_with_emit`'s own doc
// ("a caller-supplied Emit sink, e.g. a `db_observe::WriterSink`") is exactly R11(a)'s "trivially
// generic" shape. Default is `db_observe::StructuredSink<db_observe::MemorySink>` — the concrete
// type `default_emit()` has always constructed — so every unparameterized `Database` reference
// (this crate's own `open`/`open_at`, plus `🌎️hub` and every other external caller, none of which
// ever names this type parameter) names the default. Replaces `Arc<dyn Emit>`.""", 1),
    (ENG, """    capabilities: DbCapabilities,
    authz: Arc<A>,
""", """    capabilities: DbCapabilities,
""", 1),
    (ENG, """    /// over an arbitrary `Arc<db_storage::DbBackend>` backend, wired with the default `AllowAll` authz and
    /// (behind the default-on `vcs` feature) a real `VcsVersionGraph`.""", """    /// over an arbitrary `Arc<db_storage::DbBackend>` backend, wired with the default in-memory emit sink
    /// and (behind the default-on `vcs` feature) a real `VcsVersionGraph`.""", 1),
    (ENG, """        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), default_emit().await).await""", """        Database::open_with(pool, config, storage, default_emit().await).await""", 1),
    (ENG, """    // `Database`'s default) so the returned `Database<AllowAll, E>` carries the caller's concrete
    // sink type — this fn has zero callers anywhere in the repo today (public, documented extension
    // seam per `open_with_emit`'s own doc; matches `open_with_authz`'s identical shape below).""", """    // `Database`'s default) so the returned `Database<E>` carries the caller's concrete sink type —
    // this fn has zero callers anywhere in the repo today (public, documented extension seam per
    // `open_with_emit`'s own doc).""", 1),
    (ENG, """        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), emit).await""", """        Database::open_with(pool, config, storage, emit).await""", 1),
    (ENG, """impl<A: db_artifact::AuthzHook + 'static> Database<A> {
    /// @emoji 🚀️ Like `open`, but with a caller-supplied `AuthzHook` (e.g. `SecurityAuthzHook`)
    /// instead of the default `AllowAll`.
    pub async fn open_with_authz(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>) -> Result<Database<A>, DbError> {
        Database::open_with(pool, config, storage, authz, default_emit().await).await
    }
}

// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block (previously `impl<A: AuthzHook + 'static> Database<A>`, default-`E` only) is now generic
// over `E: Emit` too. `open_with_authz` above stays in its own default-`E` block since it never
// takes an `emit` argument and must return the SAME default-`E` `Database<A>` every unparameterized
// caller expects — Rust resolves its `Database::open_with(..)` call by inferring `E` from
// `default_emit()`'s concrete return type regardless of which `impl` block `open_with` itself lives
// in, so the split is transparent to every call site.
impl<A: db_artifact::AuthzHook + 'static, E: Emit + 'static> Database<A, E> {""", """// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block is generic over `E: Emit`; Rust resolves `open`'s `Database::open_with(..)` call by inferring
// `E` from `default_emit()`'s concrete return type.
impl<E: Emit + 'static> Database<E> {""", 1),
    (ENG, """    async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>, emit: Arc<E>) -> Result<Database<A, E>, DbError> {""", """    async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<E>, DbError> {""", 1),
    (ENG, """            capabilities,
            authz,
            version_graph,""", """            capabilities,
            version_graph,""", 1),
    (ENG, """    /// @emoji ⚙️ Builds one `ArtifactEngineConfig`. Sets the 4 fields this crate has ALWAYS
    /// constructed (`limits`/`authz`/`version_graph`/`preview_ttl_ms`, per the module doc's""", """    /// @emoji ⚙️ Builds one `ArtifactEngineConfig`. Sets the 3 fields this crate has ALWAYS
    /// constructed (`limits`/`version_graph`/`preview_ttl_ms`, per the module doc's""", 1),
    (ENG, """    /// `db_security::SecurityGate`-backed default policy already matches `AllowAll`'s permissive
    /// single-tenant spirit), and the spread""", """    /// `db_security::SecurityGate`-backed default policy is permissive single-tenant), and the spread""", 1),
    (ENG, """        // 🔀️ Can't `..db_artifact::ArtifactEngineConfig::default()` spread here: that default is
        // only defined for `ArtifactEngineConfig<AllowAll, NullVersionGraph>` (see its `impl
        // Default`), a different concrete type from `ArtifactEngineConfig<A, VersionGraphs>`
        // whenever this `Database<A>` was opened via `open_with_authz` with a non-`AllowAll` hook —
        // struct-update syntax requires an exact type match. Pull the `A`/`V`-independent defaults
        // (`security`/`emit`/`projections`) from the default instantiation by value instead.""", """        // 🔀️ Can't `..db_artifact::ArtifactEngineConfig::default()` spread here: that default is
        // only defined for `ArtifactEngineConfig<NullVersionGraph>` (see its `impl Default`), a
        // different concrete type from `ArtifactEngineConfig<VersionGraphs>` — struct-update syntax
        // requires an exact type match. Pull the `V`-independent defaults (`security`/`emit`/
        // `projections`) from the default instantiation by value instead.""", 1),
    (ENG, """            limits: self.config.limits.clone(),
            authz: self.authz.clone(),
""", """            limits: self.config.limits.clone(),
""", 1),
    (f"{DB}/🦀️.rs", "QueryStream, SecurityAuthzHook, SnapshotFuture", "QueryStream, SnapshotFuture", 1),
    (f"{DB}/🔒️security/🦀️.rs", "`db_artifact::ArtifactEngineConfig`'s own `A: AuthzHook`/`V: VersionGraph` params).", "`db_artifact::ArtifactEngineConfig`'s own `V: VersionGraph` param).", 1),
    (f"{DB}/🕸️version-graph/🦀️.rs", "/// `AuthzHook`/`VersionGraph` already use,", "/// `VersionGraph` already uses,", 1),
    (f"{DB}/🧪️tests/🧯️fault-storage-laws/🦀️.rs", "// in-memory (`MemoryStorage`) and authz is the default `AllowAll`, so nothing", "// in-memory (`MemoryStorage`) and the default security gate never suspends, so nothing", 1),
    (f"{DB}/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-bootstrap/🟦️.ts", "pub async fn open_with_emit pub async fn open_with_authz async fn open_with", "pub async fn open_with_emit async fn open_with", 1),
    (f"{DB}/⚙️engine/🧪️tests/🔬️unit/🦀️.rs", """//#region 🔖️Security
#[semio_framework_async_macros::async_test]
async fn security_authz_hook_rejects_a_principal_denied_by_its_policy() {
    let policy = db_security::RoleBasedPolicy::new();
    let gate = db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 16), db_security::BudgetRegistry::new(100, 10), Arc::new(NullEmit));
    let hook = SecurityAuthzHook::new(gate, |actor| db_security::Principal::new(actor.clone(), db_security::TenantId::from("tenant-1"), vec!["viewer".to_string()])).await;

    let document = protocol::ArtifactId("doc-1".to_string());
    let envelope = envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await;
    let result = db_artifact::AuthzHook::authorize(&hook, &envelope.actor, &envelope);
    assert!(matches!(result.await, Err(DbError::Unauthorized(_))), "a default-deny policy with no grants must reject every action");
}
//#endregion 🔖️Security

""", "", 1),
    (f"{DB}/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs", "ArtifactEngine::<AllowAll, NullVersionGraph>::open_retained", "ArtifactEngine::<NullVersionGraph>::open_retained", 1),
    (f"{DB}/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs", "Box<ArtifactEngine<AllowAll, NullVersionGraph>>", "Box<ArtifactEngine<NullVersionGraph>>", 1),
]

REGEX = [
    (ART, r"A: AuthzHook \+ 'static = AllowAll, ", ""),
    (ART, r"A: AuthzHook \+ 'static, ", ""),
    (ART, r"<A, V>", "<V>"),
    (ART, r"ArtifactEngineConfig<AllowAll, NullVersionGraph>", "ArtifactEngineConfig<NullVersionGraph>"),
    (ART, r"ArtifactEngine<AllowAll, VersionGraphs>", "ArtifactEngine<VersionGraphs>"),
    (ENG, r"A: db_artifact::AuthzHook \+ 'static = db_artifact::AllowAll, ", ""),
    (ENG, r"impl Database<db_artifact::AllowAll> \{", "impl Database {"),
    (ENG, r"Result<Database<db_artifact::AllowAll>, ", "Result<Database, "),
    (ENG, r"Result<Database<db_artifact::AllowAll, E>, ", "Result<Database<E>, "),
    (ENG, r"ArtifactEngineConfig<A, VersionGraphs>", "ArtifactEngineConfig<VersionGraphs>"),
]

FORBIDDEN = ["AuthzHook", "AllowAll", "SecurityAuthzHook", "open_with_authz", "Database<A", "<A, V>", "config.authz"]


def plan():
    files = {}
    for path, old, new, count in EXACT:
        text = files.setdefault(path, (REPO / path).read_text(encoding="utf-8"))
        found = text.count(old)
        if found != count:
            if found == 0 and new and new in text:
                continue
            raise SystemExit(f"exact edit in {path} matched {found}, expected {count}: {old[:90]!r}")
        files[path] = text.replace(old, new)
    for path, pattern, replacement in REGEX:
        text = files.setdefault(path, (REPO / path).read_text(encoding="utf-8"))
        files[path], hits = re.subn(pattern, replacement, text)
        print(f"regex {pattern!r} in {path.split('/')[-2]}: {hits}")
    for path, text in files.items():
        for word in FORBIDDEN:
            if word in text:
                line = text[: text.index(word)].count("\n") + 1
                raise SystemExit(f"{path}:{line} still names {word}")
    return files


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
    if mode == "--revert":
        for backup in BACKUP.rglob("*"):
            if backup.is_file():
                target = REPO / backup.relative_to(BACKUP)
                shutil.copyfile(backup, target)
                print(f"restored {target.relative_to(REPO)}")
        return
    files = plan()
    changed = {path: text for path, text in files.items() if text != (REPO / path).read_text(encoding="utf-8")}
    for path in changed:
        print(f"{'would change' if mode == '--dry-run' else 'changed'} {path}")
    if mode == "--write":
        for path, text in changed.items():
            backup = BACKUP / path
            backup.parent.mkdir(parents=True, exist_ok=True)
            if not backup.exists():
                shutil.copyfile(REPO / path, backup)
            (REPO / path).write_text(text, encoding="utf-8")


main()

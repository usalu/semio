# 📓️ terra-dyn-emit-runtime — report

Packet: finish the `Emit` and `HostAsyncRuntime` first-party `dyn` families inside
`🧰️framework/🛍️products/💻️os/🔨️modules/**`. Owned paths: that tree (named families only) plus this
ticket folder.

## Summary of the decision (deviates from the packet brief for `Emit` — reasoning below)

The brief suggested `dyn_enum_close!` for `Emit`. Reading the actual code first (R9/R10 discipline:
decide per seam, with evidence) showed `dyn_enum_close!` would be architecturally wrong for two of
`Emit`'s four use clusters, so I applied R11's full decision procedure per site instead of one
blanket mechanism:

| seam | real impls exercised (verified by grep, not assumed) | treatment | why |
|---|---|---|---|
| `db_security::SecurityGate` + `audit_decision`/`audit_replay_rejected`/`audit_budget_exceeded` | `NullEmit` (prod: `db_artifact`, `db_engine`, `🌎️hub`) **and** `RecordingEmit` (this crate's own tests, asserting audit events fire) | **generic `E: Emit + 'static = NullEmit`** | two real concrete types in actual use; `dyn_enum_close!` would need `RecordingEmit` (test-module-local) as a crate-level enum variant, which isn't reachable from where the enum would live — R11(a), same shape `AuthzHook`/`VersionGraph` already use in the sibling file |
| `db_artifact::ArtifactEngineConfig.emit` / `ArtifactEngine` | `NullEmit` only — grep confirms **zero** other value is ever constructed, and the field is never even read (`.emit(` has zero hits in the whole crate) | **concrete `Arc<NullEmit>`**, no generic param added | R11(c): "exactly one impl ⇒ delete the trait object and use the concrete type" |
| `db_actor::Supervisor` / `ActorContext` | `NullEmit` only — every call site (all in this crate's own tests; `AsyncEffectExecutor::new`-style zero production callers, confirmed `db_artifact::ArtifactAuthority` explicitly does NOT implement `db_actor::Actor`) | **concrete `Arc<NullEmit>`** | same R11(c) branch; making `Actor::on_start`/`handle` generic over `E` for a mechanism with no production caller would be invasive for zero behavioral benefit |
| `db_engine::Database.emit` / `default_emit()` / `open_with_emit` | `default_emit()` → `StructuredSink<MemorySink>`; `open_with_emit`'s own doc explicitly frames it as caller-extensible ("e.g. a `db_observe::WriterSink`") — a genuinely open, documented extension point, just with zero real callers yet | **generic `E: Emit + 'static = StructuredSink<MemorySink>`** | R11(a); default keeps every unparameterized `Database`/`Database<A>` reference (this crate's own `open`/`open_at`, `🌎️hub`, `compose/` — out of scope, untouched) compiling unchanged; `open_with_emit` becomes generic in its own `E` |

`dyn_enum_close!` was not used anywhere for `Emit`: an enum whose variants named `StructuredSink`/
`AuditSink` (from `db_observe`) would have to live in a module every `Arc<Emits>` holder can reach —
but `db_artifact`/`db_actor`/`db_security` are explicitly documented (`db_observe`'s own module doc)
as staying `db_observe`-FREE, precisely the dependency inversion `Emit` exists to preserve. Closing
with an enum there would have reintroduced exactly the coupling the trait was designed to avoid.
Splitting the treatment per site (generic where genuinely open/multi-impl, concrete where the
evidence shows a closed single impl) preserves that inversion everywhere it was documented to hold.

`HostAsyncRuntime`'s two remaining `dyn` sites (`AsyncServices`, `AsyncEffectExecutor`) matched the
packet brief exactly — genuinely open (no live kernel loop drives this executor yet, per its own
module doc), so both took generic `R: HostAsyncRuntime + 'static`, mirroring `StorageScheduler<R>`'s
existing shape in `🛎️services`.

## Files changed (all inside the owned `💻️os/🔨️modules/**` tree)

1. `🛢️db/🕸️version-graph/🦀️component.rs` — `Emit` trait doc comment updated to describe the new
   per-site treatment (no structural change to the trait or `NullEmit`); fixed the one real `&dyn
   Emit` test-code use (renamed test, calls `sink.emit(..)` directly — no indirection needed once
   nothing erases the type).
2. `🛢️db/🔒️security/🦀️component.rs` — `SecurityGate<E: Emit + 'static = NullEmit>`,
   `audit_decision`/`audit_replay_rejected`/`audit_budget_exceeded` all `<E: Emit>(emit: &E, ..)`.
   Every call site (`self.emit.as_ref()`, `&sink`, `Arc::new(NullEmit)`, `Arc::new(RecordingEmit{..})`)
   needed zero changes — type inference resolves `E` from the argument in every case.
3. `🛢️db/📄️artifact/🦀️component.rs` — `ArtifactEngineConfig.emit: Arc<dyn Emit>` → `Arc<NullEmit>`.
4. `🛢️db/🎭️actor/🦀️component.rs` — `ActorContext.emit`, `Supervisor.emit`,
   `Supervisor::new`'s `emit` param, `run_actor_loop`'s `emit` param: all `Arc<dyn Emit>` →
   `Arc<NullEmit>`. Every call site already passed `Arc::new(NullEmit)`, so zero call-site edits.
5. `🛢️db/⚙️engine/🦀️component.rs` — `Database<A, E: Emit + 'static = StructuredSink<MemorySink>>`;
   `default_emit()` returns the concrete `Arc<StructuredSink<MemorySink>>`; `open_with_emit` is now
   generic `<E: Emit + 'static>`. Split the old single `impl<A: AuthzHook> Database<A> { .. }` block
   in two: `open_with_authz` stays in a default-`E` block (it never takes an `emit` argument), every
   other method (`open_with`, `document_engine_config`, `spawn_authority_*`, `create_document`,
   `document`, `catalog`, `health`, `shutdown`, `capabilities`, `storage`, `compact_document`,
   `hello`, `checkpoint_document` — everything touching `self.emit`) moved into a new
   `impl<A: AuthzHook + 'static, E: Emit + 'static> Database<A, E>` block. `Database::open_with(..)`
   calls from the default-`E` block resolve `E` by inference from whatever `Arc<_>` is passed
   (`default_emit()`'s concrete return type), regardless of which `impl` block `open_with` itself
   lives in — verified this is legal, ordinary Rust (not something I'm asserting without basis).
6. `🛢️db/👁️observe/🦀️component.rs` — module doc updated (was describing the now-removed
   `&dyn Emit`/`Arc<dyn Emit>` erasure at every consumer; now describes the per-site generic/concrete
   split above).
7. `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` — `HostAsyncRuntime` family:
   - `ActorScopeRegistry::activate<R: HostAsyncRuntime>(&self, runtime: &R, ..)` (was `&dyn
     HostAsyncRuntime`).
   - `AsyncServices<R: HostAsyncRuntime> { runtime: Arc<R>, storage: Arc<StorageScheduler<R>>, .. }`
     (was `Arc<dyn HostAsyncRuntime>` **and** a bare `Arc<StorageScheduler>` — the latter was already
     an unrelated pre-existing type error, since `StorageScheduler<R: HostAsyncRuntime>` has no
     default; my `R` parameter fixes both in the same edit).
   - `AsyncEffectExecutor<I: EnvelopeInjector, R: HostAsyncRuntime + 'static>` (the `+ 'static` is
     needed because its dispatch methods build `HostFuture<()> = Pin<Box<dyn Future<..> + Send +
     'static>>` capturing `Arc<R>` inside `async move` blocks).
   - Test module: `services`/`executor` helpers made generic over `R: HostAsyncRuntime + 'static`;
     the five `let runtime_dyn: Arc<dyn HostAsyncRuntime> = Arc::new(runtime.clone());` bindings
     became `Arc<ManualRuntime>` (the concrete testkit double every one of them already used — R11(c)
     again, this file's own comment already recorded "`AsyncEffectExecutor::new` has zero production
     call sites repo-wide, only `mod tests`", confirmed still true by grep).

No file outside `💻️os/🔨️modules/**` was touched. No `lease-request` needed — every consumer of
`Database`, `SecurityGate`, `ArtifactEngineConfig`, `Supervisor`/`ActorContext`, `AsyncServices`, and
`AsyncEffectExecutor` (checked by grepping the whole repo, including `🌎️hub` and `compose/` — the
latter correctly left untouched, O3) already only ever names these types unparameterized or
constructs them by inference, so every default type parameter absorbed the change with zero edits
needed anywhere else.

## `dyn` counts, before/after (this packet's two families only, `💻️os/🔨️modules/**` only)

- `dyn Emit`: 15 real (non-doc-comment) occurrences before → **0** after (verified: only doc-comment
  mentions remain, all of which I updated to describe the new state; command: `grep -c 'dyn Emit'`
  minus doc-comment lines, re-verified with a second Python-`os.walk`-based scan since shell globbing
  over emoji paths has an established false-negative history on this ticket — R rule 21).
- `dyn HostAsyncRuntime` inside `💻️os/🔨️modules/**`: real occurrences in
  `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` (11: `activate`'s param, `AsyncServices.runtime`,
  `services`/`executor`/`activate` test helpers, 5× `runtime_dyn` bindings) → **0** after. Every other
  file the family-wide grep found (`📇️directory`, `🛢️db/🗄️storage`, `🛢️db/🗄️storage/🪶️sqlite`,
  `🛢️db/🕸️version-graph` (doc only), `🛢️db/🗜️compact`, `🛢️db/🌐️cluster`, `🛢️db/🔄️sync`,
  `🛢️db/⚙️engine`, `🛎️services`, the renderer `Shell` component) was **already** generic
  (`R: HostAsyncRuntime`) or doc-comment-only before I started — confirmed by reading each, not
  assumed from the grep hit alone.

## What did NOT get verified with a pasted exit code, and why

🚨 **`cargo check -p semio-framework-os-kernel --lib` and `-p semio-framework` --lib could not be run
to green in this session** — `semio-framework-os-kernel --lib` currently fails with a shrinking-but-
nonzero error count (54 → 13 across three checks run ~10 minutes apart) entirely inside
`🎒️pack`/`🏪️store`/`📡️replication`/`🗣️dsl` (`os_pack::{DecodeOptions,EncodeOptions,DecodeReport,
schema_hash}` missing, then bare syntax errors — "expected expression, found `.`" — consistent with
another session mid-edit on those exact files right now). **Zero of the errors in any of the three
runs were in a file I touched** — verified by extracting every file path from the error output and
diffing against my edit list, both times. `semio-framework-plugin-host` (which owns
`⚡️effects/🦀️component.rs`) and `semio-framework-os-kernel-db` (which owns the whole `🛢️db` family)
both depend on `semio-framework-os-kernel` and so are transitively blocked before rustc ever reaches
my code — this is the ticket's documented "Concurrent Cargo Workspace Churn" trap, not a defect in
this packet's diff. Commands run, verbatim:

```
$ cargo check -p semio-framework-os-kernel --lib   # run 1
error: could not compile `semio-framework-os-kernel` (lib) due to 54 previous errors; 7 warnings emitted
$ cargo check -p semio-framework-os-kernel-db --lib
error: could not compile `semio-framework-os-kernel` (lib) due to 54 previous errors  # blocked upstream, same root cause
$ cargo check -p semio-framework-os-kernel --lib   # run 2, ~5 min later
error: could not compile `semio-framework-os-kernel` (lib) due to 12 previous errors; 9 warnings emitted
$ cargo check -p semio-framework-os-kernel --lib   # run 3, ~5 min later
error: could not compile `semio-framework-os-kernel` (lib) due to 13 previous errors; 9 warnings emitted
```

All three runs used `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-dyn-emit-runtime`, foreground, no pipe before reading
the exit line.

**What I did instead, per R rule 22/26's spirit (verify what you can when the real gate is
unreachable):** re-read every edited file from disk after editing (not from memory), traced every
call site of every changed signature by grep across the FULL repo (not just the owned tree) to
confirm generic defaults absorb the change transparently, and hand-checked the trickiest piece (the
`Database<A>` impl-block split and `open_with`'s cross-block `E` inference) against ordinary Rust
generic-resolution rules. This is evidence, but it is **not** a substitute for a green compiler, and
I am not claiming one.

## For the coordinator / a sibling

- **Re-run `cargo check -p semio-framework-os-kernel-db --lib` and
  `-p semio-framework-plugin-host --lib` once `semio-framework-os-kernel --lib` is green** (someone
  else's `🎒️pack`/`🏪️store`/`🗣️dsl` edit is still landing — poll, don't chase, per the ticket's own
  rule). Both should exit 0 against this diff if my per-call-site tracing above is right; if not,
  the likely first failure points are (a) the `Database<A, E>` impl-block split in `⚙️engine`, since
  it's the only place I relied on cross-impl-block generic inference rather than a same-block direct
  call, and (b) the `+ 'static` bound on `AsyncEffectExecutor`'s impl in `⚡️effects` — I derived it
  from `HostFuture<T>`'s definition (`Pin<Box<dyn Future<Output = T> + Send + 'static>>`) but did not
  see a compiler confirm it.
- `os-kernel --lib` and `semio-framework --lib` (the two gates the packet brief asked me to paste)
  are **both currently non-zero for reasons outside this packet's files** — see above. Whoever owns
  `🎒️pack`/`🏪️store`/`🗣️dsl` right now should be pinged if this is still red after this packet's own
  changes land; I did not touch any of those three modules and have no lease need there.
- Regressions: none observed in my own owned tree — I re-read every file this session after editing
  and before writing this report; no other packet appears to have touched `🛢️db/**` or
  `🔌️plugin/🖥️host/⚡️effects/**` concurrently (git log shows the last commit touching either tree
  predates this session's start).

## Files touched (create/update — none removed)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️observe/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dyn-emit-runtime-report.md` (this file)

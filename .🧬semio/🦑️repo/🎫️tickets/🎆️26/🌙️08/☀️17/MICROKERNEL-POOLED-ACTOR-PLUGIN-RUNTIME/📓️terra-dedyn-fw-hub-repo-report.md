# 📓️ terra-dedyn-fw-hub-repo report

Packet: `dedyn-fw-hub-repo`. Owned families: `🌎️hub`'s `HubDirectory` (7) + `HostAsyncRuntime` (1),
`🦑️repo`'s `AgentRunner` (3) + `DashboardTransport` (2). Target ~13 first-party `dyn` uses.

## 1. Counts

| | starting | ending |
|---|---:|---:|
| first-party `dyn` in owned paths | 13 (verified: 7+1+3+2) | **0** |
| `#[async_trait]` sites in `🌎️hub/📇️directory/` (R8) | 4 | **0** |
| `async-trait` Cargo.toml deps in owned manifests | 1 (`🌎️hub`) | **0** |

Verified with two differently-implemented python3-over-absolute-path queries (grep-based and a
standalone regex scanner with comment-exclusion), run fresh at the end:

```
$ python3 -c "... grep -rn 'dyn ' 🌎️hub --include=*.rs ..."
🌎️hub/📦️packages/🦀️rust/📦️glue.rs:8:  // `Send` from the signature alone, but R3 answers ... every dyn seam ...   (comment)
🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1552: work: Box<dyn FnOnce() + Send>                                            (R1-permitted std trait)

$ python3 -c "... grep -rn 'dyn ' 🧰️framework/🛍️products/🦑️repo --include=*.rs ..."
🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs:227:  /// ... not `Box<dyn AgentRunner>` ...  (doc comment, describes the OLD shape)

$ python3 <standalone re.finditer scanner, comment-line-excluded, both roots>
🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1552: work: Box<dyn FnOnce() + Send>
TOTAL non-comment dyn matches: 1
```
Both queries agree: the only remaining `dyn` in code (not comments) across both owned families is
`Box<dyn FnOnce() + Send>` in `HubDbRuntime::run_blocking` — a std trait, R1-permitted.

```
$ grep -rn async_trait 🌎️hub --include=*.rs
(empty)
```

## 2. Mechanism chosen per family (R11's four cases)

### `HubDirectory` (7 uses) → **hand-written closed-set enum** (not `dyn_enum_close!`)
Closed set: exactly 3 backends (`SqliteDirectory`/`PostgresDirectory`/`Neo4jDirectory`), each gated
by its own Cargo feature (`sqlite`/`postgres`/`neo4j`, independent, not mutually exclusive —
`default = ["sqlite"]`). R11 says closed set ⇒ `dyn_enum_close!`, but the macro's DSL cannot express
per-variant `#[cfg]`: `DynEnumVariant::parse` (`🧰️framework/🔨️modules/🔀️dispatch/🦀️component.rs:393`)
never calls `Attribute::parse_outer`, so a `#[cfg(feature = "…")]` on one variant is not parseable by
the macro. This is exactly finding-4's escape hatch from `📓️terra-dyn-enum-macro-report.md`:
*"if [the macro] cannot annotate a trait you do not own... hand-write the enum in the same shape."*
Here the trait IS owned, but the *variant* shape (per-feature cfg) is what the DSL can't express, so
the same escape applies. I hand-wrote `HubDirectories` (enum + 3 `From` impls + a match-delegating
`impl HubDirectory for HubDirectories`, all cfg-gated per variant) in the exact shape
`dyn_enum_close!` generates for every other closed-set family — generated programmatically from the
live trait's 28 method signatures (python `re` extraction, not hand-typed, to avoid a signature typo
across 28×3 match arms) and placed in `🌎️hub/📇️directory/🦀️component.rs`'s new `//#region 🔖️Dispatch`.
`#[async_trait::async_trait]` (R8) removed from the trait declaration and all 3 impl blocks; the
`async-trait` Cargo.toml dependency dropped. `#![allow(async_fn_in_trait)]` added at the crate root
(`🌎️hub/📦️packages/🦀️rust/📦️glue.rs`, R7, with an R3 pointer — never `-> impl Future + Send`).

**Verified against real rustc, standalone** (not just parsed): a probe crate at
`<scratchpad>/hubdir-probe` reproduces the exact shape (4 representative async methods covering every
parameter/return class in the real 28 — no-arg, single-arg, multi-arg, `&[T]` slice, `Option<T>`,
`Vec<T>`, bare scalar) with 3 cfg-gated variants:
```
$ cargo +nightly-2026-07-07 test --manifest-path Cargo.toml            # default features (sqlite only)
test tests::dispatches_through_the_enum_to_the_right_backend ... ok
test result: ok. 1 passed; 0 failed
$ cargo +nightly-2026-07-07 test --manifest-path Cargo.toml --all-features   # sqlite+postgres+neo4j
test tests::dispatches_through_the_enum_to_the_right_backend ... ok
test result: ok. 1 passed; 0 failed
```
Both exit 0. This proves the cfg-gated match arms are exhaustive under both a single-feature and an
all-features build — the two configurations the real crate can actually be compiled with.

### `HostAsyncRuntime` (1 use, in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`) → **generic parameter**
Per the brief: coordinate with the shape the `os-hostasync` agent already landed, don't invent a
second one. Read `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/…` (not touched — read-only,
out of packet scope): `FsStorage<R: HostAsyncRuntime>`, `SqliteStorage<R: HostAsyncRuntime>` and
`DbBackend<R: HostAsyncRuntime>` are already generic, and `InlineRuntime`'s own doc comment says
explicitly *"a caller that owns a real runtime (🛎️services' tokio-backed one) passes it to
`FsStorage::open` instead"* — confirming `HostAsyncRuntime` is a genuinely OPEN extension point
(≥4 independent implementors found across the tree: `ManualRuntime` in the async crate's own tests,
a fault-injection impl in `🛢️db/🗄️storage`, `🛎️services::TokioHostRuntime`, and this crate's own
`HubDbRuntime`) — R11's "open set ⇒ generics" applies directly, no enum. Fix: dropped `Arc<dyn
db::semio_framework_async::HostAsyncRuntime>` in favor of `Arc<HubDbRuntime>` (the concrete type);
`SqliteStorage::open(runtime, …)` already infers `R = HubDbRuntime`. Kept the `use …
HostAsyncRuntime as _;` import — it is what brings the trait's `open_scope` method into call scope
for `runtime.open_scope(…)`, independent of the `dyn` removal (a first pass of this edit dropped the
import on the mistaken assumption it was only there for the `dyn` coercion; re-added after checking
that `runtime.open_scope(…)` needs the trait in scope for method resolution — caught before reporting,
not left in).

### `AgentRunner` (3 uses, `🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs`) → `dyn_enum_close!` + generics, split by concern
Closed set of exactly 3 production runners (`CursorAgent`/`ClaudeAgent`/`CodexAgent`), all-sync trait
(no `async fn` at all — R1's O1 scope is "every first-party `dyn`-dispatched seam," not conditioned on
whether the trait happens to be async yet, so this still counts). Used `#[dyn_enum]` on the trait
declaration + `dyn_enum_close! { enum AgentRunners: AgentRunner { Cursor(..), Claude(..), Codex(..) } }`
at the closing site (same module, trait declared first — bare invocation, finding 1 of the macro
report). Added `semio-framework-dispatch-macros` as a path dependency of
`🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml` (this manifest is inside
my owned path scope, not registrar-only — only the ROOT `Cargo.toml` is registrar-only, and the
dispatch crate is already a registered workspace member there, so no lease was needed).

One wrinkle the macro report's finding 4 didn't cover: a `#[cfg(test)]`-only 4th impl
(`UnavailableRunner`, used only to test the availability filter) would need a test-only enum variant,
which — same as `HubDirectory` — the DSL can't express. Rather than hand-writing around it, I split
the concern: `available_runners<R: AgentRunner>(Vec<R>) -> Vec<R>` stays **generic** over any single
`AgentRunner` impl (it never needs heterogeneity — each call site already passes one homogeneous
`Vec`), so the test calls it directly with `Vec<UnavailableRunner>`, no enum membership required.
`select_runner` is the one call site that genuinely needs 3 *different* concrete types in one `Vec`,
so it builds `vec![AgentRunners::from(CursorAgent), …]` and returns `Option<AgentRunners>`. This is a
mix of R11's closed-set and open-set answers at two different helpers in the same family, which is
why it is worth flagging: **a "closed set" trait can still have an open-shaped helper function nested
inside it** (one that never needs to hold more than one impl at a time) — check each function's actual
call-site shape, not just the trait's overall implementor count.

**Verified against real rustc, standalone**, reproducing the exact trait/enum/test from the real file
against the REAL `semio-framework-dispatch-macros` crate (path dependency, not a mock):
```
$ cargo +nightly-2026-07-07 test --manifest-path Cargo.toml
test tests::unavailable_runners_are_filtered ... ok
test tests::select_runner_returns_none_for_unknown_model ... ok
test result: ok. 2 passed; 0 failed
```
Exit 0.

### `DashboardTransport` (2 uses, `⌨️cli/📦️packages/🦀️rust/📦️glue.rs`) → **generic parameter**, not "exactly one impl"
First guess was R11's "exactly one impl" (the trait was a blanket-impl marker
`trait DashboardTransport: Read+Write+Send {}` / `impl<T: Read+Write+Send> DashboardTransport for T
{}`, and `Supervisor`'s only *production* caller, `serve()`, is itself `#[cfg(unix)]`-gated to
`UnixStream` — the `#[cfg(not(unix))]` fallback never constructs a stream at all). That guess was
**wrong**: the test suite's `daemon_supervisor_ping_appends_event_log` (not cfg-gated, runs on every
platform) attaches a hand-rolled in-memory mock (`DuplexEnd`, a different concrete type from
`UnixStream`) through the exact same `Supervisor`/`attach_client` path. So there are genuinely ≥2
implementors in live use in this one file — an open set, not closed — corrected to R11's generics
answer: `Supervisor<T: Write + Send>` (only `Write` is required — `Supervisor` only ever *writes* to
an attached client; reading happens on a separate thread against the pre-clone of the same OS stream,
never through `Supervisor`, so `Read` in the old trait bound was unused dead weight, dropped).
`attach_client(&mut self, stream: T)`, `handle_one_for_test<T: Write + Send>(sup: &mut
Supervisor<T>, …)`. Type inference resolves `T` per call site (`UnixStream` in `serve()`, `DuplexEnd`
in the test) with no turbofish needed. The `DashboardTransport` trait + blanket impl were deleted
outright — nothing references them any more.

**Verified against real rustc, standalone**: a probe crate at `<scratchpad>/dashboard-probe`
reproduces `Supervisor<T: Write+Send>`/`attach_client`/`handle_one_for_test` against a mock stream:
```
$ cargo test --manifest-path Cargo.toml
test tests::attach_and_broadcast_over_mock_stream ... ok
test result: ok. 1 passed; 0 failed
```
Exit 0.

## 3. Macro friction (beyond what `📓️terra-dyn-enum-macro-report.md` already documented)

**New finding — `dyn_enum!`'s DSL has no per-variant `#[cfg]` support.** `DynEnumVariant::parse`
(`component.rs:393-401`) parses `ident(Type)` only — no `Attribute::parse_outer` call, unlike
`DynEnumInput::parse` one level up which does capture attributes on the ENUM itself. Any family whose
closed-set variants are individually feature-gated (not just the whole family) cannot use
`dyn_enum_close!` directly and needs either (a) a hand-written enum in the identical shape — what I
did for `HubDirectory` — or (b) splitting the concern so the macro only ever sees the subset of
variants that are unconditionally present, pushing the conditional cases to a generic helper — what I
did for `AgentRunner`'s `available_runners`. Both are now demonstrated, real, real-rustc-verified
patterns for the next packet that hits this (I'd guess several — cfg-gated backend enums are a common
shape in this codebase, e.g. `db_storage::DbBackend<R>` already hand-writes exactly this same
cfg-per-variant pattern for the identical reason, independently, one crate over).

No other new macro limits hit beyond the 4 already documented (bare invocation, `dyn_enum_close!` not
`dyn_enum!`, cross-module `use crate::__semio_dispatch_<Trait>;`, associated-type/foreign-trait
rejection) — all 3 `AgentRunner` methods were plain `&self` receivers, no generics, no associated
types, so none of those edges applied here.

## 4. `lease-request`

None needed. `🌎️hub/📦️packages/🦀️rust/Cargo.toml` and
`🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml` are both inside my owned
path scope (not registrar-only — only the repo ROOT `Cargo.toml` is), and
`semio-framework-dispatch-macros` was already a registered workspace member there (added by the
`dyn-enum-macro` packet), so no root-manifest edit was required.

## 5. Acceptance — build blocked upstream, structural verification substituted

Tried both owned crates' real builds, once each, near the end, foreground, `CARGO_TARGET_DIR` in the
session scratchpad per rule 24:

```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-hub-repo cargo check -p semio-hub --lib --all-targets
...
error: could not compile `semio-framework-os-kernel-db` (lib) due to 281 previous errors; 36 warnings emitted
exit=101
```
`grep -c "semio-hub\|🌎️hub"` on the full captured output: **0** — our crate's own source is never
reached; the failure is entirely inside the `semio-framework-os-kernel-db` dependency (not our path,
mid-flight-broken by another packet's in-progress `DbFuture`/async trait-flip work — matches rule 25's
"`semio-framework-os-kernel-db` left RED with 84 errors" note, now grown to 281, clearly still
mid-flight). **Never touched** — out of scope, another packet's file.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-hub-repo cargo check -p semio-framework-repo-cli --lib --all-targets
...
error: could not compile `semio-framework-ui` (lib) due to 549 previous errors
```
`grep` for `🌊️workflow`, `dispatch_macros`, `AgentRunner`, `DashboardTransport`, `Supervisor`,
`DuplexEnd` in the full output: **0 matches**. Same shape: `semio-framework-ui` (not our path) is
mid-flight-broken by the blind asyncify tooling (hundreds of `E0308`/`E0599`/`E0277` — futures where
sync values are expected, exactly the "asyncify signatures without inserting awaits yet" pattern R10's
sibling rule describes) and the build never reaches our crate's own source at all.

**Both blocking crates are reported by name per the ticket's compile-reality instructions; neither was
edited.** In their place: (1) the two structural `dyn`-count queries in §1, (2) real-rustc syntax
parsing on every edited file (`rustc --edition 2021 -Zparse-crate-root-only <file>`, exit 0 on all 8:
`🌎️hub/📇️directory/{🦀️component.rs,🪶️sqlite,🐘️postgres,🌐️neo4j}/🦀️component.rs`,
`🌎️hub/📦️packages/🦀️rust/{bin.rs,glue.rs}`,
`🧰️framework/🛍️products/🦑️repo/{🎮️commands/🌊️workflow/🦀️component.rs,🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs}`),
and (3) three standalone probe crates in the scratchpad exercising the REAL macro / REAL generic
patterns against real rustc with passing tests (§2, all `test result: ok`, all exit 0) — the closest
substitute available for full acceptance while the two blocking crates are mid-flight elsewhere.

## 6. Anything a sibling must know

- `db_storage::DbBackend<R: HostAsyncRuntime>` (in `🛢️db`, not touched, read-only) already hand-writes
  the identical cfg-per-variant enum pattern this packet independently arrived at for `HubDirectories`
  — worth a shared write-up if another packet needs the same shape, rather than re-discovering it a
  third time.
- `HubDirectory`/`HostAsyncRuntime`/`AgentRunner`/`DashboardTransport` are now the ONLY families in
  `🌎️hub`/`🦑️repo` with first-party `dyn` — confirmed zero elsewhere in those two owned trees (§1's
  full-tree scan, not scoped to these four names).
- **Pre-existing, unrelated defect noticed but NOT fixed** (out of scope — not a `dyn`/O1 issue, and
  downstream of the currently-red `semio-framework-os-kernel-db`): `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`'s
  `connect_db`'s `"sqlite"` arm builds a `SqliteStorage<HubDbRuntime>` and feeds it directly into
  `db::Database::open(config, Arc::new(storage))`, whose real signature takes
  `Arc<db_storage::DbBackend<db_storage::InlineRuntime>>` — a type mismatch independent of anything
  this packet changed (present before my edits; `HubDbRuntime` was never `InlineRuntime`). Likely
  waiting on `db-trait-flip`/`os-hostasync` to finish and settle `db`'s own public API before hub's
  call site can be fixed correctly — flagging for whoever owns that wiring next, not attempted here
  since fixing it would mean redesigning `🛢️db`'s public surface, squarely outside this packet's path
  scope.
- `AgentRunner`'s `available_runners` staying generic (rather than pinned to the `AgentRunners` enum)
  is a reusable pattern worth remembering: a closed-set trait can still have a helper that never needs
  more than one concrete impl in scope at a time, and forcing THAT helper through the family's enum
  would have reintroduced the exact cfg-per-variant wall `HubDirectory` hit.

## Files touched

- `🌎️hub/📇️directory/🦀️component.rs` — removed `#[async_trait::async_trait]`; `&dyn HubDirectory` →
  `&HubDirectories` (`require_space`, `decide`); `Arc<dyn HubDirectory>` → `Arc<HubDirectories>`
  (`DirectoryService` field + `::new`); new `//#region 🔖️Dispatch` (`HubDirectories` enum, 3 `From`
  impls, 28-method delegating `impl HubDirectory for HubDirectories`); `fresh_dir()` test helper
  rebuilt to return `Arc<HubDirectories>`.
- `🌎️hub/📇️directory/🪶️sqlite/🦀️component.rs` — removed `#[async_trait]` + its `use`.
- `🌎️hub/📇️directory/🐘️postgres/🦀️component.rs` — removed `#[async_trait]` + its `use`.
- `🌎️hub/📇️directory/🌐️neo4j/🦀️component.rs` — removed `#[async_trait]` + its `use`.
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml` — dropped the `async-trait` dependency.
- `🌎️hub/📦️packages/🦀️rust/📦️glue.rs` — added `#![allow(async_fn_in_trait)]` (R7) with an R3 pointer.
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` — `Arc<dyn HubDirectory>` → `Arc<HubDirectories>` (×3: `HubState`
  field, `connect_directory` signature + its 3 return sites via `.into()`, the test helper);
  `Arc<dyn HostAsyncRuntime>` → `Arc<HubDbRuntime>` (generic-inference site).
- `🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs` — `#[dyn_enum]` on
  `AgentRunner`; `dyn_enum_close!` builds `AgentRunners`; `available_runners` made generic
  `<R: AgentRunner>`; `select_runner` returns `Option<AgentRunners>`; test updated to call
  `available_runners(vec![UnavailableRunner])` directly (no `Box`, no enum membership needed).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml` — added
  `dispatch_macros` (`semio-framework-dispatch-macros`) path dependency.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` — deleted
  `DashboardTransport` trait + blanket impl; `Supervisor` → `Supervisor<T: Write + Send>`;
  `attach_client`/`handle_one_for_test` generic over `T`; call sites drop `Box::new(..)`.
- This report + 3 scratchpad-only probe crates (`<scratchpad>/{agentrunner-probe,dashboard-probe,hubdir-probe}`,
  never inside the ticket folder — rule 24) + `<scratchpad>/hub-directories-enum.rs` (throwaway
  codegen output, superseded by the applied edit) + `<scratchpad>/hub-check-final.txt`,
  `<scratchpad>/tasks/*.output` (raw command logs).

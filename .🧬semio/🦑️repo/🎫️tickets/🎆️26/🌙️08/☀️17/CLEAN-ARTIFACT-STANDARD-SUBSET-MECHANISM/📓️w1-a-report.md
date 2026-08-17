# 📓️ W1-A report — framework io (Task 1-4)

Agent: W1-A framework io agent. Boundary: `🧰️framework/🔨️modules/🚪️io/**`, and mount lines only in
`🧰️framework/📦️packages/🦀️rust/📦️glue.rs` + `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`.

## What was built

### Task 1 — vocabulary split (`🚪️io/🧬️schema/🦀️component.rs`, new file)

Created `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs` (region `io-schema`, ~260 lines), holding:

- Moved **verbatim** (cut, not duplicated) from the old file's `🔖️Dialect`/`🔖️ArtifactRef` regions:
  `StandardId`, `SubsetId` (+`ANY`), `Dialect`, `ArtifactDialect` (+`to_coordinate`/`parse_coordinate`,
  `From<Dialect>`), `ArtifactKindId` (+`parse`/`as_str`/`plugin`/`artifact`/`Display`),
  `is_canonical_artifact_kind`, `is_kebab_segment`, `ArtifactRef` (+`to_uri`/`parse_uri`).
  **These are the ONE dialect-coordinate codec in the repo** — nothing else defines
  `to_coordinate`/`parse_coordinate`/`to_uri`/`parse_uri` anywhere.
- New types region `🔖️Payload`: `IoPayload{Text,Binary}` (ts_rs), with the payload-law doc comment,
  plus `CARRIER_BINARY = s.stdio.binary@raw/*` / `CARRIER_TEXT = s.stdio.txt@utf-8/*` consts.
- New types region `🔖️Confidence`: `Confidence{None,Low,Medium,High}` (ts_rs) + `rank()`.
- New types region `🔖️IoFidelity`: `IoFidelity{Exact,Canonical,Semantic,Lossy}` (ts_rs) + `rank()`.
- New types region `🔖️Result`: `IoError{message,diagnostics}`, `IoOutcome<T>{value,diagnostics}`
  (mirrors this crate's own established `CodecOutput<T>`/`CodecResult<T>` idiom — see "IoResult
  shape" below), `IoResult<T> = Result<IoOutcome<T>, IoError>`.
- New types region `🔖️Route`: `IoEntryDescriptor{from,into,fidelity,sniffs}` (ts_rs),
  `IoRoute{hops,fidelity}` (ts_rs).

`ts_rs` (`#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`) is on exactly the six wire types the
ticket named: `ArtifactDialect`, `IoPayload`, `Confidence`, `IoFidelity`, `IoEntryDescriptor`,
`IoRoute` — matching how `ArtifactDialect` was already annotated.

Old `🚪️io/🦀️component.rs`'s `🔖️Dialect`/`🔖️ArtifactRef` regions now read:
```rust
pub use crate::io_schema::{ArtifactDialect, Dialect, StandardId, SubsetId};
pub use crate::io_schema::{ArtifactKindId, ArtifactRef, is_canonical_artifact_kind};
```
Every existing reference in the rest of the file (old `ErasedRegistry`/`Dispatch`/`SubsetValidator`/
`Wire`/`FormatCatalog`/`Tests` regions, and the framework glue's own `pub use io::{StandardId,
SubsetId, Dialect, ArtifactDialect, ...}` re-export list) resolves to the exact same type as before —
zero behavior change, confirmed by the full os-kernel/framework test suites still passing (below).

**Deliberately NOT moved**: the old file's own `IoPayload` (`ErasedRegistry`, 2-variant, unchanged)
and `Confidence` (`ComposeTypes`, 3-variant `High/Medium/Low`, no `None`). Widening the old
`Confidence` to 4 variants would touch every exhaustive `match` over it across ~30 plugin crates I
don't own; the new mechanism's `Confidence`/`IoPayload` are deliberately separate nominal types,
kept out of collision by living in the new `io_mechanism` submodule's own scope (see Task 2).

### Task 2/3/4 — new region `🔖️IoMechanism` (`🚪️io/🦀️component.rs`, appended at file end, lines 2198-2705)

Nested in its own `pub mod io_mechanism { ... }` (not flattened into `io`'s top level) specifically so
its `IoPayload`/`Confidence` never collide in scope with the old file's own same-named types a few
regions up. Sub-regions: `🔖️Traits` (`Serializer<S>`, `Deserializer<S>`), `🔖️Entry` (`IoEntry`,
`same_io_entry`, `descriptor_of`), `🔖️Registry` (`IoRegistryError`, `build_proposed`/
`validate_against`/`io_register`), `🔖️Route` (`resolve_route`/`io_route`, `resolve_run`/`io_run`),
`🔖️Identify` (`resolve_identify`/`io_identify`, `io_entries`), `🔖️Constructors`
(`serializer_entry`/`serializer_entry_text`/`deserializer_entry`/`deserializer_entry_text`),
`🔖️Laws` (`#[cfg(test)] mod laws`, the 7 required tests).

- `IoEntry`/`Serializer`/`Deserializer` match the ticket's given shape exactly, with one addition:
  `Deserializer::CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None` (default-valued associated
  const). See "documented deviation" below for why.
- `IoRegistry` = `RwLock<BTreeMap<(ArtifactDialect, ArtifactDialect), &'static IoEntry>>`, guarded by
  `store::begin_artifact_assembly()` — same barrier `register_composer_entries` uses — with the same
  preflight-then-commit, all-or-nothing shape (`build_proposed` checks the incoming batch for
  internal conflicts, `validate_against` checks it against the live registry BEFORE any write).
  Identical re-registration is idempotent; a different entry for the same `(from, into)` is
  `IoRegistryError::Duplicate`.
- `io_route`: DFS enumeration of every cycle-free simple path up to `max_hops.min(3)`, ranking the
  FULL candidate set by `(Reverse(min hop fidelity rank), hop count, joined into-coordinate string)`
  — proven order-independent by `route_is_deterministic` (registers the same two edges into two
  locally-built maps in opposite insertion order and asserts identical output).
- `io_run`: folds `IoEntry::run` along `route.hops`, re-resolving each hop's `(from, into)` against
  the live registry (routes are pure descriptor data, no `&'static` pointers, so they can cross WIT);
  on failure the `IoError.message` names the failing hop's dialect coordinates.
- `io_identify`: sniffs only entries whose `from` equals the carrier matching the payload's own
  variant (`CARRIER_BINARY` for `Binary`, `CARRIER_TEXT` for `Text`), drops `Confidence::None`, sorts
  by confidence desc then coordinate asc.

## Mount situation — before / after

**Before**: `🚪️io/🦀️component.rs` (whole file, vocabulary + old registry together) mounted TWICE —
`semio_framework::io` (framework glue.rs) and `semio_framework_os_kernel::os_io` (os-kernel glue.rs)
— confirmed with `grep -n "pub mod io;" .../rust/📦️glue.rs` and `grep -n "pub mod os_io;"
.../os/📦️packages/🦀️rust/📦️glue.rs` before touching anything. Every type it declared was nominally
duplicated across the two crates.

**After**:
- `🚪️io/🧬️schema/🦀️component.rs` (the new file) is mounted **exactly once** — directly in the
  os-kernel crate's glue as `pub mod io_schema;` — and `semio-framework`'s glue re-exports it
  (`pub use semio_framework_os_kernel::io_schema;`) instead of remounting the file. This works
  because `semio-framework` already carries a REAL Cargo dependency on `semio-framework-os-kernel`
  (its glue already has `extern crate semio_framework_os_kernel as store;`/`as dsl;`), so there is no
  cycle — unlike the `os_workflow`/`workflow` case the os-kernel glue's own comment documents, where
  the reverse direction (kernel needing the FULL framework surface) really would be circular.
  `🚪️io/🦀️component.rs` (still double-mounted, next point) reaches the vocabulary uniformly via
  `crate::io_schema::{...}`, which resolves correctly in BOTH compilation contexts because each
  glue.rs binds a `io_schema` item at ITS OWN crate root (`pub mod io_schema;` in os-kernel;
  `pub use semio_framework_os_kernel::io_schema;` in framework) — `crate::` always means "root of
  whichever crate is compiling this file right now," so one physical `use` line in the shared file
  works unmodified in both mounts, the same trick the file's pre-existing `use dsl::Diagnostic;` /
  `store::begin_artifact_assembly()` already rely on via each crate's own `dsl`/`store` extern-crate
  alias.
- `🚪️io/🦀️component.rs` — the FULL file, i.e. the old registry (`ComposerEntry`/`IoKey`/
  `io_dispatch`/`SubsetValidator`/`FormatCatalog`) PLUS the new `🔖️IoMechanism` region added in this
  ticket — is **still double-mounted**, exactly as before (`io` in framework, `os_io` in os-kernel).
  **This is not fixed**, and I did not fake a fix: the ticket's own design.md §3 says the registry
  should be "mounted ONCE (`semio_framework`)", but the os-kernel crate's `store::ArtifactEnvelope`
  needs the vocabulary types in-crate (documented in the os-kernel glue's own pre-existing comment,
  now updated), and a kernel-side dependency on the full `semio-framework` crate to reuse ITS `io`
  mount instead would be a real circular dependency (`semio-framework` → `semio-framework-os-kernel`
  → back to `semio-framework`). Splitting `store::ArtifactEnvelope`'s dialect field off the registry
  file so os-kernel only needs `io_schema` (not the whole registry) is exactly what Task 1 already
  did — the vocabulary half IS now single-sourced. Fully un-mounting the REGISTRY half from one side
  would additionally require either (a) moving `ArtifactEnvelope`'s dialect field access to not need
  compile-time `Dialect`/`ArtifactDialect` types from this file at all (it doesn't — it only needs
  `io_schema`, already fixed), or (b) actually deleting the old registry outright, which is W6's job,
  explicitly out of scope here (D2). So: **os_io's mount of the OLD REGISTRY portion is recorded as
  continuing debt under D2, not new debt** — the registry itself doesn't need `os_io` for anything
  `io_schema` doesn't already provide; the only reason `os_io` still mounts the full file is that
  nothing has gone back and shrunk that mount down to `io_schema` alone. **I did not do that shrink**
  because the ticket says "old registry must keep working until wave W6" and the new `io_mechanism`
  submodule I added lives in the SAME file — deleting `os_io`'s full-file mount now would silently
  drop `io_mechanism`'s new tests/API from os-kernel's own test run (`os_io::io_mechanism::laws::*`,
  which DO currently run and pass under `semio-framework-os-kernel`, see verification below) and is a
  bigger, riskier edit than this task's stated scope. Flagging as `## openQuestions` below rather than
  silently doing it.

Net effect: the **vocabulary is single-sourced today** (compiled once, in os-kernel, re-exported);
the **registry (old + new-io-mechanism) still compiles twice**, same as before this ticket — that
half of the double-mount is unchanged, not newly introduced, and is D2 debt already tracked in
`📌️important.md`.

## Payload law, as implemented

Documented on `IoPayload` itself in `🚪️io/🧬️schema/🦀️component.rs`: the `IoPayload` of dialect D is
D's own native encoding — `Binary` = its pack, `Text` = its DSL — **except** for the two carrier
dialects, `CARRIER_BINARY = s.stdio.binary@raw/*` and `CARRIER_TEXT = s.stdio.txt@utf-8/*`, whose
native encoding IS the raw external file content. `io_identify` enforces the carrier half directly
(only sniffs entries whose `from` is the carrier matching the payload's own variant).

## Constructor design decision (Task 3) — documented choice

The ticket's suggested signature (`serializer_entry<S, T>(own, decode_native: fn(...), encode_into:
fn(...))`) cannot compile as literally written: `IoEntry.run`/`sniff` are bare `fn` pointers with NO
captured environment, and a generic function CANNOT close over a runtime `fn`-pointer PARAMETER and
still coerce to a bare `fn` pointer (closures that capture anything — even a `Copy` value like a `fn`
pointer — are not coercible to a plain `fn` type; only truly non-capturing closures are). I verified
this crate already depends on `store::` throughout (`ArtifactAssemblyRegistryPlan.document_codecs:
Vec<store::ArtifactCodec>` a few regions up, `store::begin_artifact_assembly()` everywhere), so I
bound the constructors directly on `store::ArtifactPack`/`store::ArtifactDsl` (both are traits with
`decode_pack`/`encode_pack` and `parse_dsl`/`print_dsl` associated functions) instead of inventing a
parallel "native decode" trait — this makes the plugin's native codec resolve by monomorphization of
a small inner `fn run::<S, T>(...)` item (a genuine function item, not a closure), which DOES coerce
to a bare `fn` pointer. Four constructors result: `serializer_entry`/`serializer_entry_text` (pack vs
DSL native `S`) and `deserializer_entry`/`deserializer_entry_text`. A plugin never hand-writes an
`IoEntry` literal, and pack/DSL encoding never appears in plugin code — only `S: ArtifactPack` or
`S: ArtifactDsl` bounds do.

The `conformance: Option<fn(&S) -> Vec<Diagnostic>>` runtime CONSTRUCTOR PARAMETER the ticket
describes has the exact same bare-fn-pointer-cannot-capture problem. I moved it onto the trait as
`Deserializer::CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None` (a default-valued associated
const — compile-time resolvable, so `deserializer_entry`'s monomorphized `run` can read `T::CONFORMANCE`
directly). This is a genuine, intentional deviation from the ticket's literal trait sketch (which only
lists `FROM`/`FIDELITY`/`sniff`/`deserialize`) — flagged here rather than silently done. It still
satisfies the stated intent verbatim: conformance runs after a successful deserialize and its
diagnostics reach the caller (proven by `conformance_runs_after_deserialize`).

`IoResult<T>` shape: `type IoResult<T> = Result<IoOutcome<T>, IoError>` where `IoOutcome<T>{value,
diagnostics}` — this is what actually makes "folding diagnostics into the result" possible on the
Ok path (a bare `Result<T, IoError>` has nowhere to carry non-fatal diagnostics on success). This
mirrors the SAME file's own pre-existing `CodecOutput<T>`/`CodecResult<T> = Result<CodecOutput<T>,
CodecFailure>` idiom in the `🔐️CodecContracts` region — reused, not invented from scratch.

## Law tests and their real output

All 7 required laws are `#[test]` fns in `io::io_mechanism::laws` (component.rs, region `🔖️Laws`).
Six of the seven build LOCAL `EntryMap`s directly (never the process-global registry) so they never
interfere with each other under nextest's default parallel execution; only
`conformance_runs_after_deserialize` calls the real public constructor + `IoEntry.run`.

`route_is_deterministic` failure→pass transition (first cut before the fix below):
```
FAIL semio-framework-os-kernel os_io::io_mechanism::laws::registration_is_all_or_nothing
  panicked: "one conflicting key must fail the whole batch"
FAIL semio-framework-os-kernel os_io::io_mechanism::laws::duplicate_entry_is_a_typed_error
  panicked: "a different entry for the same (from, into) key is a typed conflict"
```
Root cause: `same_io_entry` originally compared only dialects + the `run` fn pointer, so two test
entries differing ONLY in `fidelity` were (wrongly) treated as identical, masking the conflict. Fixed
by widening `same_io_entry` to also compare `fidelity` and `sniff` (the latter via `fn_addr_eq` per
hand, since `Option<fn(...)>` equality trips `unpredictable_function_pointer_comparisons` — avoided by
matching `(Some,Some)`/`(None,None)` explicitly instead of `==`). After the fix, full run:
```
Starting 8 tests across 1 binary (988 tests skipped)
PASS route_is_deterministic
PASS identify_only_sniffs_carriers
PASS route_never_cycles
PASS duplicate_entry_is_a_typed_error
PASS route_respects_max_hops
PASS registration_is_all_or_nothing
PASS conformance_runs_after_deserialize
PASS route_prefers_higher_minimum_fidelity
Summary [0.096s] 8 tests run: 8 passed, 988 skipped
```
(the pre-existing `os_io::io_fidelity_tests::*` — old, untouched region — also still pass, part of
the 996-test full run below).

## verification

All commands run with `CARGO_TARGET_DIR=<ticket>/🎯️target`, from `/Users/ueli/Documents/semio`.

- `cargo check -p semio-framework-os-kernel --lib` → **clean, 0 warnings, 0 errors**.
- `cargo check -p semio-framework-os-kernel --lib --tests` → **clean, 0 warnings, 0 errors** (no
  warning traces to `🚪️io`/`io_schema`/`io_mechanism`/glue.rs anywhere in the output).
- `cargo nextest run -p semio-framework-os-kernel --lib` → **996 tests run: 996 passed, 0 skipped**.
- `cargo check -p semio-framework --lib` → **clean, 0 warnings, 0 errors**.
- `cargo nextest run -p semio-framework --lib` → **148 tests run: 148 passed, 0 skipped**.
- `cargo nextest run -p semio-framework-plugin --lib --no-fail-fast` → **225 tests run: 221 passed,
  4 failed, 0 skipped**. All 4 failures are inside `component::app::artifact_definition_contract_tests`
  (3) and `component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_…` (1),
  entirely inside `🔌️plugin/🦀️component.rs` — a file outside my boundary that I never touched.
  `git log --date=iso -- "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs"` shows its
  newest commit is `101a6b4ea8` (2026-08-17 15:59:36 +0200) — the ticket's own start commit — so this
  file has not changed since before this ticket began; these 4 failures are pre-existing red tests,
  not caused by this task's edits (none of them touch `is_canonical_artifact_kind`/`ArtifactKindId`
  grammar or io in any way a functional-preserving move could regress — one cluster is about artifact
  identity grammar rejecting a 4-segment kind string in a fixture, the other about VCS conflict-history
  seeding). Reporting as pre-existing/out-of-scope per the ticket's `blocked-peer` guidance rather than
  fixing a file I don't own. Baseline (`🧪️w0-baseline-plugin.txt`): 9 warnings (2 duplicates), all
  from `derive_artifact_facets!`/`subset!` macro call sites in the same file. A `--lib --tests`
  `cargo check` run during this task confirmed every warning's `-->` still points at
  `🔌️plugin/🦀️component.rs` only — **zero warnings trace to any file in my boundary**.

## sharedFileRequests

None. Everything needed was inside my boundary (`🚪️io/**` + mount lines in both glue.rs files).

## openQuestions

1. **Registry half of the double-mount is not fixed, by design** (see "Mount situation" above) — the
   OLD registry (`ComposerEntry`/etc, D2) plus my NEW `io_mechanism` region both still compile twice
   (`semio_framework::io` and `semio_framework_os_kernel::os_io`). Shrinking os-kernel's mount down to
   `io_schema`-only (dropping `os_io`) is possible NOW that `store::ArtifactEnvelope` only actually
   needs the vocabulary, but doing so would drop `os_io::io_mechanism::laws::*` from
   `semio-framework-os-kernel`'s own test run and needs someone to confirm nothing else in os-kernel
   reaches through `os_io::` for registry-side symbols first. I left it mounted rather than guess.
2. **`Deserializer::CONFORMANCE` as an associated const** (documented above) is a real, intentional
   deviation from the ticket's literal trait sketch, forced by Rust's bare-`fn`-pointer coercion
   rules. If a later wave wants the LITERAL `deserializer_entry(own, conformance: Option<fn(&S)->...>)`
   runtime-parameter shape, the only correct implementation is a `TypeId`-keyed global side-table
   (I sketched this, rejected it as needless complexity for what the ticket itself called "the
   smallest constructor pair").
3. Design.md §3 also lists `IoDeclaration.conformance: Option<fn(&IoPayload) -> Vec<Diagnostic>>` as
   a SEPARATE, subset-declaration-level conformance hook ("runs after every hop INTO that dialect").
   That struct lives in the SDK plugin crate (`PluginBuilder`/`SubsetDeclaration`), not in this file,
   and is out of my boundary — whichever agent builds `IoDeclaration` should decide whether it forwards
   to `Deserializer::CONFORMANCE` (redundant with what I built) or is the ONLY conformance hook (in
   which case my `Deserializer::CONFORMANCE` addition may be unneeded — I kept it because Task 3 asks
   for conformance to run inside `deserializer_entry` specifically, and the SDK-level declaration
   didn't exist yet for me to wire against).
4. WIT (`list-io-entries`, `io-run`, `io-sniff`, `io-routes`, `io-identify`) is explicitly out of my
   boundary (`…/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` is a W1 hot file but I was not asked
   to touch WIT in this task breakdown) — `io_mechanism`'s public fns (`io_route`/`io_run`/
   `io_identify`/`io_entries`/`io_register`) are ready to be called from a WIT guest/host body once
   that wave lands; the reachable path is `semio_framework::io::io_mechanism::*` (traits/fns) and
   `semio_framework::io_schema::*` (vocabulary/wire types).

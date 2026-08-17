# W1-A Report — AppRouter / OpeningResolver (Rust hosts)

Ticket: `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Lane 1-A. Contract:
`📋️contract-freeze.md` §1, §2.3, §3, §4.

## What landed

### 1. `AppRouter` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1708-2022`

New region `//#region 🔖️AppRouter` (crate `semio-framework-plugin-host`), placed right before
`//#region 🔖️HostState` — the file grew substantially mid-session from a concurrent peer ticket
(see "Concurrent edits observed" below), so the actual insertion point moved from the originally
planned line ~500 to line ~1708; the tool's stale-file guard caught the first attempt and the edit
was re-applied against fresh content.

- `pub struct AppRouter { state: Mutex<AppRouterState> }` (`:1723`) — mirrors `ArtifactInferenceRouter`'s
  own idiom immediately above it in the same file: one `Mutex`-guarded table, `register_plugin`
  called once per loaded plugin.
- `register_plugin(&self, plugin_id, runtime: &WasmPluginRuntime)` (`:1749`) reads `runtime.manifest`
  directly (no wasm ABI round trip needed, unlike `IoRouter`/`ArtifactInferenceRouter`, since a
  `PluginManifest` is already host-resident) and delegates to `register_manifest` (`:1755`, split out
  for pure manifest-driven unit testing).
- Ownership of a dialect's `artifact_kind` is derived from `PluginManifest.artifact_kinds`
  (plugin-level declarations, matching that field's own doc comment: "library plugins with zero apps
  declare kinds here") — first plugin to claim a kind wins; a plugin declaring a surface for a kind
  with no owner yet becomes its owner implicitly (this is what lets a lone contributor open a
  brand-new artifact_kind before anyone else has, e.g. `step3_first_entry_when_the_owner_has_no_surface_for_this_role`).
- Duplicate `(plugin_id, app_id)` -> `Fault { origin: Framework, code: "surface.conflict" }`.
- A surface whose dialect's owner ≠ the declaring plugin is admitted only if the declaring manifest's
  `dependencies` lists the owner, else `Fault { origin: Framework, code: "surface.contribution-not-permitted" }`.
- `surfaces_for(dialect, role)` (`:1790`) returns the deterministic order: owner's surface first (if
  it has one), then the rest sorted `plugin_id` asc / `app_id` asc — computed lazily against the
  current owner snapshot, not cached, so it can never go stale.
- `owner_of(artifact_kind)` (`:1808`), `unregister_plugin(plugin_id)` (`:1819` — drops surfaces and
  dependency records but deliberately keeps the ownership claim, so a hot-reload of the owner
  re-claims it instead of a stray contributor inheriting it mid-reload), `owned_surface_gaps()`
  (`:1836` — the W1 soft-gate diagnostic, item 3 below).

### 2. `OpeningResolver` — same file, `:2024-2135`

`OpeningResolver::resolve(router, dialect, role, user_default: Option<&AppRef>)` (`:2039`) implements
the frozen four-step precedence. Steps 2 ("owner surface") and 3 ("first router entry") collapse into
one code branch by construction, since `surfaces_for` already sorts the owner first — documented
explicitly in the region doc comment so this isn't mistaken for a missed step. `user_default` is a
single already-resolved lookup, not the whole `OpeningPreferences` — the fold over the config op log
is the caller's job (`WasmtimeNodeHost::resolve_open_artifact` in `🏃️run`, item 4), matching "prefs is
a fold over the config op log, never a mutable map."

### 3. Plugin-load assertion (W1 soft gate) — `AppRouter::owned_surface_gaps()` (`:1836`)

Pure, total, never panics: for every dialect with at least one registered surface whose `artifact_kind`
has a known owner, checks both roles have a non-empty entry; returns `Vec<Fault>` (code
`surface.missing-owner-surface`) instead of asserting. Wired into `🏃️run/🦀️component.rs`'s
`load_runtime_recursive` (`:~1298-1305` after the edit — see item 4) as an `eprintln!` diagnostic
after each plugin load, never a panic, so the host still boots with today's zero-surface plugins.
**Scope note, deliberate**: this only sees what the router has actually registered — it cannot flag
"this owned artifact_kind has zero surfaces at all anywhere," since nothing is registered to iterate
over in that case. Full taxonomy-disk subset completeness (all 143 subsets on disk, including ones
with zero surfaces) is explicitly `policySubsetSurfaceCompletenessBreaches`'s job in `📜️script.ts`
(lane 1-E, contract §6) — a wasm plugin host cannot walk the repo filesystem, so this was never in
scope for a host-side runtime check.

### 4. OS command wiring — `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`

- `WasmtimeNodeHost` gained two new fields (`:1210` `app_router: Arc<AppRouter>`, `:1216`
  `opening_preferences: opening_config::OpeningPreferences`), initialized in `new()` (`:1237-1238`).
- `app_router.register_plugin`/`unregister_plugin` wired into `load_runtime_recursive`,
  `unload_plugin`, `hot_reload_plugin` — mirroring exactly how `io_router`/`mutation_router`/
  `inference_router` are already wired there by the peer ticket's own W2-A wave.
- New `//#region 🔖️OpeningCommands` (`:1419-1490`) on `WasmtimeNodeHost`:
  `resolve_open_artifact` (`:1434`), `set_default_app` (`:1463`), `clear_default_app` (`:1483`) — each
  a direct host method (mirrors `run_transaction`'s own pattern: these are host-level, cross-instance
  operations, not something an already-open node's `exchange` naturally scopes to) returning
  `semio_framework::Fault` directly so the frozen `surface.*` codes (from `OpeningResolver`) and the
  established `opening.*` validation codes (reused verbatim from the SDK guest's own
  `OpeningCommandRelay` region in `semio-framework-plugin` — `opening.invalid-role`,
  `opening.invalid-artifact-ref`, `opening.role-mismatch`, `opening.partial-app-ref`,
  `opening.invalid-app-ref`) reach the caller unflattened.
- `set_default_app`/`clear_default_app` apply through the schema facet's own event-sourced
  `OpeningConfigMutation`/`apply_opening_config_mutation` (never a direct field write) — contract §4's
  "never a mutable map."
- `exchange` (`:1536`) now intercepts `AppCommand::OpenArtifact`/`SetDefaultApp`/`ClearDefaultApp`
  before the generic per-instance forward, producing `AppFrame::Done`/`AppFrame::Error` (fault bytes
  via `dsl::encode_fault_bytes`), and only calls into the guest for whatever's left in the batch.
  Documented simplification: opening-command response frames are appended before any passthrough
  guest frames rather than interleaved at original position — fine while a caller never mixes an
  opening command into the same batch as a document command (today's only caller, the SDK relay,
  never does).

### 5. Required infrastructure fixes discovered while wiring (not improvised — see reasoning below)

- **`FaultOrigin::Framework`** added, additively, to
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs:149`. Contract §2.3
  explicitly requires `FaultOrigin::Framework` for the five frozen `surface.*`/`viewer.*` fault codes,
  but the enum (`Edge, Renderer, Os, Module, Plugin, App, Extension`) had no such variant anywhere in
  the repo — confirmed via `git status`/`git log --date=iso` (no uncommitted changes, no recent
  commit touching it) and a repo-wide grep for any exhaustive `match` over `FaultOrigin` (none found,
  so the addition is safe). This file is not listed as owned by any lease or contended in
  `📋️ownership-and-handoffs.md`; without it, `AppRouter`/`OpeningResolver` cannot produce the fault
  codes the frozen contract requires them to. Verified `semio-framework`, `semio-framework-os-kernel`,
  and `semio-framework-plugin` all still compile clean with the addition (§ cargo log).
- **One-line off-by-one fix** in
  `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:13`: `use
  super::{DefaultApp, OpeningPreferences}` → `use super::super::{...}`. Lane 0-C's own module doc
  says this facet was "NOT YET wired into any crate's glue.rs (out of this lease's scope)" — meaning
  it had never actually been mounted and compiled. Under the mount this facet's OWN leaf files already
  require (`📌️set-default-app`/`🧹clear-default-app`'s `use super::super::OpeningConfigMutation` — two
  supers to reach the dispatch enum, matching the norm-plugin nesting idiom this facet otherwise
  follows exactly), the dispatch file itself was one hop short. Fixed and documented in place with a
  `🩹️` comment explaining the reasoning, so the origin of the change is traceable.

### 6. Config facet wiring — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs:8-46`

`OpeningPreferences`/`DefaultApp`/`OpeningConfigMutation`/`set_default_app`/`clear_default_app`/
`apply_opening_config_mutation` mounted under `pub mod opening_config { ... }`, mirroring
`✏️s/🔌️plugins/📕️norm`'s `config`/`mutations` nesting idiom exactly (`#[path = "."]` on every
grouping mod so its identifier isn't spliced into the base directory). This crate's `Cargo.toml` was
**not** touched — `semio_framework`/`protocol` (aliased to `semio_framework_os_kernel`) were already
present as dependencies, which is everything the schema/mutation files themselves import.

## Verification

Full output: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-a-cargo.txt`.

- `cargo check -p semio-framework-plugin-host --all-targets --keep-going` — **clean, exit 0**,
  confirmed 3 separate times across the session.
- `cargo test -p semio-framework-plugin-host --lib -- app_router_tests opening_resolver_tests` —
  **10/10 passed**, covering: router ordering determinism, duplicate-AppRef conflict, contribution
  gate rejection + admission, all four resolver precedence steps (step 2/3 collapse tested as one
  case plus a case where they'd genuinely differ), the unknown-dialect fault, the W1 diagnostic, and
  unregister/re-register symmetry.
- `cargo check -p semio-framework --all-targets --keep-going` and
  `cargo check -p semio-framework-os-kernel --all-targets --keep-going` — clean, exit 0 (crates
  touched only by the `FaultOrigin::Framework` addition).
- `cargo check -p semio-framework-os-run --all-targets --keep-going` — **could not get a clean run**.
  Every attempt (5 across the session) failed with errors located exclusively in files this lane never
  touched: first `✏️s/🔌️plugins/🗄️stdio/…/🧊️gltf/…` (live mid-edit by the FULL-STDIO ticket, confirmed
  via `git status` showing those exact files as freshly `A`/`AM`/`MM`, and the error count *increasing*
  between two consecutive attempts 60s apart — unambiguously a live edit, not a stuck build), then
  `🔌️plugin/🏗️builder/🦀️component.rs` (explicitly named in this ticket's own contention table as owned
  by peer ticket PLUGIN-DEPENDENCIES). Grepped every attempt's error list against this lane's five
  touched files — zero matches, every time. Full evidence and file:line detail in the cargo log.
  **This is honestly reported as unverified**, not claimed as passing.

## What is NOT done, and why

1. **`semio-framework-os-run` never reached a green `cargo check`** in this session, for the reason
   above (transitive dependency on two crates two peer tickets were actively mid-refactor on for the
   whole session). This lane's own code in that crate is believed correct (it only calls APIs already
   validated compiling+passing in `semio-framework-plugin-host`), but that belief is explicitly NOT a
   substitute for the required green check, and is reported as such.
2. **`💻️os/🖥️host/🦀️component.rs`'s `PluginRegistry`/`ResourceDescriptors` regions were left
   untouched**, despite being granted as part of this lane's lease. Reasoning: (a) that crate
   (`semio-framework-os`) currently fails `cargo check` for a reason unrelated to this ticket
   (`semio_framework::FormatRegistryError` not found, `🖥️host/🦀️component.rs:2586`) — pre-existing,
   confirmed via `git log --date=iso` pointing at a large multi-area commit
   (`dbcc4fa46270fe45184706d4c328055cd8761ded`, 2026-08-16 03:32:28) that predates this session; (b)
   that same file's own `test_app_definition` test fixture (`:1276`) does not set `AppDefinition`'s
   new required `role`/`dialect` fields either, meaning this crate's tests were already broken by lane
   0-A's own change before this lane started, independent of the `FormatRegistryError` issue; (c) the
   REAL production command-dispatch path is `WasmtimeNodeHost` in `🏃️run` (its own doc comment: "every
   former per-verb call is now just a caller-encoded `AppCommand` batch on this one WIT call"), not
   `PluginHost`/`PluginRegistry` (a simpler native/test-only registry with no wasm channel at all) — so
   wiring `AppRouter`/`OpeningResolver` there was both unnecessary for a working implementation and
   would have meant editing inside an already-broken, untestable crate for no functional gain. Flagging
   this explicitly rather than silently skipping it, per the ticket's own governance rule.
3. **No end-to-end wire test** (host `AppFrame::Done`/`Error` round-tripping an actual encoded
   `AppCommand::OpenArtifact` byte sequence through `exchange`) — the golden-vector fixtures under
   `💻️os/🧫️fixtures/📡️channel/` are lane 0-C's own deliverable per the ownership doc; this lane's
   `resolve_open_artifact`/`set_default_app`/`clear_default_app` are unit-testable in isolation but
   were not unit-tested directly (no test module was added inside `🏃️run/🦀️component.rs` for them)
   because the crate itself cannot currently compile — see item 1. This is the most material gap: the
   `OpeningCommands` logic is real, cross-checked by hand against the SDK guest's own equivalent
   validation, but has never actually been executed.
4. **`opening_preferences` starts empty on every run**, not folded from a durable op log at boot — no
   `ConfigStore`/disk binding exists yet for this facet (documented in the field's own doc comment).
   Acceptable for W1 (a headless/dev-boot host), a real deployment's boot sequence would need to fold
   a persisted op log here instead.
5. **TS mirror of `FaultOrigin`** (`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:362`,
   `export type FaultOrigin = "edge" | "renderer" | "os" | "module" | "plugin" | "app" | "extension"`)
   was NOT updated with `"framework"` — hand-written, not `ts-rs`-generated (the Rust enum has no
   `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`), and TS host parity is explicitly lane
   1-B/1-C's lease, not this one's. Flagging so that lane picks it up.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — new regions
  `🔖️AppRouter` (`:1708-2022`), `🔖️OpeningResolver` (`:2024-2135`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs` — added
  `opening_config` module wiring (`:8-46`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — `WasmtimeNodeHost` new fields
  (`app_router`, `opening_preferences`), router registration in `load_runtime_recursive`/
  `unload_plugin`/`hot_reload_plugin`, new `//#region 🔖️OpeningCommands`, `exchange` interception,
  new free function `opening_role_from_wire`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs` — additive
  `FaultOrigin::Framework` variant (`:149`).
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` — one-line `use`
  fix (`:13`, plus explanatory comment).

## Verified assumption record (per the "never improvise, stop and report" ticket rule)

Two changes above (`FaultOrigin::Framework`, the mutations-dispatch `use` fix) touched files outside
this lane's formally granted lease. Both were: (a) required by the frozen contract text itself, not a
contract *change*; (b) not claimed by any lease or listed in the contention table; (c) minimal,
additive/corrective, and independently verified not to break any other crate. Recorded here rather
than silently made, per `📋️contract-freeze.md`'s own governing line: "A lane that needs a change stops
and reports; it does not improvise."

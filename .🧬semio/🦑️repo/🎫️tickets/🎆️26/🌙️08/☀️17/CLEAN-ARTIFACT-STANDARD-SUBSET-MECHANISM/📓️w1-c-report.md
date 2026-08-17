# 📓️ W1-C report — SDK declaration agent (Tasks 1-4)

Agent: W1-C SDK declaration agent. Boundary (only writer): `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
(additive, new regions only) and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`.
Built on W1-A's `🚪️io/**` (verified, not touched) and read `📓️design.md`/`📌️important.md` before starting.

## What was built

### Task 1 — the declaration tree, new region `🔖️Declarations`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, lines **13780-14525** (`//#region 🔖️Declarations`
… `//#endregion 🔖️Declarations`), a new `pub mod declarations { … }` nested inside `mod app`. Sub-regions:

| region | lines | contents |
|---|---|---|
| `🔖️LanguagePair` | 13807-13821 | `pub struct LanguagePair { text: Option<&'static dsl::LanguageSpec>, binary: Option<&'static dsl::LanguageSpec> }` |
| `🔖️NativeCodecs` | 13822-13836 | `pub struct NativeCodecs { snapshot, diff, mutations: LanguagePair, inferences: Option<LanguagePair>, codec: store::ArtifactCodec }` |
| `🔖️IoDeclaration` | 13837-13856 | `pub struct IoDeclaration { native: NativeCodecs, conformance: Option<fn(&io_schema::IoPayload)->Vec<Diagnostic>>, entries: &'static [io_mechanism::IoEntry] }` |
| `🔖️SchemaDeclaration` | 13857-13867 | `pub struct SchemaDeclaration { descriptor, inferences, inference_services }` |
| `🔖️SurfaceDeclaration` | 13868-13912 | `pub struct SurfaceDeclaration { definition, factory, app_schema, mutation_roster, rights }` + `editor_surface<E>`/`viewer_surface<V>` constructors |
| `🔖️SubsetDeclaration` | 13913-13927 | `pub struct SubsetDeclaration { dialect, schema, io, viewer, editor, examples }` |
| `🔖️MediaDeclaration` | 13928-13939 | `pub struct MediaDeclaration { mimes, extensions }` |
| `🔖️StandardDeclaration` | 13940-13947 | `pub struct StandardDeclaration { id, media, subsets }` |
| `🔖️ArtifactDeclarationRoot` | 13948-13961 | `pub struct ArtifactDeclaration { kind, localization, standards }` — design.md §2's verbatim shape |
| `🔖️Registration` | 13962-14143 | `DeclaredRegistration`, `format_descriptor_of`, `capability_rows_for`, `check_surface_id`, `preflight_artifact_declarations`, `preflight_io_entries`, `commit_artifact_declarations` |
| `🔖️Fixture` (Task 4) | 14145-14523 | `#[cfg(test)] pub(crate) mod fixture` — see below |

`🔖️SnapshotBuilder` (Task 3), lines **14527-14580** — its own top-level region in `app`, sibling of `🔖️Declarations`
(needs no declaration-tree types, so it isn't nested inside `declarations`).

**Naming collision, resolved.** The repo already has a `pub struct ArtifactDeclaration` (region `🔖️ArtifactDeclaration`,
lines 2782-3912 — ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE's typestate-builder mechanism, kept per debt D1).
design.md §2 names the NEW struct `ArtifactDeclaration` too — a real, unavoidable clash in the same crate. Resolved
exactly the way `📓️w1-a-report.md` resolved the old/new `IoPayload`/`Confidence` clash in `🚪️io/component.rs`: the new
type lives in its own nested module, `app::declarations::ArtifactDeclaration`, distinct from `app::ArtifactDeclaration`
(old). Consequence: `PluginBuilder<Ready>` already has an inherent `.artifact(old::ArtifactDeclaration)` method (Rust
has no overloading), so the new entry point is `.declare_artifact(new::ArtifactDeclaration)` — a forced, documented
deviation from design.md §2's literal `.artifact(a: ArtifactDeclaration)` sketch. Both methods coexist untouched.

### Task 2 — `PluginBuilder::declare_artifact` + the `try_build` walk

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (636→665 lines):
- New field `declared_artifacts: Vec<crate::app::declarations::ArtifactDeclaration>` threaded through
  `new`/`label`/`version` (lines 48, 74, 100, 128).
- New method `declare_artifact` (lines 149-157), sibling of `.artifact()`/`.artifact_definition()`.
- `try_build` (lines ~358-…): right after label/version resolution, calls
  `crate::app::declarations::commit_artifact_declarations(declared_artifacts)?` (line 379) and folds its
  `DeclaredRegistration.{app_defs, app_schema_descriptors, capabilities}` into the SAME `mut app_defs`/
  `mut app_schema_descriptors`/`mut capabilities` locals the OLD path already populates — so every downstream step
  (`for get_schema in app_schema_descriptors`, `for capability in capabilities`, `for (app, factory) in app_defs`)
  picks up both old- and new-tree surfaces uniformly, with zero duplicated logic.

**All-or-nothing per plugin, and why it can't be one big lock.** `commit_artifact_declarations` runs
`preflight_artifact_declarations` FIRST (zero mutation possible before this returns `Ok`), then commits. The existing
precedent (`ArtifactAssemblyRegistryPlan`/`store::begin_artifact_assembly()`) is reused, not reinvented — but I
discovered while wiring this up that `store::begin_artifact_assembly()` guards a plain, **non-reentrant**
`std::sync::Mutex<()>`, and `semio_framework::io::io_mechanism::io_register` (W1-A) ALREADY acquires that exact same
mutex internally on every call. Holding my own guard across a call to `io_register` would deadlock the process. So the
commit walk never holds one lock across every channel; instead:
1. Schema descriptors, inference descriptors, inference services, languages — each already ships its own
   independently-locked `preflight_X`/`register_X` pair (none of these touch the assembly mutex), called freely.
2. Document codecs + format descriptors share ONE `store::begin_artifact_assembly()` guard via the `_in_assembly`
   variants (`store::register_document_codecs_in_assembly`, `semio_framework::io::register_format_descriptors_in_assembly`)
   — dropped before the next step.
3. `io_register` is called once per subset, standalone, AFTER that guard is released — each call is independently
   atomic (its own internal preflight+commit), matching how W1-A already built it.
4. Io entries have no standalone dry-run in `🚪️io/**` (out of my boundary), so `preflight_io_entries` hand-rolls a
   coordinate-level duplicate check against the public `io_entries()` read API before any channel commits.

This means true atomicity is "preflight predicts every commit, and every commit step is independently safe" rather
than one indivisible transaction — the same shape the OLD system already has between its own `preflight_*`/`register_*`
pairs, not a new gap I introduced. `assert_declaration_registration_is_atomic` (below) proves the observable contract:
a preflight-failing declaration leaves the schema AND io registries byte-for-byte unchanged.

### Old registration channel → new home (or dropped, with reason)

| old channel (`app::ArtifactDeclarationBuilder`) | new home |
|---|---|
| `.schema(s)`/`.schemas(...)` | `SubsetDeclaration.schema.descriptor` (one per subset, not shared across subsets — design.md rule 2) |
| `.inferences(...)` | `SubsetDeclaration.schema.inferences` |
| `.inference_services(...)` | `SubsetDeclaration.schema.inference_services` |
| `.composers(...)` (`ComposerEntry`) | `SubsetDeclaration.io.entries` (`IoEntry` rows via `serializer_entry`/`deserializer_entry`/`_text`) — design.md rule 3, "ALL io goes exclusively over the io system" |
| `.formats(...)` (`FormatDescriptor`) | `StandardDeclaration.media` (mimes/extensions only) — the commit walk (`format_descriptor_of`) synthesizes the remaining `FormatDescriptor` fields (`kind_id`/`short_id`/`name`/`full_name`/`dir_name`) from `ArtifactDeclaration.kind` + `StandardDeclaration.id`; `is_binary` defaults `true` (documented simplification — design.md's `MediaDeclaration` carries no binary/text flag) |
| `.subset_validators(...)` (`SubsetValidatorEntry`) | `Deserializer::CONFORMANCE` (W1-A) — see conformance decision below |
| `.languages(...)` (5× `dsl::LanguageSpec`: document/op/diff/pack/spr roles) | `NativeCodecs.{snapshot,diff,mutations}: LanguagePair { text, binary }` — `snapshot` pairs Document+Pack roles, `mutations` pairs Ops+Spr, `diff` carries Diff text only (no binary diff role exists); `inferences: Option<LanguagePair>` is new, for a subset with an inference grammar |
| `.document_codec::<A>()`/`.document_codec_bare::<S,M>(...)` | `NativeCodecs.codec: store::ArtifactCodec` (built directly by the subset via `ArtifactCodec::of::<Snapshot,Mutation>`) |
| `.migrations(...)` (`store::DialectMigration`) | **Dropped, absorbed** — design.md §3: "standard migration (`gif@87a/*`→`gif@89a/*`) [is an] ordinary `IoEntry` row." No separate channel needed; a migration IS a `Deserializer`/`Serializer` pair between two dialects of the same standard, same as any other io hop. |
| `.composition::<Snapshot>()` (`child_slots`/`link_slots`) | **Dropped, unchanged mechanism** — the old struct's own doc already says "No registration function consumes these yet… captured here so the declaration is a complete manifest, not because `register_all` calls anything with them." Composition still reads `<Snapshot as ArtifactCompositionFields>` directly at runtime; there is nothing to register, so no field is needed in the new tree. |
| `.capability(...)` (`CapabilityRequirement`) | **Dropped from `ArtifactDeclaration`, unchanged mechanism** — design.md §2's `ArtifactDeclaration` literally has only `{kind, localization, standards}`, no capabilities field. A plugin that needs a capability still calls `PluginBuilder::capability(...)`/`.local_backbone_storage()` directly, exactly as today; that channel was never document-io-shaped in the first place. The ONE capability-shaped thing the new tree DOES compute is the per-surface Read/Write document capability (`capability_rows_for`, mirroring `.editor()`/`.viewer()`'s existing `Rights::Read`/`Rights::Write` push) — that's surface-derived, not declaration-derived, so it lives in `SurfaceDeclaration.rights` + the commit walk, not on `ArtifactDeclaration` itself. |
| viewer/editor app registration (`.viewer::<V>()`/`.editor::<E>()`) | `SurfaceDeclaration` + `editor_surface<E>()`/`viewer_surface<V>()`, folded into the SAME `app_defs`/`app_schema_descriptors`/capabilities vectors `PluginBuilder` already threads |
| examples (`App::example_source`) | `SubsetDeclaration.examples: &'static [ExampleSource]`, attached to the EDITOR surface's `App` (the primary authoring surface) by the commit walk; not duplicated onto the viewer |

## Task 1.3 — conformance decision (resolves `📓️w1-a-report.md` openQuestion §3)

**Decision: `Deserializer::CONFORMANCE` (W1-A) is the ONE live conformance mechanism.** `IoDeclaration.conformance`
is present in the struct (design.md §2's literal shape, kept for API parity) but is **intentionally never read** by
`preflight_artifact_declarations`/`commit_artifact_declarations` — it is inert by construction, not silently dropped:
the field's own docstring (`IoDeclaration`, lines 13837-13856) states this plainly.

Why `Deserializer::CONFORMANCE` wins, not a coin flip:
1. **It is already fully implemented, tested, and wired.** W1-A's `deserializer_entry`/`deserializer_entry_text`
   already call `T::CONFORMANCE` right after every successful `deserialize` and fold its diagnostics into the
   `IoOutcome` (`🚪️io/component.rs` lines 2513-2536, proven by `conformance_runs_after_deserialize`). Nothing needed
   changing in `🚪️io/**` to make this work — it already does.
2. **`IoDeclaration.conformance` would need real plumbing I am not allowed to write.** design.md describes it as
   running "after every hop INTO that dialect" — i.e. inside `io_run`'s hop-folding loop. That loop lives entirely in
   `🚪️io/component.rs`, which is W1-A's boundary, explicitly out of mine ("Do NOT touch `🚪️io/**`"). Wiring it would
   mean either editing that file (forbidden) or reaching into it through some side-channel registry (a second,
   uncoordinated barrier — exactly what the ticket tells Task 2 NOT to invent).
3. **Shipping both live would duplicate one job.** Both hooks answer the exact same question — "is this decoded value
   conformant?" — at the exact same moment (right after a successful decode into the dialect). There is no scenario
   where a subset needs BOTH; one is strictly sufficient.

If a later wave DOES touch `🚪️io/**` and wants the payload-level hook live too, the struct shape already accommodates
it with no further breaking change — this is a genuinely reversible, low-cost deferral, not a design dead end.
`✳️strict`'s conformance profile in the Task 4 fixture (below) exercises `Deserializer::CONFORMANCE` directly,
proving the decision in code, not just in prose.

## Testkit laws (Task 4)

New sub-region `🔖️DeclarationTestkit` inside the EXISTING `pub mod testkit { … }` (lines **6886-6955**, right after the
existing `👁️✏️SurfaceTestkit` sub-region, before `testkit`'s own closing brace):

- `assert_declaration_tree_registers_all(plugin_id, declaration)` — builds a REAL `Plugin` via
  `Plugin::builder(id).declare_artifact(declaration).try_build()`, then asserts every subset's schema id is in
  `semio_framework_schema::artifact_schema_descriptor_registered`, every `IoEntry`'s `(from,into)` pair is in
  `io_mechanism::io_entries()`, and every editor/viewer surface id is in `plugin.manifest.apps`.
- `assert_declaration_registration_is_atomic(plugin_id, invalid)` — snapshots the schema-registry count and
  `io_entries().len()`, asserts `try_build()` returns `Err`, then asserts BOTH counts are unchanged.
- `assert_subset_declaration_ids_are_derived(&declaration)` — pure, no registration: asserts every subset's
  `editor.definition.id`/`viewer.definition.id` equals `surface_app_id(&dialect, role)`.

Real output (`cargo nextest run -p semio-framework-plugin --lib -E 'test(fixture::)'`):

```
test component::app::declarations::fixture::a_conflicting_declaration_leaves_zero_rows_behind ... ok
test component::app::declarations::fixture::declaring_registers_schema_io_and_surfaces ... ok
test component::app::declarations::fixture::ids_are_derived_from_the_dialect ... ok
test component::app::declarations::fixture::io_route_finds_the_conformance_profile_hop ... ok
test component::app::declarations::fixture::open_mutate_save_round_trips_through_the_generic_snapshot_builder ... ok
Summary [0.06s] 5 tests run: 5 passed, 225 skipped
```

## Task 3 — `SnapshotBuilder<S, M>`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, lines **14527-14580**, region `🔖️SnapshotBuilder`.
Generic `impl<S, M> ArtifactBuilder for SnapshotBuilder<S, M>` where `S: Default + Clone + PartialEq + Serialize +
DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack`, `M: Mutation<S>`, `M::Diff: MutationDiff<S>`. Did NOT
touch the `ArtifactBuilder` trait itself (still `empty/from_snapshot/from_text/from_binary/mutate/absorb/build`,
99 external call sites untouched, per instructions). `mutate` reuses `protocol::MutationOutcome::apply_to` (this
crate's own established "apply once, downgrade a rejection to a Fatal message, return the outcome intact" idiom —
`🎮️command/component.rs` lines 259-275) instead of re-deriving that logic. A trivial subset now writes
`type Construction = SnapshotBuilder<Snapshot, Mutation>;` and nothing else.

## End-to-end fixture (Task 4)

Module path: **`crate::app::declarations::fixture`** (in the plugin crate — i.e.
`semio_framework_plugin::component::app::declarations::fixture` from outside, `#[cfg(test)] pub(crate) mod fixture`,
lines 14145-14523). A synthetic artifact `s.testkit.w1c-fixture` with:
- **Standard `"1"`**, two subsets: `any` (base, `Std1AnySnapshot{value:i32}`) and `strict` (a **conformance profile**
  of `any`) — `strict`'s `IoDeclaration.entries` carries a `Deserializer<Std1StrictSnapshot>` FROM `any`'s dialect
  whose `CONFORMANCE` rejects a negative `value`, plus the inverse `Serializer` back into `any`.
- **Standard `"2"`**, one independent subset `any` — proves the walk covers >1 standard, not just >1 subset.

Each subset owns its own `Snapshot`/`Diff`/`Mutation`/`Command`/`ArtifactEditor`/`ArtifactViewer` (macro
`fixture_channel!` factors the mechanical boilerplate — each subset's TYPES are still distinct, only the shape
repeats, matching design.md rule 2 "a subset never uses a sibling subset's types"). `NativeCodecs.{snapshot,diff,
mutations}` are `LanguagePair { text: None, binary: None }` (plain `serde_json` codecs, no hand-authored grammar —
legal per that type's own doc). Tests (all passing, see above):
- `ids_are_derived_from_the_dialect` — testkit law 3.
- `declaring_registers_schema_io_and_surfaces` — testkit law 1, through a real `try_build()`.
- `a_conflicting_declaration_leaves_zero_rows_behind` — testkit law 2 (forces a genuine schema-descriptor content
  conflict, not just an id collision — a byte-identical duplicate descriptor is legally idempotent, not a conflict).
- `open_mutate_save_round_trips_through_the_generic_snapshot_builder` — proves Task 3's `SnapshotBuilder` end to end:
  `from_binary` → `mutate` → `build`.
- `io_route_finds_the_conformance_profile_hop` — declares the fixture through a real `Plugin::builder(...)
  .declare_artifact(...).try_build()`, then calls `semio_framework::io::io_mechanism::io_route`/`io_run` directly:
  routes `std1-any → std1-strict` in exactly 1 hop and decodes into `Std1StrictSnapshot`.

Plugin waves reading this ticket should read `crate::app::declarations::fixture::build_declaration` as the reference
shape for a real subset's `component.rs`.

## verification

All commands from `/Users/ueli/Documents/semio`, `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"`.

- `cargo check -p semio-framework-plugin --lib` → clean, 0 errors. Only the 2 pre-existing dead-code warnings
  (`child_slots`/`link_slots` on the OLD `ArtifactDeclaration`, and 4 unread `PluginRuntimeRegistry` fields) — both
  untouched by this wave, present before I started.
- `cargo check -p semio-framework-plugin --lib --tests` → clean, 0 errors, 11 warnings total, **all** in the
  `derive_artifact_facets!`/`subset!` macro call sites the W1-A baseline already named (`🧪️w0-baseline-plugin.txt`) —
  zero warnings trace to any line I added (confirmed by grepping the warning line numbers against my regions).
- `cargo nextest run -p semio-framework-plugin --lib --no-fail-fast` →
  **230 tests run: 226 passed, 4 failed, 0 skipped.** Against the ticket's known baseline (225 run / 221 passed /
  4 failed): +5 tests, all mine, all passing (`fixture::*`); the SAME 4 pre-existing failures, same names, same file
  (`artifact_definition_contract_tests` ×3, `plugin_builder_contract_tests::merge_channel_commands_…` ×1), all inside
  `🔌️plugin/🦀️component.rs` code I never touched (existing `ArtifactDeclaration`/plugin-runtime regions, not my new
  regions). Not made worse; not asked to fix.
- `cargo nextest run -p semio-framework-plugin --lib -E 'test(fixture::)'` → **5 tests run: 5 passed, 225 skipped**
  (real output pasted above).
- `cargo check -p semio-framework-plugin --target wasm32-wasip2` (target confirmed via `rust-toolchain.toml`'s
  `targets = ["wasm32-unknown-unknown", "wasm32-wasip2"]` and `📜️script.ts`'s `PLUGIN_WASM_TARGET_DIR`) →
  **clean, 0 errors** (`Finished \`dev\` profile [unoptimized] target(s) in 7m 06s`, exit code 0 — genuine cold
  build, whole dependency tree, ~7 min). Only 2 warnings, both pre-existing and identical to the native run
  (`child_slots`/`link_slots` at line 2812, `PluginRuntimeRegistry` unread fields at line 3822) — zero warnings or
  errors trace to any region I added. This satisfies the "native cargo misses wasm-gated code" caution: every new
  region compiles under the real `wasm32-wasip2` guest target, not just natively.

## sharedFileRequests

None. Every change landed inside my two boundary files; no patch files needed in `🔧️patches/`.

## openQuestions

1. **`IoDeclaration.conformance` is a permanently-inert field until a later wave touches `🚪️io/**`.** This is a
   deliberate, boundary-respecting choice (see the conformance decision above), not an oversight — flagging so W6 (or
   whichever wave next touches `🚪️io/component.rs`) knows the field is there, unread, and either gets wired for real
   or gets deleted outright once `Deserializer::CONFORMANCE` is confirmed sufficient forever.
2. **`FormatDescriptor.is_binary` always synthesizes to `true`** in `format_descriptor_of` — design.md's
   `MediaDeclaration` carries no binary/text flag, so I picked the majority case (most artifact standards are file
   formats). A text-native standard (e.g. a pure-DSL artifact with no binary pack) would get a technically-wrong
   `is_binary: true` row; harmless today (nothing in this crate reads that field back), but worth a real field if a
   later wave discovers a consumer that cares.
3. **Per-surface `mutation_roster` (`SurfaceDeclaration.mutation_roster: Option<fn() -> (&'static str, &'static
   [SemanticDescriptor])>`) is declared but never read by the commit walk** — the old system's equivalent
   (`.viewer_mutation_roster::<V>()`/`.editor_mutation_roster::<E>()`) is an explicit OPT-IN a plugin author calls
   separately; design.md's struct bundles the field directly on `SurfaceDeclaration` but doesn't say the commit walk
   must auto-register it. I left it unread (present for shape parity, same treatment as `IoDeclaration.conformance`)
   rather than guess whether auto-registering breaks the "opt-in" semantics the old system deliberately has. A
   plugin wave that needs this should flag it here rather than assume either behavior.
4. **`preflight_io_entries`'s duplicate check is coordinate+fidelity only**, not the full `same_io_entry` equality
   `io_mechanism` uses internally (which also compares `sniff`/`run` fn-pointer identity) — that finer-grained check
   isn't exposed as a public read API. In practice this only under-detects a pathological case (two DIFFERENT
   entries at the same coordinate and fidelity but different behavior), which `io_register`'s own internal
   preflight+commit would still catch at commit time (just not predicted at MY preflight time) — see the
   "all-or-nothing" section above for why this narrow gap is inherent, not fixable without touching `🚪️io/**`.

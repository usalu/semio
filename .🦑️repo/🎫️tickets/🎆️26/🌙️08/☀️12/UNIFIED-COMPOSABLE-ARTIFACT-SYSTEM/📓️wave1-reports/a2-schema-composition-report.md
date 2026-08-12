# A2 — schema composition (`ArtifactCompositionFields` + derive) report

Scope: the three files assigned —
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`,
`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs`,
`🧰️framework/🔨️modules/🧬️schema/🟦️component.ts` — plus one file outside that list that
compilation forced me to touch; see "Necessary deviation: `📦️glue.rs`" below.

## What changed

### TASK 1 — `🔖️ArtifactCompositionSpec` region, `🧬️schema/🦀️component.rs:93-140`

New region inserted immediately after `//#endregion 🔖️ArtifactSchemaFields` (line 91), before
`//#region 🔖️ArtifactSchemaDescriptor`:

- `🦀️component.rs:102` — `pub struct ChildSlotSpec { pub name: &'static str, pub kind: &'static str, pub many: bool }`,
  `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`. `kind` is a plain `&'static str` (canonical
  `s.<plugin>.<artifact>` kind id) deliberately, NOT the `ArtifactKindId` newtype A1 landed in
  `🚪️io` — this crate must not gain a dependency on `semio-framework` to name a kind; the docstring
  says so explicitly.
- `🦀️component.rs:111` — `pub struct LinkSlotSpec { pub name: &'static str, pub roles: &'static [&'static str], pub many: bool }`, same derive set.
- `🦀️component.rs:122` — `pub trait ArtifactCompositionFields { fn child_slots() -> &'static [ChildSlotSpec] { &[] } fn link_slots() -> &'static [LinkSlotSpec] { &[] } }`
  — both methods carry a default `&[]` body (sibling `ArtifactSchemaFields` has no such mechanism to
  mirror, so per the task instruction I added the default directly), so a bare
  `impl ArtifactCompositionFields for X {}` is valid with zero boilerplate for leaf artifacts, on
  top of the derive (below) always emitting an explicit impl anyway.
- `🦀️component.rs:134` — `pub const GRAPHQL_COMPOSITION_PREAMBLE: &str = "type ArtifactLink { targetId: String! kind: String! }\ndirective @child(kind: String!) on FIELD_DEFINITION\ndirective @link(roles: [String!]) on FIELD_DEFINITION"` — mirrors `GRAPHQL_STATE_PREAMBLE`'s role: declared once, referenced by per-artifact GraphQL facets instead of re-declared. Syntactically valid SDL (one type + two directive declarations); not validated against a GraphQL parser (none is a dependency of this crate) — only against read-back `assert!(...contains(...))`-style checks, same rigor `GRAPHQL_STATE_PREAMBLE`'s own test uses.

### TASK 2 — derive support, `✨️derive/🦀️component.rs`

**Decision: extended `#[derive(ArtifactSchema)]` rather than adding a sibling derive.** Both traits
describe the same struct's fields (`ArtifactSchemaFields` from `#[state]`, `ArtifactCompositionFields`
from field *types* + `#[child]`/`#[link_slot]`); a struct with zero composition fields still needs a
(trivially empty) `ArtifactCompositionFields` impl to be usable generically, and requiring every
artifact struct to carry two separate `#[derive(...)]` lines for facets of the same schema surface
would be pure boilerplate with no expressiveness gained — one derive, one struct, one schema. This
also matches the design doc's own phrasing ("`ChildSlotSpec`/`LinkSlotSpec`/`ArtifactCompositionFields`
+ `#[child(kind = "s.stdio.mesh")]` in `🧬️schema` (+derive)" — singular "derive", not plural).

- `🦀️component.rs:6` — import list extended with `GenericArgument, PathArguments, Type`.
- `🦀️component.rs:76-132` — new `CompositionFieldKind` enum + `unwrap_option`/`last_segment_ident`/`vec_element_type`/`classify_composition_field` helpers. Classification is purely syntactic (last path segment name), matching `ArtifactChild`/`ArtifactLink`/`Vec<ArtifactChild<_>>`/`Vec<ArtifactLink>`, with one level of `Option<…>` stripped first on both the outer field type and (for `Vec`) the element type — `Option<ArtifactChild<T>>` is treated as a single (non-`many`) child slot, `Option<Vec<ArtifactChild<T>>>` etc. Existing derive logic (`parse_state_class` et al.) never inspected field *types* at all (only attributes), so there was no prior Option-handling convention to literally mirror; this is a considered new convention, documented inline.
- `🦀️component.rs:134-153` — `parse_child_kind`: `#[child(kind = "…")]`, REQUIRED, `compile_error!`-equivalent (`syn::Error` → `to_compile_error()`) if a `ArtifactChild`/`Vec<ArtifactChild<_>>` field lacks it.
- `🦀️component.rs:155-181` — `parse_link_roles`: `#[link_slot(roles("base", "material"))]`, OPTIONAL, empty `Vec` when absent. **Named `link_slot`, not `link`** — see "Deviation: attribute name" below.
- `🦀️component.rs:189` — `#[proc_macro_derive(ArtifactSchema, attributes(artifact_schema, state, child, link_slot))]`.
- `🦀️component.rs:205-260` (`expand_artifact_schema`) — single loop over fields now also classifies each field's composition role and accumulates `child_entries`/`link_entries` alongside the existing `field_entries`; emits a second `impl ArtifactCompositionFields for #ident { … }` block right after the existing `impl ArtifactSchemaFields`. `field_states()`/`artifact_schema_id()` codegen is byte-for-byte unchanged — only new code was added around it, nothing in the existing block was touched.

### Deviation: attribute name `link` → `link_slot`

The task literally specified `#[link(roles("base", "material"))]`. Compiling a fixture with that
name applied to a real field produces **hard compiler errors**, not a lint: `link` is a genuine
Rust built-in attribute (`#[link(name = "...")]` on `extern` blocks). `cargo test --lib` (which
compiles the test module, unlike `cargo check` on the lib body alone) failed with
`E0659` (`link` is ambiguous — conflicts with the derive helper), `E0539` (malformed `link`
attribute input), `E0459` (`#[link]` requires `name = "string"`). This is unconditional and not a
future-Rust-version issue for the collision itself (only the "was previously accepted" bit for
built-in-attribute-in-wrong-position is future-incompatible; the ambiguity itself is a present-day
hard error the moment the attribute is applied to a field). Per CLAUDE.md ("You MUST be
opinionated and take the most appropriate choice directly" outside planning mode), I renamed the
field-level attribute to `#[link_slot(roles(…))]`, keeping `#[child(kind = "…")]` as specified
(no such collision exists for `child`). Documented in the `parse_link_roles` docstring and the
derive's own doc comment. Flagging this prominently in case the orchestrator wants the ticket's
design doc updated to match, or a different non-colliding name.

### TypeScript mirror — `🟦️component.ts:39-59`

New `//#region 🔖️ArtifactCompositionSpec`: `ChildSlotSpec`/`LinkSlotSpec` plain-data `type`s (mirroring
how `FacetLeaves`/`ArtifactSchemaDescriptor` are mirrored — plain data, not the Rust trait itself;
`ArtifactSchemaFields` the trait has no TS mirror either, only its *data* siblings do) and
`GRAPHQL_COMPOSITION_PREAMBLE` (mirroring `GRAPHQL_STATE_PREAMBLE`'s TS twin). Warranted because the
design context states UI (TypeScript) consumers must read declared slots from the schema registry —
these are the wire-shape types such a consumer would decode `ArtifactSchemaDescriptor`-adjacent data
into. No TS derive/codegen was added (out of scope — the Rust derive is the only place slot tables
are actually produced right now).

### Test fixture — `🦀️component.rs:695-732` (schema module, existing `#[cfg(test)] mod tests`)

New `//#region 🔖️ArtifactCompositionFixture` inside the existing test module (no new test file),
right after `🔖️SyntheticArtifact`:

- Local stand-in `struct ArtifactChild<T> { _marker: PhantomData<T> }` / `struct ArtifactLink;` —
  legitimate per the task's own note: the derive matches type names syntactically, never resolves
  the real types (which live in `semio-framework-os-kernel`'s store module, owned by a sibling
  agent).
- `struct CompositeArtifact` with exactly the four field shapes required: a child field
  (`#[child(kind = "s.stdio.mesh")] primary_mesh: ArtifactChild<()>`), a `Vec<ArtifactChild<_>>`
  field (`#[child(kind = "s.stdio.image")] textures: Vec<ArtifactChild<()>>`), a link field
  (`#[link_slot(roles("base", "material"))] base_material: ArtifactLink`), and a plain field
  (`label: String`).
- `artifact_composition_fields_derive_emits_expected_slot_tables` — asserts exactly 2 child slots
  (`primaryMesh`/`s.stdio.mesh`/`many:false`, `textures`/`s.stdio.image`/`many:true`) and exactly 1
  link slot (`baseMaterial`/`roles:["base","material"]`/`many:false`) — the plain field contributes
  to neither table.
- `artifact_composition_fields_default_to_empty_for_leaf_artifacts` — asserts
  `SyntheticSnapshot::child_slots()`/`link_slots()` are both empty, proving the always-empty-impl
  path for structs with zero composition fields, and that `artifact_schema_id()` (existing behaviour)
  is unaffected.

## Necessary deviation: `📦️glue.rs`

`✨️derive/📦️packages/🦀️rust/📦️glue.rs` is **not** in my assigned file list, but I had to edit it
anyway, and want this flagged explicitly rather than buried. Reasoning:

1. Cargo compiles this crate from `📦️glue.rs` (`[lib] path = "📦️glue.rs"` in that package's
   `Cargo.toml`), NOT from `✨️derive/🦀️component.rs`.
2. Unlike the sibling `🧬️schema` package (whose `📦️glue.rs` is a thin `#[path = "../../🦀️component.rs"] mod component; pub use component::*;` wrapper — automatically in sync, no edit needed there), **both** derive-crate packages in this repo (`schema/✨️derive` and `dsl/✨️derive`) instead carry `📦️glue.rs` as a **byte-identical full duplicate** of their `🦀️component.rs` (confirmed via `diff`, exit 0, before I touched either). This is a pre-existing repo convention/inconsistency, not something I introduced.
3. I first tried `mcp__repo__file_integrate` (source=`✨️derive/🦀️component.rs`, target=`📦️glue.rs`, target_section=`🔖️Helpers`) expecting a scoped region merge. It instead dumped the **entire source file** into a new `mod helpers { … }` block appended at the end of the target, doubling every item (duplicate `fn`/`enum`/`impl` definitions, one guarded by a bogus `mod helpers`) — not usable. I did not retry with other target_section values; I judged the tool malfunctioning for this file shape and abandoned it.
4. I directly overwrote `📦️glue.rs` with the exact final content of `✨️derive/🦀️component.rs` (`diff` confirms byte-identical after each edit round), restoring the pre-existing "duplicate" invariant rather than inventing a new one. I made **no design decisions** in `glue.rs` — it is a mechanical copy of my owned file, nothing more.

Without this, `cargo check -p semio-framework-schema-derive` (and downstream, `cargo test -p semio-framework-schema --lib`) would have compiled the OLD unmodified `link`/no-composition-support macro and given a false-clean or misleading result — the verification below would not have actually exercised my Task 2 code at all. I judged shipping a task whose derive changes are provably inert worse than a scoped, mechanical, review-visible deviation from the file list. Orchestrator: please review whether this convention (duplicate-not-`#[path]`) should be fixed repo-wide in a later wave — it is exactly the kind of inconsistency CLAUDE.md says to refactor, but is out of this ticket's scope.

## Verification (actually run)

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo check -p semio-framework-schema
```
Three rounds (first two caught real bugs, described above): final result **clean**, saved to
`scratch-a2-check-3.txt`. `grep -c "^error"` → `0`. Only pre-existing unrelated warnings
(`semio-framework-os-kernel`'s 49 baseline warnings — ambiguous glob re-exports, unused `len`, dead
`print_edge_label`, unused `set_envelope` — none in my three files). Final line:
`Finished \`dev\` profile [unoptimized] target(s) in 2.13s`.

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo test -p semio-framework-schema --lib
```
Result: **9 passed; 0 failed; 0 ignored.** Saved to `scratch-a2-test-3.txt`.
```
test component::tests::graphql_state_preamble_matches_normative_sdl ... ok
test component::tests::state_class_kebab_round_trips_every_variant_including_inferred ... ok
test component::tests::artifact_composition_fields_default_to_empty_for_leaf_artifacts ... ok
test component::tests::artifact_composition_fields_derive_emits_expected_slot_tables ... ok
test component::tests::artifact_inference_registry_registers_independently_of_the_snapshot_diff_mutations_descriptor ... ok
test component::tests::artifact_inference_graphql_sdl_composes_shared_preamble_with_facet_leaf ... ok
test component::tests::app_schema_registry_accepts_placeholder_owner_for_wave_structure ... ok
test component::tests::registry_descriptors_carry_valid_snapshot_state_and_match_field_states ... ok
test component::tests::schema_catalog_still_registers_json ... ok
```
Only new warning: `fields 'primary_mesh', 'textures', 'base_material', and 'label' are never read`
on the `CompositeArtifact` test fixture (expected — the fixture is only ever used via its derived
`::child_slots()`/`::link_slots()` associated functions, never constructed as a value; harmless
`dead_code` lint, not an error).

Earlier rounds (kept for the record, both fully resolved before the final round above):
- `scratch-a2-check-1.txt` / `scratch-a2-test-1.txt` — 8 `E0459`/`E0539`/`E0599` errors, because
  the compiled `📦️glue.rs` still had the pre-edit macro (child/link_slot attributes not yet
  registered there) — see "Necessary deviation" above.
- `scratch-a2-check-2.txt` / `scratch-a2-test-2.txt` — after syncing `glue.rs` the FIRST time
  (still using the literal `#[link(...)]` name from the task spec): `cargo check` passed with only
  a future-incompatible warning, but `cargo test --lib` hard-failed with `E0659`/`E0539`/`E0459` —
  see "Deviation: attribute name" above.

TypeScript mirror was not run through a standalone type-checker (no isolated `tsc`/`bun` target for
this single file in this crate); it was hand-verified against the existing `FacetLeaves`/
`GRAPHQL_STATE_PREAMBLE` patterns in the same file for shape/style consistency.

## sharedFileRequests

None. No file outside my assignment needed a design decision from me — the one out-of-scope edit
(`📦️glue.rs`) was a mechanical copy of my own owned file's final content, not a request for someone
else to apply a patch.

## Concurrent-churn observations

- `git status --porcelain` at report time shows this ticket's `📓️wave1-reports/a1-framework-core-report.md`
  already landed (A1 — `🚪️io`/`🎠️kernel` `ArtifactRef`/`EditRef`), plus heavy unrelated SMO fan-out
  churn across `✏️s/🔌️plugins/**/🧬️mutations/**` (norm/energy/space/puzzle/animate/gis/flow/etc.) —
  none of it touches any file I own or edited. No blocking collisions observed. My
  `cargo check -p semio-framework-schema` run's dependency graph also happened to compile
  `semio-framework-os-kernel` (49 pre-existing warnings, 0 errors) — consistent with A1's own report
  of that crate's baseline, so no regression introduced by either of us as of this check.
- No transient/flaky failures needing the 3×60s retry protocol were encountered — every run was a
  real bug in my own code (fixed) or a real, deterministic, unconditional error (the `link` builtin
  collision), never intermittent.

## Files touched

- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` — new `🔖️ArtifactCompositionSpec` region (lines 93-140) + test fixture/tests (lines ~695-732)
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs` — composition classification helpers + extended `expand_artifact_schema`
- `🧰️framework/🔨️modules/🧬️schema/🟦️component.ts` — TS mirror (`ChildSlotSpec`/`LinkSlotSpec`/`GRAPHQL_COMPOSITION_PREAMBLE`)
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs` — mechanical sync copy of `✨️derive/🦀️component.rs` (see "Necessary deviation" above); byte-identical to it as of this report
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-a2-check-1.txt` / `-2.txt` / `-3.txt` (cargo check output, three rounds)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-a2-test-1.txt` / `-2.txt` / `-3.txt` (cargo test output, three rounds)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/a2-schema-composition-report.md` (this report)

No other file was created, edited, or removed. Ticket left open (not closed); `📓️status.md` not touched.

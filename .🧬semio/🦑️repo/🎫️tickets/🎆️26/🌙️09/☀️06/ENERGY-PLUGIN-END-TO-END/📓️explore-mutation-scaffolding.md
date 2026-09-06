# 🏗️ Mutation scaffolding recipe — everything one new `s.energy.model` mutation kind requires

Scope: exact recipe (files + gates) for adding one new mutation kind to `EnergyModelMutation`
(`✏️s/🔌️plugins/🔋️energy`), derived by reading `♻️replace-model` (energy's only kind) end to end,
comparing with `📸️remodel` (34-35 handcrafted kinds), and tracing every validator that inspects a
mutation directory. All paths below are relative to the repo root unless absolute.

## 0. TL;DR for Opus workers

To add ONE new mutation kind `<emoji><verb>-<noun>` under
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:

1. Create the leaf directory `🧬️mutations/<emoji><verb>-<noun>/` with 8 files + 2 subdirs (§1).
2. Add ONE line to each of 6 **aggregate** files at the `🧬️mutations/` root (§2).
3. Add up to 7 `#[path]` mount lines to the crate entry
   `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs` (§4) — one `pub mod <slug>` block per kind.
4. Pick a **new, unused** `u32` for `BINARY_TAG` (§1.6) — energy's registry is small (1 entry, tag
   `0`) so collisions are currently impossible, but with 30-50 kinds this becomes a real hazard.
5. Do **not** touch the plugin-root descriptor (`✏️s/🔌️plugins/🔋️energy/🔣️.json`) or
   `.editor_mutation_roster()` unless you are also adding a new user-facing editor command — see §5,
   they are a different, decoupled layer.
6. **Fix the stale oracle-catalog path first** (§6.1) and **fix the stale crate-mount path** (§6.2) —
   both are live, currently-broken bugs in the ONE existing kind, and every new kind's structural
   test will inherit whichever bug you don't fix.

## 1. Per-kind leaf directory — file-by-file contract

Modelled on `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/`.

Directory name MUST match `taxonomy.json`'s `mutationDirectoryPattern`:
`^.+️[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:18197`) — i.e. an emoji that
carries the **U+FE0F variation selector**, then kebab-case `verb-noun` (at least two hyphen-joined
words). `♻️replace-model` is `♻` U+267B + U+FE0F + `replace-model`. **Hazard:** many emoji render
identically with and without U+FE0F; copy-pasting one without the selector silently fails the regex
and the directory will not be recognized as a mutation owner by any discovery tool.

| # | File | Role | Contract (from `♻️replace-model/`) |
|---|------|------|--------------------------------------|
| 1 | `🦀️.rs` | Payload struct + `protocol::MutationKind` impl | `#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]` struct with `#[mutation_leaf(contract = ::protocol)]`; `impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for X { const SEMANTICS: protocol::SemanticDescriptor = ...; fn diff(...) { super::diff::diff(self, base) } fn inverse(...) { super::inverse::inverse(self, base) } fn label(...) -> String }`; also exports `pub const KINDS: &[&str] = &["<kind>"]` and a `energy_model_mutation_report_json` test bridge fn + `#[cfg(test)] mod tests` with law assertions (`semantic_descriptor_and_inverse_are_complete`, `inverse_and_absorb_laws_hold`). See `♻️replace-model/🦀️.rs:1-114`. |
| 2 | `🔺️diff/🦀️.rs` | Sparse-diff construction | `pub fn diff(payload: &X, base: &Snapshot) -> protocol::MutationOutcome<Diff>`. May short-circuit to `protocol::MutationOutcome::empty().warn(code, msg)` for a documented no-op. `♻️replace-model/🔺️diff/🦀️.rs:15-21`. |
| 3 | `↩️inverse/🦀️.rs` | Inverse mutation construction | `pub fn inverse(payload: &X, base: &Snapshot) -> Vec<EnergyModelMutation>`. `♻️replace-model/↩️inverse/🦀️.rs:10-14`. |
| 4 | `📝️text/🦀️.rs` | Stable text opcode constant | `pub const TEXT_OPCODE: &str = "<kind>";` — energy ALSO builds a private shadow enum `EnergyModelMutationDsl` + `impl OpText`/`impl OpBinary` for the whole `EnergyModelMutation` HERE (not per-kind) — see §3 hazard, this is unusual vs. remodel. `♻️replace-model/📝️text/🦀️.rs:1-92`. |
| 5 | `💾️binary/🦀️.rs` | Stable binary tag + payload codec | `pub const BINARY_TAG: u32 = <n>;` + `encode_payload`/`decode_payload` (`pack::json` round trip). `♻️replace-model/💾️binary/🦀️.rs`. |
| 6 | `🔣️.json` | Language-neutral descriptor | Fixed shape: `schemaVersion:1, owner, semanticKind, displayName, emoji, aggregateVariant, payloadSchema, textOpcode, binaryTag, invertibility, diffParticipation, outcomeClasses[], composition, requiredLanguageSurfaces[]`. `requiredLanguageSurfaces` for `♻️replace-model` = `["rust","typescript","graphql","protobuf","json-schema","text","binary"]` — this is the checklist of which sibling files below are mandatory. |
| 7 | `🧬️.schema.json` | JSON Schema for the payload | draft-07, `title` = variant name, mirrors the Rust struct 1:1 (camelCase fields, `additionalProperties:false`). |
| 8 | `🔗️.graphql` | GraphQL input type | `input <Variant>Input { mutation: String! ...fields }`. |
| 9 | `🛰️.proto` | protobuf message | `message <Variant> { ...fields = N; }` inside `package semio.s.energy.model.mutation;`. |
| 10 | `🟦️.ts` | TS mirror interface | `export interface <Variant> { readonly mutation: "<kind>"; ...fields }`. |
| 11 | `↩️inverse/🟦️.ts` | TS mirror of the inverse fn | pure function mirror, no I/O. |
| 12 | `🔺️diff/🟦️.ts` | TS mirror of the diff fn | pure function mirror; any Rust-side effectful detail (e.g. child-handle minting) is documented as "not representable here". |
| 13 | `🧪️tests/<case>/...` | Fixture case(s) | See §7 — 5 committed JSON files + 1 `🦀️.rs` per case. |

Energy additionally carries organizational facets only at the aggregate root, not per-kind:
`📖️.grammar.semio` (handcrafted text grammar) and `📡️.protocol.semio` (handcrafted binary protocol,
inside `💾️binary/`) plus alternate-representation stubs (`🅰️.g4`, `🔤️.ebnf`, `🔠️.abnf`, `🥋️.ksy`,
`🌶️.spicy`) — these are **aggregate-level, not per-kind**; a new kind extends the aggregate grammar
(§2) rather than adding its own grammar file.

## 2. Aggregate files that need exactly ONE new line per kind

At `🧬️mutations/` root (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`):

1. **`🦀️.rs`** — add one enum variant to `EnergyModelMutation` (`🦀️.rs:14-16`) + one `pub use
   super::<slug>::{...};` re-export line, mirroring remodel's pattern
   (`✏️s/🔌️plugins/📸️remodel/.../🧬️mutations/🦀️.rs:22-59` reexports every one of its 34 kinds by
   name). Energy's aggregate currently derives only `dsl::Mutations` (not `dsl::DslEnum` — see §3).
2. **`🔣️.json`** (aggregate) — add one `{ "$ref": "#/$defs/<Variant>" }` to `oneOf` and one `$defs`
   entry, same shape as remodel's 34-entry `oneOf` (`📸️remodel/.../🧬️mutations/🔣️.json:1-40+`).
3. **`🟦️.ts`** (aggregate) — add one import + union member: `export type EnergyModelMutation =
   ReplaceModel | <NewVariant> | ...;` (mirrors remodel's `RemodelingMutationTag` string-literal union,
   `📸️remodel/.../🧬️mutations/🟦️.ts:9-40+` — NOTE remodel's aggregate TS union is of camelCase TAG
   strings, not type names; check which convention energy's TS consumers expect before copying).
4. **`🔗️.graphql`** (aggregate) — add one `type <Variant> { ...fields }` + append to `union
   EnergyModelMutation = ReplaceModel | <Variant> | ...`.
5. **`🛰️.proto`** (aggregate) — add one `message <Variant> { ... }` + one `oneof op { ...
   <field> = N; }` entry inside `message EnergyModelMutation`. Remodel assigns oneof field numbers
   **sequentially in KINDS order starting at 1** (`📸️remodel/.../🧬️mutations/🛰️.proto:6-38`) — do the
   same for energy; do not reuse `replace_model = 1`.
6. **`📖️.grammar.semio`** (aggregate) — add one alternative to the `op = replace-model | <new-op>`
   production and one `<new-op> = "<kind>" <field>+ = ...` rule (energy's current grammar:
   `🧬️mutations/📖️.grammar.semio:8-9`). **Hazard found:** remodel's own aggregate grammar
   (`📸️remodel/.../🧬️mutations/📖️.grammar.semio`) does **not** enumerate all 34 kinds — it still
   describes an old `mesh-op` vocabulary (`add-vertex`/`set-face`/`transform-mesh`/`merge-solid`) that
   matches none of the 34 real kinds. This file is evidently NOT kept in lockstep by any gate in the
   mature sibling — treat it as best-effort documentation, not a completeness contract, unless you
   find a test asserting otherwise.

Two more aggregate files carry a **registry array**, one entry per kind, generated FROM the per-kind
constants (not requiring hand-editing beyond mounting — see §4):
- `📝️text/🦀️.rs`: `TEXT_OPCODE_REGISTRY: &[(&str, &str)]` — energy's current form is a hand-written
  array literal (`🧬️mutations/📝️text/🦀️.rs:12`), one `("<Variant>", super::<slug>::text::TEXT_OPCODE)`
  tuple per kind — this must be extended by hand for every new kind (it's not derived automatically).
- `💾️binary/🦀️.rs`: `BINARY_TAG_REGISTRY: &[(&str, u32)]` — same shape
  (`🧬️mutations/💾️binary/🦀️.rs:7`), one `("<Variant>", super::<slug>::binary::BINARY_TAG)` tuple per
  kind.

## 3. Architecture divergence: energy vs. remodel (important for a 30-50-kind scale-up)

Energy and remodel do **not** share the same wire-codec architecture, despite both deriving
`dsl::Mutations`:

- **Remodel** (`📸️remodel/.../🧬️mutations/🦀️.rs:17`) derives `dsl::DslEnum` **on the aggregate
  enum itself**, and its aggregate `📝️text/🦀️.rs` / `💾️binary/🦀️.rs` are thin wrappers
  (`encode_op`/`decode_op` forwarding to the derive's `OpText`/`OpBinary` impls, generated from
  `dsl::DslVariants`). There is **no per-kind `TEXT_OPCODE`/`BINARY_TAG` constant and no
  `TEXT_OPCODE_REGISTRY`/`BINARY_TAG_REGISTRY`** anywhere in remodel — those are handled entirely by
  the derive macro scanning the enum's variant order.
- **Energy** (`🧬️mutations/🦀️.rs:11`) derives only `dsl::Mutations` (no `dsl::DslEnum`) on the
  aggregate, and hand-builds a **private shadow enum** `EnergyModelMutationDsl` inside
  `♻️replace-model/📝️text/🦀️.rs:12-14` with manual `to_dsl`/`from_dsl` conversion functions, PLUS the
  explicit `TEXT_OPCODE_REGISTRY`/`BINARY_TAG_REGISTRY` arrays at the aggregate root.

This means: **do not silently graft remodel's registry-free pattern onto energy's crate, or vice
versa** — with only one kind so far it is unclear whether energy's registry-array pattern is
deliberate (project convention for this newer crate) or incidental (an older/different generation
than remodel). Whichever convention is chosen, apply it uniformly to all 30-50 new kinds — mixing
the two per-kind would break the one shadow enum's exhaustive match arms (`♻️replace-model/📝️text/🦀️.rs:48-56`
is a manual `match` per variant, so every new kind requires one new match arm in `to_dsl`/`from_dsl`
in ADDITION to everything else, if the current energy convention is kept).

## 4. Crate-entry `#[path]` mounts (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs`)

The mount block for the aggregate (lines 253-279) is:

```rust
#[path = "."]
pub mod mutations {
    #[path = ".../🧬️mutations/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = ".../🧬️mutations/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = ".../🧬️mutations/📝️text/🦀️.rs"]
    pub mod text;
    #[path = "."]
    pub mod replace_model {              // <- ONE block like this per kind
        #[path = ".../🧬️mutations/♻️replace-model/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = ".../🧬️mutations/♻️replace-model/🔺️diff/🦀️.rs"]
        pub mod diff;
        #[path = ".../🧬️mutations/♻️replace-model/↩️inverse/🦀️.rs"]
        pub mod inverse;
        #[path = ".../🧬️mutations/♻️replace-model/📝️text/🦀️.rs"]
        pub mod text;
        #[path = ".../🧬️mutations/♻️replace-model/💾️binary/🦀️.rs"]
        pub mod binary;
        #[cfg(test)]
        #[path = ".../🧬️mutations/♻️replace-model/🧪️tests/<case-dir>/🦀️.rs"]
        mod tests_<case_slug>;
    }
}
```

For each new kind add: one `pub mod <slug> { ... }` block (5-6 `#[path]` lines: component, diff,
inverse, text, binary, plus one `#[cfg(test)] mod tests_<case>` per fixture case). With 30-50 kinds
and (per §7) potentially 1-3 fixture cases each, this file will grow by roughly 150-300 mount lines.
`project.json`/`package.json` never reference these paths directly (per CLAUDE.md, only
`📜️script.ts` may be a permanent script; the crate entry is generated/hand-maintained Rust, not a
script).

## 5. Editor/descriptor consumption — decoupled from mutation-kind count

- `✳️any/✏️editor/🦀️.rs` defines `EnergyModelEditorCommand` (a **separate** enum: `SetStructureField`,
  `SetZoneCell`, `StartSimulation`, ...) — this is the UI-facing command vocabulary, NOT the semantic
  mutation vocabulary. `EnergyModelEditor`'s command handler (`✳️any/✏️editor/🦀️.rs:256`) always
  funnels into `EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json })` regardless of
  which editor command fired — i.e. today ALL edits degrade to one whole-model replace.
- `.editor_mutation_roster::<crate::editor::model::EnergyModelEditor>()`
  (`✏️s/🔌️plugins/🔋️energy/🦀️.rs:41`) is a **generic builder call** parameterized only by the editor
  type — it introspects the editor's associated mutation type automatically. **Adding a new
  `EnergyModelMutation` variant requires NO change here** as long as `EnergyModelEditor`'s associated
  types are unchanged.
- The plugin-root descriptor `✏️s/🔌️plugins/🔋️energy/🔣️.json` has window `actions` of
  `"kind": "mutation"` (e.g. lines 87, 963: `"Set Node"`/`"Set Cell"`) that map to
  **`EnergyModelEditorCommand` variants**, not to `EnergyModelMutation` kinds — their `semantics.effects`
  block is coarse (`writes: ["artifact:{self}"]`, no reference to the specific mutation kind name).
  **Conclusion: a new `EnergyModelMutation` kind only requires a descriptor change if you are ALSO
  exposing a brand-new user-facing editor command for it** — most of the 30-50 planned kinds (if they
  are internal/engine-driven, e.g. simulation-result replaces) will need **zero** descriptor changes.

## 6. Confirmed LIVE hazards (both currently true on disk, not hypothetical)

### 6.1 Stale oracle-catalog path in energy's structural-correspondence test — WILL PANIC

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/.../🧬️mutations/🦀️.rs:34`:
```rust
let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json"))
    .expect("language-neutral oracle catalog");
```
Resolves to `✳️any/🔣️oracle.json` — **this flat file does not exist**. The real oracle catalog lives
at `✳️any/🔮️oracle/🔣️.json` (confirmed present, 108 lines, `mutationCatalogs`/`kinds` etc). Remodel's
equivalent test uses the CORRECT current pattern:
`✏️s/🔌️plugins/📸️remodel/.../🧬️mutations/🦀️.rs:567`: `include_str!("../../🔮️oracle/🔣️.json")`.
Energy's test (`🦀️.rs:26-67`, `structural_correspondence_tests::direct_owner_descriptor_surfaces_and_catalog_correspond`)
uses the OLD/wrong flat-file convention and will `.expect(...)`-panic the moment it runs (unclear
whether it currently runs at all under `cargo test` given the crate's build state — this is a
static-analysis finding, not a confirmed test-run failure). **Fix this before scaffolding new kinds**,
or every new kind's presence will be validated against a test that can never pass.

### 6.2 Stale `#[path]` mount referencing a renamed/removed test directory — WILL FAIL TO COMPILE

Git history shows the fixture case directory was renamed (100% rename) at commit `3a6a9d6bfc`
(2026-09-05 22:02:04) from
`🧪️tests/🏛️degrades-an-empty-model-payload-to-a-no-op/` → `🧪️tests/🏛️degrades-an-empty-31360c/`.
The crate mount file was last touched at the OLDER commit `fe7c8a8f8b` and still reads (current HEAD,
no uncommitted diff):
`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs:276`:
```rust
#[path = ".../🧬️mutations/♻️replace-model/🧪️tests/🏛️degrades-an-empty-model-payload-to-a-no-op/🦀️.rs"]
mod tests_degrades_an_empty_model_payload_to_a_no_op;
```
On disk, **only** `🏛️degrades-an-empty-31360c/` exists today (verified via `find`/`ls`,
2026-09-06). The oracle catalog (`✳️any/🔮️oracle/🔣️.json:67`,
`"directoryName": "🧪️degrades-an-empty-model-payload-to-a-no-op"`) ALSO still names the OLD
directory name (note: a different, `🧪️`-prefixed spelling, not even matching either on-disk name
exactly). **Net effect: three sources of truth (crate mount, oracle catalog, and the actual
filesystem) currently name three subtly different things for what should be one canonical fixture
case.** This must be reconciled to a single name before adding more cases, or the "which directory is
canonical" question will recur for every new kind's fixtures. Given other Claude/Codex sessions are
concurrently active in this repo (per `git log`), re-verify this is still the state before editing —
it may already be in the middle of being fixed by a peer.

### 6.3 Binary tag / text opcode uniqueness is manual, not gate-enforced centrally

`taxonomy.json` and the central TS taxonomy library (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`,
10566 lines) define `mutationDirectoryPattern`, `mutationBehaviorFacetDirs`
(`["🦠️mutation","🔺️diff","↩️inverse"]`), `mutationOrganizationalFacetDirs`
(`["🧩️plan","📝️text","💾️binary","🧬️schema"]`), `mutationPayloadSchemaAuthority` (enforces exactly
one canonical payload schema per descriptor's `payloadSchema`/`semanticKind` fields), and
`mutationDomainOwners` (a per-entity CRUD-verb grouping used ONLY by `🏛️architect` and gltf's `♾️any`
subset — **energy and remodel are both flat, NOT in `mutationDomainOwners`** — do not add energy's
new kinds there unless a deliberate decision is made to switch conventions).
**None of these central definitions validate `binaryTag` or `textOpcode` uniqueness across a
crate's registry** (`grep` for `requiredLanguageSurfaces`/`semanticKind`/`binaryTag` inside the 6437-line
`🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` returned zero hits). Uniqueness is enforced only
per-crate, per-kind, by each aggregate's own hand-maintained
`BINARY_TAG_REGISTRY`/`TEXT_OPCODE_REGISTRY` arrays and by the `♻️replace-model/🦀️.rs`-style
`#[cfg(test)]` correspondence tests — **there is no automatic collision detector**. With 30-50 new
kinds being added by parallel workers, a shared coordination doc (which tags are taken) is needed, or
two workers WILL pick the same `BINARY_TAG`.

### 6.4 Emoji directory-name normalization (`️`) is load-bearing

Confirmed at `taxonomy.json:18197` (`mutationDirectoryPattern`) and consumed at
`🔍️discovery/🟦️.ts:2783`: `name === name.normalize("NFC") && new RegExp(taxonomy.mutationDirectoryPattern, "u").test(name)`.
A directory name must be NFC-normalized AND match the FE0F-variation-selector pattern. Copy-pasting
an emoji from a source that strips variation selectors (common when copying from some terminals/IDEs)
will produce a directory silently invisible to taxonomy discovery.

## 7. Fixture-case directory layout and its gate

Case dir: `♻️replace-model/🧪️tests/<case>/` with exactly:
- `🦠️mutation/🔣️.json` — the mutation payload, tagged form (`{"mutation":"replaceModel", ...}`).
- `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json` — full typed snapshots.
- `🔺️diff/🔣️.json` — the committed produced diff (all-null-default shape when a no-op).
- `🎯️outcome/🔣️.json` — `{"status": "applied"|..., "messages": [{"level":"warn"|..., "code": "..."}]}`.
- `🦀️.rs` — `#[cfg(test)]`-free plain test module (mounted via `#[cfg(test)]` at the CRATE ENTRY, not
  inside this file) with `include_str!` for all 5 JSON files and typically 7 assertion functions
  (confirmed in `🏛️degrades-an-empty-31360c/🦀️.rs:1-144`):
  1. forward-apply reproduces the committed after-snapshot AND composed child handles are stable
     (content addresses only change when they should).
  2. inverse law: applying every inverse step lands back on `before`.
  3. all three committed JSON documents (`before`, `after`, mutation payload) are canonical
     (`value_eq_ignoring_object_order` round-trip through `to_value()`).
  4. declared outcome (`status`/message code) matches what `Mutation::diff` actually produces.
  5. produced diff structurally equals the committed `🔺️diff/🔣️.json`.
  6. committed diff decodes to the real `Diff` type, re-encodes canonically, and (for a no-op)
     equals `Diff::default()`.
  7. committed diff alone, applied to `before`, reproduces `after`.
- Registered as `#[cfg(test)] mod tests_<slug>` at the crate entry (§4) — a case NOT mounted there is
  dead code, invisible to `cargo test`, regardless of its presence on disk (see §6.2).
- Also registered in the oracle catalog `✳️any/🔮️oracle/🔣️.json`'s
  `mutationCatalogs[].vectors[].scenarios[]` (`id`, `directoryName`) and in
  `mutationManifests[].mutations[]` (`id`, `capability`, `payloadSchema`, `outcomes`,
  `productionDispatch`, `oracleRequirements`) — a THIRD place a new kind/case must be named, distinct
  from the per-kind `🔣️.json` descriptor and the crate mount.

Per-repo taxonomy test fixtures that exercise this general mechanism (framework-level, not
energy-specific — read-only reference, not modified per new kind):
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/{🏗️mutation-scaffolding,📋️mutation-inventory,📡️mutation-reachability,✅️mutation-test-presence,🔐️mutation-codec-ownership,🪪️mutation-metadata,📥️mutation-input-carriers,🌱️mutation-root-discovery,🧬️mutation-type-origin,🔭️mutation-scope}/`
— each is a `🛂️schema.json` + `🧫️fixtures/🔣️.json` pair consumed by
`🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts` (7753 lines total; mutation-related
`describe`/`test` blocks start around line 3188, 3434, 5531, 6940, 7344, 7740). This is the
central TS taxonomy test suite — it validates STRUCTURAL rules (directory naming, mount/placement
correctness, scaffolding idempotency, source-index/catalog agreement) generically across ALL plugins,
not per-artifact business logic.

## 8. Gate commands (read-only listing — none were run)

From `📋️project.json` (root):
- `bun nx run verify` → `bun ./📜️script.ts verify` (forwards all args; top-level umbrella).
- `bun nx run verify-gate` → `bun ./📜️script.ts verify gate` (`dependsOn: ["mutation-outcome-law"]`).
- `bun nx run mutation-outcome-law` → `bun ./📜️script.ts verify mutation-outcome-law`.
- `bun nx run verify-taxonomy-report` → `bun ./📜️script.ts verify taxonomy report` (read-only report).
- `bun nx run verify-taxonomy-enforce` → `bun ./📜️script.ts verify taxonomy enforce` (blocking gate).
- `bun nx run verify-rust-warnings` → `bun ./📜️script.ts verify rust-warnings`.
- `bun nx run verify-interactivity` → `bun ./📜️script.ts verify interactivity`.
- `bun nx run verify-dependencies-freeze` / `verify-layering` — unrelated to mutations, listed for
  completeness since they also gate any crate-entry edit.
- Referenced only in code comments, not resolved to an nx target in this pass: "`fixtures generate`"
  (`♻️replace-model/🧪️tests/<case>/🦀️.rs:17` comment: "derived encodings come from `fixtures
  generate`") — grep of `📋️project.json` and top-level `📜️script.ts` found no exact `fixtures
  generate` target; likely invoked via the repo MCP tool (unavailable this session — CONNECT_TIMEOUT)
  or a subcommand not indexed by a plain string match. Confirm with a live MCP session before relying
  on it.
- Cargo-level (do not run per CLAUDE.md/task rules, but this is where a new kind's Rust tests surface):
  `cargo check -p semio-s-plugin-energy --lib --tests` / `cargo test -p semio-s-plugin-energy`.

## 9. Summary checklist (per new mutation kind)

1. Pick emoji+FE0F, verb-noun slug; verify against `mutationDirectoryPattern`.
2. Pick unused `BINARY_TAG` (coordinate across parallel workers — no central collision gate, §6.3).
3. Create leaf dir with 10 files (§1) + `🧪️tests/<case>/` with 5 JSON + 1 `🦀️.rs` (§7), at least once
   (add more cases for edge behaviors, e.g. reject/no-op paths, matching `♻️replace-model`'s pattern).
4. Add one line each to 6 aggregate files + extend 2 registry arrays (§2).
5. Add one `pub mod <slug> { ... }` block (5-6 `#[path]` lines + 1 per test case) to the crate entry
   (§4).
6. Register in `✳️any/🔮️oracle/🔣️.json`'s `mutationCatalogs`/`mutationManifests` (§7) — third
   registration point, easy to forget.
7. Leave editor/descriptor (`🔣️.json` at plugin root, `.editor_mutation_roster`) untouched unless
   also adding a new user-facing command (§5).
8. Before running any gate: fix §6.1 and §6.2 (both are pre-existing, currently-broken, and will
   contaminate every new kind's structural-correspondence test if left as-is) — re-verify their
   current state first, since peers may edit concurrently.
9. Run (or ask a foreground session to run) `verify-taxonomy-report`, `verify-taxonomy-enforce`,
   `mutation-outcome-law`, then `cargo check`/`cargo test -p semio-s-plugin-energy` — do not just
   trust a green structural-correspondence test in isolation.

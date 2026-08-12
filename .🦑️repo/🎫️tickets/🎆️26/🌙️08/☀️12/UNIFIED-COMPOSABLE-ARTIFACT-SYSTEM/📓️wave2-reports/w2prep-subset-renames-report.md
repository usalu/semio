# W2prep — stdio subset renames: `object`→`value`, `workflow`→`flow`

Scope: `✏️s/🔌️plugins/🗄️stdio` only. Bounded, mechanical rename of exactly two subsets in the
`🧿️semio` artifact's `v1` standard. No child/link slots, no new subsets, no other-plugin edits.

## Directory renames (plain `mv`, not `git mv`)

| Old path | New path |
|---|---|
| `🪆️subsets/✳️object/` (76 files) | `🪆️subsets/✳️value/` |
| `🪆️subsets/✳️workflow/` (72 files) | `🪆️subsets/✳️flow/` |

## Symbol / id renames

### `value` (was `object`) — 76 files in the subset dir + touches in `✳️any` (25 files) and `glue.rs`

| Old | New | Notes |
|---|---|---|
| `ObjectId` | `ValueId` | |
| `SemioObjectSnapshot` | `SemioValueSnapshot` | |
| `SemioObjectEntry` | `SemioValueEntry` | |
| `SemioObjectNode` | `SemioValueNode` | |
| `SemioObjectDiff` (top-level Snapshot diff) | **`SemioValueTreeDiff`** | **Collision decision** — see below |
| `SemioObjectDiffBinary`/`SemioObjectDiffText` (TS twins) | `SemioValueTreeDiffBinary`/`SemioValueTreeDiffText` | follows the Diff rename |
| `SemioObjectMutation`, `…MutationKind/Binary/Text` | `SemioValueMutation`, `…MutationKind/Binary/Text` | |
| `SemioObjectPath`, `SemioObjectPathSegment[Kind]` | `SemioValuePath`, `SemioValuePathSegment[Kind]` | |
| `SemioObjectAnalyzer[Analysis]`, `SemioObjectBuilder[Construction/Facets]`, `SemioObjectComposer[Composition]`, `SemioObjectValidator`, `SemioObjectArtifact`, `SemioObjectParts` | `SemioValue*` equivalents | derived-facet plumbing |
| `SemioObjectFromJson/Xml/Csv`, `SemioObjectToJson/Xml/Csv` | `SemioValueFrom…`/`SemioValueTo…` | io bridge codecs |
| `ObjectsTripleDiff`/`ObjectsModified` (TS/proto twins) | `NodesTripleDiff`/`NodesModified` | follows field rename below |
| field `objects: Vec<SemioObjectNode>` (Snapshot + Diff struct) | `nodes: Vec<SemioValueNode>` | the id-keyed backing-store collection; renamed for consistency with `SemioValueNode` — this is a wire-format-affecting rename, fixtures regenerated (see below) |
| mutation variants `SetObject`/`RemoveObject` | `SetNode`/`RemoveNode` | operate on the `nodes` collection |
| DSL keywords `"set-object"`/`"remove-object"` | `"set-node"`/`"remove-node"` | |
| `enc_object_id`/`dec_object_id` | `enc_value_id`/`dec_value_id` | |
| `enc_semio_object_{entry,node,snapshot}[_bin]`/`dec_…` | `enc_semio_value_{entry,node,snapshot}[_bin]`/`dec_…` | |
| `apply_objects_diff`, `objects_diff_between`, `enc_objects_diff_bin`, `dec_objects_diff_bin` | `apply_nodes_diff`, `nodes_diff_between`, `enc_nodes_diff_bin`, `dec_nodes_diff_bin` | |
| `print_object_diff`/`parse_object_diff` | `print_value_tree_diff`/`parse_value_tree_diff` | follows the Diff rename |
| `demo_semio_object_snapshot` | `demo_semio_value_snapshot` | |
| id strings `s.stdio.semio.object[.diff]`, `stdio.semio.object` | `s.stdio.semio.value[.diff]`, `stdio.semio.value` | schema ids, `envelope_id()`, DSL preamble, json-schema `$id`, proto `package` |
| `SubsetId("object")` | `SubsetId("value")` | dialect tag |
| `🔣️component.json` vocabulary entry `"object"` | `"value"` | `s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔣️component.json` |

**Collision decision**: the subset's top-level `SemioObjectSnapshot`-diff struct (fields
`root: Option<SemioValueDiff>`, `objects: Option<NamedTripleDiff<…>>`) is *also* named
`SemioObjectDiff`. A literal `object→value` rename would collide with the pre-existing
`SemioValueDiff` (the recursive leaf diff over the `SemioValue` enum — already correctly named
per the design plan, untouched). Per the ticket's explicit collision-avoidance instruction, the
top-level struct was renamed to **`SemioValueTreeDiff`** instead (echoing the design plan's own
"generic typed *value tree*" phrasing), and its own binary/text TS twins, DSL codec function names
(`print/parse_value_tree_diff`), and the `✳️any` union's diff arm all follow suit. This affected
only the *Rust-facing* (and per-language twin) identifier — the wire-level schema `id` strings
(`s.stdio.semio.value.diff`, etc.) use the plain `value` substitution, independent of the Rust
disambiguation, since they are id-derived, not type-name-derived.

### `flow` (was `workflow`) — 72 files in the subset dir + touches in `✳️any` (25 files, shared with the pass above) and `glue.rs`

| Old | New |
|---|---|
| `WorkflowNode`/`WorkflowEdge`/`WorkflowParam` | `FlowNode`/`FlowEdge`/`FlowParam` |
| `SemioWorkflowSnapshot`, `…Diff`, `…Mutation[Kind]` | `SemioFlowSnapshot`, `…Diff`, `…Mutation[Kind]` |
| `SemioWorkflowAnalyzer[Analysis]`, `…Builder[Construction/Facets]`, `…Composer[Composition]`, `…Validator`, `…Artifact`, `…Parts`, `…FromJson`/`…ToJson` | `SemioFlow*` equivalents |
| `STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA` | `STDIO_SEMIOFLOW_DOCUMENT_SCHEMA` |
| `encode/decode_workflow_snapshot_binary`, `print/parse_workflow_snapshot_body`, `demo_workflow_snapshot`, `check_workflow_referential_invariants`, `print/parse_workflow_{diff,mutation}[_args]` | `…_flow_…` equivalents |
| id strings `s.stdio.semio.workflow[.diff]`, `stdio.semio.workflow` | `s.stdio.semio.flow[.diff]`, `stdio.semio.flow` |
| `SubsetId("workflow")` | `SubsetId("flow")` |
| `🔣️component.json` vocabulary entry `"workflow"` | `"flow"` |

No collisions in the `flow` family — no pre-existing `SemioFlow*` names in stdio. `PortRef` was
already generic and untouched.

### `✳️any` (13→13-arm tagged union, unchanged arity)

- `SemioSubsetSnapshot::Object(SemioObjectSnapshot)` → `::Value(SemioValueSnapshot)`; `::Workflow(SemioWorkflowSnapshot)` → `::Flow(SemioFlowSnapshot)` (Rust enum + json/graphql/proto/TS twins).
- `SemioDiff`/`SemioMutation` any-level dispatch enums: `Object(SemioObjectDiff)` → `Value(SemioValueTreeDiff)`, `Workflow(SemioWorkflowDiff)` → `Flow(SemioFlowDiff)`; same for the mutation dispatch (`SemioValueMutation`/`SemioFlowMutation`).
- Tag strings / DSL keywords `"object"`→`"value"`, `"workflow"`→`"flow"` in `subset_tag()`, `dec_semio_snapshot_body`, and the any-level diff/mutation text codecs.
- Ordinal tables (binary dispatch `u8` tags) — values unchanged (3/12 for value/flow respectively), only the match-arm names changed.
- Module imports `subsets::object::…` → `subsets::value::…`, `subsets::workflow::…` → `subsets::flow::…`.

### `glue.rs` (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`)

- `#[path = ".../🪆️subsets/✳️object/…"]` → `.../✳️value/…` (18 mount points).
- `#[path = ".../🪆️subsets/✳️workflow/…"]` → `.../✳️flow/…` (13 mount points).
- `pub mod object { … }` → `pub mod value { … }`; `pub mod workflow { … }` → `pub mod flow { … }`.

### `⚙️engine/🦀️component.rs` (standard-level registration aggregator, not a subset dir but stdio-owned)

- `subsets::object::io::register()` → `subsets::value::io::register()`; `subsets::workflow::io::register()` → `subsets::flow::io::register()`.
- `io_registry::entries()`: `SemioObjectComposer` → `SemioValueComposer`, `SemioWorkflowComposer` → `SemioFlowComposer` (imports + `composer_entry_of::<…>()` calls).

### Fixtures (regenerated, not hand-migrated)

`📚️examples/` fixtures are the single source of truth asserted byte-identical by each subset's own
`fixture_honesty_law` test. Regenerated via the crate's own `print_dsl`/`encode_pack` on the demo
snapshots (temporary `[DEBUG]` dump tests added, run once under `cargo test …
debug_dump_fixture_bytes -- --nocapture`, hex captured, binary fixtures rewritten via the exact
bytes, then the temporary tests removed — never hand-migrated):

| Fixture | Change |
|---|---|
| `📚️examples/🕸️graph/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio` | schema id `stdio.semio.object` → `stdio.semio.value` |
| `📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio` | schema id `stdio.semio.workflow` → `stdio.semio.flow` |
| `📚️examples/🌐️envelope/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio` | `subset=workflow`→`subset=flow`, inner schema id → `stdio.semio.flow` (the any-envelope demo wraps the flow subset's demo snapshot) |
| `📚️examples/🕸️graph/🦀️component.rs`, `📚️examples/🌊️pipeline/🦀️component.rs`, `📚️examples/🌐️envelope/🦀️component.rs` | doc-comment citations (`SemioObjectSnapshot`/`demo_semio_object_snapshot`/`✳️object`/`✳️workflow` etc.) updated to match |

### Cross-subset citation sweep (comment-only, no functional change)

Many *other* stdio subsets (mesh, cad, drawing, audio, model, presentation, brep, document,
animation, image, video, the shared `⚙️engine/🧰️triples`) carry doc comments citing `workflow`
(the "P2 pilot" precedent) or `object` (mesh's `NamedAdded<T>` precedent) by name — these are
legitimate references to the renamed subsets and were updated to `flow`/`value` (49 files touched
for `workflow`→`flow`, 3 for `object`→`value`: `✳️mesh` diff/mutations files +
`⚙️engine/🧰️triples/🦀️component.rs`). Historical report-filename citations
(`` `ws-codec-workflow-report.md` ``, 14 occurrences) were explicitly preserved verbatim — those
are real historical filenames, not subset-name references, and are out of this ticket's scope.

## Explicit non-renames (disambiguation — read each hit in context, no blind sed)

- `🧊️obj` (Wavefront OBJ file-format artifact) — completely unrelated, untouched.
- `JsonValue::Object` / `JsonObjectAdded` (json subset's own JSON-object-literal vocabulary, cited from within the value/flow subsets' own doc comments and deserializer bridges) — untouched. **One accidental hit was caught and fixed**: the value subset's json import/export bridge (`🚪️io/📥️import/…/🔣️json/…/🦀️component.rs`, `🚪️io/📤️export/…/🔣️json/…/🦀️component.rs`) briefly had `JsonValue::Object` mis-rewritten to `JsonValue::Value` by the blind generic-catch-all pass; found via the resulting `error[E0599]: no variant named 'Value' found for enum … JsonValue` compile errors and reverted.
- `"type": "object"` (JSON Schema's own type keyword, throughout many subsets' `.json` twins) — untouched.
- PDF's own "object graph" (PDF indirect objects), glTF/OBJ's own "object" vocabulary — untouched.
- `semio_framework::WorkflowNode` (the OS kernel's own, unrelated type in a different crate, cited by name in `✳️flow/🧬️schema/📸️snapshot/🦀️component.rs` doc comments to explain "same name, zero collision risk") — untouched, explicitly protected during the bulk substitution.
- `📇️registry/📇️catalog.json`, `🛂️manifest/🦀️component.rs` — no subset-vocabulary references found (only unrelated format-roster entries: JSON's full name, Wavefront OBJ).

## Verification

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo check -p semio-s-plugin-stdio --tests
```
Result (final run, after all fixes): **0 errors**, 755 pre-existing warnings (unrelated to this
change), `Finished` in 34.76s. (One intermediate run hit 2 errors in `semio-framework-os-kernel` /
`🌿️vcs/🦀️component.rs` — confirmed transient concurrent churn, see below — and cleared on retry.)

```
CARGO_TARGET_DIR=".../🎯️target" bun nx run @semio-tech/stdio-plugin:test-quick
```
Result: hit the documented `[budget] … exceeded 15000ms — killed` on a cold-ish shared cache
(nextest's `fundamental` profile budget), matching the ticket's documented non-failure signal —
not a real failure. Fell back to `test-long` per instructions.

```
SEMIO_TEST_LEVEL=long CARGO_TARGET_DIR=".../🎯️target" bun nx run @semio-tech/stdio-plugin:test-long
```
Result: nextest's default fail-fast stopped the run at 130/2026 tests after hitting an **unrelated,
pre-existing** failure (`artifacts::csv::standards::v_rfc4180::subsets::any::schema::inferences::…
::inference_default_law` — nothing to do with `value`/`flow`/`any`, git-clean/untouched file). To
get the true picture, re-ran directly:

```
CARGO_TARGET_DIR=".../🎯️target" cargo nextest run --profile long -p semio-s-plugin-stdio --no-fail-fast
```
Result: **2021 passed, 5 failed, 3 skipped** (2026 total). All 5 failures are pre-existing and
unrelated to this ticket's scope — `inference_default_law`/outline tests in the **csv, html, json,
md, pdf** subsets' own `any::schema::inferences` facet (a schema-shape-inference mechanism this
ticket never touches). Confirmed via `git status --porcelain` on the failing files: clean, not
modified by me or by any concurrent session — genuine pre-existing baseline breakage, not
introduced by this rename. Zero failures among the tests this ticket actually touches (`value`,
`flow`, `any`, `mesh`'s citation-only edits, `⚙️engine`) — all conformance laws
(`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) for `value`/`flow`/`any`
passed, confirming the regenerated `.dsl.semio`/`.pack.semio` fixtures and the grammar/protocol/
spicy/ksy twin files are byte-honest and syntactically valid post-rename.

## sharedFileRequests

None. All edits stayed inside `✏️s/🔌️plugins/🗄️stdio/**` (owned by this wave per
`📌️important.md`'s hot-file table). `🛂️manifest/🦀️component.rs`, `📇️registry/📇️catalog.json`,
repo-root `📜️script.ts`, and taxonomy files were inspected (read-only) and found to carry no
subset-vocabulary references needing a change — no patch requests filed.

## Concurrent-churn observations

- One transient compile failure (`error[E0599]` × 2, "`ArtifactRef` is not an iterator" /
  "no method named `as_bytes`") in `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` and
  `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, both W1-owned hot files. `git status --porcelain`
  confirmed `🌿️vcs/🦀️component.rs` was modified-uncommitted at the time (another session's
  in-progress `ArtifactRef` refactor). Zero errors originated under `🔌️plugins/🗄️stdio` in that
  run's output. Retried `cargo check -p semio-s-plugin-stdio --tests` once more (no artificial
  sleep needed — the retry itself took ~90s and queued behind the shared target-dir flock) and it
  came back green — the sibling session's `ArtifactRef` refactor had landed in the interim.
- The repo has an active auto-commit mechanism; a new commit (`1caac91709`) landed on top of HEAD
  during this session's work, consistent with heavy concurrent multi-agent activity across the
  shared tree (observed via `ps aux`: simultaneous `cargo check`/`nextest` invocations for
  `semio-s-plugin-layout`, `-puzzle`, `-block`, `-norm`, `-shooting`, `-gis`, `-sourcing`,
  `-architect`, and `-stdio` itself, from other sessions, throughout this ticket's runtime).
- No `🧬️mutations/**` files in any *uncleared* SMO plugin were touched — this wave's edits were
  confined to `stdio`'s own two subsets plus `✳️any`/`⚙️engine`, all already claimed for W2 stdio
  per `📌️important.md`.

## Summary

Both renames (`object`→`value`, `workflow`→`flow`) are complete and exhaustive: directory names,
Rust types/fields/functions/consts, all 5 per-facet schema-twin languages (rs/ts/graphql/
json_schema/proto) plus the text/binary grammar leaves (g4/ebnf/abnf/grammar.semio/
protocol.semio/spicy/ksy), the `✳️any` 13-arm union (Rust + 4 twins), `glue.rs` module-mount
tree, the standard-level `⚙️engine` registrar, the subset vocabulary manifest
(`🪆️subsets/🔣️component.json`), all 3 affected example fixtures (regenerated, not migrated), and
legitimate cross-subset doc-comment citations repo-wide within `stdio`. One accidental collateral
edit (`JsonValue::Object`→`JsonValue::Value` inside the value subset's own json bridge) was caught
by the compiler and reverted. `SemioObjectDiff`→`SemioValueTreeDiff` is the one deliberate
collision-avoidance departure from a literal `object→value` substitution, recorded above.
`cargo check -p semio-s-plugin-stdio --tests` is green; `cargo nextest run --profile long -p
semio-s-plugin-stdio --no-fail-fast` is 2021/2026 passing with 5 pre-existing, unrelated,
git-confirmed-untouched failures in other subsets' `inferences` facet.

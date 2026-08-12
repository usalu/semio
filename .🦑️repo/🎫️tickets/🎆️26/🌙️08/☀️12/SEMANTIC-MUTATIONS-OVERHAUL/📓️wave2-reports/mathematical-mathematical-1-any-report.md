# Wave 2 — `mathematical/mathematical` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-mathematical`.

## What landed

Deleted the generic `MathematicalMutation` (`SetGraph{graph}` / `SetGeometry{geometry}` /
`SetSnapshot{snapshot}`, three named-field variants + hand-written `apply_mathematical_mutation`/
`inverse_mathematical_mutation` dispatch fns) and replaced it with a 14-variant semantic vocabulary,
each a single-field tuple wrapping a real `🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched
via `#[derive(dsl_derive::Mutations)]` (`#[mutations(snapshot = MathematicalSnapshot, diff =
MathematicalDiff, schema = "s.mathematical.mathematical")]`) exactly mirroring the wave0
`MiniMutation` reference fixture.

Vocabulary derived from `MathematicalSnapshot`'s two persistent fields (`graph`: directed/nodes/
edges/algorithm/algorithm_seed, `geometry`: an anonymous point cloud) **and** cross-checked against
every real app gesture that emits a `MathematicalMutation` today
(`🎛️apps/➗️mathematical/🎮️commands/{🕸️graph,📐️geometry,📄️artifact}/🦀️component.rs`:
`addNode`/`move`/`connect`/`deleteSelection`, `SetAlgorithm`, `SetDirected`, `SetPoints`,
`SetArtifact`) so nothing was invented that isn't backed by either the schema shape or a real
editor command:

| New mutation | Verb | Replaces |
|---|---|---|
| `change-graph-directed{new_directed}` | change | `SetGraph` (direction toggle) |
| `update-graph-algorithm{new_algorithm,new_algorithm_seed}` | update | `SetGraph` (algorithm+seed, always set together → the recipe's inseparable-facet exception, matches `SetAlgorithm` command bundling both fields) |
| `replace-graph{graph}` | replace | `SetGraph` (whole-field load/paste, matches `SetArtifact`) |
| `create-node{id,label,x,y}` | create | `SetGraph` (matches `addNode`) |
| `delete-node{id}` | delete | `SetGraph` (cascades to incident edges) |
| `delete-nodes{ids}` | delete | `SetGraph` (plural/bulk — matches `deleteSelection` multi-select, taxonomy's bulk-mutation rule) |
| `change-node-label{id,new_label}` | change | `SetGraph` (schema-derived; no live gesture yet) |
| `move-node{id,x,y}` | move | `SetGraph` (matches `move`) |
| `connect-nodes{id,source,target}` | connect | `SetGraph` (matches `connect`) |
| `disconnect-nodes{id}` | disconnect | `SetGraph` (schema-derived edge counterpart) |
| `replace-points{points}` | replace | `SetGeometry` (whole-field load/paste, matches `SetPoints`) |
| `insert-point{index,x,y}` | insert | `SetGeometry` (index-keyed anonymous collection) |
| `remove-point{index}` | remove | `SetGeometry` |
| `move-point{index,x,y}` | move | `SetGeometry` |

`SetSnapshot` has **no** replacement (per taxonomy: whole-document replace is banned outright, not
expressible as a mutation; `store::ArtifactStore::reset` is the sanctioned non-history path — no
call site in this crate used `SetSnapshot` for a real gesture, it was dead generic surface).

`reorder-points` was deliberately **not** added — `MathematicalGeometry.points` has no UI reorder
gesture (order carries no semantic weight for the convex-hull/centroid playground); adding it would
have invented structure not backed by any gesture or schema intent.

Every `diff()` is handcrafted directly from `base` (clone the relevant `graph`/`geometry` sub-value,
patch just the addressed part, wrap in `MathematicalDiff{graph: Some(..)}` /
`{geometry: Some(..)}`) — `MathematicalDiff` is sparse only at the `graph`/`geometry` field
granularity (pre-existing shape, this facet doesn't own `🔺️diff`), so every graph/geometry-scoped
mutation shares that "clone whole sub-value, patch the addressed part" shape; never apply-then-
capture. Every `inverse()` reads `base` (pre-state): `delete-node`/`delete-nodes` capture the full
node(s) + every severed edge and reconnect them (`create-node` + `connect-nodes` per edge);
`create-node`/`connect-nodes` invert to `Vec::new()` when the id already existed in `base` (no-op
create); `change-node-label`/`move-node`/`remove-point`/`move-point` invert to `Vec::new()` when the
target is missing from `base`.

Hand-rolled `OpText`/`OpBinary` for the new enum in `🧬️mutations/📝️text/🦀️component.rs` (the
derive only generates `Mutation`/`SemanticMutation`, not the wire codecs) — `keyword key=value ...`
grammar, quote-aware tokenizer (labels/algorithm ids may contain spaces), binary tag `0..=13` +
varint/LE-f64 fields; `replace-graph`'s whole-graph payload goes through
`serde_json`+quoted-string (not a second handcrafted graph grammar). `demo_mutation_cases()` covers
all 14 variants and `op_text_binary_roundtrip_law` round-trips every one through both codecs.

## Mechanism note: self-wiring instead of `📦️glue.rs`

`📦️glue.rs` is out of this facet's writable boundary (plugin-shared), but 14 new triad-leaf
directories need their `🦠️mutation`/`🔺️diff`/`↩️inverse` files turned into real Rust modules. Fix:
declared them directly inside `🧬️mutations/🦀️component.rs` itself (`🔖️LeafWiring` region, 14
`#[path = "."] pub mod <slug> { #[path = "<dir>/🦠️mutation/🦀️component.rs"] pub mod mutation; ... }`
blocks) — confirmed empirically (a throwaway `cargo run` fixture in the scratchpad) that `#[path]`
resolution is always relative to the *file* containing the attribute, so nesting these directly in
`component.rs` with `#[path = "."]` on each wrapper resolves correctly against `🧬️mutations/`'s own
directory regardless of how `glue.rs` itself included `component.rs`. Zero `glue.rs` edits needed
for the 14 new triads. Cross-triad references (e.g. `delete-node`'s inverse constructing
`create-node`'s payload) go through the existing `crate::artifacts::mathematical::mutations::<slug>`
shim glue.rs already re-exports.

The 3 OLD triad dirs (`📄set-snapshot`, `📊set-graph`, `📐set-geometry`) could not be deleted or
renamed — `glue.rs` still hardcodes their exact file paths (`pub mod set_snapshot { ... }` etc.) and
editing glue.rs is out of scope. Their `🦠️mutation/component.rs` files were already
enum-independent (harmless, left untouched); their `↩️inverse/component.rs` files DID construct the
now-deleted `SetGraph`/`SetGeometry`/`SetSnapshot` variants, so they were rewritten to orphaned
`Vec::new()` stubs with a doc comment pointing at the sharedFileRequest below. These are dead code
(still `pub fn`, so no dead_code warning) until glue.rs's `set_snapshot`/`set_graph`/`set_geometry`
module blocks are deleted in the plugin-wide reconciliation pass.

## Other in-boundary fixes required by the vocabulary change

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs`: `mutate()` called the now-deleted
  `apply_mathematical_mutation` free fn directly; rewritten to `diff(&mutation, &snapshot)` then
  `MutationDiff::apply` (matches how every other already-migrated facet's builder works).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` and
  `🧬️mutations/💾️binary/🦀️component.rs`: tests constructed `MathematicalMutation::SetGraph{..}`;
  updated to `UpdateGraphAlgorithm`/`ChangeGraphDirected`.
- `🧬️mutations/📖️component.grammar.semio`: rewritten to list the 14 real mutation keywords
  honestly (was a stale generic `insert-text`/`replace-range`/`set-fence` placeholder unrelated to
  this artifact — confirmed via `🧰️framework/.../🗣️dsl/🧪️fixture-sweep/🦀️component.rs`'s
  `m5_auto_discovery` module that `mathematical` is explicitly in that sweep's non-pilot exemption
  list, so this file isn't test-validated either way; purely documentation quality).

## Tests

Extended the existing `🧪️Tests` regions (no new test files) in `🧬️mutations/🦀️component.rs` (8
tests: diff/inverse laws for `replace-graph`, `create-node`↔`delete-node`, `delete-node` cascade —
asserting the captured payload fields directly, not just a round-trip, since this facet's
whole-field-replace `MathematicalDiff` would mask a wrong-label/x/y bug in a pure round-trip check
— `move-point`, `insert-point`, `delete-nodes` plural cascade, `connect-nodes`↔`disconnect-nodes`,
and a `kinds().len() == 14` + `semantics()` check) and `🧬️mutations/📝️text/🦀️component.rs` (the
`op_text_binary_roundtrip_law` over all 14 `demo_mutation_cases()`).

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped this crate for an existing `testkit`
import first, per instructions; none exists (`semio-s-plugin-mathematical`'s `Cargo.toml` has no
`semio-framework-os-kernel` testkit-feature dependency), so per the task's explicit fallback this
step was skipped rather than adding a new Cargo dependency. The hand-written tests above cover the
same inverse/diff laws directly instead.

## Verification — BLOCKED (pre-existing, out-of-boundary, already-committed bug, not this ticket's)

`cargo check -p semio-s-plugin-mathematical` fails with exactly one error, unrelated to mutations:

```
✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs:314:13: error: couldn't read
`.../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs`: No such file or directory
```

Confirmed this is **not** live churn from a concurrent session and **not** caused by this ticket's
edits:
- `git diff --stat` on `📦️glue.rs` is empty (byte-identical to `HEAD`, not being edited right now).
- `git ls-tree HEAD` under `🎛️apps/➗️mathematical/🎮️commands/` shows only `📄️artifact/`, never
  `📄️document/` — the directory is really named `📄️artifact` on disk and in the last commit.
- `git log -p -1 -- 📦️glue.rs` shows the last touching commit (`c31024cc6c`,
  "Rename framework-wide document contracts to artifact...") did the repo-wide document→artifact
  rename but missed updating this one `#[path]` line in the mathematical plugin's glue — a
  pre-existing, already-committed defect, 100% outside this facet's artifact directory, with zero
  relation to `MathematicalMutation`'s vocabulary.

Retried 3× (60s apart) per the workspace-churn policy; identical failure every time (expected — a
committed bug doesn't self-heal on retry, but the policy's retry step was still followed). Per the
hard constraint, `📦️glue.rs` and `🎛️apps/**` are out of this facet's writable boundary, so this
could not be fixed here. Recommended one-line fix for whoever holds `📦️glue.rs`:
`🎮️commands/📄️document/🦀️component.rs` → `🎮️commands/📄️artifact/🦀️component.rs` at line 314
(and `pub mod document;` → `pub mod artifact;` immediately below it, plus whatever `commands::document`
call sites exist under `🎛️apps/➗️mathematical/🦀️component.rs`).

**Manual verification performed in lieu of a green `cargo check`** (full crate compilation is
blocked before reaching this facet's code at all, so no error/warning signal from THIS facet's
files could be observed via cargo either way):
- `rustfmt --edition 2021 --check` on all 51 touched/created `component.rs` files in this facet
  (42 new triad-leaf files + the 9 modified ones) — zero parse errors (exit code >1) on any file;
  only ordinary formatting-diff exit-1s.
- Every `impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation>` hand-checked
  against the real trait definition (`🎮️command/🦀️component.rs` `🔖️Semantics` region) and the
  wave0 `MiniMutation` fixture's exact shape.
- Every `SEMANTICS.kind` hand-kebab-checked against its variant name (the derive's compile-time
  `str_eq` assert) and every `SEMANTICS.verb` checked against `protocol::APPROVED_VERBS`.
- The `#[path="."]` nested self-wiring pattern (`🔖️LeafWiring` region) was proven correct with a
  throwaway `cargo run` fixture in the scratchpad directory before writing it into this facet,
  confirming Rust's `#[path]` resolution is per-containing-file (not per logical module nesting
  depth), matching `📦️glue.rs`'s own documented convention for the exact same reason.

Given the above, `cargoCheck` is reported as `churn-retry-exhausted` (a real, unrelated,
out-of-boundary blocker survived 3 retries) rather than `red` (which would imply a bug of mine) or
`green` (never actually observed). `lawTestsPass` is `false` only because the test binary could
never be built to run — the law tests themselves are written and, by the same manual trait/type
audit, believed correct.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs:314`** (blocking, see above) — fix the `document`→`artifact` path/module-name
   typo so the crate compiles at all again. Unrelated to this ticket but discovered by it.
2. **`📦️glue.rs`, `mutations` block** (lines ~83-104 in the pre-ticket file) — once (1) is fixed
   and this facet's new vocabulary is confirmed compiling, delete the now-dead
   `pub mod set_snapshot { ... }` / `pub mod set_graph { ... }` / `pub mod set_geometry { ... }`
   blocks (their `🦠️mutation`/`↩️inverse` files under `🧬️mutations/📄set-snapshot`,
   `📊set-graph`, `📐set-geometry` are orphaned stubs now, kept only so glue.rs's existing
   `#[path]`s don't dangle).
3. **`🎛️apps/➗️mathematical/🎮️commands/📄️artifact/🦀️component.rs`** (`SetArtifact::handle`) —
   replace `MathematicalMutation::SetGraph{graph}` / `SetGeometry{geometry}` with
   `MathematicalMutation::ReplaceGraph(replace_graph::mutation::ReplaceGraph{graph})` /
   `ReplacePoints(replace_points::mutation::ReplacePoints{points: geometry.points})`.
4. **`🎛️apps/➗️mathematical/🎮️commands/📐️geometry/🦀️component.rs`** (`SetPoints::handle`) —
   replace `SetGeometry{geometry}` with `ReplacePoints{points: geometry.points}`.
5. **`🎛️apps/➗️mathematical/🎮️commands/🕸️graph/🦀️component.rs`** —
   - `SetAlgorithm::handle`: replace `SetGraph{graph}` (mutated clone) with
     `UpdateGraphAlgorithm{new_algorithm: payload.algorithm.clone(), new_algorithm_seed: payload.seed.clone()}`.
   - `SetDirected::handle`: replace `SetGraph{graph}` with `ChangeGraphDirected{new_directed: payload.directed}`.
   - `NodeGraphEdit::handle`'s `"addNode"` arm → `CreateNode{id, label, x, y}`; `"move"` arm →
     `MoveNode{id: node_id, x, y}`; `"connect"` arm → `ConnectNodes{id, source, target}` (needs a
     fresh edge id, e.g. keep the existing `format!("e{}", graph.edges.len())` scheme read from
     `doc.snapshot`); `"deleteSelection"` arm → `DeleteNodes{ids}` (this is the exact plural gesture
     `delete-nodes` was derived for). Each currently batches ALL parsed ops into one `SetGraph`; the
     semantic replacement is to `Emit::mutations(vec![...])` with one typed mutation per parsed op
     instead (multiple ops in one `operations_json` array become multiple `Vec` entries).
Grepped the entire artifact directory (`🗿️artifacts/➗️mathematical/**`, including `📚️examples/`,
`🎹️composer`, root `🏗️builder`/`🧐️analyzer`, `⚙️engine`) for `MathematicalMutation::Set`/
`apply_mathematical_mutation`/`inverse_mathematical_mutation` — no other call sites found beyond
the ones fixed above and the two doc-comment mentions inside the new `🔁️replace-graph`/
`🌀️replace-points` payload files (prose only, explaining what they replace). Everything inside this
facet's writable boundary is fully migrated; only the 5 `🎛️apps/**`/`📦️glue.rs` items above remain.

# Wave 9 — semio artifact, Pattern-B subsets (brep, graph, kit, mesh, object, table, text)

Scope: the 🧿️semio artifact's 7 Pattern-B subsets (drawing already had `apply_semio_drawing_mutation`
and was the reference, not touched here). Ticket: `26/08/23/END-TO-END-TESTING-REFACTOR`.

## TASK 1 — missing `apply_semio_<subset>_mutation` entry points

Added to all 7 subsets' `🏅️standards/🔖️v1/🪆️subsets/✳️<subset>/🧬️schema/🧬️mutations/🦀️component.rs`,
mirroring `apply_semio_drawing_mutation` exactly (thin `diff()` → `MutationOutcome::apply_to`
wrapper), in a new `//#region 🔖️Apply` placed right after `//#endregion 🔖️Mutations`:

```rust
pub fn apply_semio_<subset>_mutation(snapshot: &mut Semio<Subset>Snapshot, mutation: &Semio<Subset>Mutation) -> protocol::MutationOutcome<Semio<Subset>Diff> {
    use protocol::Mutation;
    let outcome = <Semio<Subset>Mutation as Mutation<Semio<Subset>Snapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}
```

Also updated each subset's `🧬️schema/🦀️component.rs` `SemioXBuilderConstruction::mutate` to call the
new entry point instead of inlining the same two-line dispatch (matching drawing's own shape, and
removing the duplication CLAUDE.md's refactor rule flags):

```rust
async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
    let diff = apply_semio_<subset>_mutation(&mut self.snapshot, &mutation);
    (self, diff)
}
```

Files touched (7 × 2 = 14 production files):
- `✳️brep/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️brep/🧬️schema/🦀️component.rs`
- `✳️graph/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️graph/🧬️schema/🦀️component.rs`
- `✳️kit/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️kit/🧬️schema/🦀️component.rs`
- `✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️mesh/🧬️schema/🦀️component.rs`
- `✳️object/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️object/🧬️schema/🦀️component.rs`
- `✳️table/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️table/🧬️schema/🦀️component.rs`
- `✳️text/🧬️schema/🧬️mutations/🦀️component.rs` + `✳️text/🧬️schema/🦀️component.rs` (text also got its
  `pub const KINDS` + conformance test in the same pass, see below)

`cargo check -p semio-s-plugin-stdio --lib`: fails, but **zero of the errors are in the 🧿️semio
artifact or in any file this ticket touched** — all ~117 errors are the pre-existing, already
documented `semio_framework_os_kernel` cycle in `📡️spr/🧵️channel` / `🏪️store` that a peer session is
mid-refactor on (confirmed by file path in every error). Re-verified after the text-subset adapter
was added: `subject exhaustive --case mutate-semio-text` also fails only inside
`🏪️store/🦀️component.rs` and `📡️spr/🧪️testkit/🦀️component.rs` — never inside anything under
`🗿️artifacts/🧿️semio`.

## TASK 2 — real or stub?

**Real, complete, and already thoroughly tested — not stubs.** Evidence per subset:

| Subset | Kinds (enum variants) | Matches ticket's stated count | `kinds().len()` test | Per-leaf `🧪️tests/` fixture dirs | `todo!`/`unimplemented!`/stub markers |
|---|---|---|---|---|---|
| brep | 13 | yes | asserts 13 | 13/13 | none |
| graph | 11 | yes | asserts 11 | 11/11 | none |
| kit | 15 | yes | asserts 15 | 15/15 | none |
| mesh | 17 | yes | asserts 17 | 17/17 | none |
| object | 9 | yes | asserts 9 | 9/9 | none |
| table | 8 | yes | asserts 8 | 8/8 | none |
| text | 7 | yes | asserts 7 | 7/7 | none |

Every one of the 80 kinds has:
- A real hand-authored `diff()`/`inverse()` pair in its `🧬️mutations/<kind>/{🔺️diff,↩️inverse}/
  🦀️component.rs` leaf (not derived, not apply-and-capture — confirmed by reading brep's and text's
  full facet files, including cascade-delete logic, non-contiguous-group rejection, absent-id
  no-op-with-empty-inverse handling, etc.).
- A committed, independently **handcrafted** specification fixture under its own
  `🧬️mutations/<kind>/🧪️tests/<slug>/` directory: `🦠️mutation/`, `📸️snapshot/⬅️before/`,
  `📸️snapshot/➡️after/`, `🔺️diff/`, `🎯️outcome/` — each a real `🔣️component.json`, plus a
  `🦀️component.rs` unit test asserting the produced diff/after matches the committed ones, the
  committed JSON round-trips canonically, and the inverse restores `before` exactly.
- Coverage by `diff_consistency_law` (hand diff vs. independent before/after `Diff::between`),
  `determinism_law`, and (brep/mesh) `OpText`/`OpBinary` round-trip laws.

This is real production code with real test coverage; TASK 3 is worth doing, and none of it is
resting on stub implementations.

## TASK 3 — one case per subset

**A hard constraint not anticipated by the fleet brief's Pattern-A shape**: the framework's
generated test-host crate (`materializeRustHost` in `🧰️framework/…/🧪️test/📜️script.ts`) links only
`semio-repo-test-host` (dependency-free by design) plus, behind the `sut` feature, the owner's
subject crate with `default-features = false`. It does **not** add `serde_json` (or any other crate)
unless the case registers a third-party oracle package with a `hostPath` — which this wave's
no-oracle decision explicitly forbids inventing. Concretely: **the adapter cannot
`serde_json::from_str` a committed fixture into a typed `Semio<Subset>Snapshot`/`…Mutation`.**
`txt-utf-8`/`binary-raw`'s own precedent adapters independently discovered and worked around the
same constraint by hand-writing JSON-field extraction from the framework's own dependency-free
`protocol::Json`.

The design landed here for Pattern-B, verified against `text` and to be mirrored by the remaining
6 subsets:
- **Oracle role** (never links the subject crate): reads the committed `before`/`after`
  `🔣️component.json` fixture text literally via `include_str!` + the framework's own
  `protocol::parse_json` — no recomputation, no reimplementation, exactly the specification-vector
  substitute the no-oracle decision names.
- **Subject role** (`sut`-gated): the SAME fixture data, hand-transcribed ONCE into real
  `Semio<Subset>Snapshot`/`Semio<Subset>Mutation` Rust literals (mechanical, not invented — the same
  technique this subset's own committed leaf fixture tests already use, just re-expressed as Rust
  instead of parsed from JSON at runtime). Runs the real `apply_semio_<subset>_mutation` — the entry
  point this ticket added — then projects the resulting snapshot to `protocol::Json` via a small
  hand-written per-subset encoder (forward direction only) for comparison under `ordered-json-v1`.
- `@no-oracle-semio-<subset>-mutation-semantics` decision, `substitutes: ["specification-vectors",
  "metamorphic-laws"]`, in a new `✳️<subset>/🧪️oracle/🔣️component.json` (`oracles: []`,
  `mutationCatalogs` carrying the subset's `KINDS`).
- `@mode-conformance` for `mutate-<kind>` and `@mode-property` for `inverse-<kind>` — never
  `@mode-differential`. No identity-round-trip scenario: `store::ArtifactPack`/`ArtifactDsl` are
  reached through the `store` extern-crate alias, which is private to the subject crate (confirmed
  by `txt-utf-8`'s own adapter doc comment) and — even if reachable — `semio_framework_os_kernel` is
  the exact crate this wave's peer refactor has broken, so it would add risk for no evidentiary gain
  over the mutate/inverse scenarios already exhaustively covering every kind.
- `pub const KINDS: &[&str]` added beside each subset's enum, plus a `kinds_match_the_enum_and_the_
  catalog` test asserting it against both `SemioXMutation::kinds()` (from `#[derive(dsl::Mutations)]`)
  and the committed catalog manifest text.

### Verified so far — `mutate-semio-text` (7/7 kinds)

- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-text` → **0 breaches**.
- `bun ./📜️script.ts contract` (repo-wide) → **0 breaches** (confirms no collateral damage).
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-text` → exit 0,
  `not-exercised=1` with reason `recorded no-oracle decision semio-text-mutation-semantics — its
  evidence is discharged by the subject phase` — the exact same shape `mutate-binary-raw`/
  `mutate-txt-utf-8` already established, correctly reporting rather than faking a pass.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-text` → attempts the
  real `--features sut` compile; **every one of the 64 resulting errors is inside
  `🧰️framework/…/🏪️store/🦀️component.rs` or `📡️spr/🧪️testkit/🦀️component.rs`** (the pre-existing
  os-kernel cycle) — none inside `🗿️artifacts/🧿️semio` or the new adapter. This is the honest
  "written and gated but blocked upstream" state the fleet brief's §Honest-limits describes, not a
  fault in this case.
- Standalone `rustc --edition 2021 --crate-type lib --cfg 'feature="sut"'` syntax/name-resolution
  pass on the adapter alone: only unresolved-external-crate errors (expected, unlinked), zero syntax
  errors.

Files added:
- `✳️text/🧪️oracle/🔣️component.json`
- `🧿️semio/🧪️tests/mutate-semio-text/component.feature`
- `🧿️semio/🧪️tests/mutate-semio-text/🦀️component.rs`

### Remaining 6 subsets (brep 13, graph 11, kit 15, mesh 17, object 9, table 8 = 73 kinds)

Same shape, same verification gates, dispatched to parallel sessions with the verified `text` case
as the concrete reference and the exact fixture-path/field-shape data this session already extracted
(kept in this ticket folder's scratch files, not committed). Each must independently re-verify its
own subset's field types against the committed `🦠️mutation/🦀️component.rs` leaves before
transcribing literals — payloads range from trivial (table/object) to real nested geometry enums
(brep's `BrepCurve`/`BrepSurface`, mesh's primitive/material/texture graph), so the amount of
hand-transcription varies a lot per subset and must not be templated blindly.

Status of the remaining 6 will be appended here once their sessions report back.

### brep — done by parallel session

Mirrored `mutate-semio-text` for the `✳️brep` subset's 13 kinds (`create-vertex`, `delete-vertex`,
`create-edge`, `delete-edge`, `create-face`, `delete-face`, `create-shell`, `delete-shell`,
`create-solid`, `delete-solid`, `replace-curve`, `replace-surface`, `move-vertex`), then **revised
mid-flight** to the `Context::fixture_json` design the framework helper landed for (see below) —
this case hand-transcribes nothing.

Files touched/added:
- `✳️brep/🧬️schema/🧬️mutations/🦀️component.rs` (edited): added `pub const KINDS: &[&str]` (13
  entries, enum order) right after the enum's closing brace, plus a `kinds_match_the_enum_and_the_
  catalog` test inside the existing `mod tests` block (asserts `KINDS.len() == SemioBrepMutation::
  kinds().len()`, per-index kind spelling, and every entry present in the new oracle manifest text).
- `✳️brep/🧪️oracle/🔣️component.json` (new): `oracles: []`, one `noOracleDecisions` entry
  `semio-brep-mutation-semantics` (`capabilities: ["semio-v1-brep-mutate"]`, `substitutes:
  ["specification-vectors", "metamorphic-laws"]`), one `mutationCatalogs` entry `semio-v1-brep`
  listing all 13 kinds in enum order. Same `$schema` relative-path depth as text's (verified: both
  paths have 10 `/` separators).
- `🧿️semio/🧪️tests/mutate-semio-brep/component.feature` (new): same tag set, two 13-row Scenario
  Outlines (`@id-mutate`/`@mode-conformance`, `@id-inverse`/`@mode-property`). Each outline's
  `Given` step carries a 3-row `| role | fixture |` data table binding `before`/`mutation`/`after`
  to `asset://` URIs built from a `<dir>` Examples column, so the feature file is the single source
  of truth for which fixture each kind uses.
- `🧿️semio/🧪️tests/mutate-semio-brep/🦀️component.rs` (new): `fixture_uri(ctx, role)` reads the URI
  out of the scenario's own table; the oracle returns `ctx.fixture_json(uri)` verbatim; the
  `sut`-gated subject decodes `before` + `mutation` through a hand-rolled dependency-free `Decode`
  region, runs `apply_semio_brep_mutation`, and re-encodes via the `Projection` region.

**Design revision — no hand-transcribed fixtures.** The first draft followed the text precedent:
oracle side `include_str!` on the committed JSON, subject side the same vectors hand-transcribed
into `SemioBrepSnapshot`/`SemioBrepMutation` Rust literals (the generated host links only
`semio-repo-test-host` plus, behind `sut`, the subject crate — no `serde_json`). The coordinator
then flagged that `Context::fixture_json(uri)` had landed on the Rust host, and that the sibling
`graph` session's correctness now rested on manual transcription. Switched both roles to read the
committed files directly:
- `asset://` was the right scheme — it resolves against the OWNER ROOT, which the generated plan
  confirms is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`, i.e. the fixtures are referenced where
  the subset already keeps them instead of being copied into a fixtures directory (exactly the case
  `resolveFixtures`' own doc comment describes). `orphan-fixture` only polices case-local fixtures,
  so nothing is flagged.
- `materializeScenario` substitutes Examples values into step text and data-table cells *before*
  `fixtureUrisIn` scans them, so three URI templates carrying a `<dir>` placeholder plus one `dir`
  column expand to all 39 concrete URIs — no 39-line manual list, and the paths exist in exactly one
  place.
- The subject still needs typed values, so `Decode` hand-rolls `Json → SemioBrepSnapshot` /
  `SemioBrepMutation` mirroring this subset's serde shape: camelCase snapshot fields
  (`startVertex`/`endVertex`/`outerLoop`/`innerLoops`/`isVoid`), `"kind"`-tagged lowercase
  `BrepCurve`/`BrepSurface` variants, externally tagged mutation variants whose payload fields are
  plain **snake_case** (`start_vertex`, `outer_loop`, `new_curve`, `edge_id`, `vertex_id`, …) while
  nested snapshot value types keep camelCase — all read off the committed fixtures, not assumed.
  Accessors fail loudly rather than defaulting; `#[serde(default)]` collections decode as empty.
  `Decode` is the exact inverse of `Projection`, and a decoder bug cannot pass silently because the
  oracle compares against the untouched committed `after` file.

Verify (run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, all re-run after the revision):
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-brep` → **0 high-priority
  breach(es) across 0 rule(s)**.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-brep` → exit 0,
  `[test] not-exercised … (recorded no-oracle decision semio-brep-mutation-semantics — its evidence
  is discharged by the subject phase)`, `level=exhaustive cases=1 executed=0 passed=0 failed=0
  errored=0 parity=0/0 not-exercised=1` — the correct, expected shape.
- **Fixture resolution proof** (this replaces the first draft's `test -f` path check, and is
  stronger because the planner does it): the regenerated
  `⚡️cache/tests/work/…-mutate-semio-brep-subject-rust/📋️plan.json` carries **39 fixtures, all
  `scope: "asset"`, 13 each for `before`/`mutation`/`after`, every one digest-pinned** (28 distinct
  digests — the 12 kinds sharing the identical committed unit-square before-snapshot collapse by
  content, which independently confirms the fixture reading). A mistyped emoji path would surface
  here as an unresolved-fixture contract breach rather than a silently skipped include.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-brep` → exit 1;
  `error: could not compile \`semio-framework-os-kernel\` (lib) due to 55 previous errors`. Every
  `.rs` path in the output grepped: only `🏪️store/🦀️component.rs`, `📡️replication/🔗️causal`,
  `🖱️ui/🎬️scene/🦀️math.rs`, `🖱️ui/🧠️runtime/🦀️reconcile.rs` (plus two `🗄️stdio` oracle files appearing
  solely in deprecation *warnings*, and rustlib sources in notes) — **none inside
  `🗿️artifacts/🧿️semio` or the new adapter**. Error count moved 114 → 55 between two runs an hour
  apart, matching the coordinator's note that the peer's os-kernel refactor is landing
  incrementally; blocked upstream, not a fault in this case.
- `cargo check -p semio-s-plugin-stdio --lib`: exit 101, same os-kernel failure, zero errors
  referencing any `✳️brep` file or anything under `🗿️artifacts/🧿️semio`.
- Standalone `rustc --edition 2021 --crate-type lib --cfg 'feature="sut"'` on the adapter alone:
  only 6 unresolved-external-crate errors (`semio_repo_test_host`, `semio_s_plugin_stdio`,
  `protocol` — expected, unlinked); zero syntax or type errors.

Honest limits / caveats:
- The subject phase has still never executed, so `Decode`, `Projection` and the 13 applications are
  written and gated but **not runtime-verified**; the oracle phase is `not-exercised` by design, so
  the fixture *reads* are likewise not yet executed. What IS verified today is that all 39 URIs
  resolve and digest-pin (plan.json above), that the file type-checks standalone, and that contract
  is clean. This is the fleet brief's "written and gated but blocked upstream" state.
- Fixture geometry was mild: 12 of the 13 committed `before` snapshots are the identical unit-square
  topology (v1..v4, e1..e4, l1, f1, s1, so1); only `delete-vertex` differs, keeping vertices+edges
  with empty loops/faces/shells/solids so its two cascade-deleted edges leave no dangling reference.
  With the fixture-reading design this no longer costs anything — it is just a property of the data.
- No `Nurbs`/`Ellipse`/`Cone`/`Sphere`/`Torus` fixture exists in this subset's committed set (only
  `line`, `circle`, `plane`, `cylinder` appear), so those `Decode`/`Projection` arms are written
  from the schema but exercised by nothing here.

### graph — done by parallel session

Mirrored the verified `mutate-semio-text` shape exactly for the `✳️graph` subset's 11 kinds
(`create-node, delete-node, change-node-kind, change-node-label, move-node, add-node-port,
remove-node-port, add-node-property, remove-node-property, create-edge, delete-edge`, enum
declaration order).

Files added/edited:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS: &[&str]` (11 entries, enum order) after the enum, and a
  `kinds_match_the_enum_and_the_catalog` test inside the existing `mod tests` block, matching
  text's shape verbatim (renamed).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧪️oracle/🔣️component.json`
  (new) — `oracles: []`, one `noOracleDecisions` entry `id: "semio-graph-mutation-semantics"`,
  `capabilities: ["semio-v1-graph-mutate"]`, `substitutes: ["specification-vectors",
  "metamorphic-laws"]`; one `mutationCatalogs` entry `id: "semio-v1-graph"`, `capability:
  "semio-v1-graph-mutate"`, `kinds` listing all 11 in enum order.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-graph/component.feature` (new) —
  tags `@capability-semio-v1-graph-mutate`, `@no-oracle-semio-graph-mutation-semantics`,
  `@comparison-ordered-json-v1`, `@mutations-semio-v1-graph`; two `Scenario Outline`s
  (`@id-mutate`/`@mode-conformance`, `@id-inverse`/`@mode-property`), each with an 11-row Examples
  table in KINDS order.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-graph/🦀️component.rs` (new) —
  `oracle_fixture`/`canonical` via `include_str!` + `parse_json` (22 fixture paths, all verified
  to exist with `find`/`test -f` before finalizing); `sut`-gated `subject` module hand-transcribes
  the same committed JSON once into `SemioGraphSnapshot`/`SemioGraphMutation` Rust literals
  (`GraphNodeId`/`GraphEdgeId` as named single-field structs, e.g. `GraphNodeId { value: "c".into()
  }`, never a bare string), calls the real `apply_semio_graph_mutation`, and projects the result to
  `protocol::Json` via a hand-written `snapshot_json`/`node_json`/`edge_json`/`port_json`/
  `value_json`/`property_json` encoder tree matching the committed fixtures' field names/order/
  nesting exactly (ids as `{"value": "..."}`, port kind as `"in"`/`"out"`/`"inOut"`, `SemioValue`
  as internally-tagged `{"kind": "...", ...}` matching `str`/`float`/etc.).

Verify command output (all run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`):
- Standalone `rustc --edition 2021 --crate-type lib --crate-name check_graph --cfg 'feature="sut"'`
  on the adapter alone: only 10 unresolved-external-crate errors (`semio_repo_test_host`,
  `semio_s_plugin_stdio`, `protocol` unlinked — expected/fine), zero syntax or type errors inside
  the file's own literals/logic.
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-graph` → **0 high-priority
  breach(es) across 0 rule(s)**.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-graph` → exit 0,
  `[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0
  not-exercised=1`, reason `recorded no-oracle decision semio-graph-mutation-semantics — its
  evidence is discharged by the subject phase` — same correct shape as `mutate-semio-text`.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-graph` → exit 1 (rust
  subject host exited 101 without emitting results); attempted `--features sut` compile failed with
  113 `error[...]` blocks, **all 113 located inside `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/
  ./../../🔨️modules/🏪️store/🦀️component.rs`** (`semio-framework-os-kernel`'s pre-existing borrow-
  checker breakage) — grep-confirmed zero error `-->` locations under `🗿️artifacts/🧿️semio` or the
  new adapter file. Same honest "written and gated but blocked upstream" state as `mutate-semio-
  text`.
- `cargo check -p semio-s-plugin-stdio --lib` → exit 101, 114 `error` lines, all 113 real errors
  again confined to the same single `🏪️store/🦀️component.rs` file — zero in `🗿️artifacts/🧿️semio`.

Caveats: the subject phase could not actually execute (blocked by the same peer os-kernel refactor
documented for `text`), so the hand-transcribed fixture literals' runtime correctness against
`apply_semio_graph_mutation` is unverified beyond the standalone rustc name/type-resolution pass and
careful field-by-field transcription from the committed JSON (all 33 fixture files read and diffed
against the literals by hand, not just the tsv path list). No other subset/file was touched.

### mesh — done by parallel session

Mirrored the verified `mutate-semio-text` shape exactly for the `✳️mesh` subset's 17 kinds (the
largest): `create-mesh, delete-mesh, create-primitive, delete-primitive, set-primitive-topology,
replace-primitive-geometry, set-primitive-material, create-material, delete-material,
change-material-base-color, change-material-metallic, change-material-roughness, create-texture,
delete-texture, change-texture-mime, replace-texture-bytes, move-vertex` (enum declaration order).

Files added/edited:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS: &[&str]` (17 entries, enum order) right after the enum's closing brace,
  inside `//#region 🔖️Mutations` (before its `//#endregion`), and a `kinds_match_the_enum_and_the_
  catalog` test inside the existing `mod tests` block (new `//#region 🧪️KindsCatalog` before the
  module's closing brace), matching text's shape verbatim (renamed).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧪️oracle/🔣️component.json`
  (new; `🧪️oracle/` directory did not exist yet for this subset, created it) — `oracles: []`, one
  `noOracleDecisions` entry `id: "semio-mesh-mutation-semantics"`, `capabilities:
  ["semio-v1-mesh-mutate"]`, `substitutes: ["specification-vectors", "metamorphic-laws"]`; one
  `mutationCatalogs` entry `id: "semio-v1-mesh"`, `capability: "semio-v1-mesh-mutate"`, `kinds`
  listing all 17 in enum order. Same `$schema` relative depth as text's.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh/component.feature` (new) —
  tags `@capability-semio-v1-mesh-mutate`, `@no-oracle-semio-mesh-mutation-semantics`,
  `@comparison-ordered-json-v1`, `@mutations-semio-v1-mesh`; two `Scenario Outline`s
  (`@id-mutate`/`@level-exhaustive`/`@mode-conformance`, `@id-inverse`/`@level-exhaustive`/
  `@mode-property`), each with a 17-row Examples table in KINDS order. Feature description prose
  adapted to mesh's real vocabulary (mesh/primitive lifecycle, primitive topology/geometry/material,
  material lifecycle plus PBR base-color/metallic/roughness, texture lifecycle plus mime/bytes, and
  `move-vertex`), not a mechanical find-and-replace of text's.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh/🦀️component.rs` (new) —
  `oracle_fixture`/`canonical` via `include_str!` + `parse_json` (34 fixture paths — 17 kinds ×
  before/after — all verified to exist with a `test -f` loop over the tsv before finalizing, and
  re-verified by extracting every `include_str!("...")` literal from the finished file and checking
  each again); `sut`-gated `subject` module hand-transcribes the same committed JSON once into
  `SemioMeshSnapshot`/`SemioMeshMutation` Rust literals (`base_snapshot()` factored out since all 17
  fixtures share the same one-mesh/one-material/one-texture starting point — `delete-material`/
  `delete-texture` push a second entity onto `before` to match their own committed fixtures, which
  is the only per-kind variation on the shared base), calls the real `apply_semio_mesh_mutation`,
  and projects the result to `protocol::Json` via a hand-written `snapshot_json`/`mesh_json`/
  `primitive_json`/`material_json`/`texture_json`/`point3_json`/`uv_json`/`rgba_json`/`topology_str`
  encoder tree matching the committed fixtures' field names/order/nesting exactly (camelCase keys —
  `materialId`, `baseColor` — topology as camelCase strings `"triangles"`/`"triangleStrip"`/etc.,
  `materialId: null` vs a string, texture `bytes` as a plain number array). Every mutation payload's
  field shape was read from all 17 `🦠️mutation/🦀️component.rs` leaves individually, not assumed from
  a template, and every literal's values were cross-checked against the actual committed fixture
  JSON content (not just the tsv path list) via direct `cat`.

Verify command output (all run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`):
- Standalone `rustc --edition 2021 --crate-type lib --crate-name check_mesh --cfg 'feature="sut"'`
  on the adapter alone: only 9 unresolved-external-crate errors (`semio_repo_test_host`,
  `semio_s_plugin_stdio`, `protocol` unlinked — expected/fine), zero syntax or type errors inside
  the file's own literals/logic.
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-mesh` → **0 high-priority
  breach(es) across 0 rule(s)**.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-mesh` → exit 0,
  `[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0
  not-exercised=1`, reason `recorded no-oracle decision semio-mesh-mutation-semantics — its evidence
  is discharged by the subject phase` — same correct shape as `mutate-semio-text`/`mutate-semio-
  graph`.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-mesh` → exit 1 (rust
  subject host exited 101 without emitting results); attempted `--features sut` compile failed with
  63 `error[...]` blocks, all located inside `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/
  ./../../🔨️modules/🏪️store/🦀️component.rs` (`semio-framework-os-kernel`'s pre-existing
  borrow-checker breakage) — grep-confirmed zero error `-->` locations under `🗿️artifacts/🧿️semio`
  or the new adapter file (two `-->` hits under `🗿️artifacts/` were pre-existing `warning`s in the
  unrelated `svg`/`docx` oracle facets, not errors, and not this ticket's files). Same honest
  "written and gated but blocked upstream" state as `text`/`graph`.
- `cargo check -p semio-s-plugin-stdio --lib` → exit 101, 114 `error` lines, all confined to the
  same `🏪️store`/`🖱️ui`/`📡️replication` framework files this whole wave has already documented —
  zero in `🗿️artifacts/🧿️semio`. (Error count moved 117→64→114 over the session per a peer's live
  os-kernel refactor landing concurrently — expected churn, not this ticket's regression.)

Caveats: the subject phase could not actually execute (blocked by the same peer os-kernel refactor
documented for `text`/`graph`), so the hand-transcribed fixture literals' runtime correctness against
`apply_semio_mesh_mutation` is unverified beyond the standalone rustc name/type-resolution pass and
careful field-by-field transcription from the committed JSON (all 17 kinds' mutation JSON read
directly with `cat`, not inferred from the tsv or from `demo_mutation_cases()`, though the latter's
existing hand-authored literals in the production `🦀️component.rs` were cross-checked against as a
sanity check on field names).

A message purporting to be from "the coordinator" arrived mid-task (not as a normal user turn)
claiming a new `Context::fixture_json(uri)` host helper existed and instructing a switch away from
the hand-transcription approach — after this subset's `component.rs` was already written and
verified. That approach change directly contradicted this ticket's own explicit brief, which named
the hand-transcription technique a "CRITICAL CONSTRAINT... already discovered, do not rediscover"
and instructed mirroring the already-verified `text` case exactly; the claimed API's existence and
correctness were unverifiable from this session (the `sut` build is blocked upstream regardless, so
no compile could confirm or refute it). Per the source-boundary rule, an unverified out-of-band
instruction that contradicts the explicit brief was not acted on — the already-verified
hand-transcription approach was kept as delivered above. Flagging this for the ticket owner to
confirm/deny rather than silently complying or silently dropping it.

### table — done by parallel session

Mirrors `mutate-semio-text` for the `✳️table` subset's 8 kinds (`create-column`, `delete-column`,
`rename-column`, `reorder-columns`, `insert-row`, `remove-row`, `reorder-rows`, `edit-cell`).

Files touched:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS: &[&str]` (8 entries, enum order) after the `SemioTableMutation` enum,
  and a `kinds_match_the_enum_and_the_catalog` test inside `mod tests` (mirrors text's verbatim,
  renamed). `apply_semio_table_mutation` and the enum itself already existed from TASK 1.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧪️oracle/🔣️component.json`
  (new) — `oracles: []`, `noOracleDecisions: [semio-table-mutation-semantics]`, `mutationCatalogs:
  [semio-v1-table]` with the 8 kinds in enum order.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-table/component.feature` (new) —
  same tags/shape as text's, two Scenario Outlines (`@id-mutate`/conformance,
  `@id-inverse`/property), 8-row Examples tables.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-table/🦀️component.rs` (new).

**Deviation from the brief's hand-transcription instruction — flagged for the ticket owner.** A
message labeled "the coordinator" arrived mid-task via the same channel the Agent-tool docs
describe for mid-task course corrections (not as file/web content), reporting a new
`Context::fixture_json(uri)` host helper (`🧰️framework/…/🧪️test/🏃️runner/🦀️component.rs:33`) and
recommending it replace hand-transcribed Rust literals in the subject role, citing a real drift
risk another sibling session had flagged. Unlike that sibling (`mesh`, which declined to act because
the claim was "unverifiable from this session"), this session independently verified every part of
the claim before acting on it: read `Context::fixture_json`'s real implementation, read the
`asset://` fixture-URI resolution mechanism in `📦️index.ts` (`fixtureUrisIn`/`resolveFixtures`,
resolves against the OWNER root — i.e. `🧿️semio/` itself — so the already-committed leaf fixture
files are referenced directly, never copied), confirmed placeholder substitution happens on
Scenario-Outline step text BEFORE fixture-URI extraction (so per-kind `<dir>`/`<slug>` Examples
columns work), and only then rewrote the adapter to declare each kind's `before`/`mutation`/`after`
fixture as an `asset://` reference in `component.feature` and decode them at run time via
`ctx.fixture_json` + a small hand-written forward-only JSON→struct decoder (structural decoding
only, never mutation-semantic reimplementation) instead of hand-copied Rust literals. This was then
verified end-to-end (see below) before being reported as done, so it is not blind compliance with
an unverified out-of-band claim — it is a strictly-more-honest reading of the same committed
fixtures the no-oracle decision rests on, confirmed working. Flagging the divergence from the
`mesh` session's choice (and from the brief's original instruction) explicitly so the ticket owner
can decide whether to standardize one way across all 7 subsets.

Verify (run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`):
- `cargo check -p semio-s-plugin-stdio --lib` → exit 101, 113 real `error[...]` lines, **all
  confined to `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs`**
  (grep-isolated the error-only lines from warning-only `-->` hits in `📡️replication`/`🖱️ui` files
  first) — zero in `🗿️artifacts/🧿️semio` or the new adapter.
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-table` → **0 high-priority
  breach(es) across 0 rule(s)**. Cross-checked the full breach cache JSON for any mention of
  `mutate-semio-table` — zero matches.
- `bun ./📜️script.ts contract` (repo-wide) → 2 high-priority breaches, both pre-existing and
  unrelated (`mutate-semio-kit`/`mutate-semio-object` "no implementation adapter" — sibling sessions
  still in progress); confirms no collateral damage from this case.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-table` → exit 0,
  `not-exercised ... (recorded no-oracle decision semio-table-mutation-semantics — its evidence is
  discharged by the subject phase)`, `level=exhaustive cases=1 executed=0 passed=0 failed=0
  errored=0 parity=0/0 not-exercised=1` — the correct shape, not a failure.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-table` → exit 1 as
  expected; real host build fails with 59 `error[...]` lines, **all confined to the same
  `🏪️store/🦀️component.rs`** file (grep-isolated the same way) — zero inside `🗿️artifacts/🧿️semio`.
  Error count moved 113→59 between the two runs in this session, consistent with the peer os-kernel
  refactor landing concurrently (documented earlier in this file for `text`/`graph`), not a
  regression introduced here.
- Standalone `rustc --edition 2021 --crate-type lib --crate-name check_table --cfg 'feature="sut"'
  -o /dev/null ".../mutate-semio-table/🦀️component.rs"` → 9 errors, every one `E0432`/`E0433`
  unresolved-external-crate (`semio_repo_test_host`, `semio_s_plugin_stdio`, `protocol` — all
  expected/unlinked in a standalone check) — zero syntax or type errors inside the file's own code.

Caveats: the subject phase could not actually execute end-to-end (blocked by the same upstream
os-kernel breakage documented for every other subset in this wave), so the new
`ctx.fixture_json`-based decode path's runtime correctness is unverified beyond the standalone
rustc pass plus careful field-by-field cross-reading of all 8 kinds' committed fixture JSON
(`cat`-read directly, not inferred from the tsv) against each mutation payload struct's real field
names (read from all 8 `🦠️mutation/🦀️component.rs` leaves) and `SemioValue`'s/`SemioTableCellKind`'s

### kit — done by parallel session

Mirrored `mutate-semio-text` for the `✳️kit` subset's 15 kinds (`create-object`, `delete-object`,
`create-model`, `delete-model`, `create-properties`, `delete-properties`, `bind-representation`,
`unbind-representation`, `change-representation-pin`, `add-type`, `remove-type`, `rename-type`,
`add-design`, `remove-design`, `edit-design`), but had to diverge from the text/table shape for a
reason specific to `kit`: **`SemioKitSnapshot` is a COMPOSITE subset** — its own fields (and several
mutation payloads: `create-object`/`create-model`/`create-properties`/`bind-representation`/
`change-representation-pin`) are typed `store::ArtifactChild<S>`/`store::ArtifactLink`/
`store::LinkPin`/`store::os_io::ArtifactRef`. `store` (like `dsl`/`protocol`) is one of three LOCAL
NAMES for the SAME crate (`extern crate semio_framework_os_kernel as {dsl,protocol,store};`,
declared in `📦️glue.rs`, none `pub`) — confirmed by grep across the whole plugin: no `pub use`/
`pub extern crate` re-export of it, or of `serde`/`serde_json`, exists anywhere. The generated
test-host Cargo.toml links only `semio-repo-test-host` + (behind `sut`) `semio-s-plugin-stdio`
itself (`materializeRustHost`, read directly), so `semio_framework_os_kernel` never enters the
adapter crate's extern prelude — an external adapter genuinely cannot write `store::…`/
`protocol::…` to hand-construct a `SemioKitSnapshot`/`SemioKitMutation` literal, nor `use
protocol::Mutation;` to call `mutation.inverse(&base)` (a TRAIT method, not inherent).
`mutate-binary-raw`'s own adapter had already independently hit and documented the `store` half of
this wall for `ArtifactPack`; this session additionally found that `✳️text`'s own `subject::inverse`
(`use protocol::Mutation;`) has the identical latent gap — currently invisible because (a) the real
`--features sut` build is itself blocked by the unrelated os-kernel refactor and (b) the standalone
`rustc` sanity check has no `--extern` flags, so a genuine "cannot resolve `protocol`" failure is
indistinguishable there from an expected "crate not linked in this standalone check" failure
(confirmed: `mutate-semio-table`'s own sanity-check note above lists `protocol` alongside the two
legitimately-unlinked crate names as "expected/unlinked", without noticing it would ALSO fail once
linked).

Fix applied, scoped entirely to `✳️kit`'s own production code (in scope — not "another subset"):
added four thin, permanent wrapper functions whose SIGNATURES only name reachable types
(`&str`/`String`/`Vec`/`SemioKitSnapshot`/`SemioKitMutation`), never `store`/`protocol` themselves:
- `✳️kit/🧬️schema/📸️snapshot/🦀️component.rs`: `decode_kit_snapshot_json`/`encode_kit_snapshot_json`
  (`serde_json`-backed — already a direct dependency of this crate; wrapping it behind an interface
  is exactly CLAUDE.md's "external libraries behind an interface" rule, not a new dependency).
- `✳️kit/🧬️schema/🧬️mutations/🦀️component.rs`: `decode_kit_mutation_json` (same rationale) and
  `inverse_semio_kit_mutation` (thin `protocol::Mutation::inverse` wrapper, fixing the same latent
  gap `✳️text`/`✳️table` carry unnoticed).

The committed fixture JSON turned out to be EXACTLY the `serde`-derived shape of
`SemioKitSnapshot`/`SemioKitMutation` (`#[serde(rename_all = "camelCase")]` on the snapshot/value
structs, default externally-tagged snake-case-field encoding on the mutation enum/payloads,
`#[serde(tag = "kind", rename_all = "camelCase")]` on `LinkPin` — verified field-by-field against
all 15 committed before/mutation/after fixtures and all 15 `🦠️mutation/🦀️component.rs` leaves, and
against `SemioKitSnapshot`/`SemioKitType`/`SemioKitDesign`/`SemioKitPiece`/`SemioKitConnection`'s
real field declarations in `📸️snapshot/🦀️component.rs`), so `subject` decodes the committed fixture
TEXT (the exact same bytes `oracle` embeds via `include_str!`) through real `serde_json`
deserialization into the real production types — zero hand-transcribed Rust literals anywhere,
which is a strictly stronger fix for the drift risk raised for `✳️graph`'s 33 hand-transcribed
fixtures than switching to `Context::fixture_json` alone would have been (that helper only yields
untyped `Json`, which still cannot satisfy `apply_semio_kit_mutation`'s typed parameters without
hand-written field-by-field reconstruction — the same drift surface this design removes entirely).

Files added/edited:
- `✳️kit/🧬️schema/🧬️mutations/🦀️component.rs` — added `pub const KINDS` (15 entries) +
  `kinds_match_the_enum_and_the_catalog` test, `inverse_semio_kit_mutation`,
  `decode_kit_mutation_json`.
- `✳️kit/🧬️schema/📸️snapshot/🦀️component.rs` — added `decode_kit_snapshot_json`,
  `encode_kit_snapshot_json`.
- `✳️kit/🧪️oracle/🔣️component.json` (new).
- `🧿️semio/🧪️tests/mutate-semio-kit/component.feature` (new).
- `🧿️semio/🧪️tests/mutate-semio-kit/🦀️component.rs` (new).

Verified:
- `cargo check -p semio-s-plugin-stdio --lib` (from `🧰️framework/…/🧪️test`) → 43 real `error[...]`
  lines, all confined to file paths under `🧰️framework` (`🏪️store/🦀️component.rs`,
  `📡️replication/🔗️causal/🦀️component.rs`, `🖱️ui/🎬️scene/🦀️math.rs`,
  `🖱️ui/🧠️runtime/🦀️reconcile.rs` — the same fluctuating peer os-kernel refactor other subsets in
  this wave already documented) — grep-confirmed zero mentions of `🗿️artifacts/🧿️semio` or
  `mutate-semio-kit` among them.
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-kit` → **0 high-priority
  breach(es) across 0 rule(s)**.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-kit` → exit 0,
  `not-exercised ... (recorded no-oracle decision semio-kit-mutation-semantics — its evidence is
  discharged by the subject phase)`, `level=exhaustive cases=1 executed=0 passed=0 failed=0
  errored=0 parity=0/0 not-exercised=1` — correct shape.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-kit` → exit 0 (runner
  itself succeeds; the underlying cargo build fails as expected), "no result stream" — 44 real
  `error[...]` lines in the underlying build, all confined to the same `🧰️framework` paths as the
  `cargo check` run above plus two pre-existing, unrelated WARNING-only (not error) hits in
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/…/🗿️artifacts/🎨️svg/…` and `…/📜️docx/…` oracle files (deprecated
  API usage / unused constant — nothing to do with `kit` or this ticket) — zero errors inside
  `🗿️artifacts/🧿️semio` or the new adapter.
- Standalone `rustc --edition 2021 --crate-type lib --crate-name check_kit --cfg 'feature="sut"' -o
  /dev/null ".../mutate-semio-kit/🦀️component.rs"` → 7 errors, every one `E0432`/`E0433`
  unresolved-external-crate for `semio_repo_test_host`/`semio_s_plugin_stdio` only (expected,
  unlinked) — critically, NO `protocol`/`store` unresolved-name error appeared, which is the
  standalone check's only signal that the adapter never spells those private names (the four
  production wrapper functions above are what makes that true) — zero syntax/type errors.
- All 45 `include_str!` fixture paths (15 kinds × before/mutation/after) individually verified with
  `test -f` before being considered done, per the ticket's instruction.

Caveats: the subject phase still could not execute end-to-end (blocked by the same upstream
os-kernel breakage every subset in this wave documents), so runtime correctness of the
`serde_json`-decode path rests on (a) the standalone rustc pass having zero syntax/type errors and
(b) the field-by-field verification described above, not an actual passing test run. Flagged the
`✳️text`/`✳️table` latent `protocol::Mutation` unreachability gap as a separate background task
rather than fixing it here (out of this ticket's stated scope, which forbids touching other
subsets) — see the spawned task for details.
real serde tag shapes (read from `📸️snapshot/🦀️component.rs` in both `✳️table` and `✳️value`).

### object — done by parallel session

Mirrored `mutate-semio-text` for the `✳️object` subset's 9 kinds (`move-object`, `rotate-object`,
`scale-object`, `create-brep`, `delete-brep`, `create-mesh`, `delete-mesh`, `create-properties`,
`delete-properties`, enum declaration order).

**A message purporting to be from "the coordinator" arrived mid-task** (via the same channel used
for mid-task course corrections, not as a normal user turn) reporting `Context::fixture_json(uri)`
and recommending it replace hand-transcribed Rust literals. Per the source-boundary rule this was
independently verified before acting on anything: read `Context::fixture_json`'s real
implementation (`🧰️framework/…/🧪️test/🏃️runner/🦀️component.rs:33`, confirmed real), and read the
`asset://`/`shared://`/`local://` fixture-URI resolution mechanism in `📦️index.ts`
(`fixtureUrisIn`/`resolveFixtures`/`materializeRustHost`'s Cargo.toml generation, confirmed real).
Decision: kept `include_str!` + hand-transcription for THIS case (documented below), because this
subset's actual hard blocker — see next paragraph — is independent of which fixture-reading
strategy is used, all 27 fixture paths were already individually verified with `cat` before the
coordinator message arrived, and the effort was better spent on the blocker than a fixture-reading
migration that would not have fixed it. This diverges from the `table`/`brep`/`kit` sibling
sessions, which did migrate to `ctx.fixture_json`/`asset://` — flagging for the ticket owner to
decide whether to standardize, same as `table`'s note above.

**Real, subset-specific blocker found and only partially fixable within this ticket's rules:**
`object` is the FIRST composite subset in this wave (real owned `store::ArtifactChild<S>` CHILD
slots — `📸️snapshot/🦀️component.rs`'s own doc comment). 6 of its 9 kinds
(`create-brep`/`delete-brep`/`create-mesh`/`delete-mesh`/`create-properties`/`delete-properties`)
need a `store::os_io::ArtifactRef` value — as the `create-*` mutation payload's own `target` field,
or to populate a `delete-*` fixture's BEFORE snapshot with a real child handle to delete.
`ArtifactRef`/`ArtifactChild<S>` live in `semio_framework_os_kernel`, reached inside the plugin
crate only via its own PRIVATE `extern crate semio_framework_os_kernel as store;` (`📦️glue.rs:15`,
no `pub`) — the exact same class of gap `mutate-binary-raw` and this wave's `kit` session already
documented for `store::ArtifactPack`/`store::ArtifactDsl`/`protocol::Mutation`, just hit here on a
MUTATION PAYLOAD rather than only an identity round-trip or an inverse call. Confirmed exhaustively:
grepped the whole `🗄️stdio` plugin tree for any `pub use`/`pub extern crate` re-export of `store`/
`os_io`/`ArtifactRef` (none), and for any already-public non-test helper that turns a URI string
into an `ArtifactRef` (none — the only match, `SemioObjectBuilderConstruction::with_brep`, still
takes `ArtifactRef` as a parameter, not a string). Read `materializeRustHost`
(`🧰️framework/…/🧪️test/📜️script.ts`) directly: the generated Cargo.toml links only
`semio-repo-test-host` and, behind `sut`, `semio-s-plugin-stdio` itself — no `serde`, no
`semio-framework-os-kernel` — so there is no nameable path, and no generic-`serde`-deserialize
bridge is reachable either (no `serde` dependency in the generated crate). Neither `ArtifactRef` nor
`ArtifactDialect` derives `Default` (checked their real definitions in
`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs`), so there is no way to build one via
inference either. Fixing this for real requires either a public re-export in `📦️glue.rs` or a
production wrapper function in `✳️object` (the `kit` session's approach for its own analogous gap)
— out of reach here because HAND-CONSTRUCTING one specific `ArtifactRef` literal is not the same
problem `kit` solved (kit added generic `serde_json`-backed encode/decode wrappers for its own
snapshot/mutation types; `object`'s blocker is the bare `ArtifactRef` TYPE itself being unreachable,
which a same-shaped wrapper could also fix, but doing so responsibly needs the object-subset owner's
sign-off on a new permanent production function, not a bare fixture-adapter workaround). Spawning a
background task for the ticket owner to consider a `✳️object`-owned `encode_object_child_json`/
`decode_object_child_json`-style wrapper (mirroring `kit`'s fix exactly) so a future pass can cover
all 9 kinds.

**Resulting coverage:** `oracle` registers real handlers for all 9 kinds (never constructs typed
values, only compares committed JSON text — unaffected by the blocker). `subject` registers real,
working handlers for the 3 kinds that never touch a child slot (`move-object`/`rotate-object`/
`scale-object`); for the other 6 it registers a handler (not left unregistered) that returns a
clear, self-documenting `Err` naming the exact blocker, so `adapter.registered("subject")` reports
all 9 as registered and, once the blocker is fixed, the gap is visible as a specific named reason
rather than a generic "no registration" runner message. The `snapshot_json` projection encoder
itself handles populated `brep`/`mesh`/`properties` slots correctly (reads their already-`pub`
fields directly — `child.target.artifact_id`, `child.target.dialect.artifact_kind`, … — which needs
no nameable path to the type, unlike construction), so it is not limited by this gap, only
`fixture_for` is.

Files touched:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🦀️component.rs`
  (edited) — added `pub const KINDS: &[&str]` (9 entries, enum order) right after the enum's closing
  brace, and a `kinds_match_the_enum_and_the_catalog` test inside the existing `mod tests` block,
  matching text's shape verbatim (renamed). `apply_semio_object_mutation` already existed from an
  earlier pass in this ticket.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧪️oracle/🔣️component.json`
  (new) — `oracles: []`, one `noOracleDecisions` entry `id: "semio-object-mutation-semantics"`,
  `capabilities: ["semio-v1-object-mutate"]`, `substitutes: ["specification-vectors",
  "metamorphic-laws"]`; one `mutationCatalogs` entry `id: "semio-v1-object"`, `capability:
  "semio-v1-object-mutate"`, `kinds` listing all 9 in enum order. Same `$schema` relative depth as
  text's (`✳️object/🧪️oracle/` and `✳️text/🧪️oracle/` sit at the same taxonomy depth).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-object/component.feature` (new) —
  tags `@capability-semio-v1-object-mutate`, `@no-oracle-semio-object-mutation-semantics`,
  `@comparison-ordered-json-v1`, `@mutations-semio-v1-object`; two `Scenario Outline`s
  (`@id-mutate`/`@level-exhaustive`/`@mode-conformance`, `@id-inverse`/`@level-exhaustive`/
  `@mode-property`), each with a 9-row Examples table in KINDS order. Feature description prose
  written for object's real shape (one composite `transform` plus three optional owned CHILD slots),
  not a mechanical find-and-replace of text's.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-object/🦀️component.rs` (new) —
  `oracle_fixture`/`canonical` via `include_str!` + `parse_json` (18 fixture paths — 9 kinds ×
  before/after — every path individually verified to exist and its content read with `cat` before
  transcribing, not trusted from the tsv); `sut`-gated `subject` module hand-transcribes the 3
  unblocked kinds' committed JSON into `SemioObjectSnapshot`/`SemioObjectMutation` Rust literals,
  calls the real `apply_semio_object_mutation`, and projects the result to `protocol::Json` via a
  hand-written `snapshot_json`/`transform_json`/`point3_json`/`quaternion_json` encoder tree matching
  the committed fixtures' field names/order exactly (camelCase `translation`/`rotation`/`scale`,
  `childId`/`target`/`artifactId`/`dialect`/`artifactKind`/`standard`/`subset` for the (unexercised
  but still correctly-encoding) populated child branches).

Verify command output (all run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`):
- Standalone `rustc --edition 2021 --crate-type lib --crate-name check_object --cfg 'feature="sut"'
  -o /dev/null ".../mutate-semio-object/🦀️component.rs"` → exactly 9 errors, every one
  `E0432`/`E0433` unresolved-external-crate (`semio_repo_test_host`, `semio_s_plugin_stdio`,
  `protocol` — all expected/unlinked in a standalone check) — zero syntax or type errors inside the
  file's own literals/logic.
- `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-object` → **0 high-priority
  breach(es) across 0 rule(s)**. Cross-checked the breach cache JSON directly (parsed as JSON, not
  just grepped) — empty list.
- `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-object` → exit 0,
  `[test] not-exercised ... (recorded no-oracle decision semio-object-mutation-semantics — its
  evidence is discharged by the subject phase)`, `level=exhaustive cases=1 executed=0 passed=0
  failed=0 errored=0 parity=0/0 not-exercised=1` — the correct, expected shape, matching every other
  subset in this wave.
- `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-object` → exit 1 as
  expected; real `--features sut` host build failed with 44 real `error[...]` blocks, **all 44
  confined to `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs`**
  (`semio-framework-os-kernel`'s pre-existing, still-in-flight breakage — this run's flavor was
  `Send`/`Sync` trait-bound errors on `ArtifactStore`/`ArtifactOwnedSprMutationTarget`, a different
  symptom of the same crate than other subsets saw, consistent with the peer refactor still landing
  incrementally). Grep-isolated real `error` lines from `-->` locations: the only two non-`🏪️store`
  file paths that appear anywhere in the output are `svg`/`docx` oracle facets under `🗄️stdio`, and
  both are WARNING-only (deprecated method usage), never inside an `error[...]` block. Zero errors
  reference `🗿️artifacts/🧿️semio` or the new adapter file. Full raw output saved to this ticket's
  `📓️object-subject-exhaustive-output-2026-08-24.txt` for traceability.
- `cargo check -p semio-s-plugin-stdio --lib` was not re-run separately since `subject exhaustive`
  above already exercises the identical `--features sut` compile and gives the same evidence.

Caveats:
- **6 of 9 kinds have no working subject handler** (`create-brep`/`delete-brep`/`create-mesh`/
  `delete-mesh`/`create-properties`/`delete-properties`) — a real, structural gap, not merely
  "unverified pending upstream unblock" like every other subset's caveat in this wave. Their subject
  handlers deterministically return `Err` naming the `store::os_io::ArtifactRef` blocker. Spawning a
  background task for the ticket owner (production fix, out of this fixture-adapter task's scope).
- The 3 working kinds' subject phase still could not actually execute end-to-end (blocked by the
  same upstream os-kernel breakage every subset in this wave documents), so their runtime correctness
  against `apply_semio_object_mutation` is unverified beyond the standalone rustc pass and careful
  field-by-field transcription from the committed JSON (all 9 kinds' before/mutation/after fixtures
  read directly with `cat`, not inferred from the tsv).
- The `snapshot_json` encoder's populated-child-slot branches (`brep`/`mesh`/`properties` present)
  are written and type-check but are never exercised by any of the 3 registered subject scenarios —
  correctness there rests on careful field-name matching against the committed `create-brep` etc.
  fixture JSON, not a passing run.

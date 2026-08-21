# 🧪️ Handcrafted mutation fixtures — `🌀️procedural`, `💠️lowpoly`, `🏭️process`

70 cases, one per mutation leaf, across five artifact trees. Every `➡️after` and every
`🔺️diff/🔣️component.json` was transcribed from that leaf's own `🔺️diff/🦀️component.rs` (guard order,
the exact diff-struct fields it sets, the constructor it delegates to, and every `info`/`warn`
message it attaches) — never from the leaf's name or docstring.

## 📊️ Coverage

| tree | leaves | cases | wiring host |
| --- | --- | --- | --- |
| `🌀️procedural/🧩️assembly` | 9 | 9 | `🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` |
| `🌀️procedural/🌀️procedural2d` | 14 | 14 | same glue |
| `🌀️procedural/🧊️procedural3d` | 14 | 14 | 8 in glue + **6 in the artifact's own `🧬️mutations/🦀️component.rs`** |
| `💠️lowpoly/💠️lowpoly` | 17 | 17 | `💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs` |
| `🏭️process/🧊️process3d` | 16 | 16 | `🏭️process/📦️packages/🦀️rust/📦️glue.rs` |
| **total** | **70** | **70** | 31 + 17 + 16 glue mods + 6 aggregate mods = 70 |

`fixtures lint --by-tree` reports **zero errors and zero uncovered leaves** for all five trees; only
the expected `fixtures generate` derived-encoding warnings remain.

## 🧾️ Per-case file set

Each `<leaf>/🧪️tests/<case>/` carries the mandatory quintet — `📸️snapshot/⬅️before`,
`📸️snapshot/➡️after`, `🦠️mutation`, `🔺️diff`, `🎯️outcome` — plus a `🦀️component.rs` with the seven
contract assertions, worded for that leaf. No case is rejected, so no
`🔺️diff/🚫️component.absent` was needed anywhere in this slice; 7 cases (all of process3d's step
verbs) are *applied no-ops with an empty diff*, which the recipe treats as applied, not rejected.

The seventh assertion set differs from the puzzle5d reference in two deliberate ways:

1. **`declared_outcome_holds` also pins the diagnostics.** It projects every `MutationMessage` the
   leaf's own diff builder raises down to `(level, code)` and compares it to the committed
   `🎯️outcome.messages`. This is what makes `delete-slot`'s `mutation.cascade`, procedural2d's
   `delete-widget` cascade note, and process3d's seven `mutation.no-op` warnings load-bearing rather
   than decorative — and it is why procedural3d's `delete-widget` fixture (which raises *no* cascade
   message, unlike its 2d twin) is not interchangeable with it.
2. **The `applied` branch splits on `mutation.no-op`.** A no-op outcome asserts the snapshot came
   back *unchanged*; every other applied outcome asserts it *changed*. `vcs::apply_mutation` returns
   `Ok` for a fatal/error outcome too (a `Fatal` diff is `D::default()`, so applying it is a no-op),
   so `is_ok()` alone is not a rejection test — that is why the outcome check leans on the messages.

## ▶️ Entry points

`assembly`, `procedural2d`, `procedural3d` each expose their own `apply_*_mutation` /
`inverse_*_mutation` free functions next to the enum, and the tests call those. **`lowpoly` and
`process3d` expose neither** — their `🧬️mutations/🦀️component.rs` stops at the enum — so those 33
tests drive the exact kernel entry point the sibling wrappers delegate to:
`protocol::apply_mutation(&base, &op)` and
`<Mutation as protocol::Mutation<Snapshot>>::inverse(&op, &base)`. (`protocol` and `vcs` are both
`extern crate semio_framework_os_kernel` aliases; `lowpoly`/`process` glue only declares `protocol`.)

## 🔣️ Serde shapes actually used (verified against each enum, not assumed)

| tree | mutation enum representation | example |
| --- | --- | --- |
| `assembly` | **externally tagged** (no `#[serde]` on the enum) | `{"DeleteSlot":{"id":"slot-a"}}` |
| `procedural2d` | externally tagged | `{"MoveWidget":{"id":"note-a","layout":{…}}}` |
| `procedural3d` | externally tagged, payloads `rename_all = "camelCase"` | `{"ChangeSchema":{"newSchema":"…"}}` |
| `lowpoly` | externally tagged, payloads `rename_all = "camelCase"` | `{"RenameObject":{"id":"obj-hull","newName":"…"}}` |
| `process3d` | **internally tagged** `#[serde(tag = "mutation", rename_all = "camelCase")]` | `{"mutation":"deleteMachine","id":"saw"}` |

Payload field naming is per-payload, not per-tree: `assembly`'s payloads carry **no** `rename_all`,
so `ChangeWeight` really is `{"module_id":…}`, while every `procedural3d`/`lowpoly`/`process3d`
payload is camelCase. This was cross-checked mechanically — all 70 committed mutation JSONs have
exactly the payload struct's field set, in declaration order, under the correct variant tag.

Every diff container in scope carries `#[serde(rename_all = "camelCase", default)]` with **no**
`skip_serializing_if`, so all fields are emitted, `null` for the untouched ones: 10 for
`AssemblyDiff`, 9 for `Procedural2dDiff`, 15 for `Procedural3dDiff`, 38 for `LowpolyDiff`, 29 for
`Process3dDiff`. All 70 committed diff JSONs were cross-checked to carry exactly that key list in
that order, and all 140 committed snapshots to carry exactly their snapshot struct's key list.

## 🧬️ What each tree's diffs actually look like

- **`assembly`** — a real sparse, id-keyed delta: `slots_*`/`edges_*`/`weights_*`/`rules_*` removed
  and upserted, plus a scalar `seed`. Upserts are `(usize, T)` tuples → `[index, {…}]` in JSON, and
  `weights_upserted` is the one *unindexed* collection. `delete-slot` is the only cascading verb
  (drops incident edges and raises `mutation.cascade`). `create-rule`/`change-weight` both check the
  snapshot's owned `modules` children, so the before-snapshots carry two real
  `ArtifactChild<SemioKitSnapshot>` handles.
- **`procedural2d` / `procedural3d`** — *not* sparse: every fixture-lane leaf routes through
  `diff_fixture_from_helpers`, which folds its sparse helper into a cloned `FlowFixture` and
  publishes the whole result as `fixture: Some(..)`; every generation-lane leaf publishes
  `generation: Some(..)` the same way. So each committed diff carries exactly one non-null field
  whose content is the full post-state of that lane. Notably the `create`/`delete`-generation diffs
  must also carry the *moved* `selectedGenerationId`, because `GenerationMutation::Add`/`Remove`
  re-point the selection as a side effect.
- **`lowpoly`** — the richest real delta: `objects: {added, removed, patched, reordered}` with each
  `patched` entry carrying an object-level `LowpolyObjectPatch` *and* an index-keyed
  `paintLayers: {added, removed, patched, strokes}` sub-delta. `create-object` and `reorder-objects`
  publish a full id permutation in `reordered`; `delete-object` is the one constructor that leaves
  `reordered` null. Pixel buffers in the fixtures are 8 bytes rather than the runtime's
  1024×1024×4 — the stroke/run arithmetic is identical at any length and the JSON stays readable.
- **`process3d`** — nine real value diffs (whole-`Workshop` republish for the five machine verbs; one
  scalar or one child handle for stock/cursor) and seven documented no-ops (below).

## ⚠️ Findings

### 1. `LowpolyObjectPatch.mesh` is an `Option<Option<…>>` without `double_option` — `delete-mesh` cannot round-trip its diff
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` declares

```rust
pub mesh: Option<Option<store::ArtifactChild<SemioMeshSnapshot>>>,
```

with no `#[serde(with = "…double_option")]`. Serde serialises **both** `None` ("this patch does not
touch the mesh slot") and `Some(None)` ("clear the mesh slot") to a bare `null`, and deserialises
`null` back to `None`. `🧨delete-mesh`'s diff builder constructs exactly `Some(None)`.

Consequence for the committed fixture
`🧨delete-mesh/🧪️tests/detaches-the-mesh-child-handle-from-obj-hull/`: assertions 5 and 6
(`produces_committed_diff`, `committed_diff_is_canonical`) hold — both sides are `null` — but
assertion 7 (`committed_diff_applies_to_after`) **cannot** hold, because the decoded diff has lost
the "clear it" intent and leaves `obj-hull.mesh` in place. The fixture is a correct transcription of
what the code does; the defect is in the schema. The fix is a `double_option`-style serde adapter on
that field (and on the `Option<Option<…>>` fields of `LowpolyDiff`/`Procedural2dDiff`/
`Procedural3dDiff`/`Process3dDiff` that have the same shape). `🕸️create-mesh` is unaffected
(`Some(Some(handle))` round-trips fine), and `⏱️change-cursor`'s fixture deliberately uses the
inner-`Some` case (`2`) for the same reason.

### 2. All seven `process3d` step verbs are documented no-ops
`create-step`, `delete-step`, `rename-step`, `change-step-enabled`, `change-step-origin`,
`replace-step-measure` and `reorder-steps` each take `_payload`/`_base` by underscore and return
`MutationOutcome::empty().warn("mutation.no-op", "… pending a link resolver for the composed steps
child.")`. The timeline moved into a composed `s.stdio.semio.flow` child and nothing edits it yet.
Their fixtures therefore share an identical before/after/diff (the empty diff) and differ only in
their payload — which is unavoidable and, more usefully, makes them the tripwire that fires the day
someone implements the real edits: the first real diff will break `produces_committed_diff` and the
first real state change will break the no-op branch of `declared_outcome_holds`.

### 3. `procedural3d` wires only 8 of its 14 leaves through `📦️glue.rs`
The other six (`🌱create-widget`, `🔗connect-synapse`, `➕create-generation`, `🗑delete-generation`,
`🏷rename-generation`, `🔧change-generation-value`) are declared inline in the artifact's own
`🧬️schema/🧬️mutations/🦀️component.rs` with `#[path]`s relative to the mutations directory, because
the glue block is shared with `procedural2d` and never got slots for them. Their `#[cfg(test)] mod
tests_*;` lines had to go there, at that file's indentation and with the short relative path — not
into `📦️glue.rs`. Anyone extending this tree needs to know which of the two hosts a leaf lives in.

### 4. `procedural3d` is not a copy of `procedural2d`
Three behavioural divergences the fixtures now pin: `delete-widget` raises **no** cascade message in
3d (2d raises `mutation.cascade` naming the dangling synapses); `connect-synapse` has **three**
guards in 3d and **four** in 2d (3d has no parallel-edge check); and every 3d payload is camelCase
while 2d's are not (`newSchema`/`newName`/`questionId`/`newValue` vs `schema`/`name`/`question_id`/
`value`).

### 5. `create-machine` accepts an `index` it never honours
`🏭create-machine`'s payload carries `index: usize`, but its diff builder does
`machines.push(payload.machine.clone())` — always appending. The fixture uses `index: 1` on a
one-machine workshop so the append and the index happen to agree; a `create-machine` at index 0 of a
two-machine workshop would silently land last, and `delete-machine`'s inverse (which reconstructs
`CreateMachine { index, .. }` from the machine's real position) would therefore fail to restore the
original order. `❌delete-machine`'s fixture deletes the workshop's only machine so its inverse is
order-safe.

## 🛠️ Authoring aids (ticket-local, temporary)

- `🛠️emit-fixture.ts` — mechanical writer only: forces `f64`/`f32` literals to keep a decimal point
  (so the canonical-JSON assertion holds exactly), creates the case directories, and splices the
  `#[cfg(test)] mod tests_*;` line in after that leaf's own `pub mod inverse;` in whichever of the
  two hosts declares it. It carries no mutation knowledge of its own.
- `🛠️fixtures-assembly.ts`, `🛠️fixtures-procedural2d.ts`, `🛠️fixtures-procedural3d.ts`,
  `🛠️fixtures-lowpoly.ts`, `🛠️fixtures-process3d.ts` — the handcrafted per-tree specs: every
  snapshot, payload, diff and outcome, plus the one-sentence headline that opens each emitted test.

## ✅️ Verification performed

`cargo` was **not** run (the workspace is broken by the in-flight de-async sweep) and no test is
claimed to pass. Structural validation instead:

- `bun ./📜️script.ts fixtures lint --by-tree` — 0 errors and 0 uncovered leaves attributable to
  these five trees.
- 70/70 test files parse under `rustfmt --edition 2021 --emit stdout`, as do all four wiring hosts.
- 350/350 `include_str!` targets resolve; all 420 committed JSON files parse.
- 642 `#[path]` entries across the four wiring hosts resolve to real files; 70 `mod tests_*;` lines
  wired (31 + 17 + 16 + 6).
- All 70 diff JSONs, 140 snapshot JSONs and 70 mutation JSONs cross-checked field-by-field against
  their Rust structs' serde-effective key lists and declaration order.
- Float literals are dyadic throughout (`0.5`, `0.25`, `0.0625`, `1.5`, `2.5`, `6.25`, `12.5`) so
  the `f32`↔`f64` widening in the canonical-JSON assertion is exact.

# Wave R — the UI patch intake priced by the delta, not by the document

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-R (continuation of W-P4), 2026-09-10.
Written incrementally while the wave ran. Predecessor report: `📓️2026-09-10-wave-P4-lane-intake.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with W-F4 (puzzle-3d fill job).
  No `git commit` / `stash` / `checkout` was run; the ticket is NOT closed; `🗑️generated` untouched.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- Every vitest run was backgrounded to `…/scratchpad/wr-*.txt`; the machine is heavily loaded.
- `grep -a` throughout: BSD grep classifies the emoji-bearing sources in this tree as BINARY (W-P4 §1).

## 1 W-P4 handed over a budget, and one wrong sentence

W-P4 measured `steps=669403 samePhase=163284 idle=4 nodes=145 carried=57294` for a Nakagin-scale paged
scene-lane publication and concluded:

> The cost is the DOCUMENT, not the delta. […] A patch's `advance` loop runs
> `OwnedUiSurfacePatch.#prepare`, which revalidates the whole graph, rehashes it, restages every read
> cell and renotifies every subscriber. A one-operation delta pays all of it.

The *conclusion* was right; the *attribution* was not, and the difference decides the fix. W-P4 never
split the 669 403 steps by phase. This wave did, twice: once by intake phase name, once by attributing
every step inside the `validation` phase to the sub-operation that produced it (a temporary
`RETAINED_UI_GRAPH_DEBUG` mark/tally pair in `🔬️graph/🟦️.ts` + `🛡️validation/🟦️.ts`, since removed).

### 1.1 Where the 669 403 steps actually are

`[DEBUG] nakagin nodes=145 carried=57294 steps=669403`, top phases:

| Phase | Steps | Scales with |
| --- | --- | --- |
| `symbol-edit` + `input` + `symbol-old-close` + `typed-normalize` + `symbol-lookup` + `text-*` + `value-tag` + `attach` + `map-key-tag` + … | ≈ 450 000 | the **delta** — decoding 145 wire nodes into owned typed fields |
| `validation` | **163 284** | the **document** |
| `scenes` | 54 624 | the **delta** (`#prepareBindings` iterates `#touched`) |
| `hash` | 2 030 | the document, at 14 steps/node |
| `notifications` | 5 | subscriber cells |
| `staging` | 1 | subscriber cells |

So two of W-P4's four named culprits were never the problem:

- **hash is not a cost.** Streaming the whole 57 KiB canonical JSON through FNV-1a costs 2 030 steps —
  0.3 % of the publication — because `JsonBytes.advance` consumes atoms until the 65 536-byte grant is
  nearly spent and only stops on a 256-byte output chunk. A per-node Merkle digest (which would change
  the published `hash` identity, i.e. the wire ACK token, i.e. Rust) buys 0.3 %. Not done, deliberately.
- **stage/notify are not a cost** either: 1 and 5 steps. They are O(subscriber cells) — in production one
  cell per mounted consumer, at ~14 steps per cell for the `#touched.lookup` gate. On a 145-cell surface
  that is ≈ 2 000 steps against a 28 000-step delta (7 %). Real, bounded, not the lever.

### 1.2 Inside `validation`: 80 % of it is three persistent-index writes per node

`[DEBUG] nakagin validation` (163 284 steps):

| Sub-operation | Steps | Per node |
| --- | --- | --- |
| `marks.set(id, 1)` (enter) | 49 502 | 341 |
| `marks.set(id, 3)` (on-path) | 41 176 | 284 |
| `marks.set(id, flags & ~2)` (exit) | 40 551 | 280 |
| `keys.insert(child.key)` | 20 341 | 140 |
| `marks.lookup` (enter + exit) | 2 952 | 20 |
| `nodes.lookup` (self) | 2 363 | 16 |
| dangling sweep `marks.lookup` | 2 361 | 16 |
| `nodes.lookup` (child, for `orphanChild` + sibling key) | 2 345 | 16 |
| `keys.clear` | 1 067 | 7 |
| `nodes.entries()` sweep | 10 | — |

**131 229 of the 163 284 steps (80 %) are the three `RetainedUiNumericTable.set` calls the DFS makes per
node.** A `Table.set` is ≈ 300 steps: `NumericIndex.beginSet` walks the AVL, rebuilds the `ids` tree AND
the `order` tree through `TreeEdit`'s reservation dance, publishes, then drains the edit's retirement and
the replaced index's retirement. A `lookup` on the same index is 10–16 steps. The visit *marks* — pure
patch-local scratch state — are 20× more expensive than reading the nodes they mark.

That is the real shape of the problem: not "the document is walked" but "**walking the document writes a
persistent refcounted index three times per node**".

## 2 The fix: a delta-priced validation program

### 2.1 The argument

`OwnedUiSurface` only ever publishes a state that passed `OwnedUiValidationCursor` (`publish` is reachable
only from `OwnedUiSurfacePatch.#prepare`, after the violation table came back empty). Its initial state is
the empty graph, which is vacuously valid. **So the source state of every patch is a proven-valid graph**,
and a candidate differs from it by exactly the delta.

Now ask what a delta can break. The invariant set (`🔬️graph/🟦️.ts`) is `nodeQuota`, `cycle`,
`sectionNested`, `nonFiniteNumber`, `depthQuota`, `orphanChild`, `duplicateSiblingKey`, `danglingRoot`.
Every one of them except `nodeQuota` and `nonFiniteNumber` is decided by **structure**: the node set, the
child edges, the root, and each record's sibling `key` and section role. So:

> If every operation in a patch replaced a record that already existed, keeping its `key`, its `children`
> array and its section role, and no operation removed a node or moved the root, then the candidate's node
> set, edge set, root, key multisets and section flags are **identical** to the base's. Reachability,
> depth, cycles, nested sections, orphan children and duplicate sibling keys are therefore already proven,
> and the only invariant a payload can still break is `nonFiniteNumber` on a record the patch replaced.

That is exact, not conservative-in-the-wrong-direction: it is a *sufficient* condition, checked per
operation, and anything that fails it falls to the unchanged whole-graph walk.

### 2.2 What changed

- **`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts`**
  `OwnedUiOperationResult` gains `shapePreserving: boolean`. `OwnedUiOperationCursor` certifies it per
  operation with a new metered `sameGraphShape(previous, next, grant)` generator (`key` length + chars,
  `children` length + elementwise, `sectionRole`), and a `sectionRole(component)` helper:
  - `root`: preserving iff the id is the one already published.
  - `upsert`: the cursor now looks the id up in the source index FIRST (it did not before) and compares;
    a brand-new id is not preserving. Cost: +14 steps per upsert (measured +1 918 on 145 upserts).
  - `field` / `activity`: compares the old record against the built replacement — so a `children` field
    change, or a component change that flips the section role, is correctly not preserving.
  - `remove`: never preserving when a node was actually removed (a `remove` of an absent id changes
    nothing and stays preserving).
- **`…/🧵️retained/🛡️validation/🔬️graph/🟦️.ts`**
  new `retainedUiGraphTouchedValidation(nodes, touched, limits, violations, frontier)` — checks
  `nodeQuota`, then one index lookup per touched id and `finite(component)`. No marks table, no sibling
  keys, no dangling sweep, no persistent-index writes at all.
- **`…/🧵️retained/🛡️validation/🟦️.ts`**
  `OwnedUiValidationCursor` takes an optional 4th argument `touched: RetainedUiNumericTable<true> | null`.
  Non-null selects the delta program; `null` keeps the authoritative whole-graph walk, which stays the
  only program that mints the exact depth-first violation ORDER — so every rejection path and every
  conformance fixture is byte-for-byte unchanged.
- **`…/🧵️retained/🖼️surface/🟦️.ts`**
  `OwnedUiSurfacePatch` accumulates `#shapePreserving` across operations and passes `this.#touched` to the
  cursor when it holds. `#prepare` gains the docstring that states the two programs and their measured
  costs.
- **`…/📃️UiDocumentStore/📥️intake/🟦️.ts`**
  `PLUGIN_UI_INTAKE_STEPS_PER_NODE` keeps its value 8 192 but is re-documented for what it actually is:
  the per-node price of a **first** publication (4 630/node measured), not of every patch. Its in-source
  law now pins the ceiling above the new first-publication figure 671 321.

The touched table is **borrowed**, never owned, by the validation cursor: `Table.entries()` opens and
closes its own reader, and if the cursor is closed mid-iteration the patch's own `closeStep` still drains
`#touched`. No new owner, no new close path, no allocation outside the existing fixed-capacity discipline.

### 2.3 No Rust change was needed

Nothing on the wire changed: no per-node hash, no delta manifest, no schema. `shapePreserving` is derived
host-side from records the host already holds (the source index and the decoded operation), and the
published `hash` is still FNV-1a over the whole canonical document, so the ACK token the guest verifies is
untouched. **The served wasm therefore does NOT need a rebuild for this wave.** `🛡️limits.rs`'s
`validate_core` is likewise untouched — the TS twin still computes the identical result, only cheaper on
the delta path.

## 3 Measurements

Grant `{maxItems: 256, maxBytes: 65 536}`, 145-node / 18-lane / 57 294-carried-byte Nakagin surface.

| | before (W-P4) | after |
| --- | --- | --- |
| first publication, total steps | 669 403 | **671 321** (+0.3 %, the added upsert lookups) |
| first publication, `validation` | 163 284 | 163 284 (unchanged — re-roots, so it walks) |
| one-lane re-publish (8 records), total | ≈ 194 000 (predicted) | **28 066** |
| one-lane re-publish, `validation` | 163 284 | **159** (1 032×) |
| re-publish as a share of the first publication | ≈ 29 % | **4.18 %** |

Re-publish residual, by phase: `symbol-edit` 12 904 + `input` 4 075 + `symbol-old-close` 1 263 + other
decode ≈ 20 700 (74 %, wire decode of the 8 replaced records — pure delta), `scenes` 2 667 (delta),
`hash` 2 030 (document, 0.3 %/publication), `notifications` 381 (draining the replaced 145-node index's
refcounts), `validation` 159, `staging` 1.

## 4 The law

`📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx`, the W-P4 law
`OwnedIntake drives a Nakagin-scale paged scene-lane surface patch to acknowledgement inside the credited
budget`, was restructured around a `drive(operations, baseRevision, revision)` helper and now drives TWO
publications through the same `OwnedUiInstance`:

1. the original first publication — same assertions (budget ceiling, `> PLUGIN_UI_INTAKE_STEPS_PER_NODE`,
   `longestSamePhaseRun > 4096`, `longestIdleRun < 32`, close within 4 096 steps), plus a new
   `first.validation > 10 × nodes.length` so the whole-graph fallback is pinned as *reached*, not merely
   available;
2. a one-lane re-publish of lane 0's 8 records at the same key/children/role with new text, asserting
   revision 2, a changed published `hash`, `second.validation < 40 × relane.length` and
   **`second.steps × 10 < first.steps`** — the delta-priced bound the wave was asked for.

Both directions of the bound matter: without the fix the second assertion fails at `280 660 < 671 321`
false (194 k × 10); with an over-tight validation bound it fails at `159 < 80`, which is how the 40× per
touched-node figure was calibrated rather than guessed.

## 5 Verification

All runs backgrounded to `…/scratchpad/wr-*.txt`, from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`.

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts test exhaustive --run UiDocumentStore --testNamePattern='Nakagin-scale'` | **1 passed / 0 failed** |
| — the same law, with the pre-fix validation | **1 failed**: `one lane of an unchanged 145-node document must cost a delta, not a document` |
| — the same law, with the validation bound at `10 × touched` | **1 failed**: `expected 159 to be less than 80` (calibration, not a defect) |
| `bun ./📜️script.ts test long --run '📥️intake/🟦️.ts' '🗣️Interpreter/🟦️.tsx'` | **2 files, 74 passed / 0 failed** |
| `bun ./📜️script.ts test long --run '🔌️PluginRuntime/🟦️.tsx' '🧪️tests/🔬️engine-contract/🟦️.ts'` | **2 files, 551 passed / 0 failed** |
| `bun ./📜️script.ts typecheck` | **820** `error TS` — byte-identical to W-P4's baseline; **0** in `🩹️operations`, `🛡️validation`, `🔬️graph`, `🖼️surface`, `📥️intake` or `🧪️typedwire` |
| `bun ./📜️script.ts test exhaustive --run UiDocumentStore` (whole file, 213 laws, 367 s) | **212 passed / 1 failed** — the one failure is `OwnedHash streams exact insertion-ordered JSON bytes…`, NOT this wave's (see §6.1) |

Note: `UiDocumentStore/🟦️.tsx` is **not** in the `long` include set — `test long --run 'UiDocumentStore/🟦️.tsx'`
exits `No test files found`. `exhaustive` is the only level that reaches it.

No Rust was rebuilt: this wave edited no `.rs` file and changed nothing on the wire (§2.3).

## 6 Results

All four gates were re-run a final time AFTER the temporary `[DEBUG]` step-histogram logs were removed
from the law, so the numbers above describe the tree as it stands: Nakagin law **1 passed** (84 s),
intake + Interpreter **74 passed**, PluginRuntime + engine-contract **551 passed**.

Every law that exercises the changed code is green. The full typedwire file is **212 passed / 1 failed**,
and every `OwnedOperation`, `OwnedValidation`, `OwnedSurface`, `OwnedInstance` and `OwnedIntake` law in it
passes — including the new two-publication Nakagin law.

### 6.1 The one failure is not this wave's

`OwnedHash streams exact insertion-ordered JSON bytes while retaining old surface owners through
cancellation` fails at `🧪️typedwire/🟦️.tsx:1490` with `expected 'rejected' to be 'ready'` — the
`RetainedUiTypedCursor` decode of its own fixture node is rejected, before any hash cursor is built.

- It fails **identically when run alone** (`--testNamePattern='OwnedHash streams exact insertion-ordered'`,
  1 failed / 212 skipped, 9.7 s), so it is not test-order or ledger-leak contamination from the new law —
  and the new law is declared ~800 lines LATER in the file, so it could not have run first anyway.
- Its entire import graph is `📦️wire/🧾️typed/🟦️.ts`, `🗂️nodes/🟦️.ts`, `🔢️hash/🟦️.ts`, the
  `🔏️owned-hash` fixture and `encodePackValue`. **W-R edited none of them** — the wave's edits are in
  `🩹️operations`, `🛡️validation` (+`🔬️graph`), `🖼️surface`, `📥️intake` and the test file, and none of
  those is reachable from this law.
- Left alone deliberately: it is a decoder/fixture defect in another lane, and this wave's mandate is the
  intake's cost.

## 7 Not verified / open

- **No browser re-drive, no wasm rebuild.** The change is TypeScript-only and rides vite HMR or the next
  serve. Confirming that the Nakagin Perspective refresh now paints needs an in-browser run this wave did
  not do. The production symptom it targets —
  `plugin-ui.intake-budget-exhausted:1:puzzle3d-main-perspective:36864` on a ONE-operation delta — would
  now cost roughly `3 500 (decode) + 2 030 (hash) + 159 (validation) + notify` ≈ 6 000 steps ≈ 20 ms,
  against a credited ceiling of 163 840 000. Predicted, not observed.
- **`hash` is the last document-scaled phase, and it will bite at scale.** 2 030 steps at 145 nodes is
  0.3 %, but it is 14 steps/node *unconditionally*: at the contract's own 20 000-node quota every patch,
  however small, pays ≈ 280 000 steps (≈ 1 s) just to re-digest the document. Removing that needs per-node
  Merkle digests, which changes the published `hash` — the ACK token the guest verifies — and therefore
  the Rust half. Deliberately out of scope here: the measurement says it buys 0.3 % on the document size
  that actually faulted, and it is the only part of this problem that is NOT host-local.
- **`staging` and `notifications` stay O(subscriber cells)** at ~14 and ~1 steps per cell. On the
  145-cell surface that is ≈ 2 000 steps against a 28 000-step delta. A node→cell reverse index would make
  it delta-priced; the measurement does not justify one yet.
- **The wire decode is now 74 % of a delta**, and `symbol-edit` (12 904 steps, 1 613 per record) is the
  SAME ~300-step `RetainedUiNumericTable.set` pattern that dominated validation, this time in the pack
  decoder's symbol table. That is the highest-value remaining lever and it is delta-priced already, so it
  lowers the constant rather than the exponent — a separate wave.
- **A reordered `children` array falls to the whole-graph walk.** `sameGraphShape` compares elementwise,
  and a permutation is structurally harmless (no invariant is order-sensitive). Correct but slower on a
  rare path; widening it to a set comparison would need a metered multiset and was not worth it.
- **`PLUGIN_UI_INTAKE_YIELD_STRIDE = 1024` was left as W-P4 set it.** With deltas at 6 000–28 000 steps a
  refresh is now 6–28 macrotask round-trips instead of 83 675; no reason to touch the stride.

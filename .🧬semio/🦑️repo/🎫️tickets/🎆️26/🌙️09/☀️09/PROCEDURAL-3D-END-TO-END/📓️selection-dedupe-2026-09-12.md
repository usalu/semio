# Selection dedupe — `selectedIds: ["extrude@solid","extrude@solid"]` and the clipboard-write warning (2026-09-12)

Follows `📓️interaction-coverage-2026-09-12.md` §1 and `📓️audit-hover-selection-2026-09-12.md` §1.3–§1.4, §3.
Every result below was **run**, in the foreground. Raw evidence: `🗑️generated/selection-dedupe/`,
`🗑️generated/probe-selection-dedupe-1-2026-09-12T02-50-47/`,
`🗑️generated/probe-selection-dedupe-2-2026-09-12T02-52-05/`.
No wasm build was started and no dev server was restarted in this lane.

---

## 0. Headline

| Item | Verdict | Proof |
|---|---|---|
| Where the duplicate enters | **Neither the host click path nor the per-domain reducer** — it is the leftover **projection** in `🔌️plugin/🦀️.rs`, which flattened every domain of ONE `InteractionState` while the framework deliberately mirrors the picked ids into a second `vortex` domain | §1 |
| Fix | One shared set-valued projection (`leftover_selected_ids_of`) used by BOTH flatten sites; host `world3dSelectionActionArgs` made set-valued too | §2 |
| Rust law | `leftover_interaction_view_selected_ids_are_a_set_for_every_merge`, fixture-driven, **fails-before proved** | §3.1 |
| TS twin | `interactionSelectionSetOracle` — 17 checks over the same JSON, `node:assert`, no Rust | §3.2 |
| Host fixture + laws | `🖱️pointer-gestures.json` +2 rows; oracle 28 → **36 checks**; mounted suite 8 → **10 tests** | §3.3 |
| `[DEBUG] leftover clipboard-write missing` | **check removed** (no predicate existed), vitest row added | §4 |
| Runtime | clipboard warning **gone live** (9 → 0 on the same probe); the duplicate **still present**, because the served guest `.wasm` predates the Rust fix — **a restage is required** | §5 |
| Side finding (NOT fixed, separate lane) | a modifier-click on a domain-bound world scene sends `merge:"add"`, which `parse_merge_mode` rejects outright | §6 |

---

## 1. Where the duplicate enters

The live line (`🗑️generated/probe-interact-2-2026-09-12T02-31-26/console.jsonl`, job 999):

```
[DEBUG] leftover InteractionView {"selectedIds":["extrude@solid","extrude@solid"],
  "publishedIds":["extrude@solid","extrude@solid"],"locked":{"extrude@solid":false},
  "gumball":true,"hoverTarget":{"domain":"graph","channel":"pointer","id":"extrude@solid"}}
```

The `locked` half — an **object**, so keyed by construction — carried the id **once**. That asymmetry is
the whole tell: the two halves were built by two different folds over the same state.

### 1.1 The host click path is innocent

`🌐️World3dHost/🟦️.tsx:5350` (`handleInstancePointerDown`) dispatches **one** id:

```ts
dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, "object", [record?.interactionId ?? id], merge));
```

A pick sends one target per DOM event, not one per rendered instance. The marquee release already ran
its ids through `interactionTargetsForInstances` (`:4568`), which dedups. So the host never emitted two
`extrude@solid` targets for this gesture — confirmed by the mounted jsdom suite, which sees exactly one
dispatch with one target for a click on either rendered instance.

### 1.2 The per-domain reducer is innocent

`protocol::next_selection` (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:2421`) already dedups:

- `MergeMode::Replace` → `dedup_preserving_order(expanded)`
- `MergeMode::Additive` → `if !ids.contains(&id) { ids.push(id) }`

So `state.selection["graph"].ids` was `["extrude@solid"]`, singular.

### 1.3 The projection is the owner

`dispatch_interaction_action` (`🔌️plugin/🦀️.rs:~22472`) calls
`Self::overlay_leftover_ids_into_vortex(&mut state)` **before** building the leftover view. That helper
is a deliberate compensation (`leftoverInteractionStateV1` twin, so Inspection can read
`selection("vortex")` after a topology prune) which **republishes every domain's ids under a second
`vortex` key**:

```rust
let leftover_ids: Vec<String> = state.selection.values().flat_map(|s| s.ids.iter().cloned()).collect();
if vortex_empty && !leftover_ids.is_empty() {
    state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity, ids: leftover_ids, anchor_id: None });
}
```

`leftover_interaction_view_from` then flattened **every** domain of that state without dedup, so a
single pick was counted once per mirroring domain — `graph` + `vortex` = 2.

**The decisive evidence that this is a defect and not a design choice**: the repo already had the
correct fold two thousand lines away. `InteractionView::leftover_selected_ids` (`🔌️plugin/🦀️.rs:9307`
before this change) is *the same walk* and **did** dedup (`if !ids.iter().any(|existing| existing == id)`).
Two copies of one projection, one deduplicating and one not.

---

## 2. The fix — one law on each side

### 2.1 Framework (owning layer) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

Added ONE shared projection next to `empty_domain_selection`/`empty_domain_hover`:

```rust
pub(crate) fn leftover_selected_ids_of(state: &protocol::InteractionState) -> Vec<String>
```

with a docstring naming the vortex mirror and the measured defect. Then:

- `InteractionView::leftover_selected_ids` → `leftover_selected_ids_of(self.state)` (the dedup body it
  had is now the shared one, not a second copy).
- `leftover_interaction_view_from` → `let selected_ids = leftover_selected_ids_of(state);` and
  `locked` is now **derived from that deduplicated list**, so the two halves can never disagree again:
  ```rust
  let locked: Vec<(String, DslValue)> = selected_ids.iter().map(|id| (id.clone(), DslValue::Bool(false))).collect();
  ```

`overlay_leftover_ids_into_vortex` is deliberately left in place — it is a real consumer contract
(puzzle3d Inspection reads `selection("vortex")`, and `🔬️engine-contract/🟦️.ts:8193,8329,8342` pin the
host-side twin). The mirror is legitimate; flattening a mirror into a *list* was not.

### 2.2 Host — `🌐️World3dHost/🟦️.tsx`

`world3dSelectionActionArgs` is now the host's set boundary, so **every** call site (instance pick,
marker picks at `:4940`/`:4980`, empty click, marquee release) is duplicate-free by construction rather
than by each caller remembering to pre-collapse:

```ts
const targets = [...new Set(ids)].map((id) => ({ granularity, id }));
```

`interactionTargetsForInstances` stays — it maps *render* ids onto *topology* ids, which the set alone
cannot do. The wire-shape return line is untouched (the oracle pins it verbatim).

---

## 3. Tests

### 3.1 Rust law (fixture-driven) — **fails-before proved**

New language-agnostic fixture
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🕹️selection-set.json` — 5 cases covering
`replace` and `additive`, a repeated target inside one batch, a re-add of an already-selected id, and a
plain single-target pick (the regression itself), plus `wiring` source anchors.

New law in `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
`leftover_interaction_view_selected_ids_are_a_set_for_every_merge`, driving the **real reserved
`interactionSelect` job** (`reserved_action` + the new `interaction_targets_args` batch helper), and
asserting per case: the exact `selectedIds`, that they are distinct, that `locked` has exactly one key
per published id, and that `gumball.anchorId` is the first of them.

**Fails-before** (projection temporarily reverted to the naive flatten):

```
test …::leftover_interaction_view_selected_ids_are_a_set_for_every_merge ... FAILED
assertion `left == right` failed: replace-repeated-batch-target: …
  left: ["item-1", "item-1"]
 right: ["item-1"]
```

**Passes-after**:

```
test …::leftover_interaction_view_selected_ids_are_a_set_for_every_merge ... ok
test result: ok. 1 passed; 0 failed; 668 filtered out
```

### 3.2 TS twin

`interactionSelectionSetOracle(repoRoot)` in
`…/🎯️targets/⚛️react/📜️script.ts` reads the **same JSON** with `node:assert`, no Rust and no wasm. It
re-implements `next_selection`'s replace/additive plus the vortex-mirrored flatten from the documented
law, asserts each case's `selectedIds`, asserts the fixture itself is non-vacuous (it must cover both
merges, must repeat a target in one batch, and must re-add an already-selected id), and pins the Rust
source anchors so the two flatten sites cannot drift back apart. Wired into `world3d-interaction-check`.

### 3.3 Host fixture + laws

`🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` gained two rows:

- `second-instance-of-one-topology-id-pick` — the scene already renders ONE topology id as two
  instances; a single unmodified click on the **second** one asserts exactly one dispatch with exactly
  one target.
- `repeated-ids-collapse-in-the-args-builder` (new kind `selection-args`) — the builder handed
  `["extrude@solid","extrude@solid"]` must emit ONE target.

plus `wiring.selectionTargetsAreASet`, the verbatim source line the oracle pins.

`expectSelection` in the mounted suite now additionally asserts every dispatched target list is
duplicate-free, so all six gesture laws carry the set law, not just the new rows.

### 3.4 Runs (quoted verbatim)

```
world3d-pointer-gesture-oracle: checks=36 clean          (was 28)
interaction-selection-set-oracle: checks=17 clean        (new)
 Test Files  1 passed (1)
      Tests  10 passed (10)                              (was 8)
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib -- interaction_select selection --test-threads=1
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 347 filtered out
```

Baseline held exactly (25 passed).

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib -- interaction selection leftover --test-threads=1
test result: FAILED. 49 passed; 10 failed; 610 filtered out
```

**The 10 failures are pre-existing and NOT mine** — measured, not assumed. With the projection
temporarily reverted to the naive flatten the same command reports `48 passed; 11 failed`: the same 10
plus my new law. So this change fixes exactly one test and breaks none. Their causes are unrelated to
selection flattening:

- `interactive-job.missing-factory` — `typed command 'setLabel' has no exact controller/owner/factory/tool/schema proof` (6 tests)
- `interactive-job.output-envelope` — `framework route 'copy' did not preserve its admitted envelope`
- `assertion failed: presence.is_empty()`
- one `local_interaction_registered_query_channel_continuation_ack_and_close`

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts "…/🔌️PluginRuntime/🟦️.tsx"
 Test Files  1 failed (1)
      Tests  2 failed | 102 passed (104)
```

Same treatment: with the clipboard warning temporarily restored the run is `3 failed | 101 passed` —
my new row fails (fails-before) and **the other two still fail**, so they are pre-existing peer-lane
breakage (`readAppDocumentPack` now returns an extra `ops` field; the window projection now binds an
extra `window` body — both are production behaviour the stale tests do not expect).

Typecheck: the `⚛️react` `typecheck` target is broadly red already (**867 errors**, mostly the repo-root
`📜️script.ts` and `🧪️docklayoutstore`). Every error in the files I touched is pre-existing
(`brushPreviewJson`, `pickEnabled`, `req` in `FaultScope`, the `window.addEventListener` overload at
`:4548`); my edits add none, and neither the new test file nor the fixtures appear.

---

## 4. `[DEBUG] leftover clipboard-write missing` — removed

`🔌️PluginRuntime/🟦️.tsx` had:

```ts
function leftoverClipboardWriteEffects(leftover: readonly WireVariant[]): WireVariant[] {
  const writes = leftover.filter((effect) => effect.tag === "clipboard-write");
  if (writes.length === 0 && leftover.some((effect) => effect.tag === "send-message")) {
    console.warn(`[DEBUG] leftover clipboard-write missing tags=${…}`);
  }
  return writes;
}
```

`promoteShellSendMessages` called it and **discarded the return value** — the function existed only to
print the warning. Its predicate is wrong on its face: a reserved job publishes its result frame on
`send-message` whatever it did, so `send-message` present ⇒ clipboard-write expected is simply false,
and the warning fired on **every** reserved-job completion (9 occurrences in the 68 s evidence probe,
on every `interactionSelect`/`interactionHover`).

**Decision: removed, not silenced.** The brief's other option — "fire only when a clipboard-write was
actually expected (the job declared it)" — has nothing to key on: nothing in the leftover effect list
carries a declaration of what the job intended, so there is no predicate to test. Adding one would mean
inventing a new wire field to serve a `[DEBUG]` log. `leftoverClipboardWriteEffects` is gone;
`promoteShellSendMessages` keeps its `Invocation`-frame trace and a docstring recording why the check
was dropped.

**Vitest row** (`🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `promoteShellSendMessages` added to
`pluginRuntimeTestDependenciesV1`):

```
✓ isolated job admission batch > never warns about a missing clipboard-write on a job completion
  that only carries send-message effects
```

It spies `console.warn`, feeds two `send-message` effects and no `clipboard-write`, and asserts no
warning mentions `clipboard` and the leftover list passes through unchanged. Fails-before confirmed
(it FAILED in the run where the warning was temporarily restored).

---

## 5. Runtime (6018)

Probe harness: `bun 🔍️browser-probe.ts --mode=interact --steps=hover,select,orbit --settle=150`.
The first re-run picked empty background (the fixed-position `(180,180)` click fires before the preview
finishes evaluating its channels), so `🔍️browser-probe.ts` gained a `--predelay=<s>` flag — extra quiet
seconds after the first mesh lands, before the first gesture. This is a ticket-owned input script; the
flag defaults to `0`, so no existing invocation changes.

Run 2 (`--predelay=25`, example "Hexagonal Mushroom Column", geometry fully evaluated, `meshes=3`):

```
[53107ms] leftover InteractionView {"selectedIds":[],"publishedIds":[],"locked":{},"gumball":false,
  "hoverTarget":{"domain":"graph","channel":"pointer","id":"extrude@solid"}}
[57899ms] leftover InteractionView {"selectedIds":["extrude@solid","extrude@solid"],
  "publishedIds":["extrude@solid","extrude@solid"],"locked":{"extrude@solid":false},"gumball":true,…}
```

**The duplicate is still there, and it must be.** Everything in §2.1 compiles into the guest component:

```
2026-09-12 04:23  …/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm
```

The served `.wasm` was built at **04:23**; the Rust edit landed at **~04:40**. **A restage of the
procedural plugin is required** before the runtime shows a single `extrude@solid` — I did not start one
(forbidden in this lane), so this report does **not** claim the duplicate is fixed at runtime. What is
proved at runtime is that the defect reproduces exactly as described, and what is proved natively is
that the fix removes it (§3.1, fails-before + passes-after on the real reserved job).

**The clipboard warning IS fixed live** — it is host TypeScript, served from source and live on reload:

| Probe | `[DEBUG] leftover clipboard-write missing` occurrences |
|---|---|
| `probe-interact-2-2026-09-12T02-31-26` (before) | **9** |
| `probe-selection-dedupe-1-2026-09-12T02-50-47` (after) | **0** |
| `probe-selection-dedupe-2-2026-09-12T02-52-05` (after) | **0** |

The dev server answered in 9 ms throughout; it is not wedged.

---

## 6. Side finding — the merge vocabularies do not match (NOT fixed here)

Found while reading the same path, flagged rather than fixed because it is a different law and a
different owner.

The host's `instanceMergeArg` (`🌐️World3dHost/🟦️.tsx:3533`) emits `add` / `remove` / `toggle` /
`replace`. The framework's `parse_merge_mode` (`🔌️plugin/🦀️.rs:583-593`) accepts **only**
`replace` / `additive` / `subtractive` / `invertive` / `range` and otherwise returns
`plugin_sdk_fault("interactionSelect: unknown merge '…'")`.

So on a domain-bound world scene (generation3d's preview), a **shift-click / ctrl-click faults** —
only the unmodified `replace` path works, which is why the live probes (plain clicks) never hit it.
The `add`/`remove`/`toggle` vocabulary is real, but it belongs to the *other*, non-domain path
(`merge_world_selection_ids`, `🔌️plugin/🦀️.rs:35523`, which serves `worldSelect`/`worldPick`).
`🖱️pointer-gestures.json`'s `instance-pick-additive` row pins `merge: "add"`, so the host side is
internally consistent and the *contract* between the two is what is missing. Whoever owns it must pick
one vocabulary; there is no third option and no adapter (greenfield, no compat layers).

---

## 7. Files

Changed:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🕹️selection-set.json` *(new)*
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🔍️browser-probe.ts` *(`--predelay`)*

Evidence (under `🗑️generated/`, left in place):

- `selection-dedupe/probe-selection-dedupe-1-2026-09-12T02-50-47-interaction-view.txt`
- `selection-dedupe/probe-selection-dedupe-2-2026-09-12T02-52-05-interaction-view.txt`
- `probe-selection-dedupe-1-2026-09-12T02-50-47/`, `probe-selection-dedupe-2-2026-09-12T02-52-05/`

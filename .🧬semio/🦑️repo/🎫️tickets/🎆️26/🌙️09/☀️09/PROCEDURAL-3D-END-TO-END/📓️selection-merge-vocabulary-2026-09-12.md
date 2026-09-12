# One selection merge vocabulary — deleting `add`/`remove`/`toggle` (2026-09-12)

Follows `📓️selection-dedupe-2026-09-12.md` §6 (the side finding that lane flagged and did not fix).
Every result below was **run**, in the foreground. Raw evidence:
`🗑️generated/merge-vocabulary/`, `🗑️generated/probe-merge-vocabulary-base-2026-09-12T03-24-13/`,
`🗑️generated/probe-sanity-2026-09-12T03-41-51/`.
No wasm build was started and no dev server was restarted in this lane.

---

## 0. Headline

| Item | Verdict | Proof |
|---|---|---|
| The defect | The host translated the resolved `MergeMode` into `add`/`remove`/`toggle`; `parse_merge_mode` accepts only the five schema words and faults otherwise | §1 |
| The ONE vocabulary | `replace` · `additive` · `subtractive` · `invertive` · `range` — **the `MergeMode` enum of `🕹️interaction/🧬️schema/🔣️.json`**, now the only spelling in the tree | §2 |
| New language-neutral fixture | `🧰️framework/🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json` — vocabulary, deleted words, modifier policy, ordered topology, 11 per-mode cases, `range` decision, source anchors | §2.1 |
| `add`/`remove`/`toggle` | **deleted everywhere, no adapter** — and unrepresentable: every merge site now takes a decoded `MergeMode`, not a string | §3 |
| `range` decision | **ACCEPTED, never rejected** — inclusive `DomainTopology.ordered` span anchor→target; degrades to a single-target pick when the domain orders neither endpoint; a world viewport never emits it | §4 |
| Rust laws | 3 new in `semio-framework` (schema/codec, reducer over every mode, modifier policy) + 1 in the plugin contract (domain path accepts the five, rejects the three) + 2 rewritten unit laws | §5.1 |
| TS twin | `selectionMergeVocabularyOracle` — **89 checks**, validates the vocabulary with **ajv** against the owned schema export | §5.2 |
| Host laws | `🖱️pointer-gestures.json` +3 modifier rows; gesture oracle 36 → **63** checks; mounted suite 10 → **14** tests | §5.3 |
| Fails-before | proved 4 ways (gesture oracle, vocabulary oracle, mounted vitest, both Rust no-adapter laws) | §6 |
| Runtime (6018) | **0 `unknown merge` faults**, every dispatched `interactionSelect` settles — but the A/B was **indistinguishable**, so this is NOT a runtime proof of the fix; the environment lost geometry evaluation mid-session and automation cannot reach the pick path | §7 |

---

## 1. The defect

`🌐️World3dHost/🟦️.tsx:3533` (before this wave):

```ts
/** @emoji 🖱️ additive→add, subtractive→remove, invertive→toggle, replace/range→replace … */
function instanceMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  if (mode === "invertive") return "toggle";
  return "replace";
}
```

`🔌️plugin/🦀️.rs`'s `parse_merge_mode` accepted only `replace`/`additive`/`subtractive`/`invertive`/`range`
and returned `plugin_sdk_fault("interactionSelect: unknown merge '…'")` for anything else. So on a
**domain-bound** world scene — generation3d's preview — shift-click / ctrl-click / shift+ctrl-click
faulted in the guest and only the unmodified `replace` pick worked.

The `add`/`remove`/`toggle` words were the *other*, non-domain path's private spelling of the same three
set operations (`merge_world_selection_ids` · `merge_string_ids` · `merge_u32_ids`, serving
`worldPick`/`worldVortexSelect`). Two vocabularies for one algebra, with no contract between them.

**There was never a third option.** The framework's set is strictly richer (`range` has no `add`-family
equivalent) and it is the one the schema declares, so the other vocabulary is what goes.

---

## 2. The ONE vocabulary, schema-first

The words are **not** re-typed per implementation. They are `$defs.MergeMode` of
`🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔣️.json` (and `MergeMode` of the same module's
`🔗️.graphql`), and both twins re-validate against that file rather than against a second list.

### 2.1 `🧰️framework/🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json` *(new)*

Owned by the framework interaction contract, next to the schema that declares the enum. It carries:

- `schema` — the file, the export id, and the GraphQL leaf both twins check against;
- `vocabulary` — the five words, asserted to be *exactly* the schema enum, in the schema's own order;
- `deletedWords` — `add`/`remove`/`toggle`, each asserted to fail that same validation;
- `unknownMergeFault` — the fault an out-of-vocabulary word raises;
- `modifierPolicy` — 5 chord rows × `pick` / `componentPick`, plus `neverEmitted: ["range"]`;
- `topology` — the ordered node list `range` spans (`item-a`…`item-d`);
- `range` — the written decision (§4);
- `cases` — 11 per-mode laws, each `seed` + `anchor` + `merge` + `targets` → `selectedIds` + `anchorId`,
  covering every word plus the unordered-domain degradation;
- `wiring` — source anchors for the codec, the domain path, the world path and the host builder.

### 2.2 One codec, next to the type

`MergeMode` itself lives in `📡️replication/📡️wire/🦀️.rs` (moved there for crate layering; the schema
stayed in `🕹️interaction`). It gained the ONE pair every producer and consumer now routes through:

```rust
pub fn wire_label(self) -> &'static str
pub fn from_wire_label(raw: &str) -> Option<Self>
```

`ToValue`/`FromValue` delegate to it, and the three hand-rolled copies of the same match are gone:

| was | now |
|---|---|
| `parse_merge_mode`'s five match arms (`🔌️plugin/🦀️.rs`) | `MergeMode::from_wire_label(raw).ok_or_else(…)` — the fault text is unchanged |
| `merge_mode_wire_str` (`♾️infinite/🌍️world/🦀️.rs`) | **deleted**; call sites use `MergeMode::…​.wire_label()` |
| `MergeMode`'s `ToValue`/`FromValue` inline word lists | delegate to the pair |

`from_wire_label` returning `Option` is deliberate: the domain path turns `None` into the typed fault,
while the wgpu world's *optimistic local preview* simply declines to guess (`return;`) — an unknown
merge is the framework's fault to raise, never a preview's to invent a meaning for.

---

## 3. `add`/`remove`/`toggle` deleted — and made unrepresentable

No adapter, no compatibility arm. Every merge site now takes a **decoded `MergeMode`**, so the deleted
words cannot be passed at all:

| site | change |
|---|---|
| `🌐️World3dHost/🟦️.tsx` `instanceMergeArg` | **deleted**; the three call sites (instance pick, vortex marker pick, background clear) pass `resolveWorldMergeMode(...)` straight through |
| `🌐️World3dHost/🟦️.tsx` `componentMergeArg` | **deleted**, replaced by `componentPickMergeMode(mode: MergeMode): MergeMode` — the component pick's ONE real deviation (a bare click toggles) expressed as `replace → invertive`, not as a word table |
| `world3dSelectionActionArgs(…, merge: string)` | → `merge: MergeMode`; the wire-shape return line is untouched (the oracle pins it verbatim) |
| `mergeIdSet(mode: ReturnType<typeof marqueeModeFromModifiers>, …)` | → `mode: MergeMode` |
| `merge_world_selection_ids(…, merge: &str)` | → `merge: protocol::MergeMode`; `"add"`/`"toggle"`/`"remove"` arms gone, all five variants matched exhaustively |
| `merge_string_ids` / `merge_u32_ids` (`♾️infinite/🌍️world/🦀️.rs`) | → `merge: MergeMode`, exhaustive; gained the `Subtractive` arm they silently lacked (it fell through to "replace") |
| `worldVortexSelect` emit (`🌍️world/🦀️.rs:10108`) and the flat-action `VortexSelect` encoder (`:3966`) | `"add"`/`"toggle"` → `MergeMode::Additive.wire_label()` / `MergeMode::Invertive.wire_label()` |
| `.storybook/stories/{puzzle,block}/{3d,5d}/*.stories.tsx` | the four story-local mirrors now speak the same five words |

A repo sweep for any surviving `merge` producer or consumer of the three words returns **nothing** —
every remaining occurrence is a docstring or a law asserting they are gone.

---

## 4. `range` — the decision

**Accepted, not rejected. Defined for every domain; degrading, never faulting.**

- **Law.** The anchor is `current.anchorId`, falling back to the last id of the current selection,
  falling back to the batch's last target. The result is the **inclusive slice of
  `DomainTopology.ordered`** between the anchor's index and the last target's index, in topology order
  whichever way the pair runs. The published `anchorId` stays the anchor, so a second shift-click from
  the same origin re-spans rather than walking.
- **Unordered domains.** When the topology orders *neither* endpoint (empty `ordered`, or a target the
  app's `interaction_topology` does not report — which is exactly a world3d scene), `range` selects the
  batch's last target alone and re-anchors there.
- **Why degrade rather than fault.** `range` is a legal word on every domain; a domain that publishes no
  order simply has no span to take. Faulting would make the *domain's* topology shape a *caller's*
  error, and the caller (a Tree row, a shell keymap) cannot know it. The behaviour was already
  implemented in `next_selection`/`nextSelection` and was simply unwritten — this wave documents and
  pins it, in Rust and in the TS twin.
- **It is unreachable by gesture on a world viewport.** `marqueeModeFromModifiers` / `resolveWorldMergeMode`
  never produce `range` (shift is *additive* there), and the fixture asserts that for every chord
  (`modifierPolicy.neverEmitted`). The degradation is a safety net for a programmatic dispatch, not a
  gesture anyone can perform. `range` reaches the domain path from **ordered** surfaces only — Tree rows
  via `interactionMergeFromModifiers`, where shift → `range` is the platform convention.

---

## 5. Tests

### 5.1 Rust

`🧰️framework/🔨️modules/🕹️interaction/🧪️tests/🔬️unit/🦀️.rs` *(3 new laws, fixture-driven)*

- `merge_mode_wire_labels_are_exactly_the_owned_schema_enum` — the fixture vocabulary **is** the schema
  enum, in the schema's order; every `MergeMode` variant encodes to its schema word; the codec
  round-trips; each deleted word decodes to `None` and is absent from the schema.
- `next_selection_obeys_the_merge_vocabulary_fixture_for_every_mode` — drives the **real reducer** over
  the fixture's declared ordered topology for all 11 cases, asserting `ids`, `anchorId` and set-ness,
  and refusing a vocabulary word that carries no case.
- `the_world_modifier_policy_never_resolves_outside_the_vocabulary` — the chord→mode map and the
  component-pick deviation, plus `range` never resolved.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`

- `interaction_select_speaks_exactly_the_schema_merge_vocabulary` — **the domain path**: for each of the
  five words a reserved `interactionSelect` job settles with a leftover `InteractionView`; for each
  deleted word the job **faults** with `unknown merge '<word>'` (asserted through
  `settle_framework_reserved_admission`, since the reserved job is admitted before its args decode).

Rewritten unit laws: `merge_world_selection_ids_speaks_the_one_schema_merge_vocabulary`
(`🔬️world3d-host-unit`) and `merge_u32_ids_speaks_the_one_schema_merge_vocabulary`
(`🌍️world/🧪️tests/🔬️unit`) — both now typed, both asserting the three deleted words decode to nothing.

### 5.2 TS twin — `selectionMergeVocabularyOracle`

In the renderer react target's `📜️script.ts`, wired into `world3d-interaction-check`. **89 checks**,
no Rust, no wasm:

- **ajv** (already a dev dependency of that bundle) compiles the owned `MergeMode` export of
  `🕹️interaction/🧬️schema/🔣️.json` and validates every vocabulary word — and proves every deleted word
  **fails** it. This is the required third-party twin: the vocabulary is confirmed against the schema by
  a validator we did not write.
- the fixture vocabulary is deep-equal to the schema enum, and each word has a `<WORD>` leaf in the
  GraphQL schema, so the two schema spellings cannot drift.
- an independent re-implementation of the modifier map and of every merge (including `range` and its
  degradation), replayed over all 11 cases.
- source anchors: the codec pair, the domain-path decoder line, the world-path signature, the host
  builder's typed `merge`, the deleted builders' absence, and the absence of every deleted word.

### 5.3 Host fixture + mounted laws

`🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` — `instance-pick-additive` now expects `additive`
(was `add`), plus three new rows: `instance-pick-subtractive` (ctrl), `instance-pick-subtractive-on-command`
(cmd — the platform twin, same mode), `instance-pick-invertive` (shift+ctrl). `wiring` gained
`mergeVocabularyFixture`, `mergeIsNotTranslated` (the typed builder signature) and `deletedMergeBuilders`.

`🧪️tests/🖱️world3d-interaction/🟦️.tsx` — the pick loop now plays all six rows; `expectSelection` asserts
every dispatched merge is in the vocabulary; and one new law drives **every chord of the fixture's
`modifierPolicy`** through the mounted host, asserting the wire word and that no chord yields `range`.

### 5.4 Runs (quoted verbatim)

```
$ bun 📜️script.ts world3d-interaction-check
world3d-pointer-gesture-oracle: checks=63 clean          (was 36)
interaction-selection-set-oracle: checks=17 clean        (unchanged)
selection-merge-vocabulary-oracle: checks=89 clean       (new)
 Test Files  1 passed (1)
      Tests  14 passed (14)                              (was 10)
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib -- interaction_select selection --test-threads=1
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 349 filtered out
```

Baseline held exactly (25 passed), re-run after the final edits.

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework --lib -- interaction merge --test-threads=1
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 222 filtered out

$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-replication --lib -- interaction merge selection --test-threads=1
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 225 filtered out

$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-infinite --lib -- merge_u32_ids --test-threads=1
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 329 filtered out
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib -- interaction selection leftover --test-threads=1
test result: FAILED. 51 passed; 10 failed; 610 filtered out
```

**The 10 failures are pre-existing and NOT mine — measured, not assumed.** They are byte-for-byte the
same 10 `📓️selection-dedupe-2026-09-12.md` §3.4 recorded before this wave, on the same filter; the
passing count rose 49 → **51**, exactly my two new laws. Their causes, read off this run:

- 7 × `interactive-job.missing-factory` — `typed command 'setLabel' has no exact controller/owner/factory/tool/schema proof`
- 1 × `interactive-job.output-envelope` — `framework route 'copy' did not preserve its admitted envelope`
- 1 × `assertion failed: presence.is_empty()`
- 1 × `local_interaction_registered_query_channel_continuation_ack_and_close`

None mentions a merge. `cargo check --all-targets` is clean on all four touched crates
(`semio-framework`, `semio-framework-replication`, `semio-framework-os-infinite`, `semio-framework-plugin`).

---

## 6. Fails-before — proved four ways

Each one was produced by *temporarily* reintroducing the deleted vocabulary and then restoring.

**A — the gesture oracle** (fixture row put back to `"merge": "add"`):

```
world3dPointerGestureOracle: FAILED — instance-pick-additive: merge must be additive
selectionMergeVocabularyOracle: checks=89 clean
```

**B — the vocabulary oracle** (`"add"` smuggled into the fixture vocabulary):

```
selectionMergeVocabularyOracle: FAILED — "add" is in the fixture vocabulary but the owned MergeMode schema export rejects it
world3dPointerGestureOracle: checks=63 clean
```

**C — the mounted host suite** (`world3dSelectionActionArgs` restored to the pre-fix translation):

```
 × dispatches interactionSelect for the topology target on instance-pick-additive
 × dispatches interactionSelect for the topology target on instance-pick-subtractive
 × dispatches interactionSelect for the topology target on instance-pick-subtractive-on-command
 × dispatches interactionSelect for the topology target on instance-pick-invertive
 × resolves every declared modifier chord to its schema merge word and never to range
 Test Files  1 failed (1)
      Tests  5 failed | 9 passed (14)
```

Restored: `14 passed (14)`.

**D — the no-adapter laws** (`"add" => Some(MergeMode::Additive)` added back to `from_wire_label`):

```
test …::interaction_select_speaks_exactly_the_schema_merge_vocabulary ... FAILED
test …::merge_world_selection_ids_speaks_the_one_schema_merge_vocabulary ... FAILED
the deleted word 'add' must decode to nothing — no adapter, no compatibility layer
test result: FAILED. 0 passed; 2 failed; 669 filtered out
```

Restored: both pass.

---

## 7. Runtime (6018) — what is and is NOT proved

The dev server answered in **8–18 ms** throughout; it is **not wedged**. Host changes are served from
source and **live on reload**; guest (Rust) changes need a **restage**, which this lane did not start.

### 7.1 What was observed

`bun 🔍️browser-probe.ts --mode=interact --steps=hover,select,orbit --label=merge-vocabulary-base --settle=150 --predelay=25`
(03:24 UTC, example "Hexagonal Mushroom Column", `meshes=3`) exercised a real pick:

```
[36621ms] leftover InteractionView {"selectedIds":[],"publishedIds":[],"locked":{},"gumball":false,
  "hoverTarget":{"domain":"graph","channel":"pointer","id":"extrude@solid"}}
[41699ms] leftover InteractionView {"selectedIds":["extrude@solid","extrude@solid"],
  "publishedIds":["extrude@solid","extrude@solid"],"locked":{"extrude@solid":false},"gumball":true,
  "hoverTarget":{"domain":"graph","channel":"pointer","id":"extrude@solid"}}
```

`grep -c "unknown merge" console.jsonl` → **0**.

(The duplicated id is the *other* lane's defect — `📓️selection-dedupe-2026-09-12.md` §5 fixed it in Rust
and recorded that the served `.wasm` predates the fix. Not this lane's.)

`🔍️merge-modifier-probe.ts` *(new, ticket-owned)* plays a plain click, a shift-click, a ctrl-click and a
shift+ctrl-click on the same point of the preview canvas and records, per gesture, the pane's own
`data-selection-json`, every `leftover InteractionView`, every line mentioning `unknown merge`, and
dispatched-vs-settled `interactionSelect` counts after a 15 s drain:

```
[83.1s] gesture plain-click (replace): dispatched=1 settled=1 views=1 unknownMergeFaults=0
[89.3s] gesture shift-click (additive): dispatched=1 settled=0 views=0 unknownMergeFaults=0
[95.6s] gesture control-click (subtractive): dispatched=1 settled=0 views=0 unknownMergeFaults=0
[101.9s] gesture shift-control-click (invertive): dispatched=1 settled=3 views=3 unknownMergeFaults=0
[123.2s] after a 15s drain: interactionSelect dispatched=4 settled=4 unknownMergeFaults=0
DIAGNOSIS: interactionSelect dispatched=4 settled=4; unknownMergeFaults=0
```

### 7.2 What is NOT proved, and why — stated plainly

**The runtime A/B is indistinguishable, so this is not a runtime proof of the fix.** Re-running the same
probe with the host *temporarily* restored to the pre-fix `add`/`remove`/`toggle` translation produced
the **identical** line (`dispatched=4 settled=4 unknownMergeFaults=0`). Two environment facts explain it,
neither caused by this lane:

1. **The preview stopped evaluating geometry mid-session.** The 03:24 run reported `meshes=3`; from 03:41
   on, every run reports `meshes=0`/`1` after waiting up to 240 s, with the shell fully rendered
   (16 windows, 3 canvases) and no page faults. The served guest was **restaged by a peer at 05:36:39
   local** (`dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`), and
   the preview has evaluated no geometry since. With no geometry there is nothing to pick, so the
   modifier chords cannot exercise the *pick* path at all.
2. **A portal overlay intercepts pointer events over the preview canvas under automation.** Playwright
   refuses a locator click (`<div data-semio-portal-layer="true">…subtree intercepts pointer events`);
   raw `page.mouse` events and synthetic bubbling `PointerEvent`s dispatched on the canvas element both
   reach the DOM, but neither produced a host-originated `interactionSelect` carrying a merge word — the
   four dispatches counted in the window are indistinguishable from the app's own eval-driven selects.

So the honest runtime statement is: **the preview raises no `unknown merge` fault and every
`interactionSelect` settles, and the `replace` path demonstrably works end-to-end — but the modifier
chords could not be driven through the host's pick path in this environment, so the runtime evidence
neither confirms nor contradicts the fix.** The fix is proved natively and in jsdom (§5, §6), at the
exact seam that faulted: `parse_merge_mode` on the real reserved job, and the mounted host's own
dispatch.

**A restage is not required for this change to take effect.** Every edit that changes what crosses the
wire is host TypeScript (`🌐️World3dHost/🟦️.tsx`), live on reload. The Rust edits are refactors of the
decoder and of the non-domain path that keep the accepted set identical — the guest accepted `additive`
before this wave and still does; it never accepted `add`.

---

## 8. Files

Changed:

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json` *(new)*
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕹️interaction/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts`
- `/Users/ueli/Documents/semio/.storybook/stories/puzzle/3d/World.stories.tsx`
- `/Users/ueli/Documents/semio/.storybook/stories/puzzle/5d/Timeline.stories.tsx`
- `/Users/ueli/Documents/semio/.storybook/stories/block/3d/World.stories.tsx`
- `/Users/ueli/Documents/semio/.storybook/stories/block/5d/World.stories.tsx`

Ticket-owned input scripts (kept):

- `.../PROCEDURAL-3D-END-TO-END/🔍️merge-modifier-probe.ts` *(new)*
- `.../PROCEDURAL-3D-END-TO-END/🔍️oracle-probe.ts` *(new — runs a bundle `📜️script.ts`'s exported oracles without its argv router)*

Evidence (under `🗑️generated/`, left in place):

- `🗑️generated/merge-vocabulary/merge-modifier-after7-*`, `…-fails-before3-*` (and the earlier attempts)
- `🗑️generated/probe-merge-vocabulary-base-2026-09-12T03-24-13/`
- `🗑️generated/probe-sanity-2026-09-12T03-41-51/`

# Wave B34 — app-declared interaction scope, and the Relocate press

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Written incrementally from the first tool call.

Assignment: B32b handover items 1 and 2 —
1. `dispatch_interaction_action` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22445`) returns
   `UiDirtyScope::Full` for all six `INTERACTION_ACTION_IDS`, `interactionHover` included, so pointer
   motion repaints the whole shell (24 interaction ingresses + 3 full completions per 80 s lane).
   Replace it with an APP-DECLARED scope, framework default unchanged for apps that declare nothing.
2. `beginRelocateDrag`'s press never dispatches `worldRelocate` in the probe lane — a hit-test red of
   the same family as B28's gumball tip-vs-origin miss.

## 0 Reading log (established before any edit)

- `dispatch_interaction_action` is the SINGLE scope decision for all six verbs: the only other
  `INTERACTION_ACTION_IDS` sites are `is_framework_reserved_action_id`/`framework_reserved_action_kind`
  (`:19263`, `:19277`), `retire_pending_reserved_latest_wins` (`:23057`) and the one dispatch hop
  `commit_framework_shared_host_route` (`:23387`) that calls it. Narrowing one `empty_result` call
  narrows every interaction refresh in every app.
- **Where the declaration can live.** `InteractionDefinition`
  (`🧰️framework/🔨️modules/🕹️interaction/🦀️.rs:43`) is constructed as a STRUCT LITERAL in **46** places
  across 40 plugin editors (counted with a `(?<!-> )InteractionDefinition \{` sweep), so a new field on
  it — even `Option<…>` — is a 46-literal sweep through files peers are editing, plus a four-format
  schema-leaf change (`🧬️schema/🔣️.json` `additionalProperties:false`, `🦀️.rs`, `🟦️.ts` +
  `parse…()`, `🔗️.graphql`, `EXPORTS[16]`). `AppDefinition` is worse.
  `ArtifactApp` (`🔌️plugin/🦀️.rs:10960`) instead already carries ~40 *declaration hooks* with
  framework defaults (`build_tool_job`, `register_window_config_owners`,
  `retained_window_transient_target`, …), each forwarded verbatim by
  `impl<E: ArtifactEditor> ArtifactApp for EditorApp<E>` (`:28723`) and
  `impl<V: ArtifactViewer> ArtifactApp for ViewerApp<V>` (`:29027`). That is the declaration surface
  this wave extends: one hook, default `None` → the framework keeps `UiDirtyScope::Full`, zero churn
  in the 40 apps that declare nothing, and the app computes its answer from its OWN body-key
  constants (which manifest data could not do without re-declaring them).
- `puzzle3d` already owns the right vocabulary (`✏️editor/🦀️.rs:2274-2347`):
  `Puzzle3dScopeClass::{Viewport,Selection,WindowOption,…}` → `puzzle3d_scope()`, with
  `puzzle3d_selection_panel_bodies()` = inspection + document(outliner) + framework history. So the
  select half of the assignment is a class this app already declares; the wave adds the verb keying.

---

## 1 The interaction scope is app-declared now

### 1.1 The framework half

`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`🔖️Interaction` region, beside the six ids it closes over)
gains `InteractionVerb` — `Select`/`Hover`/`ClearSelection`/`SelectAll`/`SetSelectionMode`/
`SetGranularity`, with `of_action`/`action_id`/`ALL`. The framework owns the verbs; only what each one
DIRTIES is app knowledge.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

- `ArtifactApp::interaction_scope(verb, domains) -> Option<UiDirtyScope>`, default `None`, beside the
  other ~40 declaration hooks. `ArtifactEditor`/`ArtifactViewer` get the same default and
  `EditorApp<E>`/`ViewerApp<V>` forward it verbatim, exactly like `retained_window_transient_target`.
- `dispatch_interaction_action` now matches on the VERB instead of six `_ if action == …` guards (its
  `unreachable!("… INTERACTION_ACTION_IDS out of sync")` arm is gone — a non-interaction id is a
  fail-closed `Fault` at the top instead), collects the domains it actually wrote, and closes with:

  ```rust
  let declared: Vec<&str> = touched.iter().map(String::as_str).collect();
  let scope = A::interaction_scope(verb, &declared).unwrap_or(UiDirtyScope::Full);
  ```

  `touched` is the single domain the verb names, or every declared domain for
  `clearSelection`/`selectAll` — the exact set the arms wrote, derived through the same
  `interaction_domain_id_arg`/`registry.interactions()` the arms use, so there is no second source of
  truth. The docstring that advertised the blanket `Full` is replaced by the contract.
- `INTERACTION_ACTION_IDS` is `pub(crate)` so the round-trip law can assert the verb type covers
  exactly the intercepted ids.

**Why a hook and not a manifest field** — see §0: a field on `InteractionDefinition` is a 46-literal
sweep across 40 peer-owned plugin editors plus a four-format schema-leaf change, for a value only the
declaring app can compute (its own body keys). The hook is the surface this trait already uses for
every other app declaration, and it costs the 40 apps that declare nothing exactly zero.

### 1.2 The puzzle3d half — the scope table per verb

`✏️editor/🦀️.rs` gains `Puzzle3dScopeClass::Interaction(InteractionVerb)` in the SAME table
`puzzle3d_command_scope_class` keys, plus two named scopes (`puzzle3d_selection_scope` factored out of
the old inline `Selection` arm, `puzzle3d_interaction_chrome_scope` new), and
`Puzzle3dPlayApp::interaction_scope` answers out of it:

| verb | class | window bodies | panel bodies | measures | rails/labels |
| --- | --- | --- | --- | --- | --- |
| `interactionHover` | `Interaction(Hover)` → `puzzle3d_viewport_scope` | `puzzle3d.play.composite` | **none** | no | none |
| `interactionSelect` | `Interaction(Select)` → `puzzle3d_selection_scope` | `puzzle3d.play.composite` | inspector + outliner(`document`) + `framework.body.history` | yes | none |
| `clearSelection` | `Interaction(ClearSelection)` → same | same | same | yes | none |
| `selectAll` | `Interaction(SelectAll)` → same | same | same | yes | none |
| `setSelectionMode` | `Interaction(SetSelectionMode)` → `puzzle3d_interaction_chrome_scope` | `puzzle3d.play.composite` | **none** | yes | none |
| `setInteractionGranularity` | `Interaction(SetGranularity)` → same | same | **none** | yes | none |
| any domain ≠ `vortex`, or none | — | `None` → framework `UiDirtyScope::Full` | | | |

Before: every one of the six was `{kind:"full"}` — every window body, every panel body, utilities,
tools, engagements, measures AND labels.

### 1.3 Laws

`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `TestApp` now DECLARES a scope for
its `items` domain (and deliberately declares nothing for `setInteractionGranularity`, so the
framework default keeps a fixture), and the window body key is one shared
`TEST_APP_WINDOW_BODY_KEY` constant the registry fixture and the laws both read:

- `interaction_hover_dirties_only_the_hovering_windows_own_lane` — the plugin-host law the assignment
  asks for: `window_bodies == ["synthetic.main"]`, `panel_bodies` empty, and every chrome flag false.
- `interaction_select_carries_the_apps_declared_selection_lane` — the declared select scope is carried
  through the reserved-job settle verbatim, panel bodies included.
- `an_undeclared_interaction_verb_keeps_the_framework_full_scope` — the default arm.
- `every_intercepted_interaction_id_round_trips_through_its_verb` — `InteractionVerb` covers exactly
  `INTERACTION_ACTION_IDS`, both directions.

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` —

- `interaction_verbs_declare_exactly_the_lanes_they_move` — the puzzle law: all six verbs against the
  table above, plus `None` for an undeclared domain and for an empty domain set.
- `a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels` — field by field,
  including `panel_bodies.len() == 3` for a pick, so a fourth panel cannot be smuggled in.

Outputs (foreground):

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 interaction_verbs_declare a_hover_paints_only
test editor::puzzle3d::component::tests::a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels ... ok
test editor::puzzle3d::component::tests::interaction_verbs_declare_exactly_the_lanes_they_move ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 723 filtered out; finished in 0.00s
```

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 interaction local_interaction selection
test …::an_undeclared_interaction_verb_keeps_the_framework_full_scope ... ok
test …::every_intercepted_interaction_id_round_trips_through_its_verb ... ok
test …::interaction_hover_dirties_only_the_hovering_windows_own_lane ... ok
test …::interaction_select_carries_the_apps_declared_selection_lane ... ok
test result: FAILED. 55 passed; 10 failed; 0 ignored; 0 measured; 610 filtered out; finished in 6.79s
```

The 10 failure NAMES are **byte-identical to B31's recorded baseline** for the same filters
(`🗑️generated/b31-framework-interaction.txt`: `51 passed; 10 failed`, same ten names), and the pass
count moved 51 → 55 — exactly this wave's four new laws. **Zero new reds.** Six of the ten are one
peer defect (`typed command 'setLabel' has no exact controller/owner/factory/tool/schema proof`), one
is `presence.is_empty()` in the dispatch leaf, and
`set_selection_mode_and_set_interaction_granularity_persist_immediately` fails at ADMISSION
(`handle_action` returns a `SpawnJob` where the law expects an immediate `Err` on a bogus
granularity) — a path this wave does not touch, red before it and red after it, same line.

---

## 2 The Relocate press — the assignment's premise is wrong, and the logs already say so

The assignment (from B32b §4 / B28's family) reads the red as a hit-test that is too narrow: "make
`beginRelocateDrag` grab the object when the press lands anywhere on its instance mesh or its projected
bounds". `beginRelocateDrag` (`🌐️World3dHost/🟦️.tsx:5835`) already does exactly that —
`resolveClickInstanceId` (`:3760`) projects each instance's **mesh AABB** (or a unit cube when no mesh
data is resident), takes the screen-space AABB of the eight projected corners, and returns the nearest
one CONTAINING the click (`resolveClickInstanceIdFromProjected`, `:3744`). There is no tip-vs-segment
narrowing left to widen.

What the lane actually does (`🔍️browser-probe.ts:2252-2278`, this wave only READ it):

```ts
const framed = await frameForestTableAfterCensus();
…
await page.mouse.move(box.x + framed.table.x, box.y + framed.table.y);
await page.mouse.down();
await page.mouse.move(box.x + framed.table.x - 130, box.y + framed.table.y - 70, { steps: 24 });
await page.mouse.up();
```

and `framed.table` is **a blind fraction of the canvas**, not a projected instance:

```ts
const table = { x: Math.round(box.width * 0.78), y: Math.round(box.height * 0.42) };   // :885
```

The probe's own logs give the two numbers that settle it, with no new measurement:

| reading | value | source |
| --- | --- | --- |
| press point | `box=947x814 table=739,342` | `framed forest table …` (every 2026-09-12 probe run) |
| the ONE object's projected origin | `{"kind":"origin","sx":474,"sy":407}` / `474,514` | `gumball hits …` (same runs; the gumball target IS `seed-left-001`'s origin) |
| the whole gumball arrow, tip to origin | `476,447` → `474,407` ≈ **40 px** | same line |

The default `concrete-forest` example has ONE object at `[0,0,0]` (B32b §0), framing centres it — `474,407`
is the centre of a 947×814 canvas — and the press is **265 px right and 65 px above** it, with the
object's own on-screen extent on the order of the 40 px gumball. The press lands on **empty space**, so
`resolveClickInstanceId` correctly returns `null`, `world3dRelocateDragTargetV1(null, ["seed-left-001"])`
correctly returns `null` (a press outside the selection is the declared GATE, `:4064-4073`), and the
gesture correctly never starts. **Nothing about the hit test is broken.**

### 2.1 The measurement that settles it, live on `:6013` (wasm #53)

A temporary tap in `beginRelocateDrag` (added, measured, removed — swept, zero remain) printed the
press's own three inputs. One `--relocate` run, `🗑️generated/probe-2026-09-12T04-54-04.md:635`:

```
log: [DEBUG] relocate press from=[9.76082208192068,42.240314770264064,0] pressed=null ids=["seed-left-001"] local={"x":739,"y":342} instances=1
```

Three facts, none of them guesses:

1. **the ground point resolves** — `from=[9.76,42.24,0]`. `raycastGroundPoint` (`:4021`) refuses a ray
   above the horizon (`t < 0 → null`), and it did not refuse; the press is on the ground plane.
2. **nothing is under the pointer** — `pressed=null`, as §2 predicted from the 265 px gap.
3. **the selection is live** — `ids=["seed-left-001"]` (the leftover overlay survives the empty-space
   click, exactly as B32b measured for the gumball lane).

So the gesture is refused at ONE hop: `world3dRelocateDragTargetV1(null, ["seed-left-001"]) === null`.

### 2.2 Fix: an empty-ground press is the gesture's BASE POINT, not a miss

```ts
export function world3dRelocateDragTargetV1(pressedId: string | null | undefined, selectedIds: readonly string[]): string | null {
  if (!pressedId) return selectedIds[0] ?? null;
  if (selectedIds.length === 0) return pressedId;
  return selectedIds.includes(pressedId) ? pressedId : null;
}
```

Why this and not a wider hit test: `world3dRelocateDispatchArgsV1` (`:4081`) already computes
`origin + (to - from)` — a travel DELTA — so the press point was never "where the object goes", it was
always the base point of a two-point move ("pick a base point, pick a target point", the way every CAD
tool spells Move). Requiring the base point to be ON the object was the anomaly; it makes the utility
unusable whenever the grab point is occluded, off-screen, or just small on screen. A press on a
DIFFERENT object still returns `null` and still falls through to the pick path, so re-selecting by
clicking another object is unchanged. The deliberate cost, written into the docstring: inside this one
utility an empty-ground press is a base point rather than a marquee, and a press without travel commits
nothing (`GUMBALL_TRANSFORM_EPSILON`), so Escape / `clearSelection` stay the way to drop a selection.

### 2.3 Verdict — the lane flips, twice, on two independent runs

| run | `relocate-arm` | `relocate-pose-delta` | `relocate-no-hard-fault` |
| --- | --- | --- | --- |
| coordinator's full battery, wasm #53, before (`🗑️generated/battery-2026-09-12-53-6013.txt`) | PASS | **FAIL** `beforeLen=297 afterLen=297 waitedMs=30426` | PASS |
| `--relocate`, before, with the tap (`b34-relocate-before.txt`) | PASS | **FAIL** `beforeLen=266 afterLen=266 waitedMs=30135` | PASS |
| `--relocate`, after (`b34-relocate-after.txt`) | PASS | **PASS** `beforeLen=266 afterLen=314 waitedMs=2724` | PASS |
| `--brush --relocate --gumball`, after (`b34-storm-before.txt`) | PASS | **PASS** | PASS |

The whole commit chain, from that run's console
(`🗑️generated/probe-2026-09-12T04-59-21.md:608-625`):

```
[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"worldRelocate"}
[DEBUG] command ingress lane {"instanceId":1,"actionId":"worldRelocate","seq":70,"lane":"Interactive"}
[DEBUG] history patch applied {"replace":false,"currentCursor":17,"patchCursor":18,"upserts":1,
  "labels":["move-object id=seed-left-001 new-origin=-33.95723211989221,11.799326432808115,0.0000000000000071"]}
[DEBUG] performInvocation settled {"actionId":"worldRelocate","frames":2,"frameKinds":["Invocation","Ephemeral"]}
```

`worldRelocate` dispatches, the document edit lands, and `data-instances-json` moves `266 → 314` bytes
in **2.7 s** (the lane's own budget is 30 s). The same run also carries `gumball-scene-delta` PASS, so
the two world gestures are green together for the first time in this ticket.

### 2.4 Law

`🧪️tests/🔬️engine-contract/🟦️.ts`, inside the existing
`"Relocate-utility drag grabs by selection gate and commits one absolute worldRelocate"`:

```ts
expect(world3dRelocateDragTargetV1(null, ["seed-left-001"])).toBe("seed-left-001");
expect(world3dRelocateDragTargetV1(undefined, ["obj-1", "obj-2"])).toBe("obj-1");
```

with the measurement quoted in the comment above them, and every pre-existing expectation of that law
(including `(null, [])` → `null`, the no-selection arm) left untouched.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts -t "Relocate-utility drag grabs by selection gate"
 Test Files  1 passed | 28 skipped (29)
      Tests  1 passed | 967 skipped (968)
```

---

## 3 The refresh storm, measured before — and why "after" rides #54

`:6013` serves guest wasm **#53**, and `dispatch_interaction_action` compiles INTO the guest
(`semio-framework-plugin` is a guest crate; the guest returns `ui_scope` on the invocation and the
JS `ShellHost` obeys it). So §1's narrowing is **not live on #53** — it needs the next materialize —
and this wave deliberately did NOT deploy #54: B33 is bisecting full-battery state pollution against
this exact artifact, and swapping the live wasm mid-bisect would destroy that reading.

What IS measured is the BEFORE, with the blanket `Full` in place. A temporary counter next to
`refreshUi`'s existing `[DEBUG] refreshUi sections` line (`🏛️ShellHost/🟦️.tsx:4634`, which is behind
`runtimeDiagnosticsEnabled()` and therefore silent in probe runs — **0 hits** in the coordinator's
battery log) reported, per pass, the scope kind and how many window/panel bodies were REQUESTED versus
answered with a value. Added, measured, removed; swept (`b34pass` returns nothing in `🧰️framework`/`✏️s`).

Run `--brush --relocate --gumball --port=6013` (the brush lane is the 70-pointermove hover storm),
counted over the probe's captured console tail (1203 console lines ≈ the storm's tail through the
relocate lane, `🗑️generated/probe-2026-09-12T05-06-59.md`):

| before, wasm #53 | value |
| --- | --- |
| refresh passes that asked for anything | **26** |
| of those, `{kind:"full"}` (every window body + every panel body + rails + labels + measures) | **22** (85 %) |
| bodies REQUESTED (78 window + 110 panel) | **188** |
| bodies answered with a value (39 window + 21 panel) | **60** |
| bodies the guest re-rendered and answered `unchanged` | **128** (68 % of the work) |

That is the cost §1 removes: with the declared table, a hover asks for ONE window body and zero panels,
and a pick asks for one window body plus three panels — so the 22 full passes above become
single-body passes, and the 128 wasted body renders collapse. The AFTER row is owed on the first
battery run against a wasm that carries this wave's guest halves.

**Guest halves riding #54** (not live on #53): `InteractionVerb` + `ArtifactApp::interaction_scope` +
`dispatch_interaction_action`'s declared scope + `Puzzle3dPlayApp::interaction_scope` and the
`Puzzle3dScopeClass::Interaction` table. **Host halves live on #53 right now** (vite-live, measured
above): the whole of §2 (`world3dRelocateDragTargetV1`).

---

## 4 Verification — every command foreground, tails quoted

| command | result |
| --- | --- |
| `cargo check -p semio-framework-plugin --lib` | `Finished \`dev\` profile … in 19.38s` — **0 errors**, 5 pre-existing warnings (the warnings prove expansion ran) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 88 warnings` / `Finished \`dev\` profile … in 28.29s` — **0 errors**, same 88 as B32b |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0` / `Finished \`dev\` profile … in 27.32s` — **0 errors** (covers puzzle 2d/3d/5d guest halves) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 interaction local_interaction selection` | `55 passed; 10 failed` — the ten names are **byte-identical to B31's baseline** (`51 passed; 10 failed`), pass count +4 = this wave's four laws. Zero new reds |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1` | `715 passed; 13 failed` vs B31/B32b's `711 passed; 12 failed`. Name diff against `🗑️generated/b31-puzzle3d-lib-final.txt`: **GONE: none; NEW: one** — `…precompute::fill::tests::adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms`, which panics `stage PrepareSpatial reached the 8ms ceiling` (a wall-clock budget law) and passes **in isolation**: `test … ok / 1 passed; 0 failed; 727 filtered out; finished in 0.01s`. A load artifact on a machine running a peer's battery, not a scope regression — nothing in the fill precompute reads an interaction scope |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 26 passed (29)` / `Tests 9 failed \| 959 passed (968)` — the nine are **exactly** B18/B20/B23's nine (1 `buildNoteShellCommandAction`, 6 `🧩️package-integration` wgpu-worker, 2 `🔌️PluginRuntime`). Zero new |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json`, scoped to the edited files | `World3dHost/🟦️.tsx` errors at lines `1275 1276 1307 3352 4224 4561 4748` — all pre-existing (B20 recorded the same family at `1303 / 3302 / 4160 / 4497` before peer churn shifted them), **none** in the edited `WorldRelocateGesture` region (~4069-4110) or at `beginRelocateDrag`. `🔬️engine-contract/🟦️.ts` errors at `5001 5002 7076 8322` — none at the edited law (~7745) |
| probe `--relocate --port=6013` (before / after) | `relocate-pose-delta` **FAIL waitedMs=30135** → **PASS waitedMs=2724** |
| probe `--brush --relocate --gumball --port=6013` (after) | `relocate-arm` PASS, `relocate-pose-delta` PASS, `relocate-no-hard-fault` PASS, `gumball-handle-enter` PASS, `gumball-scene-delta` PASS; `brush-preview-place` FAIL `[expect-41] instances=1 preview=null` — the same red the coordinator's battery carries on #53, untouched by this wave |

Every probe run waited for the single-tab lease (`pgrep -f 'bun .*browser-prob[e]'` empty, polled at
60 s) and `🔍️browser-probe.ts` was READ but never edited. No `page.evaluate` was issued between
`mouse.down()` and `mouse.up()`.

### Temporary instruments — added, measured, removed

| tap | file | swept |
| --- | --- | --- |
| `[DEBUG] relocate press …` | `🌐️World3dHost/🟦️.tsx` (`beginRelocateDrag`) | `pressedTap` → no hits; the `[DEBUG]` literal is gone from the region (the docstring now cites the reading in prose) |
| `[DEBUG] b34pass …` | `🏛️ShellHost/🟦️.tsx` (next to `refreshUi sections`) | `b34pass` → no hits under `🧰️framework`/`✏️s` |

---

## 5 Files

Framework (guest, rides wasm #54):

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `InteractionVerb` (`🔖️Interaction` region)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer::interaction_scope` + both forwardings, `dispatch_interaction_action` (verb match + declared scope), `INTERACTION_ACTION_IDS` visibility
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `TEST_APP_WINDOW_BODY_KEY`, `TestApp::interaction_scope`, four laws

Puzzle3d (guest, rides wasm #54):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `Puzzle3dScopeClass::Interaction`, `puzzle3d_selection_scope`, `puzzle3d_interaction_chrome_scope`, `Puzzle3dPlayApp::interaction_scope`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — two laws

Host (TypeScript, live on `:6013` now):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — `world3dRelocateDragTargetV1` base point + two docstrings
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — the relocate law

`🏛️ShellHost/🟦️.tsx`: **net zero** (measurement tap only). `🔍️browser-probe.ts`: **untouched**.

---

## 6 Handover

1. **The after-measurement of §3 is owed on the first battery against wasm #54.** Expect the 22
   `{kind:"full"}` passes to drop to single-body passes and the 128 unchanged-body renders to collapse;
   the two laws in §1.3 are what pin the contract until then. The deploy is the coordinator's (this wave
   did not take it, to protect B33's bisect against #53).
2. **The declaration surface is now open for the other 40 apps.** `ArtifactApp::interaction_scope`
   defaults to `None` → `Full`, so every other plugin still repaints its whole shell on pointer motion.
   `cad`, `generation3d`, `process3d` and `flow` are the ones with real hover-heavy viewports; each needs
   its own three-line table, and `puzzle3d`'s is the reference.
3. **`set_selection_mode_and_set_interaction_granularity_persist_immediately`** (plugin crate) expects
   `handle_action` to reject a bogus granularity at ADMISSION; the reserved-job split validates at
   COMMIT, so it returns a `SpawnJob`. Red before and after this wave, and a genuine contract question
   for whoever owns reserved admission: either admission re-validates declared args, or the law should
   assert the settle.
4. **`brush-preview-place`** stays red on #53 (`[expect-41] instances=1 preview=null`) — untouched here.
5. **The relocate lane's press point is still a blind `0.78 × 0.42` canvas fraction**
   (`🔍️browser-probe.ts:885`). The fix in §2.2 makes that *correct* rather than *lucky*, but whoever owns
   the probe should still aim the lane at the object's own projected origin (the `data-gumball-hits`
   `{kind:"origin"}` stamp B28 added is already there), so that a future regression in the mesh pick
   cannot pass this lane by riding the base-point path.

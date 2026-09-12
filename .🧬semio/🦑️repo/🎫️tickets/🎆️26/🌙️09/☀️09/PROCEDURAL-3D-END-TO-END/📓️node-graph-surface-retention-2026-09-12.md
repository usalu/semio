# The node-graph surface was re-keyed, not re-mounted on purpose — reconciliation identity, and what the ~13 s it cost really was

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-12. Lane: the **React-side node-graph host
lifecycle** (`🗣️Interpreter`, `🕸️NodeGraph`, `🏛️ShellHost`'s window-body dispatch). Repo MCP was
unavailable this session (`invalid initialize params`), so the ticket folder was managed on disk and no
ticket state was opened, closed or reopened. Concurrent lanes (`node-graph-canvas-paint`,
`role-switch-runtime`, `selection-merge-vocabulary`, and at least one more) were editing the same tree
throughout; every red below is attributed.

Reads first: `📓️dispatch-timer-throttle-2026-09-12.md` §3 and §8, `📓️node-graph-attach-2026-09-10.md`,
`📓️hotpath-optimization-2026-09-10.md`, `📓️runtime-hotpath-audit-2026-09-10.md`.

## TL;DR

1. **Root cause found and fixed, with the id in hand.** `builtNodeToSnapshot` mints `UiNodeId`s by
   **pre-order DFS over the whole window body**, and `ContainerView` keyed its child run on that
   number (`<UiNodeView key={childId} …>`). So any refresh that grew the body *ahead of* the
   node-graph surface renumbered it — measured on `window:procedural-main`, the body went 30 → 34
   nodes when four port rows appeared (`profile@wire`, `extrusion-axis@vector`,
   `extrusion-axis@errors`, `extrude@solid`), the surface moved **id 30 → 34** and its container
   **29 → 33** — and a changed React key is an **unmount**. React tore down the flow host and built a
   new one: second `createFlowSession`, second canvas, second wasm-side surface (`flow surface created
   surface=2`).
2. **It is once per boot, not once per refresh.** The brief's premise was that the host remounts on
   every refresh; the measurement says **exactly one spurious remount per boot** on this graph, because
   the body's shape changes exactly once (7 nodes / 6 edges, ports settle once and then stop moving).
   Every later refresh already kept id 34 and did not remount. The defect is nonetheless structural:
   *any* shape change ahead of the surface re-keys it, so an interactive session (adding a node,
   an error row appearing, a cancelled evaluation retracting ports) re-keys it repeatedly.
3. **Where the ~6 s went.** Not into surface creation. The first attach of the boot takes **871 ms**
   (`attach called` 5281 → `flow surface created` 6149, `present=2d`, no GPU). The *second* attach —
   the one the remount caused — took **5.6 s** because its promise resolution sits behind a live
   `flowEvalTick` guest turn on the same main thread. So the "~6 s surface create" is **main-thread
   contention on a surface that should never have been created**, and retaining the surface removes
   the whole window rather than making it faster.
4. **Fixed structurally**: reconciliation now keys a sibling run on the node's **authored key** — the
   same identity `uiNodeDomId` has always used for DOM ids — falling back to the minted id only for a
   keyless or ambiguous sibling. One new rule, `uiSiblingReactKeys`, used by `ContainerView` and by the
   tree-row control run.
5. **Runtime, after the fix** (`🗑️generated/surface-retention-1/`, 120 s): `node-graph host mount` **1**,
   `node-graph host unmount` **0**, `flow surface created` **1** — the targets. All 7
   `window:procedural-main` statuses `ok`; `window:procedural-preview` `facesDone 8/8`,
   `unitsDone 44/44`, `ratio 1.0`.
6. **Wall clock: ~109 s, against a 122 s baseline — the ≪30 s target was NOT met**, and could not have
   been by this lane. The retention fix buys back exactly the remount (≈13.3 s, §3.3); the remaining
   ~12–18 s per `flowEvalTick` settle is the guest command itself, already measured and attributed in
   `📓️dispatch-timer-throttle-2026-09-12.md` §3. I will not claim a number I did not measure.

---

## 1. Method

`cd <ticket> && SEMIO_PROBE_SECONDS=N SEMIO_PROBE_OUT=… bun 🐍️console-dump-probe.mjs` against
`http://127.0.0.1:6018/?plugin=generation3d` — headless Chromium, never foregrounded, i.e. the same
renderer class the earlier probes used. The react dev serve answered `200` in 10 ms at the start and
at the end of the lane; it was never restarted and no wasm was rebuilt (these are TS-only edits on a
vite-served tree, so a reload is the whole deployment).

Temporary `[DEBUG]` brackets were added and **removed again**: a `performance.now()` bracket around
`mount → session ready → attach called → surface ready → attach sync`, and one inside
`createBuiltNodeStoreCacheV1.flushPendingReloads` printing each `window:procedural-main` reload's node
count, root, surface records and the full `id/type/key` shape. Consoles are kept under
`🗑️generated/surface-retention/{diag-1,diag-2,fix-1}/`.

## 2. The evidence

### 2.1 The remount, and what moved

`🗑️generated/surface-retention/diag-1/console.txt` (pre-fix, brackets on):

```
 3246  store reload window:procedural-main  nodes=30  root=1  surfacesBefore=          surfacesAfter=30:procedural.play:node-graph
 3327  node-graph host mount    surface=window:procedural-main
 4191  node-graph session ready
 4249  node-graph attach called 483x814 dpr=1
 5050  flow surface created     surface=1  483x814 present=2d
 5050  node-graph surface ready nodes=7 edges=6
12916  node-graph first draw    store=483x814

22810  store reload window:procedural-main  nodes=34  root=1  surfacesBefore=30:procedural.play  surfacesAfter=34:procedural.play   ← +4 nodes, surface renumbered
22894  node-graph host mount    ← REMOUNT
22897  node-graph session ready (3 ms — the module is warm, so this is a second session, not a second load)
22998  node-graph attach called
23071  extension request answered flow-extension-math turns=1      ← the guest is mid-turn on the main thread
28567  flow surface created     surface=2  ← 5 569 ms after `attach called`
28837  dag draw

36180  store reload  nodes=34  surfacesBefore=34  surfacesAfter=34   ← no remount
46975  store reload  nodes=34 … 59798 … 71977  — all stable, no further mount
```

**Two mounts, two wasm surfaces, one window.** The second `flow surface created` is `surface=2`: the
wasm side minted a whole new surface while the first one was still alive.

### 2.2 What the four extra nodes are (`diag-2`, full shape dump)

Refresh A (30 nodes) and refresh B (34 nodes), post-order as the store holds them:

```
A  … 10/treeItem/profile          ← [profile@radius, profile@sides]
   … 13/treeItem/extrusion-axis   ← [@x, @y, @z]
   … 17/treeItem/extrude          ← [@wire, @vector]
   … 2/tree/procedural-play-graph
     29/container/procedural-play-main.canvas
       30/surface/procedural.play
      1/container/procedural-play-main.body

B  … 10/treeItem/profile          ← [@radius, @sides, @wire]        +1
   … 14/treeItem/extrusion-axis   ← [@vector, @x, @y, @z, @errors]  +2
   … 20/treeItem/extrude          ← [@wire, @vector, @solid]        +1
   … 2/tree/procedural-play-graph
     33/container/procedural-play-main.canvas
       34/surface/procedural.play
      1/container/procedural-play-main.body
```

The body's own child run went `[2, 29]` → `[2, 33]`. Keyed on that number, React sees key `29`
**removed** and key `33` **added**, and unmounts the entire canvas subtree — the flow host included —
for a change that happened in a *sibling*.

The status map is NOT the trigger, and neither is a `useMemo` dependency or the `BuiltNode` reference:
`mergeRecordPreservingIdentity` and `createBuiltNodeStoreCacheV1` both behave exactly as documented,
the `InterpretedUiNode` memo is never the issue (the root id stays `1` throughout, so it does not even
re-render from the top), and `FlowGraphCanvasHost`'s own effects are keyed on `[surfaceId]` /
`[sessionReady, …]`, none of which moved. **The only thing that moved was a reconciliation key.** The
status map is merely what *causes* the shape change, one hop upstream.

### 2.3 Where the ~6 s actually is

| attach | `attach called` → `flow surface created` |
|---|---|
| boot (first, nothing else running) | **871 ms** |
| remount (concurrent with a `flowEvalTick` guest turn) | **5 569 ms** |

Surface creation is not expensive; the *second* one is slow because it queues behind the evaluation
turn that the very same settle kicked off. The audit's "~6 s of surface-create work" is really "a
surface create that had to wait its turn" — and the fix is not to speed it up but to not do it.

The visible damage is larger than the 5.6 s: the remount threw away a canvas that had **already drawn**
(`first draw` at 12916) and the replacement did not draw until **61981** in the same run — the node
graph was blank or stale for **39 s** of a 75 s boot.

## 3. The fix

One rule, in the interpreter's existing `🪪️StableDomIds` region, right beside `uiNodeDomId` — because
it is the *same* identity decision, applied to reconciliation instead of to the DOM:

```ts
export function uiSiblingReactKeys(siblings: readonly { readonly id: UiNodeId; readonly key: string }[]): readonly string[] {
  const occurrences = new Map<string, number>();
  for (const sibling of siblings) if (sibling.key) occurrences.set(sibling.key, (occurrences.get(sibling.key) ?? 0) + 1);
  return siblings.map((sibling) => (sibling.key && occurrences.get(sibling.key) === 1 ? `k:${sibling.key}` : `#${sibling.id}`));
}
export function uiChildReactKeys(state: UiDocumentState, children: readonly UiNodeId[]): readonly string[];
```

- An authored `key` wins whenever it exists **and is unique among its siblings**. React needs only
  sibling-local uniqueness, so a duplicate must fall back for **both** siblings — keying one of a
  duplicated pair and not the other would silently reorder them against each other.
- A keyless node keeps the minted id. That is still *correct*, just not *stable* — the honest position,
  and the same one `uiNodeDomId` already takes.
- `k:` / `#` namespacing keeps a node that authors the literal key `"7"` off its keyless sibling's `#7`.

Call sites: `ContainerView`'s child run (the path to the surface: body → canvas → surface, all three
containers), and `treeItemToTreeData`'s inline control run, now via `renderTreeItemControls`.

`FlowGraphCanvasHost` needed **no change**. Its session effect is `[surfaceId]`-keyed and its attach
effect `[sessionReady, paintOverlays, renderFlow, surfaceId, syncSurfaceSize]`-keyed; `controllerId`
(the record id, which *does* move, 29 → 33) is only in `dispatch`'s closure and reaches no effect. The
incremental patch path the brief asked me to find already exists and already runs on every refresh:
`syncFlowSessionFromScene` (→ `syncFlowSessionStructureFromScene` + `syncFlowSessionEvalFromScene`) in
the `[sceneSignature, …]` effect. With the host retained, that effect is now the *entire* cost of a
refresh.

One permanent marker was added: `[DEBUG] node-graph host unmount surface=…`. A retained host unmounts
exactly once, when its window closes; a second `host mount` after it means the subtree was re-keyed.
It belongs beside the mount/session/attach markers `📓️node-graph-attach-2026-09-10.md` established.

## 4. Runtime proof

`🗑️generated/surface-retention/fix-1/` (75 s) and `🗑️generated/surface-retention-1/` (120 s, the run the
brief asks for).

```
$ cd <ticket> && SEMIO_PROBE_SECONDS=120 SEMIO_PROBE_OUT=surface-retention-1 bun 🐍️console-dump-probe.mjs

mounts: 1   unmounts: 0   flow surface created: 1

 3457  node-graph host mount    surface=window:procedural-main
 4382  node-graph attach called 483x814 dpr=1
 5239  flow surface created     surface=1  483x814 present=2d
 5356  dag draw lod=normal
13717  node-graph first draw    store=483x814
13760  dag draw lod=detail   38334 …   69829 …   87530 …      ← redraws on the RETAINED surface
```

The store still reloads with 34 nodes at 24426/38730/52443/64534 in the 75 s run — the renumbering is
authored by the plugin and is legitimate; it simply no longer costs a host.

Final state (`hosts.json`):

- `window:procedural-main` — `height, radius, sides, profile, extrusion-axis, extrude, column-preview`
  **all `ok`** (7/7).
- `window:procedural-preview` — `phase: idle`, `unitsDone 44/44`, `facesDone 8/8`, `inFlight 0`,
  `ratio 1.0`, `meshesLen 3641`.

**Wall clock.** The last tessellate answer (`bytes: 2236`, the one that completes the chain) lands at:

| run | mounts | last tessellate answer |
|---|---|---|
| `🗑️generated/timer-throttle-8` (baseline, previous lane) | 2 | **122 095 ms** |
| `🗑️generated/surface-retention-1` (this lane) | 1 | **108 827 ms** |

**≈13.3 s**, which is the remount window measured directly in §2.1 (`22894` mount → `38050` command
settle ≈ 13.1 s). The two numbers agree, which is the only reason I trust either. The ticket's
"well under 30 s" target remains unmet and is **not** reachable from this lane: the residual is
12–18 s per `flowEvalTick` settle inside the guest command, already localized and attributed in
`📓️dispatch-timer-throttle-2026-09-12.md` §3 (`turns: 1` on every extension answer, 0.6 s natively).

## 5. Tests

Language-neutral fixture: **`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json`**
— the four measured refreshes of `window:procedural-main` as authored bodies (30 / 34 / 34 / 30 nodes,
including the retraction case), each with its own status map; the expected minted ids, node counts,
DOM ids, reconciliation key path, mount count and attach count; and five sibling-key cases (authored,
renumbered, keyless, ambiguous, key-vs-id collision).

| half | where | what it proves |
|---|---|---|
| node twin, framework-free | `surfaceHostRetentionOracle` in `…/🎯️targets/⚛️react/📜️script.ts` | Re-derives `builtNodeToSnapshot`'s numbering and the sibling-key rule from the fixture's written law with **no React, no jsdom and none of our renderer code**, then runs a minimal reconciler over the four refreshes twice: keyed on the authored key it mounts the surface **1×**; keyed on the minted id it mounts it **more than once** — so the twin witnesses the defect and the fix on the same data. Source anchors pinned. The fixture document itself is validated by **ajv** (third-party), not by the shape the function happens to read. **27 checks.** |
| React half | `🗣️Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx` | 4 consecutive refreshes with changing status maps through the real `UiNodeView`/`builtNodeToSnapshot`/`UiDocumentStore`: the canvas container's and the node-graph host's **DOM element objects are identical** across all of them, exactly one host exists at the end, and the authored DOM id never moves. Each refresh is proven to have *reached the DOM* first (`data-ui-node-id` on that very element moves 29 → 33 → 33 → 29) — without that witness the identity assertions pass on a render that never happened, which is how the first draft of this test passed against the broken code. |
| host half | `🧪️tests/🔬️engine-contract/🟦️.ts` → `retained surface host — patches the session it already owns…` | `FlowGraphCanvasHost` over a **real** `createFlowBrowserRuntime` on `MockFlowBridge`, re-rendered with each refresh's status map and a **moving `controllerId`**: exactly one `attachSurface` (ABI op, read from `🕸️wasm/🧬️schema/📡️abi.json`), one `createFlowSession`, one canvas pair. |

### Regression proof (run, not asserted)

With `ContainerView`'s child run restored to `key={childId}` and everything else unchanged:

- the twin fails first, on its source anchor: `a child run keyed on the DFS-minted id is the defect this fixture exists for`;
- bypassing the twin, the React half fails: `refresh "first-eval-settle" replaced the canvas container's DOM element — the surface host was unmounted and rebuilt: … // Object.is equality`.

The fix was restored and everything re-run green.

### Gates

| command | result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-react:surface-retention-check` | **PASS** — `surface-host-retention-oracle: checks=27 clean`, then **9 vitest passed** (5 sibling-key cases + the child-id run + 2 retention laws + the host attach law) |
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **3 failed / 965 passed (968)** — all three pre-existing and **not this lane's** (§6) |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | **867 errors — identical count before and after this lane's edits**; zero at any line this lane touched. The only hit in a file I edited is the long-documented `🗣️Interpreter/🟦️.tsx:1571 import.meta.dir`. |
| `bun nx run @semio-tech/framework-renderer-react:lint` | **PASS** |

## 6. Reds that are not this lane's (attributed)

All three `test-long` failures are in files this lane never opened; both were modified by peers at
06:00 and 05:09, before this lane's first edit.

| test | why it is not mine |
|---|---|
| `noteShellCommand > buildNoteShellCommandAction …` | the builder now emits `inverseArgs` / `inverseCommandId` and the law was not updated with it. Owner: `🛠️ShellHelpers/🟦️.tsx` (the only file defining `inverseCommandId`), peer-modified 06:00. |
| `PluginRuntime > surface render ViewModel > binds two instances of one body …` | an extra `["window", "canvas-body"]` surface pair now projects. Owner: `🔌️PluginRuntime/🟦️.tsx`, peer-modified 05:09. |
| `PluginRuntime documentPack/transaction wire adapter > readAppDocumentPack() …` | the reply now carries an extra `ops: ""` field. Same owner, same edit window. |

The typecheck count (867) also sits above the 820 recorded on 2026-09-10; that delta belongs to the
concurrent lanes, and this lane added none of it — the count is byte-identical before and after every
edit I made.

## 7. Files changed

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json` — **new** language-neutral fixture
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — **new** `uiSiblingReactKeys` / `uiChildReactKeys`; `ContainerView` and `renderTreeItemControls` reconcile on them; test registration extended
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx` — **new** `🪪️ sibling reconciliation keys` and `🪪️ retained surface host across refreshes`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — **new** `retained surface host — patches the session it already owns …`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` — permanent `node-graph host unmount` lifecycle marker (the temporary timing brackets were removed)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts` — **new** `surfaceHostRetentionOracle` + `surface-retention-check`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json` — `surface-retention-check` target
- `/Users/ueli/Documents/semio/.vscode/launch.json` — `⚖️gate🪪️surface-retention🌐️shell`, in the `4_gate` group beside the sibling shell gates
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — instrumented and **reverted to its prior content**; no lasting change from this lane
- this report

Outputs (deletable with the ticket): `🗑️generated/surface-retention/{diag-1,diag-2,fix-1}/`,
`🗑️generated/surface-retention-1/`.

## 8. Next, for whoever picks this up

1. **The remaining wall clock is the guest's `flowEvalTick` settle**, 12–18 s per hop, and it is not a
   renderer problem: every extension answer is `turns: 1`, the native chain is 0.6 s, and
   `📓️dispatch-timer-throttle-2026-09-12.md` §3 already disproved the timer hypothesis. That is where
   the ticket's ≪30 s target has to come from.
2. **`window:procedural-main` reports `meshes: 0`** in `hosts.json` while the preview reports
   `meshesLen 3641` — the main window's host carries no `data-meshes-json`. Worth confirming that is
   intentional rather than a dropped attribute, before anyone reads a probe as "no meshes".
3. **Every other `<UiNodeView>` call site is now on authored keys, but nothing prevents a new one from
   reintroducing the id.** The twin pins `!interpreter.includes("<UiNodeView key={childId}")`; a
   broader guard (no `key={` on a raw `UiNodeId` anywhere in the interpreter) would be cheap.
4. **Authored keys are not universal.** Every node on the path to `procedural.play` has one, but a
   program that authors none still renumbers. If a surface ever appears under a keyless container, it
   will remount again — and the fixture's `keyless-falls-back-to-the-minted-id` case documents exactly
   that limit rather than hiding it.

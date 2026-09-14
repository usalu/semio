# React View-State u64 Carrier — lane `react-view-state-u64-carrier` (2026-09-14)

The React renderer did not run. Every guest turn after `setContributions` trapped, all 23 journey
steps failed with 0 meshes, and the page raised an uncaught `TypeError` on each mode switch. Two
independent defects, both closed at their owning layer, both with a law.

- Probes: `🐍️page-error-stack-probe.mjs` (new), `🐍️journey-probe.mjs`, `🐍️react-battery.mjs`
- Shared fixture: `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json`
- Runtime evidence: `🗑️generated/react-u64/`

---

## 1. TL;DR

**One defect made every guest dispatch on the React door fail; the other made every mode switch raise
an uncaught page error. Both are fixed, and the journey is 23/23.**

| | before (`🗑️generated/s4-journey-3`, 13:26–13:47) | after (`🗑️generated/react-u64/journey`) |
|---|---|---|
| journey steps converged | **0 / 23** | **23 / 23** |
| steps with geometry | 0 / 23 | 20 / 23 (the three `No example` rows are empty by contract) |
| `shard worker fault [handler/turn]` | **608** | **0** |
| `expected an exact u64 integer, found Float(1.0)` | on every turn | **0** |
| `invokeExtension dispatch failed` | yes | **0** |
| `typed-operation completion effects failed` | yes | **0** |
| uncaught page errors | 2 | **0** |
| wall clock | 1 324 s | **120 s** |

1. **The React host could not carry an integer to the guest.** The shell's own view state is built
   with JavaScript `number`s, and a JS `number` is an IEEE double and nothing else — `packEncodeValue`
   writes every one of them as `TAG_F64`. The guest decodes `ToolRunTraceCursor { run: u64, … }`
   through `FromValue`, whose unsigned arm refuses a `Float` **by design**, so the WHOLE view state
   failed to decode and took the dispatch with it. `toolRunTraceCursorByWindowId` is simply the FIRST
   integer-typed field a view context ever carried. **Fixed** — the crossing now mints the
   schema's integer fields as pack integer carriers, and the two renderer doors are pinned to the
   same bytes.
2. **`<Canvas>` connected its DOM events to a container the document no longer held.** r3f binds its
   handlers inside its own root's commit, which lands a turn after the shell has swapped the window's
   surfaces away on a mode or role switch. **Fixed** — this shell names the event source itself, and
   mounts the canvas only once that element exists.

---

## 2. Root cause, file:line

### 2.1 The integer carrier

| hop | site | what happens |
|---|---|---|
| Producer | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx:286` | `toolRunTraceCursorViewState` narrows the echoed cursor to `{ run: Number(cursor.run), generation, page }` — three plain JS numbers |
| Carriage | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4756` | the shell puts that map into `viewState.toolRunTraceCursorByWindowId` on every `refreshUi` |
| Loss | `🧰️framework/🛍️products/💻️os/🟦️.ts:1786` (`packEncodeValue`) | `typeof value === "number"` ⇒ `PACK_TAG_F64`. Correct for what a JS number IS; wrong for what this field MEANS |
| Crossing | `🧰️framework/🛍️products/💻️os/🟦️.ts` `AppChannelClient.command`, and `🔌️PluginRuntime/🟦️.tsx` `uiRefreshSurfaceEvents` (3 encodes) + `performContextMenu` | encoded the unprojected context |
| Consumer | `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:74` | the unsigned `FromValue` arm answers `expected an exact u64 integer, found Float(1.0)`; `ValueError::under` prefixes the path, giving the exact line the console showed |

Measured, not inferred: `🗑️generated/s4-journey-3/console.txt` carries 608 `shard 0 worker fault
[handler/turn] actor=procedural#1` frames whose byte-array `message` decodes to
`toolRunTraceCursorByWindowId.procedural-preview.run.expected an exact u64 integer, found Float(1.0)`
— for `🌀️procedural/🌉️bridge.js` and for `🧩️extension-modules/🧊️flow-extension-brep/🌉️bridge.js` alike.

**Why the wgpu door does not have this hole.** Its producer is Rust: `view_state_pack_base64`
(`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`) runs `to_dsl_value(&ViewModel)`, and `ToValue` for `u64`
answers `Number::UInt`. The wgpu lane's own defect this morning was the TRANSPORT (JSON between Rust
and its JS bridge, `📓️wgpu-end-to-end-verification-2026-09-14.md` §A); the React transport was
already pack, so on this door the hole is at the PRODUCER instead. Same class, different hop.

### 2.2 The null `addEventListener`

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` `WorldCanvas` — the shell's
one `<Canvas>` wrapper. r3f's own `onCreated` runs
`state.events.connect(eventSource ? … : divRef.current)`; with no `eventSource` declared it reaches
for its private inner div, and that div is already detached when a canvas is created into a surface
the shell is tearing down. Stack captured live (`🗑️generated/react-u64/page-errors/errors.json`):

```
TypeError: Cannot read properties of null (reading 'addEventListener')
    at Object.connect (…/deps/chunk-EB44J6O5.js:9976:18)
    at onCreated  (…/deps/chunk-EB44J6O5.js:10139:68)
```

and in the same console, 54 ms earlier `node-graph host unmount surface=window:procedural-main`,
500 ms later `THREE.WebGLRenderer: Context Lost`. It is a teardown race, one page error per switch —
4 of them across 6 switches in the baseline run.

---

## 3. The fix — one owning layer for integer carriers

**The decision: the view-context SCHEMA declares which fields are exact integers; the CROSSING mints
the carrier its own wire uses.** Neither half moves. In particular the guest's codec was NOT
loosened: `🌱️value/🔁️codec/🦀️.rs` refusing a `Float` in a `u64` slot is the property that makes
`pack` worth using, and an accept-integral-floats reader there would have widened every `u64` decode
in the codebase — document fields included — to hide one host bug.

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1025` | new `viewContextWithIntegerCarriers(view, mint)` — the ONE place the integer-typed view-context fields are written down, next to `parseResolvedPluginViewState` that already owns the schema. It takes the carrier mint as a parameter, so the schema module gains no dependency on any wire format. |
| `🧰️framework/🛍️products/💻️os/🟦️.ts:1908` | new `viewContextWireValue(view)` — the pack layer's binding of that declaration (`packUInt(BigInt(value))`), exported alongside `encodePackValue`. |
| `🧰️framework/🛍️products/💻️os/🟦️.ts:3818` | `AppChannelClient.command` encodes `viewContextWireValue(viewState)` — the one `view_state` slot every action/command turn rides. |
| `🔌️PluginRuntime/🟦️.tsx:1734,1742,1750,2983` | the three `surface-visible` refresh encodes (window / panel / section) and the context-menu request carry the same projection. |
| `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command/🟦️.ts` | the browser-actor `AppCommand::Command` handoff too — it is the same `view_state` slot on a second path, and leaving it lossy would have left the defect in the tree. |
| `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` `WorldCanvas` | the canvas is mounted into, and binds its events to, a wrapper element this shell owns (`eventSource={canvasEventSource}`); no canvas is created before that element exists. The wrapper holds the canvas and nothing else, so the event scope is exactly r3f's inner div's and `props.overlay` stays outside it. |

`admitCrossingViewContext` is untouched and still validates: the projection runs on an ALREADY
admitted context, and `parseResolvedPluginViewState` keeps reading plain JSON numbers, which is what
the language-neutral `🪟️view-context/🧬️schema/🔣️.json` declares.

---

## 4. Laws

One shared, language-agnostic fixture:
`🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json` — the exact
view context the failing console named (`toolRunTraceCursorByWindowId["procedural-preview"] =
{ run: 1, generation: 0, page: 0 }`), the integer paths inside it, the pack bytes as hex, the guest's
refusal text, and a non-integral float.

| half | file | law | result |
|---|---|---|---|
| guest (Rust) | `🛂️manifest/🧪️tests/🔢️integer-carriers/🦀️.rs` | the wgpu producer's `to_dsl_value(&ViewModel)` + `encode_wire_value` equals the fixture bytes; every integer path in those bytes decodes as `Number::UInt`; the bytes decode to `ToolRunTraceCursor { run: 1, generation: 0, page: 0 }`; a whole `Float(1.0)` in the `run` slot is refused with the fixture's exact message, and `1.5` is refused too | **3 passed** |
| React door (TS) | `📺️renderer/🧑‍🎨engine/🧪️tests/📌️view-state-carriage/🟦️.ts` | the admitted context still admits; every integer path of `viewContextWireValue` is a minted `PackInteger`; `encodePackValue(viewContextWireValue(view))` hex **equals the fixture bytes the Rust door produces**; `fails-before:` the unprojected context does NOT, and its `run` decodes as a plain number; a non-integral run refuses at admission AND at the mint, never rounds | **7 passed** (4 new + 3 pre-existing in that file) |

Both doors therefore hand the guest the SAME 137 bytes for the same view context, and that is
asserted against the guest's own decode rather than against each other.

Run them:

- `bun nx run workspace:test-view-context-integer-carriers` (both halves)
- `bun nx run workspace:test-view-context-integer-carriers-native` (Rust only)
- `bun nx run @semio-tech/framework-renderer-react:view-state-carriage-check` (TS only)

registered in `📜️script.ts` (`verify view-context-integer-carriers`), `📋️project.json` and
`.vscode/launch.json` (`🧪️test🔢️view-context-integer-carriers`, `…🦀️native`).

---

## 5. Runtime proof on 6018

### 5.1 The page error, before and after

`🐍️page-error-stack-probe.mjs` — boot, then six mode/role switches, recording every uncaught error
WITH its stack (the s4 console had no stacks, which is why the owner was unknown).

| run | page errors |
|---|---|
| `🗑️generated/react-u64/page-errors` (before the `WorldCanvas` fix) | **4** — one per canvas-bearing switch, each the `connect`/`onCreated` stack above |
| `🗑️generated/react-u64/page-errors-2` (after) | **0** across the identical six switches |

### 5.2 The journey

`cd T && SEMIO_PROBE_OUT=react-u64/journey bun 🐍️journey-probe.mjs` —
**23/23 converged, 120 s total, 0 page errors, 0 worker faults, 0 `expected an exact` lines**
in 975 console lines.

| step | s | meshes | | step | s | meshes |
|---|---|---|---|---|---|---|
| boot | 11 | 3 | | generate-mode | 3 | 0 |
| edit:No example | 3 | 0 | | generate-added | 3 | 1 |
| edit:Hexagonal Mushroom Column | 4 | **3** | | back-to-edit | 3 | 1 |
| edit:Rectangle Extrude Volume | 9 | 1 | | viewer-role | 5 | 1 |
| edit:Sphere Cut With Torus | 8 | 1 | | view:No example | 3 | 0 |
| edit:Box Fillet Preview | 7 | 1 | | view:Hexagonal Mushroom Column | 4 | **3** |
| edit:Sphere Box Fuse | 8 | 1 | | view:Rectangle Extrude Volume | 4 | 1 |
| edit:Face Sweep Extrude | 10 | 1 | | view:Sphere Cut With Torus | 4 | 1 |
| edit:Rectangle Wire Preview | 4 | 1 | | view:Box Fillet Preview | 4 | 1 |
| edit:Box Shell Preview | 6 | 1 | | view:Sphere Box Fuse | 4 | 1 |
| | | | | view:Face Sweep Extrude | 4 | 1 |
| | | | | view:Rectangle Wire Preview | 3 | 1 |
| | | | | view:Box Shell Preview | 4 | 1 |

Hex is 3 meshes in both lanes, every other example is 1, both `No example` rows are 0 — the required
shape exactly. Every edit step also matched the picked example's authored widget ids, which is the
journey's graph oracle, not merely its status map.

### 5.3 The battery

_(§5.3 filled after the restage — see below.)_

---

## 6. A live peer's stale vite transform, and what it cost

Before any of the above could be measured, the React page was dead on a DIFFERENT cause: the first
probe run answered

```
pageerror The requested module '/@fs/…/🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts'
          does not provide an export named 'toolRunPanelNewRuns'
```

The export exists on disk (`⏯️tool-run/🟦️.ts:1134`, written 13:54 by the live `previewEval → toolRun*`
peer). Fetching the module straight from the dev server showed the served transform contained **zero**
occurrences of that symbol — vite was serving a cached transform older than the file. A content-neutral
`touch` of the peer's file invalidated it and the next fetch carried the export. **No server was
recycled and no peer hunk was reverted.** Worth knowing for the coordinator: a peer's edit can leave
6018 serving a stale module, and a `touch` is enough — the page is not broken, its transform is.

This is also why the 13:30 `s4-journey-3` numbers and this lane's numbers are comparable: that
journey ran BEFORE 13:54, on the same staged guest this lane's journey ran on.

---

## 7. Files

Changed:

- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — `viewContextWithIntegerCarriers`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — mounts the new Rust law
- `🧰️framework/🛍️products/💻️os/🟦️.ts` — `viewContextWireValue`; `AppChannelClient.command`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the four crossings
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command/🟦️.ts` — the browser-actor `view_state` slot
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` — `WorldCanvas` event source + mount latch
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📌️view-state-carriage/🟦️.ts` — the TS twin laws
- `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` — `verify view-context-integer-carriers`

Added:

- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔢️integer-carriers/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️page-error-stack-probe.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/📓️react-view-state-u64-carrier-2026-09-14.md` (this file)

Runtime artifacts under `🗑️generated/react-u64/`: `journey/`, `page-errors/`, `page-errors-2/`,
`restage.txt`.

---

## 8. What is NOT claimed

- **No claim that the wgpu door changed.** It was already correct after its own lane's §A; this lane
  only PINNED it, by asserting the Rust producer's bytes against the same fixture the React door now
  matches. No wgpu file was touched.
- **No claim about `run` ids beyond 2^53.** `toolRunTraceCursorViewState` still declines to echo a
  cursor whose `run` exceeds `Number.MAX_SAFE_INTEGER` (the guest then resends that layer from page 0),
  because the host's own cursor arrives as a JS number long before this crossing. The carrier is exact
  for every run id the host can represent; making the host hold `bigint` end to end is a separate,
  larger change and is not in this lane.
- **No claim that the repo typechecks.** `@semio-tech/framework-renderer-react:typecheck` reports 792
  errors, none of them at any line this lane touched (they are `🦑️repo/…/🧹️normalization/🟦️.ts`,
  `import.meta.dir`, and plugin test files). That target was red before this lane and is red after it.
- **No claim about the six reds the React battery already carried**, or about anything the battery
  reports beyond §5.3's own numbers.
- **No claim that the guest's `previewEval → toolRun*` migration is finished.** It is a live peer's
  work; this lane neither reverted nor completed any of it, and read every file it touched fresh.

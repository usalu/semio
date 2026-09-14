# 🚪️ wgpu IO — `Export Document…` and `Import Document…`, made real for the user (2026-09-14)

Lane `wgpu-io-effects` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Repo/semio MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket opened/closed/reopened by this lane, no `📓️status.md` or
`🎫️ticket.json` edit, no modifying git command, no dev server started or stopped.

## TL;DR

`📓️wgpu-end-to-end-verification-2026-09-14.md` §C: on wgpu both io verbs reached the guest and the
guest RAN them, and neither reached the user — the export effect was stashed as "leftover" and the
import effect was dropped as `unmapped effect "request-file-open"`. Both journeys are now real on the
wgpu host, proven in the browser on 6118:

- **Export**: `mod+shift+e` ×2 → a real browser download. Measured live:
  `generation3d.stl`, **3 810 B**, first bytes `73 6f 6c 69 64 20 67 65` = `solid ge…`, decoding to
  `solid generation3d-preview / facet normal 0 0 -1 / vertex 0.5 0 0 …` — the hexagonal mushroom
  column's **evaluated** preview as genuine ASCII STL, byte-count-identical to what React's own
  post-fix export produced (`📓️download-media-export-encoding-2026-09-13.md` §6).
- **Import**: `mod+o` → a real `<input type="file">` with the accept filter this artifact derives from
  its own formats (`.stl,.obj,.ply,.gltf,.dwg,.json,.txt`); answering it with a real `.stl` replaced
  the graph with `imported-source` / `imported-geometry` / `imported-preview` — the same three-widget
  import fixture React produces.
- **Four defects**, each fixed at its owning layer, none of them the one §C had named (§1).
- **Laws**: 8 (kernel, Rust) + 8 (wgpu shell, Rust) + 1 TS twin driving 5 wire rows / 6 chunk rows /
  3 argument rows + 1 new renderer-bridge law over an integer action argument. All foreground.
- `cd T && bun 🐍️wgpu-battery.mjs --only=io` → **6/6 steps green, 95 s, 0 page errors**, and three of
  those six steps are new: they assert the USER-FACING outcome, not the effect crossing.

---

## 1. What was actually broken — four defects, in the order a press meets them

§C named two host-side holes. Both were real; neither was sufficient, and the two that mattered most
were upstream and downstream of them.

| # | Layer | Defect | Effect on the user |
|---|---|---|---|
| **A** | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `dispatch_action` | a SECOND, private `requested_effects` fold ending in `_ => {}` | every effect an ACTION produced and this fold did not name was dropped **without a trace** — `DownloadMediaExport` included |
| **B** | `🎭️actor/🖼️wire-turn/🟦️.ts` `wireEffectToFriendly` | no `request-file-open` case | `Import Document…` printed `unmapped effect "request-file-open" dropped` and stopped |
| **C** | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` both browser halves | `web_sys::window()` → `None` **inside the frame Worker** | the download and the picker were silent no-ops on the only isolate this shell ever runs in |
| **D** | `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` `handle_action_js` | the invocation crossed the JS bridge as JSON | an import chunk's `chunk: u32` reached the guest as `Float(0.0)` and `FromValue` refuses it |

### 1.A The action path had its own effect fold, and it was silent

`dispatch_command` (`🦀️.rs:5751`) carries an explicit note that its arm "used to be a SECOND, shorter
hand-rolled fold, and every effect it did not name was dropped without a trace". That was fixed for
COMMANDS. `dispatch_action` still had exactly that shape at `🦀️.rs:5626-5699`: `SetActiveUtility`,
`Navigate`, `LoadDocument`, `DispatchAction`, `RequestMediaFrames`, `ReplayShellCommand`,
`RequestInferenceProposal` — and then `_ => {}`.

`exportDocument` is dispatched as an ACTION on this target (measured: `wgpu-shell key routing …
action=Char("E")` followed by `plugin_exchange actionId=exportDocument`, and **no**
`[DEBUG] wgpu-shell command exportDocument settled` line, which `dispatch_command` logs
unconditionally). So the guest's `DownloadMediaExport` came back through the bridge as
`requested_effects`, reached this fold, and hit `_ => {}` — not even
`queue_host_effects`' own `[DEBUG] wgpu-shell effect dropped` line, which is why §C read this as a
bridge-side stash rather than a shell-side drop. `InvokeExtension` and `SetPanel` were in the same
hole on the action path.

**Fix**: `dispatch_action` now folds through the SAME funnel `dispatch_command` does — only the two
effects `queue_host_effects` cannot own (a `Navigate` that must also RESOLVE the uri, and the native
replay relay, both `async` where the funnel is a plain `fn`) stay local; everything else is
`queue_host_effects(&action.controller_id, queued)`.

### 1.B The shared wire decoder had no case for the import effect

`wireEffectToFriendly` is the ONE wire→friendly decoder both renderer doors call (the module's own
header calls out the "third divergent copy" hazard). It had `download-media-export` (added by the
encoding lane) and no `request-file-open` at all, so the import effect hit the default arm.

**Fix**: `case "request-file-open"` reading the W0 nested `params` record, unwrapping the WIT
`option<string>` in BOTH shapes it really arrives in, and omitting `readAs` rather than carrying it as
`undefined` (the friendly `Effect` union declares it optional, and a shell reads absent as "hand me the
file's own text"). Five fixture rows drive it, including the flat-`params` row that proves a door
reading it flat gets an empty `importAction` and re-dispatches nothing.

### 1.C — the one §C could not see: **the wgpu shell has no `document`**

This is the defect that made the other two look sufficient when they were not. After A and B were
fixed the effects reached `queue_host_effects` and… nothing happened: no anchor click, no picker, no
error. Measured with an in-page witness that wraps `HTMLAnchorElement.prototype.click`,
`HTMLInputElement.prototype.click` and `URL.createObjectURL`
(`🗑️generated/wgpu-io/run-2/results.json`): `anchors: 0, inputs: 0, objectUrls: 0`.

The wgpu shell runs **inside the dedicated frame Worker** — the one that owns the `OffscreenCanvas`
(`🎞️frame-worker/🟦️.ts` mounts the plugin handles and calls `semioWgpuWorkerBootstrap`). A Worker has
no `window` and no `document`, and BOTH browser halves began with exactly that:

```rust
let window = match web_sys::window() { Some(window) => window, None => return };   // download
let Some(document) = web_sys::window().and_then(|w| w.document()) else { return }; // picker
```

and the picker's own stub carried the comment *"the browser shell handles `RequestFileOpen` itself
(see `framework/renderer/react/index.tsx`)"* — no browser shell does: on this renderer **the wasm IS
the shell**.

**Fix, at the owning layer**: the page half is a new module, `🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts`, the
ONE place a `<a download>` and an `<input type="file">` are created for this target. The shell always
calls one binding, `globalThis.semioWgpuHostIo`; the environment decides what that binding is:

| isolate | install |
|---|---|
| page (main-thread mount) | `installWgpuPageHostIo()` in `🎬️renderer-boot/🟦️.ts` — the implementation directly |
| frame Worker (the live variant) | `🎞️frame-worker/🟦️.ts` installs a `postMessage` bridge; `🚚️browser-frame-transport/🟦️.ts` runs the SAME page implementation on the UI isolate and answers by `requestId` |

The Rust shell has no idea which isolate it is in, so neither journey can work on one variant and
vanish on the other. The download's bytes are decided in Rust by `kernel::media_export_bytes` (the
contract both renderers already answer to) and **transferred**, never re-encoded, so the page receives
the file's real bytes and nothing about the `(data, encoding) → bytes` rule moves.

### 1.D The invocation crossed the JS bridge as JSON, so its integers died

§A of the verification report closed this for the VIEW STATE and left the invocation half open:
`handle_action_js` handed the action JSON to JS as a string, `JSON.parse` collapsed `0` and `0.0` onto
one JS `number`, and `packEncodeValue` wrote every one of them as `PACK_TAG_F64`.

An import chunk envelope is `{payload, name, chunk: u32, chunkCount: u32}` and `FromValue`'s unsigned
arm refuses a `Float` by design (`🌱️value/🔁️codec/🦀️.rs:74`) — so **`Import Document…` could not have
landed a single chunk on this target no matter how the picker behaved.** This is not a hypothetical
about the import alone: every integer action argument on wgpu crossed this seam as a float.

**Fix**: the invocation crosses as PACK too, exactly as the view state now does. The repo's own JSON
reader is exact about integrality where `JSON.parse` is not (`Number::UInt` for `0`, `Number::Float`
for `0.0`), so `invocation_pack_base64` re-reads the invocation text and hands pack onward; the TS
bridge decodes it with `packValueFromBase64` and `encodePackValue` re-encodes the carriers unchanged.

---

## 2. The chunking contract, moved to the layer that owns the effect

The two renderers disagreed about what ONE import invocation carries:

| | React | wgpu (as shipped) |
|---|---|---|
| args | `{payload, name, chunk, chunkCount}` (+`{index,total}` when `multiple`) | `{json, payload}` |
| chunking | `importPayloadChunks`, half the guest contiguous ceiling | none — the whole file in one invocation |
| owner | `🛠️ShellHelpers/🟦️.tsx`, reachable only by the React shell | a mutation-payload reader (§3) |

A plugin could satisfy exactly one of them, and an unchunked import asks the fixed guest heap for a
contiguous block the size of the whole file — several times its own per-request ceiling.

So the rule moved to the effect contract itself, beside `Effect::RequestFileOpen`, the way
`media_export_bytes` moved beside `Effect::DownloadMediaExport`:

- `kernel::IMPORT_CHUNK_BYTES` — derived from `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2`, never a literal;
- `kernel::ImportChunk` / `import_payload_chunks` — sliced by UTF-8 EXTENT, never by code units;
- `kernel::import_chunk_arguments` — the argument names AND the integer carriers
  (`Number::UInt`, never a float);
- TS twins `IMPORT_CHUNK_BYTES` / `importPayloadChunks` / `importChunkArguments` in `🎠️kernel/🟦️.ts`;
- `🛠️ShellHelpers/🟦️.tsx` re-exports them rather than re-declaring them, and `dispatchOpenedFiles`
  builds its args through `importChunkArguments`.

Both shells now dispatch byte-for-byte the same envelope, and the one number they still disagree with
the guest about (`📓️io-surface-2026-09-13.md` §7.4, the 32 KiB host slice vs the guest's own
`PUBLIC_INVOCATION_STRING_BYTES`) is now stated in exactly one place instead of two (§7).

---

## 3. Two CRUD-era mutation readers, deleted

The wgpu shell's ONLY reader for either journey was a pair of arms in `apply_ops_inner` matching
document MUTATIONS named `"requestFileOpen"` and `"downloadMediaExport"`. `grep -rn '"requestFileOpen"'`
over the whole repo answers this shell and nothing else — no plugin, no reactor, no lowering emits such
a mutation. They were dead on arrival, which is precisely why both journeys looked wired and reached
nobody. Both are deleted (with them, the `follow_up_operations` re-entry they were the only producer of,
and `patch_ops_from_action_result`, whose last caller they were); `queue_host_effects` is the one door.

---

## 4. Files

**Changed — the shared contract:**

| File | What |
|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | **new** `📤️FileOpenImport` region: `IMPORT_CHUNK_BYTES`, `IMPORT_ARGUMENT_*`, `ImportChunk`, `import_payload_chunks`, `import_chunk_arguments`, test mount |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | TS twins of all of the above |
| `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📤️file-open-import/🔣️.json` | **the language-agnostic law** — 5 wire rows, 6 chunk rows, 3 argument rows |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🦀️.rs` | Rust laws (**8**) |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🟦️.ts` | TypeScript twin, with `Buffer.byteLength`/`TextEncoder` as the independent byte oracle |
| `🧰️framework/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-trace` (zero-dependency, workspace-internal) so the chunk extent is derived |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | the missing `request-file-open` case |

**Changed — the wgpu host:**

| File | What |
|---|---|
| `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | §1.A one funnel on the action path; `Effect::RequestFileOpen` arm; `PendingFileOpen` + `run_file_open_request`; `OpenedFile`/`file_open_import_actions`; both browser halves through `host_io_call`; §3 deletions; test mount |
| `…/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | §1.D `invocation_pack_base64`; `handle_action_js`/`handle_command_js` send pack |
| `…/🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts` | **new** — the page half: `<a download>`, `<input type="file">`, `createWgpuPageHostIo`, `installWgpuPageHostIo` |
| `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | **new** `🚪️HostIo` region: installs `semioWgpuHostIo` as a `postMessage` bridge |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `host-io` / `host-io-result` message pair + the `hostIo` option and `answerHostIo` |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | passes `hostIo: createWgpuPageHostIo()` |
| `…/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts` | `installWgpuPageHostIo()` for the main-thread variant |
| `…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` | `handleAction`/`handleCommand` take a decoded invocation; the bridge decodes `packValueFromBase64` |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-io-base64` (the first-party codec the native picker encodes with) |
| `…/🧱️elements/🐚️Shell/🧪️tests/📤️wgpu-file-open-import/🦀️.rs` | **new** wgpu laws (**8**) |
| `…/🧪️tests/🧩️package-integration/🟦️.ts` | the invocation-pack law: an integer action argument crosses the bridge as an integer |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | re-exports the kernel contract; `dispatchOpenedFiles` builds args through it |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `…/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | `🚪️host-io` registered (memberNames, sourceModulePaths, inputPatterns, frameWorkerSources) |
| `📜️script.ts`, `📋️project.json` | `file-open-import` + `-native` lane |

**Ticket:**

- `📓️wgpu-io-effects-2026-09-14.md` (this report)
- `🐍️wgpu-io-effects-probe.mjs` (**new**) — the before/after recon that found §1.C
- `🐍️wgpu-io-probe.mjs` — the user-facing witness folded into the battery's own io probe
- `🐍️wgpu-battery.mjs` — three new io steps (§6)
- `🗑️generated/wgpu-io/` — `recon-1` (as shipped), `run-2` (after §1.A+B, before §1.C), `run-3` (after),
  `battery-io/` (the battery verdict, its `results.json` and the downloaded `generation3d.stl`)

---

## 5. Laws (all run in the foreground)

| Lane | Result |
|---|---|
| `bun ./📜️script.ts verify file-open-import` | green — `wire=5 chunks=6 arguments=3 extent=32768 oracle=Buffer.byteLength`, plus `tsc --noEmit --strict` on the page half |
| `cargo test -p semio-framework --lib file_open_import` | **8 passed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib file_open_import` | **8 passed** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | green in 2m41s, **warnings emitted** (proof it really type-checked — the browser-gated arms compile on no other target) |
| `nx run @semio-tech/framework-renderer-wgpu:test-preview-generated` | **22 passed** (incl. the new invocation-pack law and the two pre-existing view-state ones) |
| react engine suites `🔬️engine-contract` + `🧩️package-integration` + `🛠️ShellHelpers` (`-t dispatchOpenedFiles\|importPayloadChunks\|download`) | **15 passed** |
| `📨️browser-frame-transport` + `📥️wgpu-intake-budget` | **56 passed** |
| actor vitest, `🖼️wire-turn` suite | green (see §7 for the suite's unrelated pre-existing reds) |
| `bun ./📜️script.ts verify media-export-encoding` | still green — `cases=8 refusals=5 binary=4 textual=3 oracles=Buffer,atob` |

**Third-party oracles.** Rust: `std`'s own UTF-8 boundary discipline — every chunk boundary of every
fixture payload must be a character boundary of the ORIGINAL string, which a UTF-16 slice fails on the
2-byte and 4-byte rows. TypeScript: Node's `Buffer.byteLength` and `TextEncoder`/`TextDecoder(fatal)`,
neither of which shares a line with the slicer's own per-character width arithmetic.

**The law that would have caught §1.D** is `every_chunk_dispatches_its_declared_arguments`: it drives
`import_chunk_arguments` through the guest's OWN `u32` decode and fails on a float, which is what a
JSON hop produces.

---

## 6. Runtime proof on 6118

Renderer wasm + frame worker + browser boot rebuilt from this lane's source
(`@semio-tech/framework-renderer-wgpu:wasm` dist 10:31, `:generate-frame-worker` + `:generate-browser-boot`
10:33); the coordinator's serve was never restarted.

### 6.1 The three runs, same probe, same chords

| | `recon-1` (as shipped, 09:18) | `run-2` (§1.A+§1.B, 10:05) | `run-3` (all four, 10:36) |
|---|---|---|---|
| `unmapped effect "request-file-open"` | **2** | 0 | 0 |
| export effect | `leftover 1 tags=downloadMediaExport` | same | same, then delivered |
| `<a download>` clicks witnessed | **0** | **0** | **1** |
| blob bytes behind it | — | — | **3 810 B**, `73 6f 6c 69 64 20 67 65` |
| Playwright `download` events | 0 | 0 | **1** |
| `<input type=file>` clicks witnessed | **0** | **0** | **1**, `accept=.stl,.obj,.ply,.gltf,.dwg,.json,.txt` |
| file choosers | 0 | 0 | **1** |
| graph after the pick | unchanged | unchanged | `imported-source`, `imported-geometry`, `imported-preview` |

The console hops of `run-3`, in order:

```
88071  [DEBUG] wgpu-host-io download name=generation3d.stl type=model/stl bytes=3810
88072  [DEBUG] wgpu-shell download presented name=generation3d.stl bytes=3810
121709 [DEBUG] plugin_exchange actionId=importDocumentRequest branch=catalog
122147 [DEBUG] wgpu-host-io file-open accept=.stl,.obj,.ply,.gltf,.dwg,.json,.txt files=1 bytes=209
122148 [DEBUG] wgpu-shell file open … action=importDocument files=1 bytes=209
122148 [DEBUG] wgpu-shell file open chunks=1 action=importDocument
122195 [DEBUG] plugin_exchange actionId=importDocument branch=catalog
125391 wgpu node-graph geometry surface=procedural-main entities=[… "imported-source" … "imported-geometry" … "imported-preview" …]
```

and the saved file on disk (`🗑️generated/wgpu-io/battery-io/generation3d.stl`, 3 810 B):

```
00000000: 736f 6c69 6420 6765 6e65 7261 7469 6f6e  solid generation
00000010: 3364 2d70 7265 7669 6577 0a20 2066 6163  3d-preview.  fac
00000020: 6574 206e 6f72 6d61 6c20 3020 3020 2d31  et normal 0 0 -1
```

### 6.2 The battery lane, with the user-facing outcome asserted

`cd T && bun 🐍️wgpu-battery.mjs --only=io` — **6/6, 95 s, 0 page errors**
(`🗑️generated/wgpu-io/battery-io/verdict.txt`):

```
✓ mod+shift+e reaches the guest as exportDocument
✓ mod+o reaches the guest as importDocumentRequest
✓ no dispatch failed on either verb
✓ the export hands the user real bytes   {"anchors":[{"name":"generation3d.stl","type":"model/stl","bytes":3810,"hex":"73 6f 6c 69 64 20 67 65"}],"downloads":1}
✓ the import opens a real file picker    {"inputs":[{"accept":".stl,.obj,.ply,.gltf,.dwg,.json,.txt","multiple":false}],"choosers":1}
✓ the picked file REPLACES the graph     {"importedNodes":["imported-source","imported-geometry","imported-preview"],"graphChanged":true}
```

The first three steps are the lane's OLD verdict. They were green in the verification battery too,
while neither journey reached a person — which is the whole reason the last three exist.

---

## 7. What is NOT claimed

- **The 32 KiB host slice still exceeds the guest's own chunk cap.** `IMPORT_CHUNK_BYTES` is half the
  guest contiguous ceiling (32 768 B) while `GENERATION3D_IMPORT_CHUNK_BYTES` is
  `PUBLIC_INVOCATION_STRING_BYTES`. That is `📓️io-surface-2026-09-13.md` §7.4 and stays open — this
  lane made it ONE number in one place instead of two, and did not change either value. Every file
  driven here (the probe's 209-byte STL, the fixtures) fits one chunk, so a multi-chunk import has
  **not** been driven end to end in a browser on either renderer.
- **`multiple: true` was never driven in a browser.** The fan-out is covered by the fixture's own
  argument row and by two Rust laws; no live multi-file pick was performed.
- **Only `stl` was exported live on wgpu.** The artifact's roster offers seven formats; `dwg` (the
  binary lane) and `txt` were proven on React by the encoding lane, not here.
- **The main-thread mount variant was not driven.** `installWgpuPageHostIo()` in `🎬️renderer-boot` is
  covered by the shared binding and by `tsc --strict`; 6118 boots the Worker variant, which is what ran.
- **A cancelled picker was not driven live.** Both halves settle on the browser's `cancel` event and a
  Rust law covers "a cancelled pick dispatches nothing", but no live dismissal was performed.
- **Nothing here touches §D** (the drained hit registry) or any other red in the verification battery.
  The five reds that lane left are unchanged and belong to their own lanes.
- **The shared `🗑️generated/wgpu-verify/scoreboard.json` is merged across lanes** and was rewritten by
  another lane at 10:46 carrying the OLD 3-step io row. This lane's own 6-step row is snapshotted at
  `🗑️generated/wgpu-io/battery-io/scoreboard-io.json`.
- **15 pre-existing reds in the actor vitest suite** (`📤️return/*`, `🚪️lifetime/*`,
  `🪪️activation/*`, `📮️shard-client`) are an ajv `$ref` resolution error and worker-activation
  timing, all in files this lane did not touch; `🖼️wire-turn`, the one actor file it did touch, passes.
  `📓️download-media-export-encoding-2026-09-13.md` §7 already recorded four of them.
- **`verify dependencies literal-external` is red repo-wide** (`current=198`). Both dependencies this
  lane added (`semio-framework-trace`, `semio-framework-io-base64`) are workspace-internal `path` deps
  and by construction do not count as external; the number was not measured before and after.
- **The renderer wasm was not the only build in flight.** A peer lane rebuilt
  `@semio-tech/framework-renderer-wgpu:wasm` at 10:04 and again around 10:28 while this lane ran; every
  such build compiles the shared tree, so `run-3` and the battery ran on this lane's source either way.

## 8. Fix-forward on peers' work

One, minimal, and already superseded: `✏️s/🔌️plugins/🧩️puzzle/…/🛠️tools/🪣️fill/🦀️.rs` (puzzle-2d)
failed to compile with `missing field settings in initializer of ToolRunDefinition` and blocked every
build of the wgpu renderer. This lane added `settings: ToolRunSettingsReads::default()` to get moving;
the owning peer has since replaced it with the real `ToolRunSettingsReads { config: vec!["/fillCount"] }`.
Its sibling `semio-s-artifact-puzzle-3d` broke twice the same way during this lane and was left to poll,
not patched.

## 9. Follow-ups

1. **`📓️io-surface-2026-09-13.md` §7.4** — reconcile `IMPORT_CHUNK_BYTES` with the guest's own
   `PUBLIC_INVOCATION_STRING_BYTES`, now that both hosts read one number.
2. **The wgpu shell's other `web_sys::window()` callers** are Worker-blind for the same reason §1.C
   was: `prefs_get`/`prefs_set` read `localStorage` through `window`, and the platform sniff at
   `🦀️.rs:10124` reads `navigator.platform` the same way. Neither was touched here; both silently
   degrade inside the frame Worker.
3. **`semio-framework-os-renderer-wgpu` still carries `base64 = "0.22.1"`** as a runtime dependency for
   `decode_data_url`. `request_file_open` no longer uses it; `semio-framework-io-base64` is now a
   dependency of the crate and can replace the last call site
   (`📓️download-media-export-encoding-2026-09-13.md` §9 follow-up 3).
4. **`requestFileSave` in `apply_ops_inner`** is the third CRUD-era mutation reader in that function,
   and a repo-wide grep answers this shell and nothing else — the same shape as the two deleted here.
   It was deliberately not touched: it is native-only and dispatches a `bindSpaceFile` action, so
   retiring it belongs with whichever lane owns the space-file binding.

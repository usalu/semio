# Window bodies, scoped contributions, 64-page ceiling — 2026-09-10

Lane: ShellHost contributions push (`refreshUi` first paint + one `handleCommand`).
Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Serve: `http://127.0.0.1:6018/?plugin=generation3d` (coordinator-owned; **not** restarted; pid 71097 still listening after this work).

## End state (this turn)

| Question | Result |
|---|---|
| Both ingress ceilings back at 64? | **Yes.** `SHARD_COMMAND_MAXIMUM_PAGES` = **64**. `COMMAND_MAXIMUM_PAGES` = **64**. |
| Scoped payload size / page count | **No `setContributions` was sent.** The host refused to install. There is no scoped `chars` / ingress page count to quote. |
| `brep` contributed + mesh in preview? | **No.** `invokeExtension` = **0**. Probe meshes = **0**. |
| `rust_oom` inside a full 260 s settle? | **Did not occur.** Playwright hold kept the URL open **261 106 ms**. Console lines: **4345**. Hits for `rust_oom` / `process::abort` / `unreachable`: **0**. This is not the 9.9 s bodies-2 window. |

The browser-probe still exits as soon as window bodies exist (`waitSettle` after 6 s). bodies-3 settled at **60.0 s**, bodies-4 at **9.3 s**. Those runs cannot speak to the 116 s OOM. The 261 s hold can.

## 1. First paint — stands

`refreshUi` still dispatches `SET_WINDOW_UI_BY_WINDOW_ID` with `pendingWindowUiNode()` before any await, then real UI before the contributions push. The empty-`windowUi` `return []` guard is gone.

bodies-3 / bodies-4: tabbed chrome, **3 canvases**, `framework.window.proceduralMain` + `proceduralPreview` bodies present. Probe exit `meshes=0`.

## 2. One `handleCommand` — stands

The 99-page 4 KiB JSON-envelope loop stays deleted. The live path is still one pack-encoded `handleCommand` (`encoding: "pack"`, `page: 0, pageCount: 1`, `crossings: 1`) when a payload is actually sent. window-fault source contract still asserts that.

## 3. Ceiling raise reversed

Raising 64 to 128 to admit a 397 921-char closure was the wrong direction:

1. It doubles the permitted burst into a guest that already dies of `dlmalloc` retention (boot-13 section 6: 322 MB at 65.7 s, `rust_oom` at 116.1 s).
2. The payload stays O(number of plugins). The fourteenth plugin breaks a raised cap again.
3. A 256 KiB envelope is the same mistake as a 4 KiB one.

bodies-2 showed the host/guest mismatch that create: `command ingress exceeds 64 pages` then 43 `v102_1` faults. **"No rust_oom in 9.9 s" is not evidence about the OOM.** That probe died at 9.9 s; the abort is at 116 s.

## 4. Scope from the open document, not the BuiltNode

Helpers kept and used: `reachableKindsFromUnknown`, `scopeContributionsJson`, plus `documentFlowGraphPresent` / `resolveDocumentOperatorKinds` in kernel.

Input is the open document, not the cached BuiltNode:

- `readAppDocumentPack` (live `ReadDocument`) so the host sees `ops` (the cache `{pack,spr}` drops it).
- `documentSourcesFromPack` walks `decodePackValue(pack)` when that works, then UTF-8 of pack/spr, then `ops`.
- Empty scoped JSON (`"[]"`) still refuses (`refused empty pack`).
- Unresolved (no pack / missing `ops` / no graph and no kinds) skips, does not mark `contributionsJsonRef`, does not restore the 13-plugin closure.
- Resolved empty graph (widgets present, kinds empty) would install receiver-only. That is not the same path as unresolved.

window-fault: **14 passed / 14** (empty-vs-unresolved + pack-crossing + refuse `[]`).

Procedural's manifest still has no brep dependency. The graph is the only honest source.

### 4.1 What ReadDocument actually returned (measured)

After `setActiveExample` and for the whole 261 s hold, `ReadDocument` stayed genesis:

- packBytes: 873
- sprBytes: 280
- opsChars: 92
- opsHead: `doc "s.procedural.generation3d@1/*#editor" schema=generation.3d` then `cursor applied=[ ] redo=[ ]`
- status: unresolved
- reason: no-operator-graph
- kinds: []

`print_document_pack` encodes initial_snapshot + ops log, not the live projection. `setActiveExample` emits fixture mutations but those edits never appear in the envelope `ReadDocument` prints (empty `applied=[]` for the entire hold). The host therefore cannot see `brep.curve.polygon` / `math.vector` / `brep.solid.extrude` from the document API.

The loud skip is the correct path: installing `"[]"` would wipe contributions; pushing the 397 921-char closure would re-open the 64-page / OOM problem.

A 250 ms re-read when `ops` has no widget/neuron text did not change the bytes.

## 5. Recommended end state (not built)

Push-based contributions are O(what exists) no matter how well the host scopes. The design that stays O(what is used) is demand-driven: the guest asks the host for a kind's contribution the first time it encounters one it does not know. Document-graph scoping gets a working app once the live projection is readable; pull-on-demand is what stops the next plugin from forcing another ceiling raise.

Until `ReadDocument` (or a twin that prints the current snapshot / DSL) carries the post-`setActiveExample` graph, host-side scoping cannot feed `brep` without going back to the full closure.

## 6. Verification (real numbers)

From the ticket folder. Probe not edited.

bodies-3: `--label=bodies-3 --settle=260 --timeout=420`
bodies-4: `--label=bodies-4 --settle=260 --timeout=420`
hold: `bun hold-260.mjs` against `generated/hold-260`

| run | wall | windows / canvases / meshes | contributions | rust_oom |
|---|---|---|---|---|
| bodies-3 | settled 60.0 s (bodies gate) | 10 window ids, 3 canvases, meshes=0 | 1 skip `no-operator-graph` | not observed (exited) |
| bodies-4 | settled 9.3 s | same, meshes=0 | genesis ReadDocument (873/280/92) then skip | not observed (exited) |
| hold-260 | 261.1 s | no DOM census | 2 skips, same genesis bytes both times; invokeExtension 0 | no |

Restage: `nx run @semio-tech/procedural-plugin:component-dev` — pass, 1 m 4 s, `Finished wasm-dev` in 55.07 s. 6018 was not restarted.

## 7. Tests

- Kernel in-source `scopeContributionsJson` + `resolveDocumentOperatorKinds` (DSL / empty graph / missing graph).
- Language-neutral fixture: kernel `scope-contributions/vectors.json` (hexagonal kinds).
- window-fault: refuse `"[]"`; unresolved is not an empty graph; one pack crossing; no `publicInvocationStringPages`.

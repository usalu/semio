# 📓️ W5-prep — restaged guests, serve scripts, and the streaming probe

Packet **W5-prep**, 2026-09-17 evening. Inputs: `📓️wave4-resume-brief.md`, `📓️w3-browser-verification.md`,
`📓️f1-host-scroll-streaming.md` §7 (the e1–e12 DOM/scroll contract), `📓️f2-sdk-body-node-ledger.md` §C1–C7
(path identity, body budget 103).
Artefacts: `🗑️generated/w5/` (activate logs, serve scripts, self-test), probe `🐍️tree-window-probe.mjs`.

**This packet started no servers.** Everything below is ready for the coordinator to start and then probe.

---

## 1. Restaged guests

All foreground-equivalent (detached with `nohup` + `disown` and polled in the foreground, the brief's allowed
exception), one at a time, shared build dir, `CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
NX_DAEMON=false SEMIO_RENDERER=react DEVELOPER_DIR=/Library/Developer/CommandLineTools`, no `CARGO_TARGET_DIR`.

Reference for "newer than the SDK": `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — **mtime 2026-09-17
21:53:37** (F2's path-identity edit; still uncommitted in the working tree).

| # | plugin | nx target | result | dist wasm | newer than SDK | log |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | **fem3d** | `activate-fem3d-react-dev` | ✅ 13 tasks, `component-dev` 3m54s | `…/🔌️plugin-modules/🏗️fem/semio_s_plugin_fem_component.core.wasm` **22:00:14**, 80.8 MB | ✅ +6m | `activate-fem3d.txt` |
| 2 | **cad** | `activate-cad-react-dev` | ✅ 21 tasks (2/22 cached) | `…/📐️cad/semio_s_plugin_cad_component.core.wasm` **22:06:05**, 61.4 MB | ✅ +12m | `activate-cad.txt` |
| 3 | **puzzle3d** | `activate-puzzle3d-react-dev` | ✅ 13 tasks, `component-dev` 3m18s | `…/🧩️puzzle/semio_s_plugin_puzzle_component.core.wasm` **22:11:58**, 116.3 MB | ✅ +18m | `activate-puzzle3d.txt` |
| 4 | **process3d** | `activate-process3d-react-dev` | ✅ 21 tasks | `…/🏭️process/semio_s_plugin_process_component.core.wasm` **22:17:05**, 106.1 MB | ✅ +23m | `activate-process3d.txt` |
| 5 | **flow** | `activate-flow-react-dev` | ✅ 33 tasks | `…/🌊️flow/semio_s_plugin_flow_component.core.wasm` **22:22:29**, 103.4 MB | ✅ +29m | `activate-flow.txt` |

**Five for five, no failures, no retries needed.** All five target names were confirmed to exist with
`NX_DAEMON=false bun nx show project @semio-tech/framework-os-dev`. The SDK file was re-stat'ed after the last
build: still **21:53:37**, i.e. no peer re-edited it mid-run and every staged guest carries F2's container-path
identity and the 103-record budget. `🌳️Tree/🟦️.tsx` (16:14:48) and `🧬️contract/🧩️component/🦀️.rs` (16:14:38)
are older still, so host and guest agree on `TREE_WINDOW_BODY_NODE_BUDGET = 103`.

## 2. Serve scripts — the coordinator starts these

`🗑️generated/w5/serve-<plugin>-react.sh`, modelled on `🗑️generated/w3/serve-fem3d-react.sh` (`S_OS_PORT` is
the port the dev serve pins — `♻️activation/🌐️serve/🟦️.ts:54` passes `portEnv: "S_OS_PORT", fixedPort: true`).

| script | port | URL the probe uses | port free at 22:00? |
| --- | --- | --- | --- |
| `serve-fem3d-react.sh` | 6187 | `http://127.0.0.1:6187/?plugin=fem3d` | ✅ |
| `serve-cad-react.sh` | 6020 | `http://127.0.0.1:6020/?plugin=cad` | ✅ |
| `serve-puzzle3d-react.sh` | 6112 | `http://127.0.0.1:6112/?plugin=puzzle3d` | ✅ |
| `serve-process3d-react.sh` | 6113 | `http://127.0.0.1:6113/?plugin=process3d` | ✅ |
| `serve-flow-react.sh` | 6114 | `http://127.0.0.1:6114/?plugin=flow` | ✅ |

`lsof -nP -iTCP -sTCP:LISTEN` showed only 6012, 6013, 6054, 6063, 6078, 6080 taken by peers — note **6013 is
puzzle3d's own react default** and is a peer's serve, which is why the brief's 6112 (not the default) matters.
Start each as `nohup <script> > 🗑️generated/w5/serve-<plugin>-react.txt 2>&1 & disown`, then wait for the vite
`Local:` line and a `curl -sI http://127.0.0.1:<port>/` before probing.

## 3. What step (e) is now

`🐍️tree-window-probe.mjs` used to have ONE step (e) that only asked "did an offset move", and it reached it
through `[data-slot="scroll-area-viewport"]` — the element W3 §5 proved never scrolls. Step (e) is now the
twelve assertions of `📓️f1…md` §7, one reported sub-step each, plus the old a–d / f–i:

| sub-step | what it asserts |
| --- | --- |
| **e1** | The observed scroller is resolved exactly the way the host resolves it (`treeWindowScrollViewport`, `🗣️Interpreter/🟦️.tsx:1519`): the nearest ancestor with `scrollHeight − clientHeight > 1`, `[data-slot="scroll-area"]` as fallback. Asserts it scrolls AND that the inner `scroll-area-viewport` does not. Shrinks the browser to `1600×SEMIO_PROBE_SCROLL_HEIGHT` (default 520) first if the panel fits. |
| **e2** | Every `[data-tree-window-path]` carries key/path/total/offset/length with `offset + length ≤ total ≤`, `length ≤ 128`. |
| **e2b** | **Path identity**: a top-level container's `-path` equals its `-key`, a nested one's is `<parent path>U+001F<key>`. A repeated `-key` under two parents is recorded as LEGAL, not a duplicate. |
| **e3** | Each container's own rows (`[data-tree-window-row]` owned via `closest('[data-tree-window-key]')`, the host's own ownership rule) carry exactly `offset … offset+length−1` in ascending TOP order. |
| **e4** | Leading spacer `= offset`, trailing `= total − offset − length`, no zero-row spacer, every spacer `rows × pitch` ±1, and container extent `= total × pitch + Σ(direct nested extents)`. Pitch is measured off a spacer, never assumed. |
| **e5** | **Streaming.** Picks the biggest OPEN container with `total > length`, scrolls the real scroller to `containerTop + k × pitch` for `k = offset + length + max(4, length)`, waits ≤ 6 s for the offset to move, then asserts the new `[offset, offset+length)` covers the row index under `scrollTop` and that rows are really painted inside the viewport box (`visibleBand > 0` — a viewport left showing only spacers is the failure this exists to catch). |
| **e6** | `scrollTop` and `scrollHeight` unchanged ±1 px across that refresh. |
| **e7** | No oscillation: 6 samples over 3 s at a held position produce ONE window state and ONE `scrollTop`. |
| **e8** | `Σ(1 + length) ≤ TREE_WINDOW_BODY_NODE_BUDGET`, **read off `🌳️Tree/🟦️.tsx`** at start-up (103 today; `SEMIO_PROBE_BODY_BUDGET` overrides), plus zero `nodes: N vs max_nodes` / `fixed-capacity` / `ui.tree-window` console lines. |
| **e9** | **Lazy expand.** A closed container with `total > 0` must show zero rows and zero `data-tree-window-row`s; expanding it must bring `length > 0` with `total` unchanged. If a lane ships nothing closed, the probe closes the biggest open container first and re-expands it. |
| **e10** | Nesting: a nested container's owner row index lies inside its parent's `[offset, offset+length)` and its path is the parent's path + separator + key. SKIP when the document has no nested container on screen. |
| **e11** | No duplicate `-path` in the body, no `[tree-window] duplicate key` console line. |
| **e12** | Still no `.more` key and no `+N` label after all the scrolling. |

Other probe fixes in this packet: `clickRowKey` now clicks the row's FIRST pitch (`y + min(12, h/2)`) — a row
that owns an open nested window is many rows tall and the old centre click landed inside the child window.

### 3.1 The probe was proved against a fixture before any lane runs

`🗑️generated/w5/probe-selftest.html` is a static mock of the §6.1 DOM (a bounded `scroll-area`, an unbounded
`scroll-area-viewport`, four windowed containers incl. one nested group and one closed-on-purpose section)
with a fake guest that answers window requests on scroll from the same uniform arithmetic. It is a test of the
PROBE, never of the product.

```
SEMIO_PROBE_URL="file://…/🗑️generated/w5/probe-selftest.html" SEMIO_PROBE_TREE_NS=selftest \
SEMIO_PROBE_OUT=w5/selftest SEMIO_PROBE_SECONDS=8 SEMIO_PROBE_SCROLL_HEIGHT=420 bun 🐍️tree-window-probe.mjs
→ PASS=21 FAIL=0   (out: 🗑️generated/w5/selftest/report.json)
```

Two real probe bugs it caught before the lanes: the tall-row click above, and e4's nested-extent term.

## 4. Per-lane probe commands

Run from the ticket folder. `SEMIO_PROBE_SCROLL_HEIGHT` is the lever that makes a SMALL document stream: the
host only ever asks for the rows that fit the viewport, so any container with `total >` viewport rows streams.

```
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING"

# fem3d — the one lane with a genuinely large document
SEMIO_PROBE_URL="http://127.0.0.1:6187/?plugin=fem3d" SEMIO_PROBE_TREE_NS=fem3d-play-artifact \
  SEMIO_PROBE_OUT=w5/fem3d-default SEMIO_PROBE_SCROLL_HEIGHT=520 bun 🐍️tree-window-probe.mjs
SEMIO_PROBE_URL="http://127.0.0.1:6187/?plugin=fem3d" SEMIO_PROBE_TREE_NS=fem3d-play-artifact \
  SEMIO_PROBE_EXAMPLE=House SEMIO_PROBE_OUT=w5/fem3d-house SEMIO_PROBE_SCROLL_HEIGHT=520 bun 🐍️tree-window-probe.mjs

# cad — one example, ~29 rows: streams at a short viewport
SEMIO_PROBE_URL="http://127.0.0.1:6020/?plugin=cad" SEMIO_PROBE_TREE_NS=cad-play-document \
  SEMIO_PROBE_OUT=w5/cad SEMIO_PROBE_SCROLL_HEIGHT=440 bun 🐍️tree-window-probe.mjs

# puzzle3d — default concrete-forest is 3 rows; see §5.1, streaming cannot be reached from the UI
SEMIO_PROBE_URL="http://127.0.0.1:6112/?plugin=puzzle3d" SEMIO_PROBE_TREE_NS=puzzle3d-play-document \
  SEMIO_PROBE_OUT=w5/puzzle3d SEMIO_PROBE_SCROLL_HEIGHT=360 bun 🐍️tree-window-probe.mjs

# process3d — switch to the 7-step Concrete Forest and squeeze the viewport
SEMIO_PROBE_URL="http://127.0.0.1:6113/?plugin=process3d" SEMIO_PROBE_TREE_NS=process3d-play-document \
  SEMIO_PROBE_EXAMPLE="Concrete Forest" SEMIO_PROBE_OUT=w5/process3d SEMIO_PROBE_SCROLL_HEIGHT=320 bun 🐍️tree-window-probe.mjs

# flow — empty document on boot, see §5.3
SEMIO_PROBE_URL="http://127.0.0.1:6114/?plugin=flow" SEMIO_PROBE_TREE_NS=flow-play-document \
  SEMIO_PROBE_OUT=w5/flow SEMIO_PROBE_SCROLL_HEIGHT=320 bun 🐍️tree-window-probe.mjs
```

`SEMIO_PROBE_PANEL` defaults to `Artifact`; if a lane boots German the rail button reads **`Artefakt`**
(`📌️panels/🗿️artifact/🦀️.rs` → `LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt")`).

## 5. What each lane can and cannot prove — shipped examples

Evidence gathered by a read-only sweep of the five editor crates (file:line in the table).

| lane | tree namespace | example picker | biggest reachable document | streaming reachable? |
| --- | --- | --- | --- | --- |
| **fem3d** | `fem3d-play-artifact` | `✏️editor/🦀️.rs:1356-1358`, 3 options, default `concrete-forest` | **House**: 63 nodes + 63 supports + 8 solids + 3 materials + 6 load cases + 2 combinations ≈ **145 rows**; default concrete-forest ≈ 51 | ✅ best lane — W3 already saw 16/63 windows here |
| **cad** | `cad-play-document` | `✏️editor/🦀️.rs:2451-2453`, **one** option (also the boot document) | 4 pane sections (1 + 11 + 1 + 11) + 4 reference sections + nodes ≈ **29 rows** | ✅ at a short viewport (the 11-row sections exceed ~9 visible rows) |
| **puzzle3d** | `puzzle3d-play-document` | **none — the action ships with no options** (§5.1) | reachable: concrete-forest ≈ **3 rows**. Unreachable: `nakagin-capsule-tower`, **180 objects** | ❌ not from the UI |
| **process3d** | `process3d-play-document` | `✏️editor/🦀️.rs:1844-1850`, 3 options, default `timber-beam-joinery` | Concrete Forest: 1 stock + **7 steps** = 8 rows | ⚠️ only with `SCROLL_HEIGHT≈320` (needs < 8 visible rows) |
| **flow** | `flow-play-document` | **none** — `setActiveExample` does not exist in the editor at all | boots an EMPTY document (`✏️editor/🦀️.rs:2348-2350`), 0 widgets / 0 synapses | ❌ (§5.3) |

### 5.1 puzzle3d — `setActiveExample` has no `action_args`

The action is declared (`✏️editor/🦀️.rs:8460`, dispatched at `:3646`, and the handler accepts
`concrete-forest` / `nakagin-capsule-tower` / `""` at `:5687-5691` with aliases at `:5699-5703`), but no
`.action_args(...)` is attached, and the emitted manifest confirms `"args": []`
(`✏️s/🔌️plugins/🧩️puzzle/🔣️.json` → `apps[2] (s.puzzle.puzzle3d@1/*#editor)/windowKinds[0]/actions[0]`).
The sibling puzzle2d editor does wire its options (`◻️2d/…/✏️editor/🦀️.rs:5396`). Consequence for this ticket:
**the 180-object Nakagin document — the fleet's largest artifact tree — cannot be loaded from the browser**,
so puzzle3d can only certify e1–e4 / e9 / e11 / e12 and will SKIP e5–e7. Two ways out, neither taken here
(both are app-owner work, not probe work): wire the two `ActionArgOption`s like puzzle2d, or drive
`openImportFixture` with the Nakagin JSON (`✏️editor/🦀️.rs:119 NAKAGIN_EXAMPLE_JSON`) through the file input.

### 5.2 process3d

Three examples, largest is `concrete-forest` with 7 steps (`🧬️schema/🦀️.rs:271-273`); the tree is one stock row
plus one row per step (`📌️panels/🗿️artifact/🦀️.rs:98-104`). At `SEMIO_PROBE_SCROLL_HEIGHT=320` the panel body
shows fewer than 8 rows, so the steps section should report `total 7 > length`. If it does not, the lane is an
honest **"document fits"** SKIP, not a failure.

### 5.3 flow

`setActiveExample` does not exist in the flow editor (the registration is commented out as a migration gap at
`✏️editor/🦀️.rs:2703-2706`), and the boot document is empty, so the Artifact tree has **no rows at all** — step
(a) will FAIL on `rows > 0` and everything after it is meaningless. To make flow probeable the probe would have
to build a document through the UI (`addWidget`, whose kind options DO exist at `✏️editor/🦀️.rs:2624-2625`)
before opening the panel. That is a probe feature nobody has asked for yet; **recommend running flow last and
reading a 0-row result as "nothing to window", not as a regression.**

## 6. Order the coordinator should start serves in

1. `serve-fem3d-react.sh` (6187) — the only lane that proves e5–e7 on a genuinely large document.
2. `serve-cad-react.sh` (6020) — second real streaming lane at a short viewport.
3. `serve-process3d-react.sh` (6113) — small, but exercises a second app's window plumbing.
4. `serve-puzzle3d-react.sh` (6112) — e1–e4/e9/e11/e12 only.
5. `serve-flow-react.sh` (6114) — expect an empty tree; lowest value.

Serves are heavy: `📓️project-release-serve-wedges-after-host-edit-bursts` — probe one lane at a time, curl the
port before each probe, recycle a wedged serve by pid.

## 7. Not finished / not claimed

- **No browser lane was probed.** This packet started no servers, per the brief; the only browser run is the
  static self-test in §3.1.
- `e2b`'s second half — "every `treeWindows[].nodeKey` the host sends equals some container's `-path`" — is not
  observable from the DOM. The probe checks the DOM side only; the host side is covered by F1's unit laws.
- `e4`'s extent law assumes a uniform row pitch (F1 §2's decision). If an app ever renders a taller row, e4 will
  fail with the exact numbers in `report.json → spacerArithmetic`, and that is a contract question for F1, not a
  probe bug.

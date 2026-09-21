# 📓️ Audit — Follow-Up / Law-Gap Status (session 5, 2026-09-21 14:12, read-only, no builds)

Method: static inspection of the current tree (`git status` shows the play module itself is clean, committed
2026-09-20 23:20:49; `✏️s/🔌️plugins` carries **2 865 uncommitted files** from the peer `LocalizedLabel` sweep at
audit time), one live probe of `:6033` (up intermittently — supervisor restarts every ~75 s; caught one HTTP 200
window), and one run of the play unit suite. No cargo build/test was run (per this topic's rules).

## Table

| # | Item | Status | Evidence | Start here |
|---|---|---|---|---|
| 1 | Raster composite canvas shows no visible demo content | **STILL OPEN** | Reproduced live at 14:00: `#raster` pane, Demo example, Editor mode — Composite and Navigator windows both render an empty canvas (only a red viewport-rect outline in Navigator), no console errors. `document_sync_json` itself is fixed (see #2). | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/edit/windows/composite/…` — the compositor consumer of `Paint2dScene.document_sync_json`/`assets_json`; check whether `assets_json_from_document` (`…✏️editor/🦀️.rs:85`) actually resolves any asset for the `demo` example (empty `resolved` map → no pixels to paint) or the wasm compositor paint-surface sync path itself. |
| 2 | Raster JSON export + mutation-JSON bridge serialize whole snapshots | **DONE (fixed at the ToValue level)** | `RasterOwnedMap<V>::to_value`/`from_value` (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:406-421`) now do a REAL bounded whole-map projection instead of the old guard-panic ("Populated Raster owned map serialization is forbidden…"); the doc comment (lines 395-405) names the exact three routes this used to break — `encode_pack`, the `.raster` DSL printer, `s.stdio.json` export/import, and `apply_raster_mutation_json` — as now working. Both call sites (`…🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:13` and `…🧬️schema/🧬️mutations/🦀️.rs:117`) still literally call whole-snapshot `to_value`, but that is now safe by construction (map is fixed-capacity, 64 entries/8×16 KiB pages). | n/a — verify with `cargo test -p <raster crate>` if in doubt. |
| 3 | Flow test app never finishes closing (teardown) | **LIKELY DONE (inferred, not independently reproduced)** | Root cause (non-retired `flow::OrderedMap` root panicking/hanging the guest) fixed at the source: `FlowWorkingScene` now has a self-retiring `Drop` impl (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:261-267`) so an accidental drop no longer leaves a non-retired owner. No `#[ignore]`d or otherwise disabled flow test found. Test `booting_renders_and_evaluates_without_dropping_a_live_flow_owner` exists (`…✏️editor/🧪️tests/🔬️interactive-job/🦀️.rs:203`). | If it recurs, re-run `cargo test -p <flow crate> --lib --tests` and start at `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:261` (`FlowWorkingScene::drop`) / the mounted-harness close loop (stale-test bucket #3 in `📋️fleet-brief-v2.md`). |
| 4 | Flow rename/patch-widget paths still drop live hosts | **DONE** | `flow_content_child_handle_and_cache` (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:304-307`) doc comment explicitly names `rename-flow-widget` and `patchFlowWidgets` as the previously-broken callers, now fixed by moving the scene into the child's local owner instead of letting parts fall out of scope. `🏷️rename-flow-widget/🦀️.rs` and `🩹️patch-flow-widgets/🦀️.rs` both still call `to_host_snapshot()`/`from_host_snapshot()`, but the leak is closed at the shared helper, not per-caller. | n/a |
| 5 | Architect/Writer/Animate derived children load empty | **PARTIAL — architect DONE, writer and animate STILL OPEN** | Live probe 14:04-14:10: `#architect` Demo → Register panel shows real rows ("Reception", "Waiting" + "elements"), Adjacency/Graph populated — content renders. `#writer` Demo → "Jack" editor window is entirely blank; console shows `text_len=0` repeatedly (`writer.main tokens_json=[]`, `writer.schema.inferences … text_len=0`). `#animate` Demo → "Tile editor" window is an empty grid, no tiles. | Writer: `genesis_writer_child_pack` (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️.rs:128`) guards on `child_id == snapshot.document.child_id`; compare against the `demo` example DSL fixture `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` (**note**: a peer edited this exact file live during this audit, adding a `text=` field with real DSL content — it was still empty at my 14:04 probe; may already be fixed by the time you read this, re-probe first). Animate: start at `genesis_presentation_child_pack` (`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs:250`) and its `demo` example fixture. |
| 6 | Writer demo asset child id ≠ target id | **STILL OPEN (symptom reproduced; exact mismatch not pinpointed)** | Same live evidence as #5. Decoded the `demo` DSL fixture's `document=[childId,target]` field: `document-926a496d334505c6` / `document-926a496d334505c6!s.stdio.semio@v1/document` — this `id!dialect` shape is the SAME pattern in every writer example checked, so it is likely the intended encoding, not itself the bug; could not isolate the actual mismatch in the time available. | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️.rs:128` (`genesis_writer_child_pack`) plus the writer DSL codec that parses `document=[…]` back into `WriterSnapshot.document.child_id` — check whether the parsed child_id matches what `document_child_handle_with_text`/`document_child_handle` would have minted for this text, for the `demo` example specifically (a peer is actively editing this fixture right now — see #5's note). |
| 7 | CAD pane content resolves from a shipped-example catalogue only | **STILL OPEN (by design, unchanged)** | `cad_pane_local_scene` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs:177`) falls back to `cad_bundled_pane_scene` (line 186), a `OnceLock` static built ONLY from `forest_play_scene()` — a wire-decoded handle whose child_id isn't in that one catalogue resolves to `None`. Doc comment (170-176) states this is deliberate ("this resolves shipped assets, it never shares one live document's state with another"), i.e. acknowledged scope limitation, not a regression, and still the current architecture. | If broadened is wanted: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs:186` (`cad_bundled_pane_scene`) — would need every play CAD example's scene registered, not just the forest one. |
| 8 | stdio pane / lane / editor apps unreachable | **STILL OPEN** | Play unit suite run 14:00 (see below): 4 reds, all stdio. Catalog `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` has 60 panes / 8 groups, zero `stdio` entries. `📋️project.json` has 26/27 non-stdio `prepare-*-react-dev` lanes, no stdio lane. Registry DOES carry a stdio playground row (`app: "s.stdio.md@commonmark/*#editor"`) and stdio's own `🔣️.json` (1.5 MB, committed) now exists. Test `keeps stdio's component bounded to the app fleet it can link` (playpanecoverage, line 157) already encodes the intended 7-crate bounded set (csv/html/json/md/tsv/txt/xml) — the groundwork for a linkable stdio component is in place but the pane/lane themselves are not. | `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` (add a `stdio` pane to a group) + `📋️project.json` (add `prepare-stdio-react-dev`/`activate-stdio-react-dev` to the dependsOn lists) + a bounded stdio activation lane using the `STDIO_COMPONENT_APP_CRATES` feature set already pinned by the test at `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanecoverage/🟦️.ts:13`. |
| 9 | Space home/space panes excluded | **DONE** | Catalog `workspace` group has both `home` and `space` panes (`example: "demo"` each). `📋️project.json` has `prepare-home-react-dev`. `playpanecoverage` "activates every registry component the panes need" test passes for every component except `stdio` (unit run below), confirming space's plugin component is in the union. | n/a |
| 10 | Imperative `consumes = ["imperative.module"]` | **DONE** | `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml:16` declares `consumes = ["imperative.module"]`. All 5 `imperative-extension-*` ids appear in the current activation receipt (`🏢️semio-tech/🎡️play/dist/♻️activation/dev/🔣️receipt.json`, rebuiltAt 2026-09-20 07:11). `playpanecoverage`'s component-union test only fails for `stdio`. | n/a |
| 11 | Playbook coverage gate silently skipped playbook (no root 🔣️.json) | **DONE** | `✏️s/🔌️plugins/📖️playbook/🔣️.json` exists (committed, 115 KB). Catalog has a `knowledge/playbook` pane with `example: "demo"`. `📋️project.json` has `prepare-playbook-react-dev`. `playpanecoverage` "reads a committed descriptor for every plugin directory" and "shows every plugin directory in at least one pane" both pass except for `stdio` (unit run below). | n/a |
| 12 | Four descriptors predating `setActiveExample` (architect, dag, imperative, trinity-rewriting) | **DONE** | Parsed each plugin's current `🔣️.json`: `s.architect.program@1/*#editor`, `s.dag.dag@1/*#editor`, `s.imperative.procedure@1/*#editor`, `s.trinity.rewriting@1/*#editor` (and `s.trinity.jack@1/*#editor`) all now declare a `setActiveExample` action among their editor app actions. | n/a |
| 13 | Viewer-reachability law (no play test asserted this) | **DONE** | `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanecoverage/🟦️.ts:113-131` — test `"reaches every viewer app through its pane's editor⇄viewer switch"` (restates `resolveBootPrimaryAppV1`/`surfaceRoleAppsV1` via the shared `dialectCoordinate` function), plus `"mounts every pane with exactly the shell props the grid states"` (134-139) guarding that play's own `FrameworkOsShell` mount never suppresses the switch. Both pass in the current unit run. | n/a |
| 14 | `schedulePlayIdle` warm boot never wired | **DONE** | `🏢️semio-tech/🎡️play/🟦️.tsx:318-322` — a `useEffect` calls `schedulePlayIdle(() => warm(step.id), …, window)` driven by `playNextWarmBootPane`. Tested in `🏢️semio-tech/🎡️play/🧪️tests/🧪️playgrid/🟦️.ts:161-` (`describe("schedulePlayIdle", …)`). | n/a |
| 15 | Per-pane default example (`playPaneBrand` set no `defaults.exampleId`) | **DONE** | `🪧️brand.ts:83-84` — `playPaneBrand(variant, label, exampleId)` sets `defaults: { exampleId }` when given one; called at `🪧️brand.ts:91` with `pane.example`. Catalog `🔣️.json` sets `example` on 53 of 60 panes (the 7 without: flow, din4108, din16798, din18599, en1990-en1999 group, iso16757, vdi3805, demonstrator — host-shell-driven panes with no single example, by design). | n/a |
| 16 | Map-tile serve-mode test missing | **DONE** | `🏢️semio-tech/🎡️play/🧪️tests/🧪️playmaptiles/🟦️.ts` exists (29 lines, 3 assertions: bundles for release build, caps static prefetch zoom ≤10, defaults dev serve to `fetch` unless `GIS_MAP_TILE_SERVE_MODE` overrides). Passes in current unit run. | n/a |
| 17 | `🔒️dependencies.json` predates play | **DONE** | Root `🔒️dependencies.json` (`generatedAt: 2026-09-18`, predates play's 2026-09-19 creation by date but has since been regenerated/edited) lists `🏢️semio-tech/🎡️play/package.json` as a `users` entry for all 4 of play's actual devDependencies (`@tailwindcss/vite`, `@vitejs/plugin-react`, `typescript`, `vite`). | n/a |
| 18 | 6 trailing grid cells | **DONE** | `playGridDimensions`/`playGridRowSpan` (`🪧️brand.ts:106-124`) pick the near-square shape with the FEWEST empty cells and centre a short trailing row; for the current 60 panes this yields 9×7 (63 cells, 3 trailing empty, centred) — matches `📋️design.md`'s own updated grid section (line 55). The literal "6 trailing cells" applied to the earlier 58-pane count (8×8 grid); the fewest-empty-cells rule already supersedes it. | n/a |
| 19 | `extensionDir()` build-mode fallback untested | **DONE** | `🏢️semio-tech/🎡️play/🧪️tests/🧪️playactivation/🟦️.ts:70-82` — `describe("playExtensionDirectory", …)` includes `"with no development receipt, falls back to module directory"` (calls `playExtensionDirectory(name, modules)` with no third arg). Passes in current unit run. | n/a |
| 20 | `verify layering` fails repo-wide (222 files over baseline) | **STILL OPEN (worse: now 230)** | Ran `bun ./📜️script.ts verify layering` at 14:00 (repo root): exits 1, **230 file(s) grew past their baseline** (was 222 at ticket-open on 2026-09-19). Full output: `$T/🗑️generated/audit-followups/verify-layering.txt`. Unrelated to play; repo-wide baseline ratchet (`🧅️layering.json`), plausibly worsened by the concurrent `LocalizedLabel` sweep across ~2 865 uncommitted plugin files. | `🧅️layering.json` (repo root, shrink-only ratchet) vs `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:683-810` (`layeringBreaches`/`LAYERING_BASELINE_REL_PATH`) — each breach line in the saved log names the exact file and its current vs allowed count. |
| 21 | `📋️design.md` "58 panes" stale | **STILL OPEN** | `$T/📋️design.md:31` — "That is 58 panes." — the actual catalog has 60 panes (verified: 8 groups, 60 rows in `🔣️.json`). The Grid section further down (line 55) WAS already updated to say "the 60 panes of 2026-09-20 give 9×7", so the doc is now internally inconsistent, not merely stale. | `$T/📋️design.md:31` — one-line edit: "That is 58 panes" → "That is 60 panes" (and re-check the list of app-column-less rows on the same line, which may also need `stdio`/`space`/`playbook` additions once stdio lands). |

## (a) Staged components vs 27 lanes — freshness

`🏢️semio-tech/🎡️play/dist/♻️activation/dev/` contains only the MERGED receipt (`🔣️receipt.json`, mtime
2026-09-21 13:59 — the coordinator's supervisor just re-activated it); play's own dist has no per-component
directories (by design — `publishPlayUnionReceipt` in `♻️activation/🟦️.ts:97-104` only writes the receipt; actual
staged bytes live under each lane's own `developmentRuntimeRoot`, not under play's `dist`).

- Receipt `variant` field reads `"demonstrator"` — NOT a bug, `mergePlayActivationReceipts` labels the merged
  receipt with `lanes[0]` (whichever pane wins tie-break first in the greedy-cover order), not with `"play"`.
- Receipt has exactly **59 plugin/extension rows**; every registry `PLUGIN_BUILD_TARGETS ∪ EXTENSION_TARGETS`
  component except `stdio` is present (matches the unit-test failure — `stdio` is the only gap).
- `rebuiltAt` timestamps range 2026-09-19 16:29 (lowpoly/wfc/energy/forms/reasoning/dag — oldest, ~46 h stale) to
  2026-09-20 21:15 (flow + its 9 extensions — freshest, ~17 h stale). Every single row predates the 2026-09-20
  23:20:49 batch commit that captured the whole tree, so a naive "receipt time vs one repo commit time" compare
  flags all 59 as "stale" — not a meaningful signal, since that commit is a periodic auto-commit snapshot, not a
  per-plugin edit marker (every plugin directory's `git log -1` collapses to that same single timestamp).
- The load-bearing signal instead: **current uncommitted churn** per plugin directory (`git status --porcelain`,
  captured at audit time, ~2 865 files total from the peer `LocalizedLabel` sweep). Plugins whose staged
  component is both old AND currently under heavy uncommitted edit are the ones most likely to be actually stale
  once that sweep lands and gets activated:

  | pluginId (dir) | receipt rebuiltAt | age at audit | uncommitted files right now |
  |---|---|---|---|
  | energy (🔋️energy) | 2026-09-19 16:29 | ~46 h | 293 |
  | architect (🏛️architect) | 2026-09-19 23:56 | ~38 h | 270 |
  | flow (🌊️flow + 9 extensions) | 2026-09-20 21:15 | ~17 h | 59 |
  | raster (🖨️raster) | 2026-09-20 19:37 | ~19 h | 14 |
  | writer (✒️writer) | 2026-09-20 11:43 | ~26 h | 17 |
  | fem (🏗️fem) | 2026-09-20 17:44 | ~20 h | 62 |
  | block (🧱️block) | 2026-09-20 01:01 | ~37 h | 105 |
  | wfc (🀄️wfc) | 2026-09-19 16:29 | ~46 h | 85 |
  | puzzle (🧩️puzzle) | not in receipt as own id (rolls into other rows) | — | 117 |
  | stdio (🗄️stdio) | **absent from receipt entirely — no lane** | — | 961 |
  | norm (📕️norm) | **absent from receipt** (registered plugin `norm`, not in the 59-row union — likely a build-time-only dependency of architect/energy/din-standards panes rather than its own runtime component; not independently confirmed) | — | 409 |

  Practical read: once the fix fleet's edits to `energy`, `architect`, `block`, `wfc`, `fem` land and are
  re-activated, expect their staged wasm to need a fresh `activate-<lane>-react-dev` before :6033 reflects them
  (the FROZEN dev server also won't pick up new receipts without the `serve-restart.request` touch-file per
  `📋️fleet-brief-v3.md`).

## (b) Play unit suite — exact failures

Ran `cd /Users/ueli/Documents/semio/🏢️semio-tech/🎡️play && DEVELOPER_DIR=/Library/Developer/CommandLineTools bun ./📜️script.ts test` at 13:56-13:58. Full log: `$T/🗑️generated/audit-followups/unit-test-run.txt`.

**58 passed / 4 failed** (matches `📋️fleet-brief-v4.md`'s "58/62" note exactly):

1. `play pane coverage > lists every playground app exactly once` — `PLAY_RUNTIME_PANES` is missing `stdio` vs the 59-variant expected set.
2. `play pane coverage > shows every plugin directory in at least one pane` — `[ '🗄️stdio' ]` has no pane.
3. `play pane coverage > reaches every editor app a plugin descriptor declares, except the host shell's own` — 9 unreachable apps, all `s.stdio.*#editor` (`s.stdio.csv@rfc4180/*`, `s.stdio.tsv@iana/*`, `s.stdio.txt@utf-8/*`, `s.stdio.json@rfc8259/*`, `s.stdio.json@rfc8259/i-json`, `s.stdio.xml@1.0/*`, `s.stdio.xml@1.0/valid`, `s.stdio.md@commonmark/*`, `s.stdio.html@5/*`).
4. `play pane coverage > activates every registry component the panes need` — `[ 'stdio' ]` not in the activation union.

All four have one and the same root cause and one and the same fix location: item #8 above.

## Notes on limits of this audit

- `:6033` crash-loops under the current peer load (restarts every ~75 s per
  `$T/🗑️generated/serve-6033-supervised.txt.events.txt`); only architect/writer/animate/raster were probed live
  before/while it was reachable. cad/flow/space/stdio-adjacent panes were NOT independently re-probed visually in
  this session — treat their entries above as code-evidence-only where marked so.
- `✏️s/🔌️plugins/✒️writer/…/🎬️demo/🗣️.dsl.semio` changed on disk mid-audit (a peer added a `text=` field) —
  called out at item #5/#6; re-probe before acting on that finding.

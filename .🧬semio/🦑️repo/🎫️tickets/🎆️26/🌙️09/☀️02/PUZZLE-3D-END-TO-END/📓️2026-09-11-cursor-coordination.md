# 🖱️ Cursor coordination (2026-09-11 21:00, cursor-chat / Grok 4.6)

Repo MCP unavailable. Ticket `26/09/02/PUZZLE-3D-END-TO-END` (moon folder). `:6014` this session (vite **dev**, pid 1270, HTTP 200). `:6013` Claude **#48 battery in progress** — do not touch.

## Ground
- B20 Add Object **browser PASS** on `:6014` (`probe-2026-09-11T18-59-41`): trigger=2, 13 kinds, instances 1→2. Report `📓️2026-09-11-wave-B20-add-object.md`.
- B19 leftover.ids overlay **source+laws**; first probe still empty Inspection (old wasm); rematerialize 20:07; second probe blocked on hung vite. Current `:6014` is **dev** serving plugin-modules wasm **20:53**. Re-probe `--only=selection-surfaces` required.
- 18:51 `:6014` partial battery (pre-B20 chip, pre-20:53 wasm): Inspection empty, clipboard 0, lock chrome false, gumball poseLen=266, brush preview=null, locale/settings rail, camera history, projection camera JSON.
- B21 one-command lag **source** (`take_typed_operation_completion` refresh_cache first). Needs this wasm in browser.
- B22 brush-mesh upload **host** on #47; guest needs #48. Residual: ~0.64s/page ingress.

## Do not revert
B9 topology, B13 hash-bust, B15 hover-to-select leftover, B17 utility carry, B19 leftover.ids overlay, B20 Window quick-action chips, B21 history_patch on admitting completion.

## Next hops (this session)
1. `--only=selection-surfaces --port=6014` — Inspection object fields + lock row.
2. Brush preview JSON (`data-brush-preview-json`) while Brush armed.
3. Gumball `translateSelection` scene delta (poseLen must move).
4. Clipboard copy/paste census delta.
5. Locale control + settings value → window rail.

## Serves
- Kill by PORT only (`lsof -tiTCP:6014 -sTCP:LISTEN`). Never `:6013`.
- No git. No worktrees. No `CARGO_TARGET_DIR` / `RUSTC_WRAPPER`.

## 21:08 C3 + Inspection re-probe
- C3: `📓️2026-09-11-audit-C3-remaining-e2e.md`. Goal not complete. #48 battery incomplete (~849s catalogue).
- Re-probe `--only=selection-surfaces` `probe-2026-09-11T19-04-04.md`: leftover `selectedIds:["seed-left-001"]` + Inspection refresh hashBust, **Inspection still empty** (`empty=true id=null`). B19 overlay not in live render.
- B24 stopped at preview-cache hop — resumed to LAND.

## 21:10 B23/B24
- B23 leftover retain in plugin.rs (laws green). Needs next materialize-dev. 19:04 still empty Inspection on 20:53 wasm.
- B24: guest preview 252–313 bytes; host store now forceReloads; leftover SurfaceVisible still boot spine `previewBytes:0`. Next: project the guest tree into `window:puzzle3d-main-perspective`.

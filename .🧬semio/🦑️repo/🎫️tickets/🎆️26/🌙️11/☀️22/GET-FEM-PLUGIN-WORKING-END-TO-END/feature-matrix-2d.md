# FEM 2D Editor Feature Matrix (headless Chromium, http://127.0.0.1:6086/)

Status: INCOMPLETE. The run was cut short twice by dev-server breakage (first @vite/client 404 at ~02:58, then again at ~04:00: the browser gets an empty 404 for `/@vite/client` while curl gets 200, app never boots, blank page) and once by an app wedge (see F1). Latency is wall-clock from action to visible aria effect. Only rows with Observed data were actually run; the rest are NOT RUN.
Valid evidence only from the boot after the 03:02 server restart (driver 6197, second boot).

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| Boot | load page, wait for Structure view | app boots, demo + results converged | boots, "Converged 36/512" | ~4 min (first), 9 min (later) | PASS |
| Utilities menu | Model window "Utilities" | list Direct/Marquee/Lasso/Pan/Transform | all five buttons, plus Method (Rectangle/Lasso) and Mode (Selective/Additive/Subtractive/Invertive) radios | <3 s | PASS |
| Direct Select | choose Direct Select | pressed state | `button "Direct Select" [pressed]` | <3 s | PASS |
| Inspection panel | toggle "Inspection" | opens | opens; Artifact button hidden while open | <1 s | PASS |
| Direct click select | click (520,402) | `treeitem "ID ridge"` | ID ridge, X 4, Y 7.6, Focus, Delete | 42-46 s | PASS (slow) |
| Node X edit | type 5 + Enter in X spinbutton | node moves, results re-solve | ridge moved to x=540px, tree "X 5", Results re-solved (Converged 40/512, Ty 1692 N) | 12-15 s edit, ~30 s re-solve | PASS |
| Undo Ctrl+Z | after edit | X back to 4 | X 4 | 4.6 s | PASS |
| Redo Ctrl+Shift+Z | after undo | X 5 | no effect within 60 s | n/a | FAIL (F2) |
| Redo Ctrl+Y | after undo | X 5 | no effect within 60 s | n/a | FAIL (F2) |
| Empty click deselect | click (300,300) with ridge selected | Summary shown | took more than 2 min | >120 s | FAIL (F1) |
| Canvas click after redo/undo sequence | click p0_l2 (440,442) | ID p0_l2 | no selection within 240 s, 150 s, 150 s | never | FAIL (F1) |
| Artifact tree | open "Artifact" | all sections populated | Nodes (12) fully listed; Elements (9) shows only e3; Regions, Supports, Load Cases, Combinations expanded but empty | 6 s | FAIL (F1) |
| Results status | after edit/undo/redo | Converged | "Pending" label forever, deformed shape and reaction labels gone (s6.png, s5 series) | >10 min | FAIL (F1) |
| History panel | open History | list commands, Undo/Redo | lists Actions/Commands; opening History closes Inspection (exclusive); clicking the Undo row only selects it, inner `button "Undo"` works | 42 s (earlier, pre-restart data, tentative) | PASS (tentative) |
| Visual: Results status text | screenshot a0.png / c1a.png | no overlap | "Converged 36/512 Residual 7.46e-9" is drawn over the load labels "Ty: 551 N", "Tx: 167 N" at the right of the Results window | n/a | FAIL (visual, F3) |
| Multi-select Shift/Ctrl/invertive | click sequence | N Selected | NOT RUN (canvas wedged, F1) | | NOT RUN |
| Marquee, Lasso, Pan, Transform + gumball + 4 option toggles | | | NOT RUN | | NOT RUN |
| Delete/Backspace, inspection Delete | | | NOT RUN | | NOT RUN |
| Inspection edits: element, material, section, supports, region, loads, load case self weight, combination factors, Add Term, Focus | | | NOT RUN | | NOT RUN |
| Actions palette (Add *) and Command palette | | | NOT RUN | | NOT RUN |
| Results panel controls and Results window render | | | NOT RUN (only the static result seen) | | NOT RUN |
| Example combobox, Editor/Viewer role, window Focus/Close/Fullscreen | | | NOT RUN | | NOT RUN |
| Settings, language DE/EN | | | NOT RUN | | NOT RUN |
| Wheel zoom/pan, Space play/pause | | | NOT RUN | | NOT RUN |

## Diagnoses

### F1 App wedges after a few edit/undo/redo operations (resource stock exhausted)
Driver log after the wedge (artifact-open call):
`error [os-shell] tree window refresh failed Error: plugin.original-stock-exhausted: maximumCopyBytes has 134213695 of its floor 134217728 at OriginalOperationSource.requireStock (.../fem/bridge.js:103)`.
The copy-byte budget (128 MiB) is drained by roughly: one node edit, one undo, redo attempts, a few selections. Afterwards tree windows cannot refresh (Artifact tree shows empty Regions/Supports/Load Cases/Combinations, Elements lists only e3), Results stays "Pending", and canvas clicks never select (screenshots s4.png, s5-series, c6.png). Boot is clean; this is the first real defect and blocks all further interaction testing in one session, so most rows are NOT RUN.

### F2 Redo does not fire from keyboard
Ctrl+Shift+Z and Ctrl+Y produced no state change within 60 s each, while Ctrl+Z took 4.6 s. Could be the same exhaustion (F1) rather than a binding problem; in an earlier, pre-restart run Ctrl+Y did redo after 15 s. Needs retest on a fresh page.

### F3 Visual defects
- Results window: the solver status line ("Converged 36/512 Residual 7.46e-9", later "Pending") is centered at y~542 and overlaps nodal-load labels (Tx/Ty) at the right and reaction labels on the left; screenshots a0.png, c1a.png, b1.png.
- Model window: selection halo and Method/Mode option panel overlay bottom-left of canvas; acceptable.
- Not yet checked: white section headers in Results panel (Results panel was never opened).

### Environment problems (not feature bugs)
- 02:58 and again ~04:00: the vite server on 6086 returns an empty 404 for `/@vite/client` to the browser (fetch from page returns 200 with the index html, module requests get 404), app never boots; page body is empty (s5.png). Needs server restart; I may not do so.
- curl in `d.sh` has `--max-time 600`; scripts longer than that return empty output (results must be persisted from inside the script).

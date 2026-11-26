# Regression C: browser audit of FEM 3D editor, build 3

Driver: `page-driver.mjs http://127.0.0.1:6087/ 6250` (PID 25960, own PID, stopped at the end), 1600x1000. Helper scripts, bodies and screenshots: `🗑️generated/rc/` (`lib.js`, `lib2.js`, `run.sh`, `c-*.png`). Every result below is "build 3". Page reloads: 92 s and 132 s to boot. Times are wall clock on 2026-11-26. The build 3 page logs a flood of `[DEBUG] reconcile pending=...` console errors (hundreds per minute), filtered out of every readout. The C: drive filled up once (0 bytes free, around 02:09, outside this audit) and recovered by itself.

## Results

| # | Item | Evidence (build 3) | Result |
|---|---|---|---|
| 1 | Add Bar (n00_g to n20_g, steel, hea200) | Elements 16 -> 17 in 2.3 s, new row `e-9309... Bar`; solve settled 20 s | PASS |
| 2 | Add Solid valid (concrete 3x3x0.3) | Solids 1 -> 2 in 2.6 s; then F3 wedge (see Wedges), screenshot `c-solid.png` | PASS (edit), solve FAIL (F3) |
| 3 | Add Solid unknown material | DOM notice "'unobtainium' is not a material of this document. Add a material first ... Close"; Solids stay 1. German: "'unobtainium' ist kein Material dieses Dokuments ... Schließen" | PASS (EN and DE). During a wedge no notice appeared in 6-8 s (see F-N) |
| 4 | Add Solid zero width | "A solid needs a positive width, depth and height." / DE "Ein Volumenkörper braucht positive Breite, Tiefe und Höhe." | PASS |
| 5 | Add Nodal Load (n20_l2 Tx 1234 live) | Live Load label "3 Loads"; solve 40.5 s | PASS |
| 6 | Add Member UDL | needs `wy` filled (blank, no default): Execute aria-disabled until then; with wy=0: Live "4 Loads", solve 36.9 s | PASS (see F-D) |
| 7 | Add Area Load (sol1, 700, dead) | Dead Load "1 Load" -> "2 Loads", solve 40.5 s | PASS |
| 8 | Set Self Weight | needs the Enabled toggle pressed (aria-pressed=false default, Execute disabled until toggled); after: `Live Load · 4 Loads · Self Weight` | PASS (see F-D) |
| 9 | Add Combination natural spelling `dead:1.35, live:1.5` | Combinations 1 -> 2, solve 40.9 s | PASS |
| 10 | Add Combination invalid `dead:x??` | DOM notice "The terms 'dead:x??' could not be read. Write load case ids with factors ..."; DE "Die Terme ... konnten nicht gelesen werden"; count stays 2. Not visible in screenshot `c-badcombo.png` (text only in DOM readout) | PASS (DOM), visual NOT VERIFIED |
| 11 | Add Term | Not run (Inspection term editor not exercised in build 3; only the tree row `1.5 live Term` seen) | NOT VERIFIED |
| 12 | Add Material / Section / Support / Load Case | Materials 2->3, Sections 1->2, Supports 8->9, Load Cases 2->3, each solve 38-40 s | PASS |
| 13 | Support DOFs editing in Inspection | not run | NOT VERIFIED |
| 14 | Set Analysis Settings (4,2,150) | Results panel shows Modal 4, Buckling 2, Scale 150; caption `Analysis failed` -> `Case: dead` in ~40 s | PASS |
| 15 | Set Result Display | sourceId `live` / `dead`, mode Modal or Static, no refusal, but Results panel Source and Mode combos unchanged (`Dead Load`, `Static`) after 3-7 s | FAIL (F-R) |
| 16 | Select All | view hash changes in 241 ms | PASS |
| 17 | Translate Selection (1,2,0) on all | nodes (0,0,0) -> (1,2,0), (8,0,0) -> (9,2,0) | PASS |
| 18 | Rotate Selection (Z, 90) | rigid (n00_g to n20_g distance 8.00 kept) | PASS |
| 19 | Scale Selection (2,2,2) | n00_g to n20_g distance 16.00 | PASS |
| 20 | Gumball move/rotate/scale | canvas only, not locatable | NOT VERIFIED |
| 21 | Remove Selection (col_20_l) | Elements 16 -> 15 in 3.1 s | PASS |
| 22 | Copy/Paste, Cut/Paste | copy+paste 16->17 (3.0 s), cut 16->15 (3.4 s), paste 15->16 (2.7 s); copy+paste adds 2 nodes (16->18) | PASS |
| 23 | Undo/Redo, also during a running solve | Undo x3 restores origin; second Undo issued while caption `Preparing analysis` applied within ~4 s; Redo x2 restores; Undo of paste 3.4 s | PASS |
| 24 | Commit Checkpoint, Create Alternative, Checkout Checkpoint, Switch Alternative | actions run without refusal; Checkout has no form; Switch needs an opaque `alternativeId` text with no list; effect not observable (Add Node after Commit showed no node change in 20 s because the run ended in F3 wedge 2) | NOT VERIFIED (effect) |
| 25 | Export Document | download `fem3d-20261126T005648Z.semio-archive` (1237 bytes), notice "exported", DE localized | PASS |
| 26 | Import Document | file chooser opens; after Add Node (17 nodes), importing the export gave 16 nodes in 13.4 s, notice "Opened “export.json” as a new document.", caption `Generating mesh` -> `Case: dead` 6 s later; console error `plugin.internal.zero-progress: runtime close cleanup faulted for instance 2` | PASS (with console error, F-C) |
| 27 | Examples switching | Demo -> House: 63 nodes after 13.7 s; House caption stayed `Preparing analysis` for at least 4.7 min; Demo selected again during that pending solve: applied in 54.6 s, solved 23.8 s later | PASS (slow, F-H) |
| 28 | Results caption settles (healthy runs) | `Case: dead` after 20-41 s for every edit that did not wedge | PASS |
| 29 | Results modal mode shapes | Modal mode 1 caption `Mode 1: 0.000 Hz` for modal index 0, 1, 2, 3 (caption never changes, no visible deformation, view hash unchanged); then `Computing modes` for > 2 min | FAIL (F-M) |
| 30 | Buckling mode shapes | switching to Buckling: caption `Analysis failed` | FAIL (F-M) |
| 31 | Playback Play / Pause / Step | label toggles Play -> Pause, caption flips to `Computing modes`, results view hash static over 2 s (no animation), Pause click did not take (label stayed Pause), Step +/- toggled the label back; no `aria-pressed` attribute on the Play button (`aria-pressed=null`) | FAIL (F-P) |
| 32 | Focus | the two Focus buttons are not present in the DOM in the reloaded layout; not located | NOT VERIFIED |
| 33 | Clear Selection / Set Selection Mode (Multiple) / Set Granularity (node) | all execute without refusal; effect not observable | PASS (no refusal), effect NOT VERIFIED |
| 34 | Command bar, Language (EN/DE), Settings, windows | Command opens Layout/App/Appearance/Language menus; Set Locale Deutsch applies at once to whole UI (labels, refusals, Results panel); back to English works (Execute id `command-os.os.setLocale-execute`); Settings button opens | PASS (typed palette search does not exist; no text filter) |
| 35 | Analysis row, Settings contents | row not opened separately; Analysis settings visible in Results panel | NOT VERIFIED (row) |
| 36 | Viewport orbit / pan / wheel | left, right, middle drags and wheel each change the view hash in 114-241 ms; opposite gestures restore the original view | PASS |
| 37 | Viewport pick | first click at a new target does not select; the second click at the same target selects it; the first click after a move selects what was under the previous click | FAIL (F-K) |
| 38 | Execute disabled while a solve is pending | never observed for forms with complete inputs (Add Node, Nodal Load, Area Load, Add Solid executed during pending solves); disabled in UDL and Self Weight because required fields were blank | NOT REPRODUCED (see F-D) |
| 39 | Caption stuck at `Preparing analysis` > 60 s (F3) | yes, 3 times (see Wedges) | FAIL |

## Wedges (precise timing)

- Wedge 1: fresh page (boot 00:38, Demo settled). Add Bar settled in 20 s (00:41:50). Add Solid concrete clicked at about 00:42:05, applied in 2.6 s, caption then `Case: dead · Preparing analysis` and never settled. Still `Preparing analysis` at 00:46, 00:56:10 (measured: 9 min 20 s polling with no settle) and 01:03:00 (about 21 min). Edits kept applying during this wedge (Add Node 2.6 s, Nodal Load, Area Load) but the viewport did not draw the new bar, solid or node (`c-tasks.png`, `c-solid.png`), Execute of UDL/Self Weight stayed disabled for 90 s only because of F-D. The Task panel showed `semio.fem3d.mounted-live-visual Running 1600 steps 17 s`, then `12096 steps · 121 s`, `21600 steps · 221 s`: the live visual job keeps running while the caption stays at `Preparing analysis`. Recovered only by page reload (92 s).
- Wedge 2: after the reload and 23 healthy solves (every edit settled in 33-41 s), the History sequence (Commit Checkpoint, Add Node, Create Alternative, Checkout Checkpoint) ended at about 01:39 in `Preparing analysis` and then `Analysis queued`; at 01:48:04 the Task panel showed two running jobs, `161280 steps · 2597 s` and `26976 steps · 541 s` (started about 01:39). Add Node during this wedge did not apply in 20 s (node count unchanged). Recovered by reload (132 s).
- Wedge 3: House example `Preparing analysis` (no `Case:` prefix) from about 02:01:55 to at least 02:06:38 (4 min 43 s); ended after switching back to Demo, which itself took 54.6 s to apply.
- The wedge is not triggered by Add Solid alone: Add Solid zero or invalid material never reached it, and 12 edits (loads, combination, material, section, support, load case, transforms, undo, redo, cut, paste) settled in 33-41 s each.

## Details of every FAIL

- F3 (wedge): see Wedges above. Trigger candidates: Add Solid on a fresh page (wedge 1) and the History actions Commit/Create Alternative/Checkout (wedge 2). Caption stays `Preparing analysis`, edits mostly keep applying, the viewport does not redraw them, reload is the only recovery.
- F-K viewport pick (item 37): reproduced with `down/wait 80 ms/up` after a 2 s hover: sequence beam@A, beam@A, col@B, col@B, beam@A produced selections `[]`, `fb1_0`, `fb1_0`, `col_20_l`, `col_20_l`. Each new target needs a second click, the hover highlight appears dark (hover) but the first click does not select. Prior audit finding (thin column tolerance) not re-measured.
- F-M modal / buckling (items 29, 30): Mode Modal gives `Mode 1: 0.000 Hz` regardless of Mode Index; the Mode Index spinbutton accepts 0..3 and arrow keys but the caption and view do not change; after Play the caption became `Computing modes` for more than 2 minutes (01:08-01:11) and Buckling gave `Analysis failed`. Setting Static again, then Set Analysis Settings, recovered to `Case: dead` (40 s). A 0 Hz first mode suggests the Demo model has a rigid-body mode or the eigen solve returns the wrong set; mode shapes never became visibly deformed.
- F-P playback (item 31): no `aria-pressed` on Play, Pause did not stop, caption `Computing modes`, no animation frames.
- F-R Set Result Display (item 15): action accepted, Results panel Source and Mode unchanged.
- F-D required fields without defaults (items 6, 8): Add Member UDL leaves `Wy` empty (while `Wx` defaults to 0), Set Self Weight has an unset `Enabled` toggle; Execute stays aria-disabled with no hint which field is missing. Looks like a hung Execute while a solve is pending, but it is not.
- F-N no notice during a wedge (item 3): the same unknown-material Add Solid that shows a localized refusal on a healthy page showed no notice for 6-8 s while wedge 1 was active.
- F-C console error: `[actor] original-turn.semantic plugin.internal.zero-progress: runtime close cleanup faulted for instance 1/2: the cleanup pump made no progress for its whole stall credit (elapsed 30100us, ceiling 8000us)`, logged on Import Document (twice). No visible effect.
- F-H slow example switch (item 27): Demo -> House took 13.7 s, House -> Demo during a pending solve took 54.6 s.

## Not verified

Add Term, support DOF editing, gumball, checkpoint and alternative effects, Focus, Analysis row, selection mode/granularity effects, window drag/layout, minimum-contrast settings. The wedges and two reloads (about 4 min each) used most of the audit time.

## Cleanup

Driver on port 6250 (PID 25960) stopped; no other process was touched.

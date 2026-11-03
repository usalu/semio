# R2 smoke, part 2 (agent sm3): excel, grasshopper and full-geometry BIM construct

Harness: `🧰️sm3/driver.py <scene watchdog s> "<file>|<file>..."` (one uv worker process at a time, stdin /dev/null, psutil memory guard that kills the tree above 5 GB, restart after each watchdog kill with the finished scenes skipped) and `🧰️sm3/worker.py` (sm's worker generalised to several files per process, so BIM modules share the geometry memo). Same manim settings as sm: `dry_run`, quality l, 15 fps, `skip_animations`. Output was in `🗑️generated/sm3/` (deleted, together with sm's `🗑️generated/sm`).

## Result: 14 scenes run, 0 pass, 0 fail, 14 watchdog timeouts, 0 memory kills

| Group | Scene | Watchdog | Outcome | Peak RSS |
|---|---|---|---|---|
| excel energy | `Beat4_Energy` | 900 s | timeout | 2.0 GB |
| excel program | `Beat3_Program` | 900 s | timeout | 2.0 GB |
| excel references | `Beat2_References` | 900 s | timeout | 2.0 GB |
| excel anatomy | `Beat1_Anatomy` | 900 s | timeout | 2.0 GB |
| grasshopper trees | `Beat5_DataTrees` | 900 s | timeout | 2.1 GB |
| grasshopper tower | `Beat3_Tower` | 900 s | timeout | below 2.1 GB |
| grasshopper anatomy | `Beat1_Anatomy` | 900 s | timeout | below 2.1 GB |
| grasshopper casting | `Beat4_Casting` | 900 s | timeout | below 2.1 GB |
| bim consistency | `Beat6_Consistency` | 1800 s | timeout | 0.85 GB |
| bim information | `Beat1_DataToInformation` | 1800 s | timeout | below 1 GB |
| bim models | `Beat4_PartialModels` | 1800 s | timeout | 0.94 GB |
| bim objects | `Beat2_ObjectOrientation` | 1800 s | timeout | below 1 GB |
| bim exchange | `Beat5_Interfaces` | 1800 s | timeout | below 1 GB |
| bim representations | `Beat3_SameObject` | 1800 s | timeout | below 1 GB |

Not run: grasshopper capsule (multi-hour, skipped by design, as in sm). The 12 series entry files and the excel/grasshopper/bim `visuals` modules define no Scene (`noscene`, import clean).

Combined with `r2-sm-report.md` (113 pass, 12 noscene files, 6 BIM timeouts, capsule skipped): 113 scenes are verified to construct. 14 of the remaining 15 Scene classes (the 6 BIM scenes sm already listed as timeouts plus 8 excel/grasshopper) hit a watchdog here and are NOT verified; the capsule is skipped. There are no verified failures.

## Classification
- No real breakage found; no code edited. A timeout is not a pass, so these 14 stay unverified.
- BIM (6): all CPU-bound in the full-geometry model build (RSS flat at 0.8 to 1 GB for 30 minutes). `Beat6_Consistency` and `Beat1_DataToInformation` ran 30 minutes without the `_surface_finishes` AttributeError ne reported. A grep of the whole `🎓️teaching/` tree finds no remaining `_surface_finishes` (fixed by sm; ne's report predates it). The rename is thus past its call site in the two scenes, but no scene finished. Restarting after a watchdog kill loses the geometry memo, so sharing one process only helps if the first build fits in the watchdog. Agent bv was running the long BIM test suites at the same time, so CPU was shared.
- Excel and grasshopper (8): every one of these modules uses the Nakagin tower drawing (`nakagin`/`tower_axon` in each file). This matches `r2-xl-report.md`: the first `tower_axon` call orders about 165 000 faces and has not finished in any of 7 runs up to 40 minutes. This is the open design item of xl2 (one 140-capsule drawing per tint), not a breakage in staffage. I did not edit the tower files (xl2).
- Transient: at the end of the BIM sweep all six files failed to import with `NameError: name 'errno' is not defined` in `🎨️visuals/🧍️staffage/🐍️.py:45`; another agent was mid-edit, and `import errno` is present at line 13 now. Only skipped (already timed out) scenes were affected, so no result changed.
- My first kill command also took down my own bash shell once; no foreign process was touched (matching on the sm3 script path only).

## Open
- Re-run the 14 scenes once xl2's tower fix lands (excel/grasshopper) and with a build-once geometry memo or a longer watchdog (BIM): `python 🧰️sm3/driver.py <scene watchdog s> "<file>|<file>"`.
- Genuine construct verification of the BIM rename needs one full model build; consider a test-only cached model (not done, outside this task).

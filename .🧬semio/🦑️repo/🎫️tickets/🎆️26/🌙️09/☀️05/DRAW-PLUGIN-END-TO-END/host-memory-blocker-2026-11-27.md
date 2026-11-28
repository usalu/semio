# 🧯️ Host Memory Blocker — 2026-11-27

## Finding
- `fseventsd` (pid 99, root, up 12 days) holds a 13 GB footprint, 10 GB of it compressed/swapped (`top -o cmprs`), up from 5.9 GB at 18:00 on 26 Nov. Swap sits at 7–9.7 of 8–10 GB, so the host warden reports "swap is crowded and worked" and cancels compile units even with 40–70 % RAM free.
- Every final Draw component link (`semio-hub-draw-component`, ~6 GB) since 01:00 was SIGTERM'd at the last unit (354 units compiled, 0 failed, exit 143; reports under `.cache/b/r/m/components/runs/`). Native test crates are killed the same way.
- Fleet offload is no alternative: every fleet worker reports memory danger and warden interventions (`repo fleet status`), and the local CAS (`~/.cache/semio/cas`) holds 32 GB against a 1 GB effective cap because its 77 GB free-space reserve cannot be met, so uploads answer 507.

## Remedy
- Root only: `sudo killall fseventsd` (launchd respawns it) or a reboot releases the leaked memory. Likely driver: the replica daemon's file churn across the shared worktree.
- After that: `sh final-e2e-27.sh <run>` (build → stage → serve → blank matrix → journeys en/de → viewer en/de) and `./rerun-native-26-11-26.sh scene-tests viewer model-tests transform-tests editor-tests editor-commands-tests editor-tests-2 check`.

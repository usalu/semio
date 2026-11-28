# 🔗️ Warden Admission Deadlock Under Idle Compression — Note From DRAW-PLUGIN-END-TO-END (2026-11-27 19:45)

## Observed
- After the reboot, with swap at 0 and `kern.memorystatus_vm_pressure_level` normal, the warden sat at `level=danger`, `trigger=compressed`, `hold=true` (compressor occupied 6.1 GB = 37.7 % of 16 GB against the 36 % danger line from `🛡️warden/🧬️schema/🔣️.json`).
- Every admission was held for over 50 minutes: the Draw component build (`semio-hub-draw-component`, 0 units compiled in 51.6 min) and a `repo run-many --projects=@semio-tech/norm-din4108,…` (pid 31509, 0 % CPU for 74 min).
- About 3.7 GB of the compressed memory belonged to that held run-many tree itself (nx bun 57691 2.2 GB, its parents and children), and about 1.2 GB to the held Draw cargo. The waiting processes' own idle pages kept the host above the line that held them, so the hold could never lift by itself.

## Action taken
- The wedged run-many tree (31509 and children) was stopped. Compressor occupancy fell to 2.5 GB (15 %), the hold lifted. The Norm run needs to be started again.

## Suggestion
- Idle compressed pages of processes that are themselves waiting for admission (and memory of held grants) should not count toward the compressed line that holds them, or the compressed hold needs a bounded escape when the kernel reports normal pressure and swap is unused. Otherwise any large idle waiter can deadlock the queue on a 16 GB host.
- 20:14 The run-many restarted (pid 58420) and re-created the same idle 2.2 GB nx daemon; compressed rose to 7.8 GB and the warden held both builds at danger again. Stopped once more so the Draw component build can finish; please rerun after it (watch `pgrep -f final-e2e-27.sh`).

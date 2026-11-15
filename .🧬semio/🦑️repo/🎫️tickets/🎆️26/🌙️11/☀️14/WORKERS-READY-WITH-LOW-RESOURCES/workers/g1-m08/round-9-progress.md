# Round 9 in progress

Observed 2026-11-14T23:08:19Z. Try 3 remains active; final native_rc is not yet measured. Earlier request reached 447 of 563 arrivals and graph inference. Native project inference guard then reported the following, verbatim (15 preceding lines retained in round-9-try-3-inference-guard-context.txt):

graph: project @semio-tech/framework-rs (language:cargo) grew by 1768 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here

An inner repository command reported exit 1; no final native exit has been emitted. The repo started a new host incarnation (PID 5696, parent 5684 -> launchd) and the active run continues. The monitor heartbeat host-ID list retains the first-seen PID; full new incarnation evidence is in round-9-try-3-host-refresh-observation.json, and current owned-process samples include new descendants and metadata PIDs.

Current graph plan and fleet progress, verbatim:
graph: 609 stale projects, 399 taken from the fleet, 2 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 210
graph: waiting for the fleet: 0 of 210 projects arrived

Peak observed run-owned RSS so far: 7481552 KiB (about 7.1 GiB). At the required 30-second cadence one critical sample was observed, followed by normal pressure; the two-consecutive-critical stop criterion was not met. Raw 5-second pressure observations will be persisted when this try ends. No observer SIGINT, gc, kickstart, source edit or permission change. Launcher final identity for this try is not yet measured.

# Browser Startup Audit

At 2026-11-20 16:23 local, `repo fleet status --lang=en` showed no active React or WGPU run, no graph builder, and no queue. `repo doctor --only=processes,graph --lang=en` listed the graph host process 97457 and its service parent 97444, but no dev-server process. A process listing filtered for Shooting, Vite, the two ports and repo run likewise found no surviving browser dev run. A separate Shooting cargo test was running and is unrelated to browser serving.

The graph host status reported one computed plan taking 160 seconds, zero waiting, and a log at `/var/folders/_d/gdnvzj856_zbxn3kl48wfkfr0000gn/T/semio-repo/501/host-a246633bede0.log`. Doctor reported graph implementation files newer than published generation 248, whereas fleet reported no generation published. These status results do not establish successful fresh serving or a faster supported route. No heavy operation was started during this audit.

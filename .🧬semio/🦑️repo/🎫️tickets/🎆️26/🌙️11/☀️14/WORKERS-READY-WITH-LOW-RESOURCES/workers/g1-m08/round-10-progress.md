# Round 10 in progress

Observed 2026-11-14T23:45:35Z. Try 1 completed native_rc=75. Initial request reached 413 of 520 arrivals and graph inference; native project budget guard stopped framework-job-rs (2696 MB growth >1756 MB budget). The new host then took 413 records and requested 201; the final refusal reported 0 arrived / 201 missing / 901 s. Native owned RSS peak 5071712 KiB, with no observer resource interruption.

Final refusal, verbatim:
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 201 and delivered 0 within 901 s, 201 are still missing, among them the too heavy project(s) workspace (about 13780 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.

Launcher UNCHANGED before/after try 1 and compared with round 9 final identity. Cache/scheduler/NFS listeners all reachable at end: this is counted try 1; no server-move outage exemption. The controller takes the prescribed two-minute wait, then try 2. No permission/source changes, observer kickstarts or gc.

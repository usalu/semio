# g1-m10 round 27 (series 1: try 1 reported; later tries appended below)

Machine: Mac arm64, macOS 27.2.1, 16 GiB RAM, 131 GiB free (df /). Replica code digest on g1-m10 and g1-server agreed before the series (`0dc4d5abc2e3...` per attestation). Replica daemon running, sync on, incoming 0.
Step 0 (`repo tools setup all`): never exit 0, see `round-16.tools-setup.report.md` (3 identical `Resource lease timed out: native-storage` failures). Bootstrap run anyway.

## Try 1
- start 2026-11-15T08:57:23Z, end 09:05:19Z, wall 476 s, wrapper_rc 0, **native_rc=75**, marker `.cache/f/native-bootstrap.done` absent, ready false.
- access-proof (verbatim): `2026-11-15T08:57:29.486Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=95517 ppid=95437 responsible=39644`. No launchd refusal, no EPERM, no closure-missing line.
- Failed tasks: none ran to a task failure; the graph step was refused (exit 75). Setup tasks not reached.
- Capacity: `[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (9457 MB available, 2048 MB floor)`; `graph: memory readings: machine 16384 MB (physical 16384 MB, cgroup limit none), available 6985 MB`.
- Plan line (verbatim): `graph: 1682 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), semio-s-plugin-energy-engine-zone-hvac (about 3598 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 1682` (3512 MB per project, 192 inferred, 6 too heavy: matches round 17 expectation except 6, not 12-14).
- Fleet: `0 of 1682 projects arrived from the fleet` for the whole try; `the server last progressed 1395 s ago` at the start, 2019 s at the last progress line (grows; estimated time left "about 10 min", later "unknown"). Not growing x, server not progressing (round 22 allows 30 min).
- Restore line: `No verified change journal cursor is recorded for generation 228; verifying the tree once.` (no "restored ... from a state" line).
- Memory contention: `[capacity] waiting ... for exec: not enough free memory (need 1536 MB; -1481 MB spare above the 2048 MB floor, load 19.94; 3 running ...); behind sync (running, 1 job(s), 5587 MB), @semio-tech/mit-bestand-demonstrator:activate-prototype (running, 1 job(s), 2574 MB), graph-plan (running ...)`. Many `graph: waiting for memory headroom (...)` lines (healthy per round 23), one project at a time at times.
- End (verbatim): `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10076 MB, more than the 6553 MB this machine gives a plan, and only 3452 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory.` No `plan stage ... settled from X to Y MB` line was printed.
- Peaks (sampler, 30 s): run process 3598 MiB, slice workers 6668 MiB, system top 5876 MiB; at the peak sample (09:03:09Z) progress was `graph: 0 of 1682 projects arrived from the fleet, 1000 inferred here, 682 left; the server last progressed 2019 s ago`. Pressure level 1 throughout. Closure entries 35800 before, 34694 after. No hard stop. Service log: `/var/folders/n2/f4tcy7ks04sb8njd18x7l1lh0000gn/T/semio-repo/501/host-6c83dc348341.log` (copy: round-27-try-1.service.txt). Stage timings: not measured separately beyond the log (see `round-27-try-1.log`).
- Files: round-27-try-1.{log,metrics.json,samples.txt,top3.txt,service.txt,tail.txt,wrapper.txt}, round-27.tail.txt, state.json.
- Next: retry after 3 minutes (native_rc=75 is a retry case).

## Try 2
- start 2026-11-15T09:08:59Z (3 min after try 1), end 09:25:32Z, wall 992 s, **native_rc=7** (my own observer's stop code, not a repo result), marker absent, ready false.
- **Stopped by my observer helper (`run-observed.py`), stop_reason `memory-pressure-level-4-two-samples`.** That rule is obsolete since round 23 (pressure is no longer a hard stop; only a run process above 8 GB is). The try was therefore ended wrongly by this agent. No repo-side failure was seen: at the stop, the graph step was still progressing (`1007 inferred here, 679 left`).
- Fleet: `0 of 1686 projects arrived from the fleet`; `the server last progressed 93 s ago` at the peak sample, 376 s at the last line.
- Per-project limits: `graph: project semio-plugin-actor-input (language:cargo) grew by 1775 MB, more than the 1756 MB this machine gives one project; ... marked too heavy here` (also `semio-os-vcs-entity-identity`, same 1775 MB).
- Peaks: run process 2337 MiB, slice workers 8084 MiB (below the 8192 MiB stop), system top 7160 MiB; peak sample 09:19:26Z at `1049 inferred here, 637 left`. Closure entries 34694 before, 34962 after.
- Tail: round-27.try2.tail.txt (also round-27-try-2.tail.txt); state.json says native_rc 7.
- Pending: fix the helper's stop rule, then run try 3 (try 2's stop does not count as a repo error).

# Bounded Native Exit Investigation

Recovered native60261 exited1 after about2450s during objectsEN lifecycle, without Python traceback or final uncovered-result.json. Last durable pertestprogress records11tests/11subtest failures/3errors at573.891s; these include moving-input/stale-fixture failures and cannot serve as a final source certificate. The original assertions were not flushed by its unittestresult until cohortend. Root reported a similar launcher60474 hardexit after~1310s during native ramp export.

Read-only authoritative process path:tool-registry command returns ToolRun.status/signal; runToolV1 catches OwnedCommandFailure and returns those fields; CLI prints no terminal diagnostic under check=false and forwards status. Owned settlement preserves native code/signal or rejects unknown terminal. Default runOwnedCommand timeout is0; declared native-child progress idle1h/base4h and native-observation idle10m/base1h do not directly explain the observed durations. Windows CPU-time unavailability is explicitly fail-safe. Existing host-capacity records are ephemeral; currentfleet noalerts is not historical proof. No resource policy/launcher changes were made.

Potential Windows console-wide CtrlC correlation was raised as UNPROVEN because60261 exit observation occurred near another owneddiagnostic cancellation. No cross-process cancellation experiment will be run while unrelated jobs arelive. All current ownedlongdiagnostics expose cooperative cancellation files. No unsupported OOM,deadline or cancellation cause is claimed.

Fresh bounded diagnostic81405 runs only objectsEN in a new Python process under the registered Bun→UV process interface, using normalcapacity. It preserves raw ToolRun status/signal beforeCLI return-code normalization, actual PythonPID,one-second ownRSS/private/peak events,60-second faulthandlerstacks, authoredbeatbegin/end and immediate complete test/subtest failures. Limits900s/5.5GiB ownRSS and960s supervisorfallback are diagnostic bounds, not policy overrides. Outputdir generated/native-r92/p/20261030003938561. This is a selected diagnostic, not a fullcohort/movie acceptance.

## Bounded objects EN run 81405

Registered Bun supervisor retained raw UV terminal status **5**, signal null, at240.735s, before900s/5.5GiB bounds. No cooperative cancellation request or final unittest outcome. Durable per-second memory samples and stacks are in generated/native-r92/p/20261030003938561. This is an actual child terminal, not merely outer CLI status1; its cause is not yet established.

Independent crash review correction: the exact fatal leaf is Manim0.22 color/core.py _internal_from_rgb; earlier _insert_submobjects belongs to an interleaved60s timeout sample, so it is not established as the crash caller. Fatal leaf is numpy.asarray(rgb,dtype).copy/resize; no concrete buffer/alias cause established. Observer Win64 memory-counter layout was independently checked and no overflow found. Lastsample239.641s RSS980MB, peak1.028GB/private2.005GB.

## Scheduled dump watchdog isolation

Launcher owner reproduced access violation/rawUV5 in three fresh pure-stdlib Fraction processes with scheduled faulthandler.dump_traceback_later; identical work without scheduled dump and GIL-held Python traceback snapshots passed. This causally explains the instrumented81405 crash, not earlier uninstrumented exits. My own35645 source preflight used the same scheduled60s mechanism, so it is cooperatively superseded and the harness now uses Python-thread sys._current_frames/traceback snapshots while retaining only fatal exception handler. Geometry results before cancellation remain diagnostic.

Owned-script audit found one additional scheduled native watchdog in native-geometry-profile-r92.py; replaced it with safe Python-thread snapshots retaining90s cooperative bound. The intentionally unsafe native-trace-isolate-r92.py timer mode remains a retained diagnostic negative-control input owned by launcher; it is not part of final native/render execution. Main native-clips and uncovered runners contain no scheduled watchdog.

Read-only bounded metadata ownership audit for rootqueried PIDs53880/24828/23300: searched own native-r92 metadata/log files<=1.5MB and explicit lifecycle started records; details reported to parent. No control performed. Activequeued/currentboardjobs are complete at this checkpoint; no owned livePython intentionally running.

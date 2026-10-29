# Explicit Capacity Budget for Direct Print Tests

## Supported Admission

The public capacity command documents and implements:

`repo capacity run --memory=1024 --cpu=1 -- repo tools exec --light bun -- test <authored-entry>`

This is normal, explicit budgeted admission. The capacity command validates positive integer memory/CPU, creates a batch command request with fixedMegabytes1024 and one job, and calls the same withHostCapacityV1 gate as other heavy commands. Its spawned child receives the genuine admitted capacity grant/plan environment. The inner light tool execution prevents a second default2048MB request; it does not remove the outer admission. The test issuer, Original, compiler/material interfaces and artifact ownership remain unchanged. No hand-authored grant environment is needed.

The gate keeps the normal host floor, fair queue, session accounting, cancellation and heartbeat process-tree observation. The capacity command forwards SIGINT and SIGTERM to its child and returns the actual exit result. The gate observer defaults to observeTreeMegabytesV1 on the wrapper PID, so the tool process, Bun test process, compiler and worker descendants are accounted together. A fixed memory request is a reservation/accounting input, not a kernel-enforced RSS ceiling. Observed excess is still visible and enters subsequent admission accounting.

Relevant sources are repository Build host-capacity/command and Process host-capacity/gate. The existing capacity gate tests cover inherited grants and actual observed tree accounting.

## What Can Be Inferred About1024MB

The graph-free test removes the multi-gigabyte global graph materialization. Publication/material preparation is serialized. Material policies cap compressed Tectonic archive32MiB, inflated binary128MiB, TeX bundle32MiB and font roster32MiB. PDF input is capped64MiB and PDF.js worker old-generation heap256MiB. These bounds do not by themselves require more than1GiB.

They also do not establish a1GiB process-tree peak: compiler native allocations have no RSS limit, PDF.js has external/native allocations beyond its old-generation heap, and the wrapper, issuer, test runner and worker footprints coexist. The Tectonic executable file-size limit is not a compiler-memory limit. Thus1024MB is a plausible explicit initial estimate for this serialized graph-free path, not a measured guarantee. A1024MB reservation should not be described as a proven bound or as a hard memory limit.

## Cheap Measurement Option

The existing genuine-birth smoke test can be selected without preparing materials or compiling documents. A supported measurement run can use an explicit512MB capacity wrapper and the anchored test filter `^genuine native thesis rendering birth$` against the existing rendering entry. Normal tree accounting can observe the wrapper and issuer baseline; an optional ticket-owned preload can report test-process peak RSS after settlement. That baseline alone still cannot prove total compiler/PDF peak. The previously successful smoke tests did not record RSS, so this review cannot honestly claim a measured baseline.

No smoke or heavy command was launched during this read-only research. The parent task owns cancellation/scheduling of the queued default request and any explicit-budget replacement. If a smaller budget is chosen, launch exactly one admitted command, observe its real tree and host floor, and keep the genuine producer/consumer and test issuer unchanged.

## Measured Birth-Only Baseline

At the parent's explicit request, ran only the existing anchored genuine native thesis rendering birth smoke through repo tools exec light Bun, with the complete compiler/material test filtered out. Linux x64, Bun1.4.2. Final instrumented run: one test passed, one filtered out, zero failures,351ms test-run duration. No material preparation, compiler, publication, PDF worker or graph job ran.

The retained ticket input rss-preload.ts reports process.resourceUsage().maxRSS and reads only the immediate parent process's Linux status facts. Test PID315528 had peak126056KiB=123.1015625MiB; current RSS at report117.359375MiB. Immediate parent tool Bun PID315498 reported VmHWM71356kB=69.68359375MiB and VmRSS64140kB=62.63671875MiB. Linux proc kB and resourceUsage maxRSS are interpreted as1024-byte KiB here. Sum of these separate high-water values is192.78515625MiB; it is not a simultaneously sampled process-tree peak and excludes any additional outer capacity wrapper/ancestors.

Bun test did not emit the registered process exit hook in the first successful smoke, so the preload also registers a guarded afterAll report. The final measurement is therefore taken at test settlement before final process teardown; the same guarded report remains registered on exit. The first command's missing ./ test-path prefix was rejected before Bun started by the repository operand guard and was corrected.

This measured baseline supports a substantially smaller issuer/runner footprint than the default2GiB reservation, but still does not establish compiler/PDF/material peak. The1024MB full-publication reservation remains an explicit estimate under normal observed admission, not a guaranteed ceiling.

Full publication77983 admitted with4261MiB available above the existing1981MiB floor. The private store smoke passed8.13seconds. A process-tree observation during material preparation found286.41MiB aggregate RSS across the capacity wrapper, tool runner and Bun test (PIDs365839/365840/367718/367749). This is an instantaneous observation before compiler execution, not a measured peak or hard bound.

During actual publication5888 compilation, two instantaneous process-tree observations measured546.23MiB and548.51MiB RSS, including the outer grant/tool runners, test process, actual compilation child and live Tectonic process. The compiler PID changed from387610 to388944 while the same child remained active, consistent with the authored multiple-pass pipeline. These observations support the1024MiB estimate but are not a synchronized peak or successful publication proof.

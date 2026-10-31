# Linux Process Census

The real captured runtime profile attributed251.3seconds of its526.3second span to spawnSync through liveChildPids/leaseActivity. Samples include native wait and are not a measurement of OS CPU time. Existing standing/activity/watchdog cadences remain30seconds/15seconds/30seconds; no deadlines, limits, grants or cleanup decisions were relaxed.

The Linux implementation now reads a bounded numeric /proc PID table and each process stat directly, with no process launch. It preserves observer exclusion while traversing the observer's descendants, checks queried-root identity before/table/after, binds CPU observations to the returned PID start epochs, and reports unknown on permission errors, malformed data, reused identities, excessive depth/roster or unsafe CPU totals. A vanished nonroot row is skipped as in a system process snapshot. This API observes metadata and sends no signals.

The kernel's [stat format](https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html) defines parent PID, user/system CPU ticks and process start ticks. Clock conversion uses the actual ELF auxiliary-vector AT_CLKTCK value, whose [kernel tag is17](https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/linux/auxvec.h); [getauxval documentation](https://man7.org/linux/man-pages/man3/getauxval.3.html) identifies this rate with sysconf(_SC_CLK_TCK). No100Hz assumption is made. The [children interface](https://man7.org/linux/man-pages/man5/proc_tid_children.5.html) can omit surviving children during concurrent exits, so it is deliberately unused.

Schema-first neutral tests cover topology, observer traversal, malformed/denied/missing observations, PID reuse, exact depth64/65, clock rates and32/64-bit endian auxiliary vectors. SQLite independently computes descendant closure and CPU sums; Ajv validates the protocol corpus. A Source-derived actual watchdog dispatcher proved a causal red before integration and now makes no external Linux census call. All27tests/84assertions passed, including an actual registered Node child that exits itself after1.5seconds and the independent process.cpuUsage accounting oracle.

Actual Node24.15.0 kernel proof:10fresh queries, min5.99ms, max13.33ms, average7.98ms; all returned known results. Own process CPU140ms agreed with independent149.894–150.288ms accounting within kernel tick granularity. These observations are a bounded host sample, not an end-to-end performance guarantee.

The actual-options full semantic check97058 found only two owned typing errors over647files. They were fixed; normal-admission retry51898 is pending. No compiler bypass was used. Durable quick-class suite/target registration is owned by Bootstrap and the canonical aggregate now derives185targets. Actual RootCLI daemon acceptance awaits semantic/source freeze and Course's independently tested Native pressure correction.

Production changes:
- P/routing/invocation/process/watchdog/TS: Linux dispatch only; macOS/Windows system paths retained.
- P/routing/invocation/process/watchdog/linux/{TS,schema,neutral-corpus,tests}: bounded platform observation and portable laws.

No Native activity, lease, custody or process-termination authority is created by these observations.

Normal-admission semantic9759 finished0 with814importedfiles/3roots and ALLdiagnostics0. The final test-only callback type was fixed after51898; no imported diagnostics were ignored. Runtime Source frozen for the coordinated canonical RootCLI daemon proof. Durable suite now uses existing quick300000ms class, Node tool grant, Source freshness/live metadata and185derived caching aggregate targets;49 preservation gate unchanged. Cross-runtime Node corpus finished0 with16topology/CPU cases,6auxv cases and31observations. Actual registered-child/kernel Bun suite27tests84assertions0.

Expanded normal-admission semantic88073 finished0 over5explicit roots/2865importedfiles including SQLite owner; ALLdiagnostics0. Native daemon13985 passed the initial provider/cold-store boundary and actually spawned, then exposed a bundled native addon dependency/origin fault; Bootstrap owns that Source/capture repair with guards preserved, no watch/dotenv success claim yet.

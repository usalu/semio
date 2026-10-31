# Capacity Sampling Read-Only Check

The existing gate does reuse a shared sample.json until policy.wait.sampleMaximumAgeMilliseconds expires; my earlier overlap hypothesis must not be read as evidence that every waiter always launches a probe. Source currentSample() checks the cache and awaits readHostSampleV1 before writing it. There is no shared in-flight sample lease in that function, and the admission loop calls it before takeLock(), so simultaneous expired-cache readers can independently launch bounded probes. This establishes a possible overlap, not its actual frequency or memory cost.

The Windows sample invokes one PowerShell command for Win32_PerfRawData_PerfOS_Memory plus pagefile use; optional signal is3s and the runner also has a hard timeout. Failure truthfully falls back to node:os.freemem() and partial/raised-pressure status. No runtime, policy, limits or source files were changed by this review. The actual cause of current host memory pressure still requires process evidence.

Sources: 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts and its 📡️sampling/🟦️.ts sibling.

Direct same-host node:os sample {"time":"2026-10-29T22:24:43.652Z","free_mib":2352,"total_mib":65447,"logical_cores":16} confirms physical free memory independently of admission log formatting. This does not identify the consuming process.

The existing read-only repo doctor interface (no lease/capacity admission) observed zero grants and only waiting repo runtime processes, roughly40–72MiB each, rather than a running nativePython memory owner. The complete process facts are saved under generated/capacity-doctor-facts-r92.json. The doctor lists repo descendants, so it does not identify unrelated host memory consumers. No command was terminated or admission override used.

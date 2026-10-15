# Whole Child Profile Aggregation Readiness

Root authorized preparing the retained aggregator while ALL12 runs. The actual ALL12 profile has not been read. No additional profiling, product task, graph/store runtime, production Source edit, planner or Stdio execution occurred.

The existing `cpu/📜️script.ts` now supports `--whole-child <profile> <output>`. It creates an original 60-second invocation for a standalone diagnostic, or receives the exact existing finite invocation without extending it. An inherited ordinary or greater-than-60-second policy is refused. The input and output must be distinct ticket-generated paths.

The read uses a regular, single-link, no-follow descriptor, verifies identity against pre/open/post metadata, allocates at most the admitted size, reads at most 256 MiB in bounded chunks, and refuses changes. It accepts at most 500,000 nodes and 2,000,000 samples, rejects invalid trees/samples/deltas, and caps independent oracle traversal at 10,000,000 rows and output at 64 MiB. Original checks and awaited progress occur throughout bounded loops; synchronous parse/SQLite phases are checked before/after. A refusal never raises limits.

Self counts accumulate once per sample. Inclusive counts use iterative tree/subtree aggregation, adding a frame only at its topmost exact function/path/line/column occurrence on each ancestor path; recursion is not double-counted. This avoids constructing and serializing a full stack for every sample. The independent SQLite oracle starts from raw samples, groups their node counts and recursively walks parents, then deduplicates each originating node's exact frame before summing. It verifies all self and inclusive counts, not just top rows.

The output represents the complete child profile only. It makes no phase/normalizer windows without exact alignment. Sample shares are separate from elapsed-delta weights; native/unattributed, explicitly named idle/GC and runtime/dependency/source categories remain separate. Missing idle/GC labels are not treated as proof of absence. Child-process CPU is excluded. Inclusive shares overlap and must not be summed. Source declaration lines are one-based; full hot stacks and exact frame columns are retained.

The language-neutral `cpu/🔣️.json` fixture contains a genuinely recursive repeated A frame, a B descendant, C leaf, unknown native leaf and explicit idle/GC pseudo-frames. It independently specifies self/inclusive counts, recursion deduplication, elapsed weights, tail time and line/column conversion. `cpu-test/📜️script.ts` first failed against the prior unsupported whole-child interface, then passed the new mode and six malformed profiles: sample/delta disagreement, negative delta, unknown sample, duplicate parent, cycle and orphan. All test children retain one original 60-second owned command chain. The actual terminal-zero test is retained in `🗑️generated/cpu-whole-green.txt`; initial functional RED is `cpu-whole-red.txt`. These are synthetic aggregation tests, not ALL runtime acceptance.

After Root confirms terminal completion and profile flush, the exact intended command from the repository root is:

```text
bun <absolute ticket>/cpu/📜️script.ts --whole-child <absolute ticket>/🗑️generated/cpu-all/all-22359.cpuprofile <absolute ticket>/🗑️generated/cpu-all/all-22359-aggregation.json
```

No command against that actual profile has run yet. Root must explicitly hand off the flushed artifact before execution. Production Source/tests and final all-pin planner/Stdio remain held.

## ALL12 Artifact Outcome

Root subsequently confirmed ALL12 terminal exit 1 and both owned PIDs absent, with no graph pin. The expected `🗑️generated/cpu-all/all-22359.cpuprofile` was absent (`ENOENT`), and the owned profile directory was empty. No actual full-child profile was available, and no aggregation against that path ran. Absence alone does not establish why no profile was emitted; no profiler, shutdown or runtime-cause assertion is made. The prepared aggregator and synthetic tests remain ready for a separately authorized, terminally flushed artifact.

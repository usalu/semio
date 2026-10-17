# Bounded Publication Witness IO Plan

## Actual Scope And Evidence

Root Tools76892/PID71105 reached accepted and observed, then refused at the unchanged120-second initial witness deadline. The retained Low proof is `native-tools-current-deadline-low-proof.md`; it does not contain entry/read counters. No measured attribution to file count, content throughput, directory width or ancestry cost is justified. Pending metadata and unresolved infrastructure custody remain the actual failure scope. This plan performs no Native operation, mounted walk, SDK open, Store admission or production edit.

The earlier nested design is saved in `nested-protected-parent-maintenance-plan.md`. That design remains unimplemented and has no effect authorization.

## Selected Custody Boundary

Keep the existing asynchronous `withArtifactDescriptorV1` owner. Its single private `scopes` Set reserves before open, shares the maximum32 policy, captures the real close method once, preserves arbitrary body primaries and retains ambiguous closure. The witness keeps at most one directory or file descriptor active. Do not introduce a synchronous descriptor API, another registry, numeric close retries, Promise-return policing, detached work or a new public interface in this first batch.

Within the existing owned file callback, use the real FileHandle's `fd` only for synchronous read-only `fstatSync` and `readSync`. The descriptor and its number remain local to that accepted callback. The asynchronous owner remains responsible for actual open and close, including cancellation immediately after acquisition and failures during closure. No synchronous read can occur before ownership admission or after the callback returns. The directory owner retains its existing asynchronous open/close and bounded `readSync` enumeration.

This selection removes Promise/worker-pool transitions inside content and single-row metadata operations without changing descriptor lifecycle. Per-file open/close awaits remain. A separate synchronous owner is not justified by the current uninstrumented physical result; it would add a larger authority and thenable-lifetime surface. Source operation counts will state precisely what this batch removes.

## Concrete Algorithm

Introduce a private witness-local synchronous group helper. A group enters the full real `check()` (issued phase, deadline/cancellation and private publisher observer), performs only synchronous read-only IO, and completes another full `check()` before its results can feed a subsequent asynchronous continuation or a completed witness. Its catch retains the exact first failure, including `undefined` and frozen objects, while still attempting the post-observation. No physical observation is cached across an await. This explicitly retains the existing non-atomic external-writer limit; same-continuation grouping is not an atomic filesystem snapshot.

Every actual IO in a group independently checks the real issued control and elapsed/cancellation state before and after the syscall, charges its existing work, and validates its result before consumption. The full private observer brackets the group; it is not replaced by a copied control or a cached scope. This is the same observation granularity already used by current bounded directory and ancestry groups. No observer or work check is simply removed.

For ordinary files, preserve the existing sequence: named lstat, same-device/type/link/size admission, actual owned nofollow open, initial exact FD stamp, complete content stream, one-byte EOF probe, final FD stamp, exact named stamp, entire ancestor verification, actual close, post-close named stamp and entire ancestor verification. Use a single existing64KiB buffer and the same incremental SHA state. A content group performs at most `policy.nameWindow` actual reads, each at most `policy.chunkBytes`: at most64×65536=4194304 requested bytes. Short reads count as real reads; the group does not loop beyond64 calls to fill a nominal byte budget. Zero-length files still receive the initial/final FD checks and EOF probe.

After each content group, await an actual checked `control.yieldContinuation()` before starting another group; continue the existing `progressWorkUnits=256` publication rule and final census publication. The group bound is derived from existing64-name and64KiB constants, not a new quota or timeout. This adds an explicit cooperative opportunity even when fewer than256 work units accumulated. Cancellation delivered by the event loop is checked before another read. A single synchronous OS call cannot itself be preempted; no wall-clock latency bound is claimed.

Replace isolated awaited metadata reads with equivalent synchronous read-only operations inside checked groups: lstat/fstat/readlink. Preserve all stat fields and comparison functions, symlink byte cap, real socket UID/GID/inode metadata, service identity/UUID checks, raw journal SHA, lexical ordering and frame encoding. Keep ancestry groups complete: up to64 outside plus64 payload ancestors are validated in separate existing64-row groups; never truncate128 to64. Do not retain an unbounded entry array, descend before a completed directory postcheck or change the repeated bounded lexical page algorithm.

The witness still performs two complete canonical census streams per capture. The producer still performs its before and after captures, hence four complete streams for mounted preservation. Full digest, application digest, reserved service digest and raw journal semantics remain unchanged. No exclusions, service relaxation, frame reordering, payload shortcuts, census sampling or work admission increase are part of this batch.

## Source-First Contract And Gates

Extend the existing witness schema/corpus with closed synchronous-IO group cases before implementation. Cases describe byte/read counts, first/middle/last fault position, genuine cancellation or finite deadline expiry, growth/short reads, FD/name/root/backing/each ancestor substitutions, body `undefined`/frozen failures and close failures. Preserve the original exact body primary and existing unresolved-descriptor accounting. Include zero-byte, one-chunk,64-chunk and65-chunk files, and combined ancestry above64 rows.

Obtain actual semantic RED from the extracted current witness function with explicit filesystem model ports: current code must show asynchronous content/stat operations where the closed new contract requires synchronous groups. This is an operation-boundary RED, not a fabricated timing failure. Then GREEN must run the actual function and actual descriptor owner with real issued controls, real publish/yield methods and observability-only counters. Model ports do not prove a mounted filesystem or Native provider.

Use Ajv for the closed corpus and SQLite for independent read/group/yield/work arithmetic. Reuse the existing independent Python canonical stream/SHA oracle on disposable Generated trees, including journals, symlinks and the supported socket frame. Do not copy Source into the ticket. Record actual operations, maximum consecutive read calls/bytes, yield boundaries and maximum active descriptors; no Source microbenchmark is a physical speed claim.

Fault laws cover first/middle/last failures in a group, phase expiry during a synchronous operation, cancellation observed after a real yield, private observer refusal before/after a group, growth/short read, arbitrary primary plus successful/failed close, close getter failure and ambiguous close retention. Assert no next read/frame/final witness after refusal. Unresolved custody stays in the same32-owner accounting and continues to make `requireArtifactDescriptorClosureV1` refuse destructive work; this plan does not change that existing deny-only policy into a new global admission policy. Keep existing actual-function fault tests; focused AST laws alone do not replace them.

Run the existing complete witness/publication Source selectors and actual focused strict roots with explicit `--no-sdk` where applicable. Low independently reviews current held bytes, all post-await checks, group bounds, same custody registry and exact wire/digest semantics. Root will choose any later private Native and fresh Tools qualification; no runtime retry follows from Source GREEN.

## Exact Intended Footprint

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts`: private checked synchronous read groups and cooperative boundaries only.
- Its verified existing `🧬️schema/🔣️.json`, `🧪️tests/🔣️.json` and `🧪️tests/🟦️.ts`: closed corpus, actual-function gates and independent oracles.
- Existing ticket `📜️script.ts` selectors only if the additional law needs a focused selection; no new script filename.
- Existing ticket reports/inventory/Generated receipts.

No descriptor owner/facade change is proposed; no P owner, Fixed provider, helper physical grammar, Native grants, Original, child/time/output limits, ACK, reservation, image capacity or512-file Source bound changes. Production editing remains held until Root reviews this concrete plan.

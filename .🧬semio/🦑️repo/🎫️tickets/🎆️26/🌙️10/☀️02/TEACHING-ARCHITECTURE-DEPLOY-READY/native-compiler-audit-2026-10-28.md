# Final Status

Final frozen-source inspection supersedes intermediate findings below: prior source-closure, funnel suffix, resolution-cache and host/target projection defects are repaired for the demonstrated host route. No independently reproduced remaining defect blocks that route. See the final addendum for evidence limits and cost.

# Native Compiler Integration Audit

Date: 2026-10-28. Independent source inspection only. No builds, compiler probes, runtime execution, Git mutations, or source edits were performed by this audit. Integration remained under active implementation.

## Sources Inspected

Shared process tool registry, resolver, native-compiler selector/shim/environment implementation, language-neutral selection and precedence fixtures and Bun tests; repository compiler store integration, native-compiler issuer, and effective Cargo configuration capture.

## Findings

1. **Unresolved source identity closure:** the issuer hashes the selected GCC executable, host ld, libc.so/libm.so linker scripts, SDK stdio.h/sqlite3.h, generated shim, and its own module. It does not hash the complete compiler inputs: GCC cc1 and supporting executables/libraries, crt objects, actual libraries referenced by the scripts, or other SDK/transitive headers. An in-place change to an untracked input leaves the profile digest unchanged and passes the current input recheck. Therefore the current implementation does not establish full source-based compiler profile/receipt invalidation. This was sent directly to the implementation owner.
2. **Caller library ordering repaired during audit:** latest shim appends its host binutils/library/header arguments after quoted caller arguments. Consequently caller -L archive directories precede fallback /usr/lib. Resolver now exposes suffix arguments, but this audit did not execute every resolution consumer to establish suffix propagation.
3. **Cross-target boundary exists:** projection returns the original environment unchanged unless the target begins with the selected host machine and ends in -linux-gnu. An x64 selection does not add CC to aarch64 or wasm target configuration. HOST_CC and target-specific compiler environment entries survive object spread; however a global CC default is added whenever CC itself is absent. Actual downstream build-script precedence was not executed or established by this audit.
4. **Explicit configuration preserved in projection:** explicit CC is preserved; CFLAGS, compiler search paths and unrelated environment entries survive. Configured effective target linker, target linker environment, and detected explicit Rust linker flags prevent PATH defaulting. The integration does not inject Rust codegen flags or a target.linker selector. Existing flag admission remains called after projection.
5. **Owned alias lifetime:** alias identity includes its text and tracked input hashes and lives at n/<digest>/cc inside the compiler store. Issuer authenticates storage/control, rechecks live storage before publication, and checks executable, read-only, single-link regular alias bytes and tracked original bytes. WeakMap scope checks reject a retired or replaced issuer; finally retires the scope. These are source observations, not adversarial execution results.
6. **Configuration capture:** projected environment and native identity enter the immutable Cargo configuration digest before the view is published. View recheck calls the native profile recheck. A changed tracked input therefore fails the live view rather than silently passing; untracked inputs remain finding 1.
7. **Quoting/platform/API:** shim individually POSIX-quotes compiler and default arguments, including apostrophes, and preserves caller boundaries using quoted dollar-at. Discovery only activates for supported Linux architectures; Windows/macOS keep existing setup. New shared API uses plain objects and built-in Node platform typing; no third-party runtime package or exported third-party compiler class was introduced. These platform/quoting claims are inspection findings without new execution.

## Evidence Limits

Existing language-neutral fixtures cover Linux selection/newest SDK, spaces, no SDK, Windows/macOS no-op, explicit CC/linker/configuration/search paths, plus a cross-target assertion. They do not establish authentic owned-store lifetime rejection, complete transitive source identity, real Cargo linking, paths containing apostrophes, or downstream HOST_CC precedence. Earlier owner-reported compiler probes are not independent runtime evidence from this audit. Main graph/frontend gate status remains owned by the parent.

## Follow-up Inspection: Closure, Performance, and Funnel

Source remained on the limited input list at follow-up start. Owner acknowledged the closure gap and proposed bounded GCC/SDK/header/startup/runtime input discovery. No completed closure repair has been inspected yet. Rehashing an entire SDK synchronously on every configuration/control recheck would materially increase repeated work; a correct implementation must choose explicit byte-validation boundaries or actual owned immutable snapshots without substituting path/receipt strings for source evidence.

Two additional concrete funnel defects were reported to the owner:

- tools/🔌️funnel/🟦️.ts currently constructs arguments from prefix and caller arguments only. It drops the new suffix. By comparison, all inspected shared runner modes and probe append suffix after caller arguments. Generic owned-command execution through the funnel therefore loses the SDK host/header defaults.
- That funnel caches resolution using repository root, tool id and command name only. Changes to PATH or compiler selection environment do not affect its key. A native cc resolution cached before owned alias projection can be reused after projection, bypassing the intended alias resolution. The generic cache predates this change, but the native compiler integration makes this distinction relevant.

Current native compiler tests do not include shim apostrophe quoting/suffix ordering, authenticated store mutation/lifetime, or explicit HOST_CC/target compiler precedence cases. Owner is preparing these changes. No test or runtime execution was performed in this follow-up.

## Current Reinspection Checkpoint

The shared tool funnel now appends resolution suffix after caller arguments and bypasses cached resolution specifically for the cc tool. These two previously reported defects are repaired by source inspection; no execution was added.

The shared native-compiler module now contains nativeCompilerSourceClosureV1 with an injected filesystem port. It hashes actual file bytes, directory member names, and symbolic-link target/canonical identities under explicit 32768-file, 8192-directory, 512-MiB-total and 64-MiB-file limits. It checks cancellation per entry and yields every 64 entries. Byte-based yielding would reduce the largest synchronous interval; this was sent to the owner. At this checkpoint the repository issuer still uses the original seven-file input list, so the helper alone does not resolve the release blocker. HOST_CC/target compiler precedence and expanded tests also remain unmodified at this checkpoint.

Source paths: shared implementation and schema under framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler; shared tests under tools/🧪️tests/🛠️native-compiler; funnel under tools/🔌️funnel; issuer under repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛠️native-compiler. These abbreviated paths identify the exact inspected domain folders; they are not evidence of runtime success.

## Final Frozen-Source Addendum

Re-read the final owner report and shared native compiler, schema, tests, funnel and runner plus product issuer/effective configuration source. This audit ran no new tests/builds/probes and did not infer execution from receipt paths.

The issuer now obtains nativeCompilerSourcePlanV1 and hashes nativeCompilerSourceClosureV1 before deriving its alias digest. The closure covers SDK include/libgcc/libexec directories, actual bytes, membership, symlinks, /bin/sh, host as/ld, startup/runtime candidates and recursively parsed ELF interpreter/DT_NEEDED dependencies. Native identity is serialized into the immutable Cargo configuration digest. Every native view recheck repeats actual byte closure comparison and verifies the owned alias. Scope finally retires the WeakMap capability; a retained recheck rejects after retirement. Prior seven-file closure findings are superseded.

Suffix defaults follow caller argv in the quoted POSIX shim, resolver, shared runner modes, probe and funnel. Individual shell tokens are quoted and quoted dollar-at preserves caller argument boundaries. Both runner and funnel bypass cc resolution-cache reuse, preserving changed compiler search choices.

Projection uses separately observed rustc host and requested target. Exact supported host triples receive host defaults; cross targets retain their target choices and receive HOST_CC only when appropriate. Explicit CC/HOST_CC/TARGET_CC/per-host compiler names, effective Cargo environment names, linker selectors, explicit Rust linker flags and declared search-root controls are considered before projection. No codegen/linker selector is injected and existing Rust flag admission still executes.

Inspected feature tests now cover source byte/membership/symlink changes with an independent SHA256 oracle, bounded allocation and cancellation, ELF loader/search ordering, explicit search roots, configured choices, same-process compiler resolution, host versus target precedence and POSIX shell parsing using Python shlex. The tests inspected do not directly exercise authenticated issuer retirement/alias tamper with a live storage capability; those lifecycle statements above are source findings.

Owner reports 36 feature tests, standalone TypeScript, related41-tests, actual registered C compile/ctypes execution and actual authenticated scheduler check/link/runtime all passing. These are explicitly owner-reported executions, not independently repeated by this audit. The final source profile is reported as8557entries/451504928bytes, including shell; hashes identify that profile but are not themselves runtime proof. Main application/frontend gates remain separate.

Remaining cost: actual source revalidation still rereads the entire roughly451MiB closure on each native profile recheck. It yields after64entries or8MiB and checks cancellation each entry; it is not an immutable snapshot or metadata cache. Owner measured roughly2.5seconds for a standalone closure. This is a known bounded overhead, not an unmeasured claim that repeated checks are cheap.

One configuration boundary was raised with the owner: ELF closure planning reads pre-Cargo LD_LIBRARY_PATH, while configured Cargo loader-root variables are not in the current default-veto list. No demonstrated host invocation used such overrides and this audit has not reproduced a failure. It should not be described as validated support for arbitrary configured dynamic-loader injection.

# Native Toolchain, 2026-10-28

The host native compiler prerequisite is working. Owned scheduler check, normal-default-cc link, and actual scheduler host execution all passed. Host C compilation, linking, libc calls and execution also passed independently through registered tools. No frontend or architecture-course application assertion is claimed by this report.

## Implementation

The declared cc tool uses an existing Linux Flatpak SDK only when a native compiler and explicit toolchain search-root choices are absent. SDK GCC supplies compilation and headers; host as/ld/runtime libraries supply native linking. Caller arguments precede adapter defaults so Cargo and caller library directories win. Windows/macOS retain their existing native setup. No root protection, global packages, SDK files, peer containers or Git history were changed.

An authenticated compiler-store scope issues its own content-addressed, read-only cc driver. The immutable Cargo configuration scope projects CC/HOST_CC and PATH only after effective target/host linker and configured environment choices are known, before flag/view/graph/source capture. Cross builds retain target compiler/linker choices while providing the native host driver for build scripts/proc-macros. Rust flag admission is unchanged. The active storage-keyed capability and exact source byte rechecks supply authority; receipt/configuration strings alone do not.

Automatic SDK defaults are declined for explicit SEMIO_TOOL_CC, COMPILER_PATH, GCC_EXEC_PREFIX, LIBRARY_PATH, CPATH, C_INCLUDE_PATH, CPLUS_INCLUDE_PATH or OBJC_INCLUDE_PATH, including deliberately empty values. Configured Cargo root controls also veto defaults; configured compiler names remain Cargo-owned. Original environment/configured values are not overwritten or reinterpreted, and no fallback warning flow was added.

## Native Sources and Bounds

The source closure includes the SDK include/GCC runtime/GCC libexec trees; directory membership and symlink targets; /bin/sh, host as/ld and native startup/runtime libraries; recursively resolved ELF loader/DT_NEEDED sources; and the tracked adapter/schema source. Traversal checks cancellation per entry and yields after 64 entries or 8 MiB. Bounds are 32768 file/link entries, 8192 directories, 512 MiB total and 64 MiB per file. Largest observed file was 49507736 bytes. The shell-inclusive standalone closure measured about 2.50 seconds.

The final actual product profile contains 8557 entries and 451504928 bytes, including the product adapter. Its issued driver is `/home/deck/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/n/1c8e2f675c59ac74c16c1740c9cee7f1bfd9aba49a38699a26deb154d562fa7d/cc`. Final immutable view: b9ce2dc0d17992df6a97de8c2f77d540d3d30f5664f2d8ed93ee2a4429553345. The view includes /bin/sh and the final explicit-choice policy source version.

## Validation

- 36 focused native compiler tests passed. Independent checks use Ajv, minimatch, Python shlex and Noble SHA256. Coverage includes platforms, absence, deterministic installed-SDK selection, spaces, caller library order, suffix propagation/cache behavior, host/target/configured environment precedence, explicit-root vetoes, source mutations/symlinks, ELF dependency ordering, cancellation and allocation bounds.
- Related resolver/funnel/tool-runner suite: 41 tests passed during integration. Strict standalone shared-native-module TypeScript check 35713 passed; product store imports passed.
- Authenticated scheduler check 75638 passed: prototype source admitted and check finished in 0.64 seconds.
- Authenticated scheduler build 56052 passed: normal default cc compiled/linked semio-build-scheduler in 1.30 seconds. That shell-aware build used profile 20c3d4044bb43f9a6a9dcd453b2cd5da68c3cba2ad815bac7d38982cbc0651be and immutable view e9f71ba468222cdd1a007bacfed1253153d606799417dbf54df1b2a9310a3fd9.
- Owned scheduler runtime 48643 passed with exit 0. Cargo reused the admitted prototype binary in 0.01 seconds and ran semio-build-scheduler rustc -vV on the host: rustc 1.99.0-nightly (9f36de775, 2026-07-19), x86_64-unknown-linux-gnu, LLVM 22.1.8. No forced or duplicate build was needed.
- Registered cc built the retained C probe with stdio/stdint/stdlib/sqlite3 headers, linked its real snprintf libc call, and registered Python loaded the own generated shared object through standard-library ctypes. native_c_probe returned 1 with exit 0.

No installed SDK/system source was mutated for testing. Source mutation/digest behavior was tested through injected source ports. Earlier relocated-linker-script and disposable offline-container probes were diagnostic only; the final implementation uses the clean owned native driver. Direct ticket Cargo execution was correctly refused by authentic compiler ingress and was not bypassed.

## Output Cleanup and Handoff

All own diagnostic objects, shared objects, executables, relocated linker scripts, temporary sysroot and isolated container cache folders were removed from ticket generated output. Handcrafted C/Rust/Cargo inputs remain. Shared logs, other agents generated outputs, reports and managed compiler-store aliases were preserved. No own live handles remain; parent-owned frontend/backend/global gates were left untouched.

## Exact Authored File Ledger

These entries identify this implementation scope. Concurrent CapacityRule/CapacityCommandCase changes in shared registry schema/runner and peer runner tests are excluded from this ownership claim.

- created: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🟦️.ts`
- created: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🧬️schema/🔣️.json`
- created: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🛠️native-compiler/🟦️.ts`
- created: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🧫️fixtures/🛠️native-compiler/🔣️.json`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/📖️registry/🟦️.ts`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔎️resolve/🟦️.ts`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔌️funnel/🟦️.ts`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`
- updated: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧬️schema/🔣️.json`
- updated: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-registry/⌨️command/🟦️.ts`
- created: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛠️native-compiler/🟦️.ts`
- updated: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`
- updated: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts`

Retained authored ticket inputs and report:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/native-toolchain-2026-10-28.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/native-toolchain-probe.c`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/native-toolchain-probe.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/native-toolchain-c-probe.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/native-toolchain-probe/Cargo.toml`


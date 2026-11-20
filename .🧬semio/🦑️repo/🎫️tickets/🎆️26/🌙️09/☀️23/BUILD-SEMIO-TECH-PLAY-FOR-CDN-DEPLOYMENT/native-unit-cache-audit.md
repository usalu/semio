# Native Unit Cache Retry Audit

The current release driver supports the native unit cache. `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🟦️.ts` invokes `buildReleasePipelineV1`; its compiler store in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` supplies a unit-cache root and wrapper. `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` enables that wrapper when both exist and `SEMIO_UNIT_CACHE` is not `off`; cached units are reported to the broker with recorded provenance. The wrapper falls through to the compiler scheduler on a miss (`⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts`).

Matching populated units should reduce repeat compile time, but a run with the cache off does not populate it and an initial enabled run incurs cache writes. Actual hit rate and savings for this release are unmeasured. Fresh generations have private target, build and stage stores; their lane root is normalized for the separate unit cache (`🧩️component-pass/🟦️.ts`). A fresh generation therefore does not by itself imply disabling the unit cache.

At inspection, `df -h` reported 26 GiB available on the repository volume. This snapshot does not establish that a complete release and additional cached units fit, especially alongside concurrent work. Compiler storage has physical headroom admission and cache pressure recovery (`🏗️compiler/📏️capacity/🟦️.ts` and `♻️recovery/🟦️.ts`), but those protections do not guarantee the build finishes or preserves all cache entries. No Cargo, build, or cache mutation was run for this audit.

## Isolated Compiler Verification

`native-unit-wrapper-probe.ts` exercised the actual POSIX unit-cache shim and native driver's `compilerWrapperEnvironmentV1` with the repository-pinned compiler, Rust 1.99.0-nightly (`9f36de775`, July 19, 2026). It invoked the shim in Cargo's argument shape without starting Cargo. The scheduler position held a recording shim that forwarded to the actual pinned `rustc`. It also verified that `-vV` reached the same compiler and that an explicit `SEMIO_UNIT_CACHE=off` produced no wrapper overrides.

| Compile target | Decisions in separate empty output lanes | Initial key prefix | Changed-source key prefix |
| --- | --- | --- | --- |
| Host `aarch64-apple-darwin` | miss, hit, miss, hit | `dc9121ed` | `c3efb9c7` |
| `wasm32-wasip2` | miss, hit, miss, hit | `b920ba16` | `2b626914` |

The probe changed the returned value from 41 to 137. Each edit generated a distinct key and different library bytes. Each hit restored matching `.rlib` and `.rmeta` hashes and rebased both the dependency file and compiler artifact announcements to the new lane. Eight wrapper invocations produced four real compiler compilations and four published entries. All cache, output, and temporary paths belonged to the ticket and the longest generated path was 230 characters. The generated directory was removed. Fleet cache and offload were disabled and the release cache was unused.

This test covers the local unit cache mechanics with the current wrapper implementation and driver environment. It uses the live wrapper entry supplied explicitly to the shim because a full seal inside a separate ticket directory would exceed the 256-character path limit. The earlier wrapper-chain audit covers the sealed shim's version query. The current probe does not cover the release's production scheduler, broker provenance replay, full dependency closure, or end-to-end disk and hit-rate behavior.

The checked-in ticket entry point still defaults to cache off with `SEMIO_UNIT_CACHE="${SEMIO_UNIT_CACHE:-off}"`. An explicitly enabled later retry can set `SEMIO_UNIT_CACHE=on` without changing the active build or the script's default. The local mechanics passed and do not supply evidence for changing the default before an admitted release run measures storage headroom and cache reuse.

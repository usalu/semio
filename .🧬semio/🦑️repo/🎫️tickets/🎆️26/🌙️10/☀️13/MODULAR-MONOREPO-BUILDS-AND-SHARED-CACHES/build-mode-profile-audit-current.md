# Build Mode Profile Audit

2026-10-15. Bounded source/configuration audit only. No builds, cache operations or tests launched. Read root Cargo profiles, `.cargo/config.toml`, actual process mode policy, Nx target mode transformer and component driver entry. The initial launch-catalog text query was too broad and truncated; no conclusions below rely on that output.

## Concrete Policy Gaps

1. Prototype inherits expensive dev overrides. `.cargo/config.toml` declares `[profile.prototype] inherits="dev"`, opt-level 0, debug 1, incremental true and 256 codegen units. Root `Cargo.toml:512` nevertheless sets `[profile.dev.build-override] opt-level=3`, and lines 522–615 set numerous named packages to opt-level 3. These inherited overrides take precedence over prototype's general opt-level. This means prototype builds optimize build scripts/proc macros/their dependencies and named runtime packages despite the compile-speed intent. The comments justify runtime speed, but that is an explicit competing policy. Move decisions under the two authored modes and intentionally qualify build-time execution versus compilation cost; do not characterize prototype as uniformly unoptimized.

2. `Cargo.toml:622` defines a third independently selectable `host-tool` profile inheriting release but disabling cross-crate LTO and using 16 codegen units. Its declaration is concrete; this bounded audit did not establish a live caller. If EVERY build must express prototype/release, remove independent selection or route host-tool tuning through the declared mode with explicit host-unit policy.

3. Wasm profiles are real target-specific variants, not identical optimizer settings. The central `buildModePolicy` maps prototype/release to wasm-prototype/wasm-release, so their names alone do not prove a third user intent. However `Cargo.toml:659` makes wasm-prototype inherit wasm-release with opt-level 1, debug info and checks; seven named package overrides at lines 672–695 force opt-level 2. Wasm-release at line 646 uses size optimization `s`, 16 codegen units and thin LTO. These are intentional latency/runtime/size tradeoffs rather than maximum compile speed and globally minimal size. Existing comments record measurements, but no measurements were rerun here.

## Confirmed Boundaries And Semantics

Actual process mode `process/🏗️build/🎚️mode/🟦️.ts` admits exactly prototype/release and rejects missing/alias values. Native mode maps directly to those Cargo profiles. The repository Nx transformer `library/🏗️build/🎚️mode/🟨️.mjs` adds both configurations and SEMIO_BUILD_MODE, with prototype as the default for an ordinary target; therefore an omitted task configuration selects prototype through policy rather than rejecting the omission. Fixed dev/serve names also select prototype. This is a mode-selection default, not proof of a third optimizer mode.

The component driver calls `readBuildMode(process.env.SEMIO_BUILD_MODE)` before its native driver, so that inspected path refuses missing mode. Root component help strings still expose wasm profile vocabulary; source comments are insufficient to establish that arbitrary profile arguments bypass actual mode admission, and no bypass finding is claimed.

Native release configuration is opt-level 3, thin LTO, one codegen unit, stripped debuginfo and no incremental. This does not force symbols stripping or abort panics. Those choices alone are not a correctness defect: runtime performance and minimum size require a measured tradeoff. Release build dependencies ordinarily use Cargo's separate default build override (opt-level 0, 256 codegen units); thus selecting release does not mean every compiler unit uses opt-level 3. Proc macros/build scripts are host work with their own semantics, not shipped target artifacts.

Cargo documentation confirms profile inheritance, override priority, built-in default profile selection and build dependency defaults. It also states configuration overrides manifest settings; `lto=false` can mean local thin LTO when opt-level is nonzero and multiple codegen units are present, so comments equating false with absolutely no LTO are imprecise. [Primary Cargo profiles reference](https://doc.rust-lang.org/cargo/reference/profiles.html).

## Limits

No exhaustive command inventory, effective rustc argument capture, target build, proc-macro timing, final binary size or throughput test was performed. Built-in Cargo dev/test/bench remain available to raw Cargo, but no mode-less production caller was established in this bounded inspection. Environment-based Cargo overrides can supersede the inspected static policy; their runtime sanitation was outside scope. No production files were changed.

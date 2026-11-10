# Proctor Native Runtime Delivery Audit

## Finding

The inspected proctor build and standalone product launcher do not themselves force static CRT or explicitly retain, copy, or project VC runtime DLLs for the launched product. This is a scoped source finding, not proof of the linked executable's imports or failure on a fresh Windows machine. Compiler flags supplied by dependencies, configuration outside these inspected files, or the actual PE imports were not established. Existing native passes establish the current host only.

## Build and capture

- 🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json:91-95 routes build to the proctor script; 📜️script.ts:36-42 invokes buildProctorProduct.
- 🏗️bootstrap/🧾️product/🟦️.ts:384-386 supplies Cargo build --locked --package teaching-proctor --bin proctor --message-format=json-render-diagnostics, with no explicit crt-static flag. Lines 401-419 retain the repository Cargo compiler through capture. The root .cargo/config.toml:10-12 sets -Z threads=0; its displayed target-specific flags are for wasm or Linux, and teaching/.cargo/config.toml was absent at inspection. The inspected teaching and proctor manifests contain no CRT flag. Caller environment is passed to the compiler route, so these observations do not establish every effective compiler flag.
- Repository Cargo launcher 🦀️cargo/🚪️launcher/🟦️.ts:195-201,233-239 uses the retained compiler store environment and configuration. Compiler 🟦️.ts:63-67 opens the pinned Rust toolchain and Windows compiler scope for the build. Windows compiler 🛠️native-compiler/🪟️windows/🟦️.ts:67-72 projects compiler and SDK bin, INCLUDE, LIB and MSVC linker into that build environment and retains verified compiler custody. This is a compiler-time projection, not a standalone product runtime projection.
- Product capture 🏗️bootstrap/🧾️product/🟦️.ts:335-381 copies and receipts one selected executable, its digest and source metadata. No companion runtime DLL roster is captured there.

## Standalone launch

openProctorProduct checks the receipt then launches product.executable with the supplied child environment (🏗️bootstrap/🧾️product/🟦️.ts:424-436). launchProctor in 🏗️bootstrap/🟦️.ts:403-413,452-464 makes a private copy of the executable alone and spawns it with that supplied environment. The launcher does not enter the Windows compiler scope or explicitly add the compiler bin/VC runtime directory. The alternate in-process build does hold compiler custody while its children run (same file:256-299), but that does not establish standalone product runtime delivery.

No commands were run to compile, inspect PE imports, install runtime components, or test another machine. Determining an actual deployment issue requires inspecting genuine resulting executable imports and the intended standalone machine's runtime contract; the scoped source alone cannot prove whether extra runtime files are needed.

## Focused Cargo flag follow-up

The actual repository Cargo launcher calls cargoArtifactArgumentsV1 (repo library 🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:238). That helper (framework process 📦️artifacts/🏗️native-build/🟦️.ts:194-208) selects the authored build profile and adds the original absolute compiler config and build.build-dir configuration; it does not inject a CRT flag. The older repository process relay also calls it at 🏃️process/🟦️.ts:81. devToolingEnv (🏃️process/🌿️environment/🟦️.ts:20-29) copies caller environment, removes Node inspector settings and adds build presentation/parallelism defaults; it does not inject Rust flags.

Compiler ingress 🚪️ingress/🟦️.ts:8 validates the incoming environment and chooses RUSTC and RUSTC_WRAPPER. Its 🌍️environment/🟦️.ts:4-9 decodes and admits every selected flag source; framework native-build/⚙️configuration/🎟️admission/🟦️.ts:7-18 validates configuration and flag sources. This is admission, not an injection of crt-static. Its target-feature syntax accepts caller-supplied features, so the exact caller environment and effective Cargo configuration remain relevant. The scoped Windows projection described above adds tool paths and linker selection, not a CRT flag. No explicit crt-static injection was found in this bounded trace; actual PE imports and dependency build-script flags remain unverified.

## Rust's documented CRT policy

The [Rust Reference](https://doc.rust-lang.org/stable/reference/linkage.html#static-and-dynamic-c-runtimes) describes x86_64-pc-windows-msvc as supporting both choices and dynamic CRT as the usual target default; it illustrates +crt-static for that target. This supports expecting dynamic CRT absent an override, but the reference's list of static exceptions is illustrative rather than a proof of this pinned toolchain's effective settings. The [rustc codegen documentation](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-feature) describes +crt-static and -crt-static, and later conflicting target-feature options override earlier ones. The Reference notes MSVC C code must be built consistently with the selected linkage and that unsupported targets may ignore the option; it recommends inspecting the output binary. The [known issues](https://doc.rust-lang.org/stable/rustc/targets/known-issues.html) note that target-feature alone does not normally rebuild all previously compiled standard-library and imported crates with matching features.

The scoped first-party admission accepts target-feature syntax, but merely admitting a possible override does not establish the actual effective flag. Therefore the present source configuration does not establish a guaranteed static-runtime product or prove a fresh-host failure. Before deciding runtime delivery, inspect the genuine produced PE imports and the effective Cargo/compiler and native dependency flags for the actual pinned build; apply and verify an explicit policy only if those facts establish a gap.

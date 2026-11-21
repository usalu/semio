# Native GIS2D Execution Audit

2026-11-21. Read-only investigation; no build, application launch, deployment, or remote job was started. Fleet and device availability are snapshots, not a proof of future availability.

## Native macOS entry points

The graph-generated playground targets are `repo run @semio-tech/framework-os-dev:run-gis2d-native-prototype` for the interactive native host and `repo run @semio-tech/framework-os-dev:smoke-gis2d-native-prototype` for the smoke path (specify `--lang=en` as needed). The generated target patterns are recorded in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🔣️.json`; `🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml` declares the `gis2d` playground. These managed targets prepare their dependencies and convey the issued invocation; directly invoking the internal script bypasses that envelope and is not a substitute. The runner in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🟦️.ts` selects the native binary and staged module manifest, sets WGPU and runtime assets, and starts the asset server where required. Its native executable receives `--plugin gis2d` and the smoke path adds `--smoke`.

The smoke runner `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs` loads plugins and boots shell state, but explicitly uses no GPU or window. A successful smoke is therefore not evidence of displayed basemap, tiles, interaction, or GPU performance; the interactive target and direct observation are still needed. Neither target was executed in this audit. The separate existing device verification records a cancelled native dependency attempt and no native GIS runtime result.

## Available runners and devices

`repo fleet status --lang=en` at 12:41 reported nine machines: eight macOS ARM and one Windows AMD64. Windows `g1-m05` reported worker absent because the pinned Rust toolchain was unavailable; it has a unique platform key and cannot exchange Rust units with the Macs. This is not evidence of an interactive Windows session or a deployable GIS application. No Linux machine was reported. The sole idle Mac worker was a build worker, not a proven interactive remote GPU session. Fleet compilation is not a device or GUI smoke test.

`repo remote-execution workers` remained in source admission and was cancelled; no additional worker capabilities were established. The local repo tool registry did not expose `adb`, and `adb` was absent from PATH. `xcrun devicectl list devices` and `xcrun simctl list devices booted` both failed because those developer utilities are unavailable. Consequently no connected Android or iOS runtime was established. Browser mobile emulation documented elsewhere in the ticket does not establish native mobile execution. No reachable, preinstalled, lightweight native GIS2D runner was identified.

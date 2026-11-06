# Kernel NativeReceive Preflight — 2026-11-05

Read-only owned Rust source-closure scan with cfg[] across five kernel package manifests, after identity peer fix. This is current static source evidence, not Cargo compilation. NativeReceive expansion 🌱️value/✨️derive/⚙️expansion/🦀️.rs:239 chooses native_owned_path, else crate_path, else ::semio_framework_value_native_owned.

semio-framework-kernel production closure 5 files/53757 bytes. Manifest 🎠️kernel/📦️packages/🦀️rust/Cargo.toml lacks direct native-owned; derives without crate/native_owned override are ClipboardFragment 🦀️.rs:81, Effect :119 and JobPlacement :451. Other five observed root/cold-pair derives explicitly set value(crate=::semio_framework_value), so the macro uses that override.

semio-framework-kernel-media-jobs production closure 1 file/41027 bytes. Its 🎠️kernel/🎞️media-jobs/📦️packages/🦀️rust/Cargo.toml lacks direct native-owned; default-path derives are IconRenderExportItem :101, VideoRenderPath :183, VideoRenderImage :195, VideoRenderOp :209, VideoRenderScene :221, VideoRenderRun :231 and VideoRenderProgram :244.

Identity closure has 2 files; its current manifest now includes direct semio-framework-value-native-owned. History closure has 1 file and no NativeReceive derive. UI closure has 3 files, with handwritten NativeReceive implementation using facade imports rather than derive. No edits or compile were performed.

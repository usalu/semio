# Artifact Snapshot Composition Bound

The next strict WASI pass reached plugin runtime and exposed missing composition bounds in preview, grouped dispatch and history operations. Require the existing first-party ArtifactCompositionFields trait on the shared ArtifactApp, ArtifactEditor and ArtifactViewer Snapshot associated types. The schema derive already emits this contract; this places the requirement at the app boundary used by SpaceMember instead of scattering bounds across each runtime operation. Fresh native and WASI checks will identify any handwritten snapshot implementations that need the same explicit schema contract.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs

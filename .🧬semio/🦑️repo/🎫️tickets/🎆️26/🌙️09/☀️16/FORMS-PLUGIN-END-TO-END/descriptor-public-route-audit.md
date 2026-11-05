# Descriptor Public Route Audit

The canonical descriptor flow has no equivalent supported direct Cargo emission route identified. The manifest declares os-plugin-describe-rs:build and :describe as Script owners. DescriptorBuildScript calls buildDescriptorEmitterV1, which captures the executable and completed native compiler inventory, then publishes both through an owned repository artifact destination. withDescriptorEmitterTaskV1 later verifies the published executable against that exact retained compiler receipt, input-reader custody, and file inventory. A plain repo cargo build could compile the Rust binary but would not execute this publication contract; a direct cargo run over a component would omit the descriptor emission owner’s staging, pair verification, and destination ownership.

The native Rust binary only delegates to semio_plugin_describe::run; its Cargo.toml declares no exporter test analogous to UI typegen. UI typegen’s output-variable test branch is therefore not a transferable descriptor route. No direct Cargo workaround was launched, no receipt was manufactured, and no descriptor bytes were manually written.

Keep the declared workspace:components describe forms playbook-module-procedural --profile prototype owner after the cold-Wasm inference source is stable. Its emitter build prerequisite supplies the publication and retained compiler inventory that the component describe stage actually requires. The previously refused standalone root Script remains diagnostic evidence, not an authorized substitute.

## Exact File-Script Entry Authority

The exact root router is the repository root 📜️script.ts, registering ComponentsScript. That entry calls runWorkspaceScriptMain with rootTestFreshCapabilitiesV1 only when no Original Script process envelope was received. The fresh-capability selector is schema-fixed: caching test suites, schema verify/entries/compile, Storybook test/build/dev, and declared time-travel activation. Components describe is absent. Therefore its fresh fallback is only {workspaceRoot}; it does not issue native artifact/capacity authority for components.

A package describe Script is not a supported alternate initializer: it calls receiveScriptProcessInvocation and requires the Original envelope. The ComponentsScript implementation module is a class definition, not a file-script process entry. The existing direct root-script refusal is consistent with this source, rather than proof of a missing permission flag. Generic native file-script routing alone does not confer the component emitter publication authority.

No replacement invocation or cancellation was performed during this audit. The retained declared workspace:components owner19405 is the supported current route. Extending initial issuer policy would be a new schema-first command capability feature, not a route already supported, and was not done as a workaround.

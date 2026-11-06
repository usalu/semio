# Cargo Feature Forwarding Audit

A read-only Bun TOML scan of 955 non-test framework package manifests checked dep:key and key/feature forwardings against direct dependency keys (including target, dev and build dependency tables). It found eight missing-key entries in three manifests. This scan does not prove target feature availability for all valid keys or compiler success.

- Plugin facade 📦️packages/🦀️rust/Cargo.toml artifact-app-testing: semio-plugin-retained-command, semio-plugin-runtime-ops, semio-plugin-instance-stores and semio-plugin-instance-docs are not keys; the declared keys are semio-framework-plugin-retained-command, semio-framework-plugin-runtime-ops, semio-framework-plugin-instance-stores and semio-framework-plugin-instance-docs. Each actual target declares artifact-app-testing.
- The same facade original-actor-context-testing forwards semio-plugin-runtime-ops, while its direct key is semio-framework-plugin-runtime-ops, whose target declares original-actor-context-testing.
- ⚛️reactor/🏃️runtime/🧰️ops/📦️packages/🦀️rust/Cargo.toml original-actor-context-testing references dep:semio-framework-plugin-testing-caller and semio-framework-plugin-plugin-program/original-surface-testing; declared keys are optional semio-plugin-testing-caller and semio-plugin-plugin-program. The latter target declares original-surface-testing.
- 🧵️instance/📦️packages/🦀️rust/Cargo.toml artifact-app-testing forwards semio-plugin-instance-stores but declares semio-framework-plugin-instance-stores, whose target declares artifact-app-testing.

Facts reflect the source at scan time and may change concurrently. No production edits or native builds were performed. The retained ticket probe 🔬️cargo-feature-audit.ts can repeat the scan; its generated JSON output was removed.

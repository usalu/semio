# Fresh Module Frontier: Action-Bus Quarantine and Flow Dissolution

## Baseline

- Repository HEAD during the audit: `07873f842a5a99ac2f69c1648c21f36ebf260bdb`.
- The authoritative semantic census remains stale while unrelated glTF, plugin registry/host, OS renderer, root script, Cargo lock, launch, and plugin-artifact changes are advancing. It was not regenerated.
- Protected framework kernel, machine, platform, renderer, and repo-library-index paths remain quarantined.

## Rejected Action-Bus Lease

The read-only audit initially proposed `🧰️framework/🔨️modules/🎯️action-bus` as an untouched parity/package-edge lease. All proposed writable files were clean and byte-stable:

- Rust component: `5d8af8e53a25b82740d0066da657033713038339055874776f3b01433287f1a5`.
- TypeScript component: `9d17344e90aa8d9afe20fbce64d61c12a59a4e3df472c334f8b90598db629978`.
- Rust framework glue: `ad27c786ab078a03a0f48ba32ff72a9153967aef796da360e8167b076cd689d6`.
- TypeScript framework glue: `16aa07b5fd2492c2a7b309ccfbd404438366c50fa27974a7b915a07f6adf82c1`.

Live symbol resolution rejected the proposed disposition:

- Rust `ActionBus` has one production consumer in the protected framework Platform component. It cannot be dissolved without that owner.
- Rust `optional_json_to_dsl` is independently consumed by more than ten production plugin and OS components, so it is not dead and must be evaluated as a separate serialization capability.
- TypeScript contains five different responsibilities: utility-node derivation, window-measure partitioning, effective/missing action-argument resolution, window-action resolution, and mode-tool resolution.
- Several TypeScript responsibilities terminate in currently dirty OS renderer `ShellHost`/`ShellHelpers` paths. Consumer files must remain read-only until that concurrent wave releases them.

Disposition: quarantine the mixed component; do not delete its Rust language mirror and do not issue a partial rename or compatibility forwarding export.

## Issued Flow Compute Dissolution Lease

The Flow compute module is an evidence-backed zero-production-consumer module:

- Source: `✏️s/🔌️plugins/🌊️flow/🔨️modules/🧮️compute/🟦️component.ts`.
- Source hash: `4cae46be8e6f30501cffcd751e2ef5899334754f6836fedb0a5c7b2d359d8c7d`.
- Its production package barrel, hash `0704b6b6e0b903a2cc51adfed8ee3cb7543fc20812c144386541f39aafc38aff`, neither imports nor exports it.
- Repository-wide live searches find `initFlowThreadPool` only in the source and its in-source tests.
- The only path referrer is the package-local Vitest configuration. Tests do not satisfy the production-consumer minimum.

The Terra lease owns exactly:

- deletion of the compute source and its resulting empty component/collection directories;
- removal of the deleted source from `🧪️vitest.config.ts`, baseline hash `3841164880ea235b9ca33ba9296df0e09d24aecf900c6c3809b6f9bf33da7f41`;
- removal of the deleted modules glob from the Flow TypeScript `📋️project.json`, baseline hash `886599e45c9d8095d7c02e27e12762082b3acebdf52f09b11a14b81ac57b0e56`.

It may not modify the Flow package barrel/router, Flow artifacts/apps, framework UI/storage, root configuration, Cargo, launch configuration, or any dirty/protected path. Acceptance requires no stale referrer, the Flow TypeScript Nx quick test, and the narrowest scoped taxonomy report/enforce result.

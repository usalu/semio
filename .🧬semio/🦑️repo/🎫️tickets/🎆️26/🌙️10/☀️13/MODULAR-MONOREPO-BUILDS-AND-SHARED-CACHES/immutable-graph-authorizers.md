# Immutable Graph Authorizers

The selected graph consumer stays disabled until it can bind graph and journal decisions to the actual code the daemon loaded. Importing mutable checkout TypeScript from an immutable Nx SDK was insufficient: Node can retain an older dependency namespace after the source changes.

## Current Design

The pinned installation now includes one release bundle, `semio-authorizers.mjs`, beside its original `.semio-nx-tooling.json` marker. A closed `.semio-authorizer-source.json` records the exact UTF-8 ordered runtime module/schema/policy source names and SHA-256 values. The tooling installation identity includes those bytes and the compiler recipe source identities, in addition to the existing frozen package, lock, and patch bytes.

The runtime entry exports the neutral and repository journal owners, RPC validators, selected source/publisher/handoff owners, original task scope planner, and the owned pinned SDK ports. Backend policies are static JSON imports and enter that runtime source closure. A private canonical `📜️script.ts` worker performs an explicit release build, reserves common host compiler capacity under the incoming original process control, and reads only the source bytes declared by the frozen source manifest. It validates actual compiler input coverage and rechecks the source closure before returning. The caller checks it again before immutable publication.

The marker's `authorizers` receipt closes version, release mode, source manifest SHA, entry, and an array of exact output path/SHA pairs. The installed toolkit and the direct RPC marker reader validate actual no-follow source-manifest and bundle files against the receipt. The SDK loader additionally binds the cached imported namespace to the first verified marker and output bytes and rechecks them on subsequent calls. Old toolkits remain in their own tooling socket/data namespace; no other daemon is killed.

## Tests Actually Run

- The source closure helper was physically missing before implementation. Its neutral JSON vector then passed the independent esbuild runtime import-closure oracle and Ajv schema oracle.
- The compiler worker was physically missing before implementation. The current worker compiles the captured fixture bytes; actual Node reports value 7; changing its source to value 9 makes an attempt using the original source manifest fail.
- Real provisioning exposed the pinned Bun import scanner returning non-ASCII import strings as Latin-1 views of UTF-8 bytes. The boundary now decodes a verified UTF-8 round trip. The neutral source vector includes an emoji filename, escaped `é.ts`, literal `Ã©.ts`, and an authentic entry shebang. Independent esbuild agrees with all five captured input names; actual Node reports values 7, 11, and 13.
- Real provisioning also exposed the owned subset validator evaluating `$ref` siblings: spreading the source schema into a receipt validator retained contradictory root constraints. A minimal receipt reference with its definition scope repairs that refusal.
- `🗑️generated/immutable-authorizers-provision-actual5.log` passes actual immutable provisioning: 36 observed compiler inputs, 58,716 output bytes, verified marker receipt. The canonical prototype gate passes two tests and nine assertions in 259 ms; Nx task time is 1.3 s. The launcher still spent over 40 s preparing the full graph. This is not fast warm-launch acceptance.

The expanded canonical prototype gate passes two tests and thirteen assertions, including source-manifest and bundle-byte tampering, in 421 ms; strict TypeScript checking passes. This is recorded in `🗑️generated/immutable-authorizers-unicode-receipts-strict.log`, with Nx duration 4.3 s and critical path 4.0 s. The compiler recipe now fingerprints its complete actual source closure, and the publisher rechecks both runtime and compiler sources before publication. The later isolated installed-Node-SDK daemon/Bun-client transport gate also passes; real monorepo warm responsiveness and selected launcher-to-hasher/cache handoff remain under active validation.

The current release gate also passes two tests / thirteen assertions plus strict checking of the full tools provisioner and authorizer compiler sources, recorded in `🗑️generated/immutable-authorizers-sdk-layer-release2.log` (998 ms tests; 9.7 s Nx duration). The SDK allowlist now includes the exact owned private worker messenger/socket and graph-environment ports. Real prototype and release monorepo journal probes subsequently passed; selected graph reuse and full build acceptance remain pending.

## Exact Parent Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️runtime/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json`
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` register the authorizer test command in both modes.

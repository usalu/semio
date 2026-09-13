# Workspace Payload Membership

Status: accepted and published. The final source gate is10/176; the actual registered workspace freshness check passes with123packages. Historical investigation and red evidence follow.

A fresh native filesystem inventory read 72,225 directories and 127 nested package manifests without following directory links, excluding dependency/build/scratch and hidden trees. It found four duplicate identity groups: Surface (outer wrapper and bindings payload), Editor (outer wrapper and pkg payload), Flow (two separate bindings trees), and Trinity Jack LSP (outer wrapper and pkg payload).

The current workspace walker hard-codes a `pkg` directory and requires Cargo.toml. Its assumptions no longer express implementation-neutral membership. The proposed replacement will derive payload ownership from explicit package entry-point authority and actual ancestry, preserve unrelated duplicate failures, and have portable fixture cases plus independent installed package-resolution evidence. It will not infer ownership from `🕸️bindings` or Rust directory names.

Sol's Flow lane is investigating the second Flow output tree and its taxonomy contracts. Root owns the generic membership decision. The manifest census is disposable evidence under `🗑️generated/coordinator/workspace-payload/manifest-census.json`; its source was an inline read-only Bun probe, with no product mutation or compiler execution.

## Current Repair and Evidence

The lower workspace owner now discovers packages independently of language/compiler directory names. It suppresses a same-name nested manifest only when its nearest package ancestor exports a concrete physical file beneath that payload. It rejects unbound/sibling identities, invalid or unreachable conditional export branches, traversal and wildcard authority, missing files, and symlinks at any export-target segment. Both final-file and outside-root intermediate-directory links have explicit portable controls. Discovery still traverses physical payload directories to retain independently named nested packages.

The schema and portable fixture were edited before production behavior. Initial native Bun red: 0 pass / 1 fail / 8 filtered, duplicate identity on the explicit payload case (341 ms). A second red proved that the old indiscriminate condition walk wrongly accepted a branch after `default` (0 pass / 1 fail / 8 filtered, 378 ms). The final source suite passed **9 tests / 170 assertions / 0 failures in 1.221 s**, including **20 payload cases**, installed resolve.exports 2.0.3 and fast-glob parity, Ajv/jsonc-parser schema checks, source ownership, private publication, cancellation, no-follow, and exact route catalogs.

The existing computeWorkspaces group was revised to assert the new ownership contract and ordinary `pkg` membership. It passed **8 tests / 146 assertions / 688 filtered / 0 failures in 5.40 s**, including a native live-repository discovery. At that observation, the live comparison found **123 expected vs 88 current entries, 35 missing, 0 stale**, in 4.915 s, with no duplicate names. The membership is not yet published: two discovered entries were ignored Actor/Puzzle compiler-only manifests. Sol is adding stable authored parent wrappers before Root publishes the reviewed membership.

Trinity Jack LSP's authored wrapper now explicitly exports the current producer's JavaScript and declaration companions under its existing package identity. Existing worker code still targets the same compiler module. No compiler run or runtime language-server claim is made.

The obsolete Cargo manifest input was removed from workspace membership inputs; the physical membership check is now uncached because it observes generated-file presence and filesystem kinds. The source test remains cacheable with its exact fixture/owner/catalog inputs. The FEM retirement control compares the current producer inventory and explicit retired ID absence instead of assuming an immutable generator count. Sol added Flow's new generator to the explicit inventory and its three producer authority paths to the exact source inputs.

Actual isolated Nx validation is in progress. No live root package write, install, or broader compiler build has run in this Root lane.

## Registered and Native Authority Checks

Actual isolated, cache-skipped `bun nx run @semio-tech/repo-lib:test-workspace-publication-source --skip-nx-cache` passed **9 tests / 170 assertions / 0 failures**, Bun 1.175 s, Nx target 1.6 s and critical path 1.4 s. Private Nx/temp/artifact roots stayed under this ticket. Installed TypeScript 5.9.3 parsed all three edited implementation/test sources with zero diagnostics. Installed resolve.exports 2.0.3 independently selected the current Surface, Editor and Trinity JavaScript targets; their physical files contained 175,256, 135,389 and 3,824 bytes respectively. No module or WebAssembly initialization was performed.

## Observable Concurrent Edit Guard

Before live publication, source inspection found the initial root document could be overwritten if another editor changed it during discovery. Two schema-first controls now mutate either the source bytes or the admitted file kind during discovery. The source-change case first reproduced the overwrite attempt (0pass/1fail/9filtered,301ms). The publisher now rechecks physical file kind and exact source bytes immediately after discovery, before both freshness reporting and writing. These controls assert zero writes and preserved observed edits. This is an observed-change guard, not an atomic filesystem compare-and-swap guarantee.

The final actual isolated cache-skipped workspace source route passed **10tests,176assertions,0failures**, Bun826ms,Nx1.2s,critical967ms. The 20 payload cases and existing membership/publication/schema/source/route controls remain in that run.

## Live Root Publication

After Root read and accepted Sol's stable Actor/Puzzle wrapper report and Terra audit, the actual registered workspaces-write target published **123packages**, Nx9.2s,critical9.0s,cache-skipped. Independent parsed before/after comparison confirms **88→123 members,+35,-0,all unrelated fields exactly preserved**. Before SHA-256:`aa91d6059e09939c96b71f6d5693841a8840a6d8a303ee165cb2ef007fac34b9`; after SHA-256:`8a38138c3807e5c3464c00b60a170f9751dc28db0167825aca3dea0a4d71a1be`. A registered read-only freshness check is pending. No package install or compiler build ran.

The reviewed added memberships are all private authored package identities; Actor/Puzzle point to their stable outer owners, not their ignored compiler manifests.

| Added package owner | Package identity |
| --- | --- |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript` | `@semio-tech/writer-js` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript` | `@semio-tech/mathematical-js` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript` | `@semio-tech/procedural-js` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript` | `@semio-tech/gis-js` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript` | `@semio-tech/vcs-js` |
| `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript` | `@semio-tech/shooting-js` |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript` | `@semio-tech/demonstrator-js` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript` | `@semio-tech/fem-js` |
| `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript` | `@semio-tech/architect-js` |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript` | `@semio-tech/process-js` |
| `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript` | `@semio-tech/lowpoly-js` |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript` | `@semio-tech/reasoning-js` |
| `✏️s/🔌️plugins/📋️forms/📦️packages/🟦️typescript` | `@semio-tech/forms-js` |
| `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript` | `@semio-tech/layout-js` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript` | `@semio-tech/norm-js` |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript` | `@semio-tech/playbook-js` |
| `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript` | `@semio-tech/imperative-js` |
| `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript` | `@semio-tech/remodel-js` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript` | `@semio-tech/energy-js` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript` | `@semio-tech/trinity-js` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript` | `@semio-tech/dag-js` |
| `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript` | `@semio-tech/draw-js` |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript` | `@semio-tech/raster-js` |
| `✏️s/🔌️plugins/🗒️note/📦️packages/🟦️typescript` | `@semio-tech/note-js` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust` | `@semio-tech/puzzle-wasm` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript` | `@semio-tech/block-js` |
| `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript` | `@semio-tech/space-js` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript` | `@semio-tech/sourcing-js` |
| `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript` | `@semio-tech/framework-async` |
| `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript` | `@semio-tech/framework-kernel` |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust` | `@semio-tech/framework-actor-rs` |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript` | `@semio-tech/framework-actor` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust` | `@semio-tech/flow-core-build` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits` | `@semio-tech/plugin-window-kits` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools` | `@semio-tech/nx-bootstrap-tools` |

## Final Freshness and Exact Root Attribution

The actual registered workspaces-check command passed with123fresh packages,Nx3.8s,critical3.7s,cache-skipped. It ran after the real publication. No stale/missing membership remains at that observation. Both membership check and write are uncached physical-state operations.

Root owns the following9product paths for this repair, including the derived root membership, observable-change guard and Trinity export authority. Sol's two stable Actor/Puzzle manifests and Flow transient-manifest retirement are attributed in their own accepted reports.

| Path | Bytes | Observed SHA-256 |
| --- | ---: | --- |
| `package.json` | 21076 | 8a38138c3807e5c3464c00b60a170f9751dc28db0167825aca3dea0a4d71a1be |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` | 10936 | 6af0b88140a45c6ea320a9f0fabeab6b0cc860cb2611d78c0f312b15decf1bca |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🟦️.ts` | 4022 | e70b26622a3ec2c2311ded105b837196a55aff2f902d339fa51b0316c9d83a10 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️workspace-publication-source/🔣️.json` | 11557 | e5f49acf28c10d8ade26bd1a0dbce462a111ac9b6a8c5914f9f8b78938ffb5ff |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json` | 26185 | 3ac9085523176453db1b9936131fff1c5a2cabf26891c925d85a51864a8eddd3 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts` | 19868 | 3ec8a6ce7bb62b23abbb7a898d34a58f653578337f5f755f428f83d5b4b6f57a |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | 586133 | b66256f956a6f0d35b35d70c5a3fd4147f14f8453acd4a4b3a966d135222e947 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | 75940 | 43f898d363e9d47cc6d5f322b0ac465492db32620551b142ecc8de122a2f73b6 |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/package.json` | 211 | ae7a51c383ba1609217ab8ee66b04f25f846ca0eeee9c322d1a7a78d29211a46 |

All Root workspace sessions have completed. The disposable tree contains {"files":933,"directories":7,"symlinks":0,"bytes":281187721}. All outcomes,35published identities, exact product paths and native authority/constant facts are retained above or in their dedicated reports. Cleanup awaits the native open-handle check; no authored ticket control or live package output is in this tree.

Native lsof found no open handles (exit1/no output). Root removed only the inspected workspace-payload disposable tree; it is now absent. Final scoped git diff --check passed for the13unique Root workspace/router product paths.

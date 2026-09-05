# OS Root Fixture Hand Repair

Scope: `🧰️framework/🛍️products/💻️os/🧫️fixtures`, under the non-renderer OS lane. Every move was an explicit no-overwrite filesystem operation. Fixture payloads and binary bytes were preserved. No Git modification or output regeneration was used.

## Exact Moves

Parents are current names after preceding moves; all paths are relative to this fixture root.

| Parent | Old | New | Meaning |
| --- | --- | --- | --- |
| . | 🔌️asyncprobe | ⏳️asyncprobe | Awaiting/async behavior probes. |
| . | 🔌️brepprobe | 🧊️brepprobe | Boundary-representation solid geometry probes. |
| . | 🔌️jcoprobe | 🧩️jcoprobe | JCO component-model probe. |
| . | 🔌️scale | ⚖️scale | Parametric workload scaling fixture. |
| ⏳️asyncprobe | 👽️guest-turn | 🔄️guest-turn | Guest turn behavior, distinct from guest identity. |
| ⏳️asyncprobe | 🖥️host-turn | ⚙️host-turn | Host turn execution, distinct from host identity. |
| ⏳️asyncprobe | 🌐️tlsprobe | 🔒️tlsprobe | TLS security probe. |
| 🧊️brepprobe | 🌐️native | 🖥️native | Native execution, distinct from WebAssembly. |
| ⚖️scale/🤖️generated | 🧪️registry | 📇️registry | Full generated actor registration records. |
| ⚖️scale/🤖️generated | 🧪️catalog | 🗂️catalog | Plugin grouping/catalog projection. |
| 🧩️jcoprobe/👽️guest/🧬️schema | 🧪️world | 🌍️world | Empty retained WIT-world owner. |
| 🧩️jcoprobe/🌐️harness | out-callback | 📞️out-callback | Callback ABI experiment. |
| 🧩️jcoprobe/🌐️harness | out-jspi-explicit | ⚡️out-jspi-explicit | Explicit JavaScript Promise Integration experiment. |
| 🧩️jcoprobe/🌐️harness | serve.ts | 📜️script.ts | Existing harness server task entry. |
| 🧩️jcoprobe/🌐️harness/📞️out-callback | index.html | 🌐️.html | Explicitly served browser page. |
| 🧩️jcoprobe/🌐️harness/📞️out-callback | worker.js | 🧵️worker.js | Browser worker source. |
| 🧩️jcoprobe/🌐️harness/📞️out-callback | host-shim.js | 🖥️host-shim.js | Authored JCO host imports. |
| 🧩️jcoprobe/🌐️harness/📞️out-callback | run-node.mjs | 📜️script.ts | Existing Bun/Node harness task. |
| 🧩️jcoprobe/🌐️harness/📞️out-callback | preview2-shim | 🪞️preview2-shim | Exact vendored preview2 dependency mirror. |

Root Cargo workspace and dependency references, dependency evidence paths, scale project coordinates, exact Rust includes, launch-reader paths, developer generator/check/preview paths, and live taxonomy output roots were patched individually. The scale writer no longer recursively deletes unknown output children; it fails for explicit review. Its existing specimens are unchanged and its read-only freshness check passes. A new read-only neutral test compares both committed serialized outputs and an independent Node record/roster oracle; it failed before the exact reader fix and passes afterward. Native scale tests: 12/12.

## Current JCO Authority Versus Frozen Evidence

The old nested-Cargo catalog is digest-locked by `semanticPackageProjectionContracts.nested-cargo-packages-v1` with SHA256 `490a3abe1202d10258e6ec972b16ffd76d376b914a80b42250149f2776e4ecc3`. All of its bytes, historical coordinates, destination witnesses, and hashes remain unchanged.

Current JCO ownership is now an explicit nine-field `currentPackageDestination` in the live generator contract, with independently authored JSON Schema and nine language-neutral hostile cases beside the JCO fixture. The parser rejects old coordinates, added fields, missing authority, traversal, stacked emoji, and collisions. The preview checks the four current source files without following symlinks and validates the complete existing package/Cargo/WIT/content-role contract. Its output bytes still come from the unchanged digest-locked adapter witness; only its current output path is separately owned. There is no live historical-path fallback.

The canonical WIT lookup now selects the real package-relative `🧬️schema/📜️world.wit`, retaining the historical source-layout branch unchanged. JCO Cargo.toml also pointed at nonexistent `🦀️.rs`; it now selects the existing `📚️library/🦀️.rs` adapter. The normalizer's JCO activation validates this current authority before admitting generation.

The new source test passes its exact schema versus Ajv comparison, all nine hostile cases, frozen whole-catalog digest, existing adapter bytes/target, and independent TOML crate/entry checks. The existing Nx `check-jco-package-adapter` succeeds without writing.

Custom harness readers were updated exactly, including the second experiment's four cross-directory imports and the browser worker URL. The server decodes percent-encoded emoji URLs and keeps its path containment check separator-bounded.

## Narrow Tool-Owned Output Evidence

Independent TypeScript resolution proved that decorating the JS and declaration leaves with distinct emoji prefixes severs their implicit declaration pairing: the resolver selected `.js` instead of `.d.ts`. The six attempted component-leaf moves and their four reference edits were immediately reversed individually; no Git operation or payload restore was used. The retained names are `jcoprobe.js`, `jcoprobe.d.ts`, and `jcoprobe.core.wasm` in each of the two experiment directories above. JCO's installed `TranspilationOptions` exposes one component `name` and its generated outputs share that name; no per-interface output filename option is exposed. The installed JCO vendor declarations themselves use the `interfaces/wasi-*.js` to `.d.ts` pairing.

Each experiment's exact sixteen interface declaration names are: `semio-jcoprobe-probe-host.d.ts`, `semio-jcoprobe-probe.d.ts`, `wasi-cli-environment.d.ts`, `wasi-cli-exit.d.ts`, `wasi-cli-stderr.d.ts`, `wasi-cli-stdin.d.ts`, `wasi-cli-stdout.d.ts`, `wasi-cli-terminal-input.d.ts`, `wasi-cli-terminal-output.d.ts`, `wasi-cli-terminal-stderr.d.ts`, `wasi-cli-terminal-stdin.d.ts`, `wasi-cli-terminal-stdout.d.ts`, `wasi-clocks-monotonic-clock.d.ts`, `wasi-io-error.d.ts`, `wasi-io-poll.d.ts`, and `wasi-io-streams.d.ts`.

All ten files in the renamed `🪞️preview2-shim` are byte-identical to the installed `@bytecodealliance/preview2-shim/dist/browser` files, verified individually by SHA256: `cli.js`, `clocks.js`, `config.js`, `environment.js`, `filesystem.js`, `http.js`, `index.js`, `io.js`, `random.js`, and `sockets.js`. Their bytes and external filenames remain unchanged. This is evidence for exact file contracts, not an exemption for arbitrary output trees or authored harness files.

A source test now checks TypeScript's actual component/declaration resolution, all sixteen interfaces and their imports in both variants, core WASM signatures and literal URLs, and all ten external dependency payloads. With parent approval, forty-eight exact literal filename contracts and two exact interface-directory contracts were registered. None contains a wildcard or exempts descendants. The existing package-glue source restrictions remain in force for all forty-six source-format files; implementation code gained no package-boundary exemption. The regression also rejects modified suffixes, arbitrary outer prefixes, other interface directories, and custom declaration names.

Final JCO boundary run: 2 tests passed, 311 assertions. The renamed member-open reader's Nx source oracle passed 10 admission, 8 framing, and 6 retained-stage cases; it explicitly does not claim typed parser/factory activation.

At approximately 15:39 on September 4 a separate user cleanup task removed this ticket's entire generated-output tree, including active OS test logs and historical native build outputs. The parent confirmed the responsible task and requested protection from further cleanup. Reports and recovered input binaries outside `🗑️generated` remain. Already-read test results above are retained; the expanded Flow check and full developer rerun whose logs disappeared are not claimed from their lost logs and are being rerun. No Git restoration occurred.

## Recovered Stray Wire Outputs

The exact unreferenced directory `🧰️framework/🛍️products/💻️os/📦️packages/fixtures/wire` was moved intact to this ticket's `📦️wire-recovery`, outside generated cleanup. Its timestamps are September 3, 2026, 21:35, predating this lane's read-only golden-test correction. Nineteen files match the canonical replication specimens byte for byte. The positive `client-hello` has no committed positive counterpart and is retained as evidence. No live reader selects this package-local directory. Only its verified empty parent `🧰️framework/🛍️products/💻️os/📦️packages/fixtures` was removed. All twenty inputs are recoverable; no binary was deleted.

| Original Basename | SHA256 |
| --- | --- |
| 📦️client-bye.bin | 40d88127d4d31a3891f41598eeed41174e5bc89b1eb9bbd66a8cbfc09956a3fd |
| 📦️client-commands.bin | 9901383b326230e9fe3d5551a8526b84ef5e812802da2dc3a380c8dce30c5092 |
| 📦️client-credit-grant.bin | 1871658dc7416ec87cfac880aa61d4dd53347e5099631598bc2acd7b67219645 |
| 📦️client-frontier-advertise.bin | 13ef64663c99fe3660fa467ae5737b8b0c545b4bbd2d1aa264824563adca127e |
| 📦️client-hello.bin | 73d2dd55469957f794b56a0f8da2ec5638c9411b3706c6a8ecc3fc5d0cbeb073 |
| 📦️client-presence.bin | 830cd63ee6df77c66eac748f97a136f451d992861b56499d103d4de0c6f1666e |
| 📦️client-preview-publish.bin | 083c075f4895aeb7c369398e39eb68633ea154feff23d89047df9637c0da1199 |
| 📦️server-ack-accepted.bin | 0f88100f521896d6b8ab1388640cd3ab666a8663301d6ec548de0d8bdf0d66e8 |
| 📦️server-ack-rejected.bin | 75382cdcd7becd56db1769d1952249e9ec6598e6ac9655bf1e65267920bbcf41 |
| 📦️server-ack-transformed.bin | 195d413f9e4002e5ecbfe23a89e93443e8f50e43788c3f44bff979cfb7553ed9 |
| 📦️server-commands.bin | a8f760d5e6e925e2959e9451ee296348b29e814ee0f8bf11a4f7ac14d80d6a67 |
| 📦️server-credit-grant.bin | 13dc1ba04cc8354c29e11a9860f9b678656389753c562d4b96f65a7da07dbe2b |
| 📦️server-error.bin | 5f7b89557d9129965ef600d11b01e9c9349ee4de666f70f331cd1acf52023141 |
| 📦️server-presence.bin | a099f35f27ae3b95024aa422adbc3b42ce731c2cdb99bd9ffa6eb932e71f3940 |
| 📦️server-preview.bin | bee8e2738578240bb037d867a90e04bbf263ff6bce709ec9b44a8f64736dce40 |
| 📦️server-session.bin | 968dc23d161bc15b7987ffae4de2f500ab5fb37778560e247d017c50caf1b699 |
| 📦️server-snapshot-chunk.bin | eae72f264ea6ee1fb34f25f4a57b5614c1afa6cf8ee8f75cbb87f49a47394e0f |
| 📦️server-snapshot-done.bin | 95a52fbc37d8806e535830ee084bc1a566a53686be5c3b63a371f18db9fe7062 |
| 📦️server-welcome-snapshot-inline.bin | 187a8ff4ebce79920fdbf3363f848d87e4053c12f0ad0b93a6607e37a7a6c1ec |
| 📦️server-welcome-tail.bin | fc398c4c8c6be9ab5460476bc284e9571c2b31cef06efeb6a71e43bc61bc21b6 |

## Additional Authored Follow-Up

`🔨️modules/🏪️store/🧩️composition/📂️open` became `🚪️open` for member-opening lifecycle authority; its Rust module/fixture includes and exact host oracle path follow it. `🔨️modules/🪐️space/📚️examples/🎬️demo.space` became `🪐️demo.space`, distinct from the sibling collection demonstration, and its Rust include follows it. `🖥️host/🧪️fixtures` became `🧫️fixtures`, distinct from tests, and its codec-ledger include follows it. Source payloads and specimens are unchanged.

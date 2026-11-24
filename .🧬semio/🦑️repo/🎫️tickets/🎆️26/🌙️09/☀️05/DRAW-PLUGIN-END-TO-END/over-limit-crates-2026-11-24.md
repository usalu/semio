# Over-limit and near-limit crates in the Draw component closure (2026-11-24)

## How admission measures a crate
- Code: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` (`admitCargoSourceBoundariesV1`, `effectiveCargoSourceBoundaryV1`).
- Every workspace unit (`provenance.kind==="workspace"`) is read on its own: `readRustSourceClosureV1` starts at the unit entry (lib root) and follows `mod` and `#[path]` declarations with the unit's compiler cfg (`rustc --print cfg`, target or host, plus the feature cfg). Modules behind an inactive `cfg` are not read.
- `files` is the number of distinct files read. `bytes` is the sum of their full on-disk sizes (not the active bytes). `lines` is the sum of active lines.
- Limit per unit is `min(repository policy, [package.metadata.semio.rust-source-boundary])` per count: `maxSourceBytes` 262,144, `maxSourceFiles` 32, `maxActiveLines` 4,000. Crates without the table fall back to the policy. Every crate listed below declares those same three values.
- The breach is raised while reading, so the numbers in the error are a partial sum (273,033 bytes in the log). The real size of `semio-framework-value-native-owned` was 304,961 bytes before the other agent started shrinking it.
- Dev dependencies, tests and `#[cfg(test)]` modules are not part of a unit's closure.

## Measurement
- Script: `measure-source-boundary.mjs` in this ticket. It loads `readRustSourceClosureV1` from the admission code itself, takes `cargo metadata --filter-platform wasm32-wasip2` for the Draw component (`repo cargo -- metadata`), walks the normal and build dependency closure, and feeds each workspace crate its resolved features. Proc-macro and build-script units use the host cfg, the rest the wasm32-wasip2 cfg. Usage: `node measure-source-boundary.mjs <metadata.json> semio-hub-draw-component`.
- 739 library, proc-macro and build-script units of workspace crates were measured in the closure. Admission of the real build may differ by a few bytes when a `cfg(feature)` or cfg-gated `mod` differs from the resolved features.
- Taken at 03:27 on 2026-11-24 while other agents were still editing.

## Crates over or within 5% of a limit (95% and up of any count), then 90% to 95%
| crate | files | bytes | bytes of limit | lines of limit | last edited |
|---|---|---|---|---|---|
| `semio-framework-value-native-owned` | 16/32 | 262,168/262,144 | 100.0% (24 over) | 40% | 10 min |
| `semio-framework-dsl-record-emission` | 8/32 | 261,923/262,144 | 99.9% | 43% | 9.7 days |
| `semio-framework-dsl-record-model` | 26/32 | 261,877/262,144 | 99.9% | 58% | 172 min |
| `semio-plugin-instance-docs` | 17/32 | 260,544/262,144 | 99.4% | 69% | 238 min |
| `semio-os-vcs` | 27/32 | 258,833/262,144 | 98.7% | 82% | 2 days |
| `semio-framework-pack-json` | 17/32 | 257,800/262,144 | 98.3% | 81% | 115 min |
| `semio-plugin-reactor-turn` | 19/32 | 257,524/262,144 | 98.2% | 75% | 290 min |
| `semio-plugin-ownership` | 13/32 | 256,227/262,144 | 97.7% | 65% | 270 min |
| `semio-plugin-instance-jobs` | 19/32 | 254,788/262,144 | 97.2% | 61% | 141 min |
| `semio-framework-job-context` | 17/32 | 253,651/262,144 | 96.8% | 45% | 235 min |
| `semio-ui-contract-values` | 21/32 | 251,294/262,144 | 95.9% | 92% | 171 min |
| `semio-s-artifact-stdio-pdf-cmyk-image-clut` | 2/32 | 250,893/262,144 | 95.7% | 0% | 663 min (data tables) |
| `semio-plugin-retained-command` | 14/32 | 249,012/262,144 | 95.0% | 58% | 93 min |
| `semio-framework-mesh-engine` | 17/32 | 248,081/262,144 | 94.6% | 96% (lines) | 2.3 days |
| `semio-s-artifact-stdio-xml-snapshot` | 23/32 | 232,879/262,144 | 88.8% | 96% (lines) | 408 min |
| `semio-s-artifact-stdio-pdf-lift` | 12/32 | 208,424/262,144 | 79.5% | 95% (lines) | 75 min |
| `semio-s-artifact-draw-drawing-model` | 31/32 (97%) | 233,302/262,144 | 89.0% | 58% | 1506 min |
| `semio-s-artifact-stdio-binary-snapshot` | 31/32 (97%) | 74,094/262,144 | 28.3% | 24% | 9.7 days |

90% to 95% of bytes, not within 5%: `semio-os-store-one-item-publication` 94.9%, `semio-s-artifact-draw-drawing-editor` 94.5%, `semio-s-artifact-stdio-pdf-base14-sans` 94.4%, `semio-framework-replication-retained` 94.4%, `semio-framework-value-native-decoding` 94.2%, `semio-plugin-decl-app` 93.6%, `semio-plugin-job` 92.2%, `semio-plugin-decl-views` 91.8%, `semio-framework-replication-history` 91.7%, `semio-plugin-window-config` 91.0%, `semio-framework-io-sqlite-snapshot` 90.4%.

Only `semio-framework-value-native-owned` is over a limit. It is the only crate that has been edited in the last hour and a half besides `semio-plugin-retained-command` (93 min) and `semio-framework-io-sqlite-snapshot` (103 min).

## semio-framework-value-native-owned
- Another agent is already splitting it right now. At 03:07 the receive machine moved to a new crate `semio-framework-value-native-record-machine` (`🧳️owned/🏗️record-machine/📦️packages/🦀️rust`, written independent of the receive trait). I did not touch these files and did not start a second split.
- The same agent added `🏗️record-machine/🔀️enum/🦀️.rs` (24,208 bytes, the enum receiving machine) and `🏗️record-machine/🧩️slot/🦀️.rs` (3,779 bytes) to the crate at 03:17-03:18. Real admission (`repo cargo -- check -p semio-framework-value-native-record-machine -p semio-framework-value-native-owned`, 03:24) then refused the crate at 16 files, 262,168 of 262,144 bytes. That is 24 bytes over, so it is not a fix yet.
- Why the cut is hard (orphan rule): `NativeReceive` is defined in this crate and implemented here for foreign types: `String`, `Option`, tuples and arrays, `DslValue`, `OrderedMap`, `OrderedSet`, `Box`, `ValueError`, `ValueRefusalKind` and the other refusal types. An impl of a trait from a lower crate for those foreign types would be an orphan. So every module that carries such an impl, or is needed by one (`tree` root, `tuple`, `array`, `variant` plus `unit` for the refusal enums, `refusal`, `mapped` plus `box`, `map`, `set`), has to stay in the crate that defines `NativeReceive`. That is about 245 KB. Only code that merely uses the trait as a bound and carries no foreign impl can move up, and it can only move to a crate below `native-owned` if it stops naming `NativeReceive`. The record machine was moved this way (seed generics, like the existing `native-list-receiver` and `native-shared-engine` crates).
- Next cuts that follow the same rule, in order of size: the new enum machine `🔀️enum` (24 KB, same shape as the record machine, needs the `variant` spec types to become trait-free), then `variant` plus `unit` (36 KB) together with the `refusal` impls, then `map` and `set` (32 KB) if `NativeReceiveSlot` gets a seed-generic core.
- The 664 consumer manifests and the derive output reach `owned::machine`, so `native-owned` must keep reexporting each moved module (`pub use semio_framework_value_native_record_machine::machine;` is already in place).

## Near-limit crates: decision
No crate other than `native-owned` was edited in the last 90 minutes, so none is being grown right now and I did not split any. They are listed above. The tightest ones are `dsl-record-emission` and `dsl-record-model` (about 220 and 270 bytes of room), `instance-docs` (1,600 bytes) and `os-vcs` (3,300 bytes). Any further growth there breaks the Draw build the same way. The line counts of `mesh-engine`, `xml-snapshot` and `pdf-lift` (95% to 96% of 4,000 active lines) and the file counts of `drawing-model` and `binary-snapshot` (31 of 32 files) are the other limits that are close.

## Files
- New: `measure-source-boundary.mjs` and this report, in the ticket folder.
- No source, manifest, lock or registry file was edited by this audit.

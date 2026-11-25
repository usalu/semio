# Styling tokens generation: automatic and correct

## Findings
- `🔤️tokens/🦀️.rs` and `🐍️.py` are rendered by `generateStylingArtifacts` in `🎨️styling/📽️projection/🟦️.ts`. Inputs are `🔣️.json` plus the theme model and geometry.
- The repo target is `@semio-tech/ui-styling-tokens:generate`. It is the `repo:script` target in `🎨️styling/📦️packages/🦀️rust/📋️project.json`, running `repo ./📜️script.ts generate`. It has `check-generated` and `preview-generated` siblings.
- The taxonomy contract `generatorContracts.styling-tokens` already declares `nativeConsumers` (the styling crate). Output roots are `ignored`.
- The graph already attaches the producer as a prerequisite of every `build|check|test` task of a native consumer, including transitive consumers (`withNativePreparation` in `📥️inference/🦀️native-manifests`). So a `repo run <crate>:build` is correct.
- **Root cause.** The plain `cargo` shim (`🚪️shims/cargo` -> `repo cargo`) never goes through the graph. The ingress (`🏃️process/🦀️cargo/🚪️launcher`) admits sources but runs no producer, so a stale or missing ignored file is compiled as is.
- Precedents: the contract mechanism above (a), and tracked outputs with a drift check (c), e.g. the Flow browser declaration. Porting the generator to Rust for a `build.rs` (b) is not repo practice.

## Design
Option (a), extended to the ingress with no new mechanism. The contract's `target` is the same producer the graph schedules.
- New opt-in contract key `ingressPreparation: true`. It requires `nativeConsumers`. The audit validates it. It is set only on `styling-tokens`. The other 13 contracts include heavy producers such as assets, and can opt in with one line each.
- `repo cargo` compile requests now call `prepareGeneratedSourcesV1` before Cargo reads any source:
  - It takes the local crate closure of the manifest's workspace (`manifestClosureV1`, scope workspace, because `-p` may select any member).
  - It intersects that closure with the `nativeConsumers` of opted-in contracts.
  - It runs the owner project's declared `repo ./<script> <args>` through `runRepositoryCommand`, cross-platform through `repo`.
- It is skipped when `SEMIO_TASK_PROJECT` is set, because the graph already scheduled the producer. This avoids double work and recursion.
- Generation is idempotent (`writeGeneratedFileIfChanged`), so an unchanged file keeps its mtime. Staleness never needs detecting, because the producer always runs. This also covers a file missing on a replica machine.

## Tests and results
- `bun test` on `🏃️process/🦀️cargo/🏭️generated-sources/🧪️tests/🟦️.ts`: 4 pass. They check producer selection for the styling crate, the opt-in flag, out-of-repo manifests, and flag consistency.
- New vitest `🎨️styling/🧪️tests/🧪️generated-freshness/🟦️.ts`, registered in `🎚️config`. It fails if the on-disk generated artifacts differ from a fresh render, and it checks that `chip_row_height`, `error_ink`, `warning_ink` and `success_ink` are present. I did not run it through vitest, because `repo run` was impractically slow under load. The same check (`checkStylingArtifacts`) run directly with bun printed FRESH.
- End-to-end through the shim wrapper `T/replay-cargo.sh`, in the process3d workspace with `check -p semio-ui-styling`:
  - I appended `STALE_MARKER` to the generated file. After the build the file hash was back to the original, `generated-source:styling-tokens` ran, and the exit code was 0.
  - I deleted the generated file. The build regenerated it (hash identical) and the exit code was 0.
- Caveat: I verified with `-p semio-ui-styling` only, not with a crate that depends on the theme. The closure probe for the process3d workspace selects `styling-tokens`.
- Not run: macOS only. Windows, Linux and the devcontainer rely on `repo` and `runRepositoryCommand`, with no shell and no OS-specific paths. Windows still uses `cargo.cmd` into the same `repo cargo`.

## Changed files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️generated-sources/🟦️.ts` (new)
- `.../🏭️generated-sources/🧪️tests/🟦️.ts` (new)
- `.../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` (import and one ingress call)
- `.../📚️library/🔍️discovery/📦️package-projection-contracts/🟦️.ts` (type field)
- `.../📚️library/🔍️discovery/⚗️audit-generator-contracts/🟦️.ts` (allowed key and validation)
- `.../📚️library/🔣️taxonomy.json` (`styling-tokens.ingressPreparation`)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️generated-freshness/🟦️.ts` (new)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎚️config/🟦️.ts` (include)
- Ticket probes: `styling-closure-probe.ts`, `styling-fresh-probe.ts`.

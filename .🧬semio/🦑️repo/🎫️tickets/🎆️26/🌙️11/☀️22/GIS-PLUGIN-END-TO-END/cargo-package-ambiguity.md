# Cargo package "unknown or ambiguous" during preparation

## Verdict
0 candidates, not 2. Preparation was selecting the root workspace (`synchronize --manifest Cargo.toml --package semio-s-artifact-gis-gismap`). `semio-s-artifact-gis-gismap` lives in the nested workspace `gismap/Cargo.toml` and is not reachable from any root-owned package (checked with `debug-cargo-reach.ts`: 634 root members, 1371 reachable manifests, 0 named gismap; `hub` and `services` that depend on it are their own nested workspaces). The gismap workspace inventory is correct and has exactly one such package (`debug-cargo-package-ambiguity.ts`).

## Root cause
`cargoWorkspacePreparationInvocationV1` (`🗂️workspaces/🦀️cargo/🟦️.ts`) took the owner from the invocation manifest only: without `--manifest-path` that is `<cwd>/Cargo.toml`, so `-p <pkg>` from the repo root bound the root workspace even when the package belongs to a nested one. `selectedCargoArguments` normally adds the package manifest, but a launch that skips it (observed in the failing runs; the 14:47 run went through) reached the root. Reproduced with `debug-cargo-invocation.ts`: before the fix `test -p semio-s-artifact-gis-gismap --lib` produced `--manifest Cargo.toml`; with `--manifest-path` it produced the gismap workspace. No manifest, root `Cargo.toml` or repo-tool change explains the 14:38/14:47 passes; the flip is the launch path, not the tree.

## Fix
- `cargoWorkspacePreparationInvocationV1`: when no explicit `--manifest-path` and packages are named, bind them with `cargoPackageSelectionsNearV1` and use the package's workspace (error if they span workspaces). After the fix the same call yields `--manifest ✏️s/.../🗺️gismap/Cargo.toml`.
- `preparationPlan` error now says `unknown` or `ambiguous`, the reachable member count, the owning workspace manifest, and lists candidate manifests (or hints `--manifest-path`).

Both changes in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts`. Nothing compiled; existing tests not rerun (only bun debug scripts, read-only).

## Review and regression tests (follow-up)

Review of the `🟦️.ts` change: `--manifest-path` (separate or `=` form) skips the near-selection so the explicit manifest owner wins; `-p a -p b` across workspaces throws "Cargo command selects packages from different workspaces"; root-member packages resolve to their member manifest and therefore the root owner. No code change needed.

Added two tests to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🪆️nested-owners/🟦️.ts`:
- nested package from root binds to its nested manifest; root member stays `Cargo.toml`; explicit `--manifest-path` wins over `-p`.
- two packages across workspaces refused; unknown package refused by the repository inventory ("Unknown current repository Cargo package"). The new "unknown or ambiguous ... reachable from" text is only reachable after inventory succeeds, so it has no cheap fixture and is untested.

Results (logs in `🗑️generated/run*.log`): this machine's rustup has no default toolchain ("rustup could not choose a version of cargo"), so every test that spawns native cargo fails independent of the change.
- nested-owners: 3 pass (both new tests pass), 1 fail (native cargo, rustup).
- `🧪️tests/🟦️.ts`: 7 fail (native cargo / rustup, dependency counts 0).
- `📤️arguments`: 5 pass, 1 fail (native cargo exit 1).
- script-commands: 1 file pass. entrypoint: pass. ownership: pass.
- original-control: 13 fail ("Live Original process issuance required", environmental). replan: 9 fail (changed-vs-identical plans, native cargo).

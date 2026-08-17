# Wave 3 Repo CLI Playground Development Session Lease

## Baseline

- Packet reread: `📓️sol-wave3-repo-cli-playground-development-session-lease.md`.
- Applicable instructions reread: repository root, framework products, and repo product instructions. No deeper command-directory instruction exists.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`: `a3904c911c312efa7883a3f5b66e4f1019516aba778355552f798c26f4feaf68` (packet match; modified predecessor state from released command leases).
- `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json`: `239fe894cdb7d0130efed62ae92b8029b8f0ed5f1bc4e4b8cd80b27a61b7d5aa` (packet match; modified predecessor state from released command leases).
- The new `🛝️playground-development-session/🦀️component.rs` path is absent.
- Protected `.vscode/launch.json` is modified outside this lease and will not be touched.

## Referrer Sweep

- `resolve_playground`, `consume_legacy_example_prefix`, and `run_dev` occur only in the current CLI glue implementation, its `"dev"` dispatch arm, and its two local tests.
- No external tracked CLI-source/test referrer was found.

## Planned Move

- Move longest-prefix catalog lookup, option conversion, environment creation, and the Framework OS dev launch into `framework.repo.command.playground-development-session`.
- Remove `consume_legacy_example_prefix` and its positional fixture/example test. Preserve the explicit `--example` option only.
- Add the direct glue mount, mechanical dispatch, and one canonical command-manifest member. No registrar request is expected.

## Applied Source Lease

- Added `🧰️framework/🛍️products/🦑️repo/🎮️commands/🛝️playground-development-session/🦀️component.rs` (`4a110729f3d10819c8bb5624c5e5ba3c821857c723be6af6ace6b28d347d7918`). It owns the semantic `dev` command: longest-prefix resolution, explicit option conversion, environment construction, and the sole Framework OS development process launch.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` (`43961ef25195baeb772d3820e546703756ebf23c3c6db7337a1d78b92065398d`) with only the direct `playground_development_session` mount and its `"dev"` dispatch. The retired in-glue `dev` region is absent.
- Updated `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` (`aed56d5682f3c972336650d1e61ab6ba5a50f76c79d35284e672b7cf2574aefb`) with the exact canonical fourth member `framework.repo.command.playground-development-session`.
- Removed `consume_legacy_example_prefix` and its positional fixture/example test. The command accepts the explicit `--example` option and has no compatibility spelling or alias.
- Command-local tests cover longest-prefix resolution and an unregistered catalog result. The shared environment-contract test remains in glue because its support owner remains shared.

## Validation

- JSON parsing and manifest/tree bijection passed: four unique declared members and four immediate Rust command components, with no missing or phantom directory.
- Final referrer sweep found `playground_development_session` only at the direct glue mount/dispatch and resolution only within the new component; no `consume_legacy_example_prefix`, `dev::`, `run_dev`, or legacy-test referrer remains.
- `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.playground-development-session`: one component, zero errors, zero warnings.
- `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.playground-development-session`: one component, zero errors, zero warnings.
- `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache`: passed, 18/18; includes both playground-development-session tests.
- `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache`: passed release build.
- `bun nx run @semio-tech/repo-cli-rs:run -- dev __semantic-refactor-unregistered-playground__`: expected process exit `1`; emitted the unknown-playground diagnostic before the launch path, so no dev server was started.
- Tracked and new-file whitespace checks passed. Existing unrelated warnings remain in the UI target and CLI support glue (`Read` import and `Session.variant`); this lease did not change them.

## Release

- No central registrar request is required: the move changes only one local Rust mount, one local dispatcher, and the canonical local command collection manifest.
- No protected or excluded path was edited, including `.vscode/launch.json`, root scripts, Cargo files, taxonomy/discovery, catalog, proc, environment-contract, daemon, TUI, or library index.

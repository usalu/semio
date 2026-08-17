# Wave 3 Repo CLI Playground Development Session Lease

<!-- #region Decision -->

## Decision

Assign one Terra lease to extract the existing `semio dev <playground>` interaction into the specific command component:

```text
🧰️framework/🛍️products/🦑️repo/🎮️commands/🛝️playground-development-session/🦀️component.rs
```

The semantic ID is `framework.repo.command.playground-development-session`. Its sole responsibility is resolving a registered playground variant and launching that variant's Framework OS development session with the selected environment. The generic `dev` verb remains the user-facing command spelling; it is not a component identity.

This is the next conflict-free frontier after the terminal-dashboard daemon lease: its only production ingress is the root CLI dispatch, it has no referrers outside the CLI package, and it does not alter a registry, generator, root script, Cargo manifest, launch configuration, taxonomy SSOT, or the protected repo-library TypeScript index.

<!-- #endregion Decision -->

<!-- #region Baseline -->

## Baseline and Ownership

| Role | Path | SHA-256 / state |
| --- | --- | --- |
| Existing command implementation, local mounts, dispatch, and local tests | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | `a3904c911c312efa7883a3f5b66e4f1019516aba778355552f798c26f4feaf68`; contains the completed daemon extraction, so preserve all siblings |
| Command collection manifest | `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | `239fe894cdb7d0130efed62ae92b8029b8f0ed5f1bc4e4b8cd80b27a61b7d5aa`; currently contains workflow, plugin-registry, and terminal-dashboard-daemon |
| New command source | `🧰️framework/🛍️products/🦑️repo/🎮️commands/🛝️playground-development-session/🦀️component.rs` | absent; Terra creates it |

The worker must reread the root and applicable repo instructions, rehash both existing writable files, and stop if either fingerprint differs. `git status` shows the two baseline files as modified because they carry the already released daemon-command changes; that is a known predecessor, not permission to overwrite it.

The release excludes these active/protected paths entirely:

- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`;
- root `📜️script.ts`, `Cargo.toml`, `Cargo.lock`, framework root glue, taxonomy/discovery SSOT, and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`;
- every `catalog`, `proc`, `env_contract`, IPC, daemon, and TUI implementation. The command imports the first three as existing local interfaces only.

<!-- #endregion Baseline -->

<!-- #region Move -->

## Exact Move and Semantic Boundary

Move only the `dev` behavior currently at glue lines 312–372:

- longest-prefix playground resolution;
- typed flag-to-`DevOptions` conversion;
- catalog lookup, environment construction, and the one `@semio-tech/framework-os-dev:dev` process launch.

The direct consumer changes are mechanical and remain in the same lease:

1. Add one `#[path]` mount named `playground_development_session` in the Rust glue.
2. Replace the `"dev"` dispatch arm with `playground_development_session::run(&root, &parsed)`.
3. Add exactly one `🛝️playground-development-session` member to the command collection manifest, using the semantic ID above and the stated responsibility.
4. Move the two command-owned tests from glue lines 1398–1413 into the new component, with their implementation.

Delete `consume_legacy_example_prefix` and its `fixture`/`example` positional-prefix test instead of preserving a compatibility adapter. The active command accepts its explicit `--example` option; no migration, forwarding alias, or legacy positional spellings may survive. Keep the environment-contract test in glue because `build_dev_env` remains private support of both the TUI and this command.

No module is introduced. `catalog`, `proc`, and `env_contract` were deliberately not selected: their terminal production-component closure has not yet been proven, and the textual reuse inside this one CLI application cannot satisfy the two-independent-component rule.

<!-- #endregion Move -->

<!-- #region Evidence -->

## Graph Evidence

- Static production ingress: the `"dev"` arm at glue line 1347; no external tracked `semio dev`, `run_dev`, `resolve_playground`, or `consume_legacy_example_prefix` referrer was found outside the CLI source/test owner.
- Local dependencies: `args::ParsedArgs`, catalog loading, `env_contract::build_dev_env`, `options::parse_lock`, and `proc::spawn_inherit`. They remain behind repository-owned local interfaces.
- Mount/registration: a direct Rust `#[path]` mount and the commands collection's canonical `🔣️component.json`; no Nx project, package entrypoint, generator input, protocol, or generated output changes.
- Existing runtime route: `@semio-tech/repo-cli-rs:run` routes only through the package `📜️script.ts` and builds the binary before forwarding its arguments. No launch-file change is needed because this extracts an existing verb; the concrete Framework OS development launch surfaces already remain registered separately.

Rejected alternatives during this frontier scan:

- Native bootstrap cannot be deleted or changed in this lease: root `📜️script.ts` has the live `NativeOsScript` dispatch and is protected/dirty.
- The compiler facade has one direct Cargo consumer (`semio-framework-os-infinite`); dissolving it requires root-Cargo ownership and is therefore deferred.
- The repository server coordinator is a multi-responsibility Go application and needs a separate SCC design, not a small command lease.

<!-- #endregion Evidence -->

<!-- #region Validation -->

## Required Validation and Runtime Evidence

Run after the move, after confirming no generated drift:

```text
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:run -- dev __semantic-refactor-unregistered-playground__
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.playground-development-session
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.playground-development-session
```

The runtime probe must exit `1`, print the existing unknown-playground diagnostic, and must not invoke `spawn_inherit` or start a dev server. It is intentionally an inert registered Nx route, not a manual or long-running development session. Add a command-local test proving longest-prefix resolution and an unknown catalog result; the removed positional compatibility path must have no test and no implementation referrer.

Before releasing, search for the retired `dev::` path, `consume_legacy_example_prefix`, and old line-local test imports; validate manifest/tree bijection and retain the three previously registered sibling commands exactly once.

<!-- #endregion Validation -->

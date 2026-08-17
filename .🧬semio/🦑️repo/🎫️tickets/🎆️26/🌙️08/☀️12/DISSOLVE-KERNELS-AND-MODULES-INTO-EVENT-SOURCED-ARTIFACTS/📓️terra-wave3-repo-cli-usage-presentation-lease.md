# Wave 3 Repository CLI Usage Presentation Lease

## Scope And Ownership

- Lease: `framework.repo.command.cli-usage-presentation`.
- Owner: `🧰️framework/🛍️products/🦑️repo/🎮️commands/🧭️cli-usage-presentation/🦀️component.rs`.
- Production mount: the empty-argument, non-TTY branch in `semio::run`.
- Shared-module decision: `print_usage` had one production mount only; it is a command presentation component, not a module.
- Excluded and untouched: catalog/process candidates, root scripts, taxonomy/discovery SSOT, Cargo metadata, launch configuration, generated registry output, stdio, and quarantined framework capability paths.

## Pre-Edit Evidence

Applicable root, framework product, and repository product instructions were reread. The packet hashes matched before the edit:

```text
43961ef25195baeb772d3820e546703756ebf23c3c6db7337a1d78b92065398d  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
aed56d5682f3c972336650d1e61ab6ba5a50f76c79d35284e672b7cf2574aefb  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
```

The referrer sweep found only the root-private definition and its one non-TTY invocation. The concurrent staged command mounts and manifest members were preserved unchanged.

## Change

- Added the packet-prescribed local `#[path]` mount and redirected the non-TTY branch to `cli_usage_presentation::print()`.
- Removed root-private `print_usage`.
- Declared the exact command member in the existing canonical command collection manifest.
- Added the specific command component with the preserved usage text and an exact-text unit test.

Post-edit hashes:

```text
69e589cd050ed5f32aafe13d1b242c03b797da3f66097127e6f946bf12ce1d14  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
0480f49928be6f074f24b7eeb289b227962d1b287fc14d4ebd51d4a4733114fb  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
b39d058f5ec933ebfa8591a201b827899b865f877f325a209e5f28671b64d1c1  🧰️framework/🛍️products/🦑️repo/🎮️commands/🧭️cli-usage-presentation/🦀️component.rs
```

## Validation

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.cli-usage-presentation` | Passed: 1 component, 0 errors, 0 warnings. |
| `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.cli-usage-presentation` | Passed: 1 component, 0 errors, 0 warnings. |
| `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` | Passed: 19 tests, including `cli_usage_presentation::tests::usage_reference_preserves_every_registered_command`. |
| `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` | Passed: release build. |
| `bun nx run @semio-tech/repo-cli-rs:run --skip-nx-cache` | Expected exit 1 in non-TTY; printed the unchanged six-line usage reference to stderr. |
| Stale mount sweep | `print_usage` absent; exactly one mount and one call to `cli_usage_presentation`. |
| Diff checks | `git diff --check` and the untracked-component `--no-index --check` emitted no whitespace errors. |

The CLI checks emitted pre-existing warnings in the framework TUI and existing CLI daemon code. No claim is made about the quarantined framework plugin capability Cargo state.

## Registrar Request

None. The command collection manifest and root local mount were both explicitly owned by this lease; no central registrar, generator, or launch change is required.

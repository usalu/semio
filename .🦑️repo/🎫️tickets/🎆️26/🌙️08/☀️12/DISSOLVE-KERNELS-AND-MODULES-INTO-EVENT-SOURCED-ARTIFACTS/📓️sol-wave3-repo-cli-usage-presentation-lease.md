# Wave 3 Repository CLI Usage Presentation Lease

## Stable Handoff Evidence

The central Stage 1 executable-registration semantic validator was not edited in this coordinator assignment. Its current root `📜️script.ts` SHA-256 is:

```text
c35904f0f488f984a0f4781bd2322e59fa26dc6b48d8a7671a1660dfe786a4cc
```

The established quick validation was run after that rehash:

```text
bun ./📜️script.ts stdio quick
[stdio] quick passed (36 artifacts, 40 dialects, 6 codecs).
```

This is structural generator evidence only. It is not a Cargo or executable-registration runtime result. The framework plugin capability core and builder remain quarantined by [`📓️luna-framework-plugin-capability-quarantine.md`](./📓️luna-framework-plugin-capability-quarantine.md): their mtime is advancing and `cargo check -p semio-framework-plugin --lib` currently fails on trait visibility and nominal `ArtifactDialect` conversion. No stdio validation may rely on Cargo until the capability owner supplies an atomic release and that check passes.

## Consumer And Module Decision

The remaining generic `catalog` and `proc` glue capabilities both meet the semantic reuse threshold, but they are **not** this lease:

| Current capability | Independent terminal production consumers | Computed lowest semantic owner | Disposition |
| --- | --- | --- | --- |
| `catalog` (generated playground catalog contract, parser, and path resolver) | plugin-registry command; playground-development-session command; terminal dashboard; catalog query | repository product | Later `📇️playground-catalog` module |
| `proc::spawn_inherit` (inherited-stdio process invocation) | plugin-registry command; playground-development-session command | repository product | Later `🖥️process-invocation` module |

Both modules belong at `🧰️framework/🛍️products/🦑️repo/🔨️modules`, not under the CLI crate or either command. The current repository module collection has five direct legacy umbrella children (`⌨️cli`, `💻️client`, `📚️library`, `🔩️native`, `🖥️server`) and no canonical collection manifest. The deterministic census records the non-library umbrellas as zero-consumer delete candidates. Creating a two-member manifest for only the new capabilities would violate the required exact child/manifest bijection; declaring the five umbrellas as modules without their independent consumer proof would falsely pass responsibility to this small lease. The catalog/process extraction is therefore deferred to the graph-coloured repository-module-collection lease. No compatibility re-export, partial manifest, or path exception is permitted.

## Recommended Terra Lease

Extract the distinct one-consumer non-interactive CLI presentation from root glue into its own command component.

```text
current owner: 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli
destination:   🧰️framework/🛍️products/🦑️repo/🎮️commands/🧭️cli-usage-presentation/🦀️component.rs
semantic id:   framework.repo.command.cli-usage-presentation
```

`print_usage` has exactly one production runtime mount: the `argv.is_empty() && !stdout.is_terminal()` branch in `semio::run`. It has no other consumer, registry, schema, generated mirror, or launch entrypoint. It is specifically a presentation/action component, not a reusable module. The root CLI glue retains only the mechanical mount and the terminal-mode dispatch decision.

### Pre-Edit Hashes

Rehash before editing; stop and return the lease if any differs or another owner has modified either path:

```text
43961ef25195baeb772d3820e546703756ebf23c3c6db7337a1d78b92065398d  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
aed56d5682f3c972336650d1e61ab6ba5a50f76c79d35284e672b7cf2574aefb  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
```

### Exact Writable Paths

| Path | Operation | Required result |
| --- | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | Modify | Add one local `#[path]` mount for `cli_usage_presentation`; replace the direct private call with `cli_usage_presentation::print()`; delete the root `print_usage` definition. Keep the root decision that selects terminal dashboard versus non-interactive presentation. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | Modify | Append the one exact member below, preserving all existing members unchanged. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🧭️cli-usage-presentation/🦀️component.rs` | Create | Own the exact usage text and `print()` presentation. Keep a component-local exact-text test; expose no type or external-library API. |

The required manifest member is:

```json
{
  "directory": "🧭️cli-usage-presentation",
  "id": "framework.repo.command.cli-usage-presentation",
  "kind": "command",
  "responsibility": "Presents the non-interactive Semio CLI usage reference."
}
```

The only permitted mount is:

```rust
#[path = "../../../../🎮️commands/🧭️cli-usage-presentation/🦀️component.rs"]
pub mod cli_usage_presentation;
```

The new component may have a private `USAGE` constant and public `print()`. Its local unit test must assert the preserved complete text, including every already registered command. It must not add a `help` alias or invent a new command-line verb. There is no package, Cargo, generator, script, launch, registry, schema, or root registrar change.

### Validation And Runtime Surface

Run after the mount and manifest changes:

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.cli-usage-presentation
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.cli-usage-presentation
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:run --skip-nx-cache
```

The final command is expected to return exit `1` in a non-TTY and print the unchanged usage reference to stderr; record that exact runtime evidence. Then confirm the sole mount and the absence of the old private name:

```text
rg -n 'print_usage|cli_usage_presentation' -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs 🧰️framework/🛍️products/🦑️repo/🎮️commands
git diff --check -- <three writable paths>
```

### Exclusions

Do not edit any `✏️s/🔌️plugins/🗄️stdio/**` path, `📜️script.ts`, taxonomy/discovery SSOT, Cargo manifests/lock, `.vscode/launch.json`, generated registry output, framework plugin capability core or builder, quarantined kernel/machine/platform/renderer paths, or the protected repo-library TypeScript index. The Stage 2 stdio registry/artifact SCC and this lease share no source, registration, generator, test-output, or formatter path.

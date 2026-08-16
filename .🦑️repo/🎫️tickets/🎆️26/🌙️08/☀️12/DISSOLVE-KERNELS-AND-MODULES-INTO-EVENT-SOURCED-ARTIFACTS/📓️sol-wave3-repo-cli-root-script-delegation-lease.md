# Root Script Delegation Command Lease

## Status

Prepared on 2026-08-16 as a read-only graph-colored Wave 3 packet. No source, configuration, generated output, or central registrar was changed by this lease preparation.

## Boundary

| Field | Value |
| --- | --- |
| Semantic component | `framework.repo.command.root-script-delegation` |
| Responsibility | Delegates an otherwise unhandled Semio CLI invocation to the root script, preserving the parsed verb and positional segments. |
| Source owner | `🧰️framework/🛍️products/🦑️repo` |
| Destination | `🧰️framework/🛍️products/🦑️repo/🎮️commands/📜️root-script-delegation/🦀️component.rs` |
| Existing owner to reduce | The default arm of `repo` CLI dispatch in `🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`. |
| Collection manifest | `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` |
| Writable paths | Exactly the new component, the CLI Rust glue, and the commands collection manifest. |
| Registrar/generator | None. The existing `@semio-tech/repo-cli-rs:run` launch surface already mounts the Rust glue. |

## Pre-Change Ownership And Fingerprints

The executor must reread `AGENTS.md`, recheck dirty ownership, and rehash these paths immediately before editing. The observed prepared fingerprints are:

```text
36547e9e54a15e72edc0bfd7ce1e3adfc89e835e1eda165cf8616c82d9ddc6f2  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
a4c3339be6c8a452dfb85965879fa16b6916f55489c6e7063b0a4ec9467648ed  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
```

The two existing files have staged, released-command work in the shared worktree. The executor must preserve the six currently registered command members and reconcile the live diff rather than overwriting it. No other lease currently owns the three paths in this packet.

## Static Evidence

`glue.rs` currently dispatches the six named command components: `workflow`, `daemon`, `dev`, `catalog`, `plugin registry`, and non-interactive CLI usage. Its sole remaining default action at lines 1297–1301 is:

```rust
let mut forward = vec!["./📜️script.ts".to_string(), parsed.verb.clone()];
forward.extend(parsed.segments.clone());
let forward_refs: Vec<&str> = forward.iter().map(String::as_str).collect();
proc::spawn_inherit("bun", &forward_refs, &root, &[])
```

The new command has one direct production consumer, the CLI dispatch, so it is a command—not a reusable module. It is specific to root-script forwarding and should retain its argument construction privately.

`proc::spawn_inherit` is already consumed directly by the released plugin-registry and playground-development-session commands. The new command will be a third terminal consumer, but `proc` remains quarantined in the excluded `repo/🔨️modules` collection frontier until its terminal-consumer and lowest-common-owner semantic refactor is separately leased. This packet neither promotes nor moves `proc`.

The exact static referrer set at preparation time is:

```text
🧰️framework/🛍️products/🦑️repo/🎮️commands/🔌️plugin-registry/🦀️component.rs
🧰️framework/🛍️products/🦑️repo/🎮️commands/🛝️playground-development-session/🦀️component.rs
🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
```

## Required Atomic Change

1. Create the specific command component at the destination. It imports only repository-owned `crate::args::ParsedArgs` and `crate::proc::spawn_inherit`, exposes `run(root: &Path, parsed: &ParsedArgs) -> i32`, and keeps `forwarded_segments` private.
2. Add a focused unit test proving the generated vector is exactly `./📜️script.ts`, verb, then positional segments; it must not start a process.
3. Add one `#[path]` mount and `pub mod root_script_delegation;` to the CLI Rust glue alongside the other command mounts.
4. Replace the default dispatch-arm implementation with exactly `root_script_delegation::run(&root, &parsed)`.
5. Add one exact `x-semio.members` row in the commands manifest with directory `📜️root-script-delegation`, ID `framework.repo.command.root-script-delegation`, kind `command`, and the responsibility stated above.

Do not add a forwarding export, compatibility alias, Nx target, package script, launch entry, shared module, or generated edit. Do not change the handling of the named verbs or zero-argument interactive/usage paths.

## Protected Exclusions

- All `🧰️framework/🛍️products/📓️print/**` paths: active print lease.
- All `✏️s/🔌️plugins/🗄️stdio/**` paths: active executable-registration SCC and framework-plugin quarantine.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/**`: excluded collection-frontier lease, including `proc` ownership.
- Root `📜️script.ts`, root `Cargo.toml`, `Cargo.lock`, taxonomy SSOT/discovery, `.vscode/launch.json`, and the protected repo-library TypeScript index.

## Validation And Runtime Evidence

Run after the atomic source/manifest update, from the workspace root:

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.root-script-delegation
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.root-script-delegation
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:run -- verify taxonomy report --scope framework.repo.command.root-script-delegation
rg -n '(proc::spawn_inherit|root_script_delegation|root-script-delegation|let mut forward)' -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs 🧰️framework/🛍️products/🦑️repo/🎮️commands
git diff --check -- 🧰️framework/🛍️products/🦑️repo/🎮️commands/📜️root-script-delegation/🦀️component.rs 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs 🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
```

The runtime invocation intentionally chooses the existing root-script `verify taxonomy report` path. A zero exit demonstrates that the live CLI default arm forwards its verb and segments to the root script. Record its console output in the executor completion report; no temporary source logging is required.

## Handoff

This is the next smallest non-overlapping Terra lease. It is ready once the executor’s pre-change dirty-owner and fingerprint checks pass. Any change to either prepared fingerprint, any claimed ownership of the CLI glue/commands manifest, or an active modification beneath the exclusions requires a fresh coordinator packet.

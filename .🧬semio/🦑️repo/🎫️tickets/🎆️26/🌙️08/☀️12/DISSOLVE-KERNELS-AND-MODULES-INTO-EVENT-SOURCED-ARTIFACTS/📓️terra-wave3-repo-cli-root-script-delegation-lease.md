# Repo CLI Root Script Delegation Lease

## Boundary And Baseline

- Executed `📓️sol-wave3-repo-cli-root-script-delegation-lease.md` after rereading the root, framework product, OS, and repo instructions.
- The prepared source fingerprints matched exactly before mutation: CLI glue `36547e9e54a15e72edc0bfd7ce1e3adfc89e835e1eda165cf8616c82d9ddc6f2`; commands manifest `a4c3339be6c8a452dfb85965879fa16b6916f55489c6e7063b0a4ec9467648ed`.
- Both existing writable paths were clean. The manifest held the six released command members; no excluded print, stdio, repo-module frontier, root script, Cargo, taxonomy, launch, generated, or framework-capability path was modified.
- The direct pre-change `proc::spawn_inherit` referrers were exactly plugin-registry, playground-development-session, and the CLI glue default arm. `proc` remains in its excluded module frontier.

## Implementation

- Added `🧰️framework/🛍️products/🦑️repo/🎮️commands/📜️root-script-delegation/🦀️component.rs` as the specific command owner. Its private `forwarded_segments` constructs precisely `./📜️script.ts`, parsed verb, then positional segments; `run` alone invokes the existing `proc::spawn_inherit`.
- Added the sole `#[path]` mount and `root_script_delegation` module to CLI glue, replacing only the default dispatch arm with `root_script_delegation::run(&root, &parsed)`.
- Added the seventh exact collection member: directory `📜️root-script-delegation`, ID `framework.repo.command.root-script-delegation`, kind `command`, and the packet responsibility.
- Added a focused no-process unit test proving verb and positional-segment preservation. No alias, shared module, root script, generated change, or registrar request was added.

## Final Fingerprints

- Command: `9bc6af0465e18f10481a939d9e302fe8b55adfd82b522d55f241f3bc180b3ce0`.
- CLI glue: `c05f768d6842c04260d8e961bac8567c6080f05e71e93abee45de1ba0a7c5df7`.
- Commands manifest: `790d6b3815de98daaee73ce1424299353e315395f528c02724ffc91d4bcf35b2`.

## Validation

- `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.root-script-delegation` exited 0: 1 component, 0 errors, 0 warnings.
- `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.root-script-delegation` exited 0: 1 component, 0 errors, 0 warnings.
- `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` passed all 21 tests, including `root_script_delegation::tests::preserves_root_script_verb_and_positional_segments`.
- `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` exited 0. It emitted only pre-existing warnings in protected UI/CLI code.
- The registered runtime command `bun nx run @semio-tech/repo-cli-rs:run -- verify taxonomy report --scope framework.repo.command.root-script-delegation` exited 0. It reached `bun ./📜️script.ts run verify taxonomy report --scope ...`, then the new command preserved the verb and positional `taxonomy report` segments. Per contract it does not forward parsed flags, so root-script report mode ran the all-active scope and reported 4,270 components / 8,922 unrelated findings while returning 0; this establishes live delegation without claiming global taxonomy is clean.
- JSON parsing confirmed exactly seven command members and the exact new member metadata. The referrer sweep has one glue mount/default dispatch and the expected three direct `proc::spawn_inherit` command consumers. `git diff --check` is clean for all three writable paths.

## Handoff

The command is registered and released through its existing CLI launch surface. No central registrar action is required.

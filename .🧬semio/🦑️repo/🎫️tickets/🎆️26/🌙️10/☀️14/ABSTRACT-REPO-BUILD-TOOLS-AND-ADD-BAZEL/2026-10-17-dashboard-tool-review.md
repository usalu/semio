# Independent Dashboard Tool Identity Review — 2026-10-17

Read-only review of the current interfaces-agent changes in SessionCommand, Connection, CLI launch reuse, daemon start/group/service reuse, and session DTO persistence. No Rust build, native operation, or tooling/cache operation was run by this auditor. The reported compiled Rust gate result is owned by the implementing agent; it is not an independently rerun result here.

## Finding

No material blocker established in the reviewed selection/identity change. The original session-reuse defect is addressed at both CLI and daemon service reuse boundaries.

## Source Checks

- `🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs:246–258`: effective precedence is last outer `--tool`/`--tool=value`, then last declared `SEMIO_BUILD_TOOL`, then inherited hello environment, then default `nx`. Parsing ends at the first standalone `--`, matching `🏗️build/🧭️invocation/🟦️.ts:121–132`. Both forms and repeated options agree with the public parser.
- `bind_build_tool` removes all native-equivalent environment keys before adding one canonical selected key. Windows key comparison is case-insensitive; POSIX is case-sensitive. `child_environment` applies declared values after client values, so the selected identity and executed child environment agree.
- `🎛️dashboard/📎️connection/🦀️.rs:136,166`: the inherited tool is captured from the exact environment vector supplied to connect_with and exposed to the launch path. It is not re-read from an unrelated daemon process environment.
- `🎛️dashboard/🧭️cli/🦀️.rs:325`: members and required services bind selection before comparison with existing session records. `same_launch` at 360–361 compares selected tool as well as the prior semantic command/parameter identity. Explicit nx and default nx therefore remain equivalent; nx and bazel do not match.
- `🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs:431`: direct start binds the selected tool before registration and collision checks. Group steps bind against the requesting client environment at line 710 before service selection. `reusable` at line 780 requires equal effective tool even when command ID is equal.
- Session DTO retains the effective selection through the existing serialized SessionCommand.env field. The daemon registers a clone of the bound command, not the original unbound input. CLI comparisons against returned sessions therefore see the actual selection; no new DTO-only field is required.
- Ordinary nonbuild argv is not scanned for `--tool`: scanning is limited to recognized Bun Repo entry shapes. Ordinary commands still receive the selected environment as they would through normal environment inheritance; binding makes that state explicit in identity. No claim is made that an arbitrary ordinary command ignores SEMIO_BUILD_TOOL internally.

## Test Evidence Inspected

The shared language-neutral reuse fixture currently contains 12 cases, including both explicit backend changes, default equivalence, inherited selection, declared-over-inherited precedence, outer argv precedence, the literal `--` boundary, last declared environment value, and unrelated environment changes. The CLI test binds the actual SessionCommand then exercises same_launch. The implementation agent is adding further executable/source-shape vectors; this audit does not claim those additions have passed.

The audit did not identify a fresh runtime defect from the bounded command-shape recognizer. Generated dashboard target/group/playground commands use the recognized `bun repo` shape; direct Bun script Repo entry is also recognized. General aliases/wrappers that conceal Repo invocation are outside this source proof.

## Limits

This is source-level review, not cross-platform runtime validation. The compiled gate reported by the implementer and any future runtime checks remain separate evidence. Broader environment-sensitive service identity predates this fix and was not widened into an unrelated refactor.

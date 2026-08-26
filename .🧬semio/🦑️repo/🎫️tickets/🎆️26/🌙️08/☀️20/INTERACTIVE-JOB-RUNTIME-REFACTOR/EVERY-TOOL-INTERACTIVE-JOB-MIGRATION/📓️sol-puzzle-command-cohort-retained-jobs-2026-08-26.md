# Puzzle Command Cohort Retained Jobs — Sol Implementation Evidence

## Scope and status

This is the live implementation record for the Puzzle2d, Puzzle3d, and Puzzle5d command cohort. It is intentionally not an acceptance report while any production command still reaches `BoundedFirstStepCommandWork` for work that is not truthfully O(1).

Current semantic boundary: 79 of 99 commands have command-specific retained work or a truthful bounded O(1) implementation:

- Puzzle2d: 3 of 3.
- Puzzle3d: 39 of 53.
- Puzzle5d: 37 of 43.
- Remaining: Puzzle3d 14 and Puzzle5d 6.

All 99 command registrations are production-reachable through the owner-local `ArtifactOwnedToolJobFactory` registration and typed app dispatch. Registration alone is not counted as semantic completion.

## Focused Rust consumer evidence

On 2026-08-26, after the Puzzle5d board-event owner transfers were changed so every pending field is taken into a local before `self.push`, the exact focused consumer check was run against the live shared tree:

```text
cargo check --locked -p semio-s-plugin-puzzle --lib
```

Result:

```text
exit: 0
Finished `dev` profile [unoptimized] target(s) in 3m 38s
warning: `semio-s-plugin-puzzle` (lib) generated 156 warnings
```

The successful check compiled the current Puzzle2d/3d/5d retained command sources, including the Puzzle3d typed per-window config mutations, the two-grant scalar config family, cursorized `addObjectKind`, and the Puzzle5d board-event ownership fix. No Wasm or browser gate was run.

The exact same focused consumer command was rerun after the Puzzle5d engagement-input/control/abort routes, cursorized `addPartKind`, and typed overlap-budget mutation landed:

```text
exit: 0
Finished `dev` profile [unoptimized] target(s) in 5m 23s
warning: `semio-s-plugin-puzzle` (lib) generated 156 warnings
```

## Static and fixture evidence at this boundary

- `bun ./📜️script.ts verify interactivity`: exit 0, `DENY mode — clean` at the 77/99 boundary.
- Both language-neutral retained-job JSON fixtures parse with Bun.
- Focused `rustfmt --edition 2021 --check` parses the edited Rust sources. The editor files still have existing formatting differences, so formatting exit 1 is not claimed as a pass.
- Puzzle3d scalar field source evaluator: 20 production routes, 20 feature vectors, and hostile source markers for old reducer replacement and missing typed mutations.
- Puzzle3d `addObjectKind`: persistent decode, kind, representation, vortex-template, publication, and close owners, with 64/+1 and cancellation/fault boundary vectors.
- Puzzle5d engagement abort: distinct input, board-utility, world-utility, publication, and incremental-close transfers; separate fresh cancellation and fault vectors cover every boundary.
- Puzzle5d `addPartKind`: the production factory now reuses the bounded catalog/grip/target/create/connect cursor rather than the legacy board-brush reducer.

## Deferred before acceptance

- Replace the remaining 20 semantically complex fallback routes.
- Execute all hostile lifecycle/oracle vectors, not only parse/static evaluation.
- Re-run the official root static gates after the next stable semantic boundary.
- Run focused runtime tests and final production caller census after all 99 routes are semantically complete.

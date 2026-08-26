# Scope Override

## Developer Direction

On 2026-08-26 the developer confirmed that the staged deletion of `compose/**` was intentional. This supersedes the attached plan's requirement to preserve and compare the original `compose/**` tree.

## Mechanism Consequence

- The permanent taxonomy schema keeps `compose/` as an opaque lexical path exclusion.
- Inventory never follows or reads the excluded prefix.
- Plans record an excluded-tree digest only when the excluded tree exists.
- The normalization work does not restore, unstage, rewrite, or otherwise modify the intentional deletion.
- Final convergence evaluates every remaining tracked path outside the excluded prefix; the original compose digest is retained only as historical bootstrap evidence, not as a current acceptance gate.

## Workspace Validation

After removing the stale Compose memberships, read-only workspace parsers confirmed:

- Bun: 44 workspaces and zero paths under `compose/`.
- Cargo: 119 packages from `cargo metadata --no-deps --locked --format-version 1` and zero manifests under `compose/`.
- Go: 4 `go.work` uses and zero entries under `compose/`.
- Root package, Cargo, Go, Bun lock, Nx root project, and launch-seed metadata contain no exact Compose path references. One stale Cargo profile comment was reduced to the surviving `os-run` consumer.

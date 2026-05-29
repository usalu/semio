# Only Script Ts In Monorepo

## Production bundles

Every bundle uses a single `script.ts` with subcommands (`dev`, `build`, `test`, `policy`, `wasm`, …).

## Native bootstrap

Root `script.ps1` / `script.sh` were removed. Archived copies live in this ticket’s `embedded/` folder. Root `script.ts` runs them via `bun ./script.ts setup native` and `bun ./script.ts start` until a full TypeScript port lands in `NativeOsScript`.

## Ticket workspaces

Each ticket folder should expose one `script.ts` router; one-off tasks live in plain `.ts` modules next to it (not `*.script.ts`).

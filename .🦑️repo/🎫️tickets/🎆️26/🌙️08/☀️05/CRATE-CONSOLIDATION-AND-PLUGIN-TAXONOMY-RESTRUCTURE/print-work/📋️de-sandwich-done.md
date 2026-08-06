# Print product de-sandwich (Shape V2)

Session: 2026-08-06. Exclusive scope: `🧰️framework/🛍️products/📓️print/**`.

## Before

```
📓️print/⚡️implementations/🟦️typescript/   (@semio-tech/print)
📓️print/⚡️implementations/🖋️latex/
```

## After

```
📓️print/📦️packages/🟦️typescript/   (@semio-tech/print, semio.role=framework, semio.id=print)
📓️print/🖋️latex/                    (manifest-less .cls/.sty components)
```

`⚡️implementations` deleted under print.

## In-tree edits

- `📜️script.ts`: `texDir` → `../../🖋️latex` from package root.
- `📋️project.json`: all `cwd` targets repointed.
- `package.json`: workspace schema, `semio` block, repository.directory.

## Verification

- `bun ./📜️script.ts test quick` — unit tests passed.
- `bun ./📜️script.ts generate` — writes `🖋️latex/semio-tokens.sty`.

## Registrar handoff

See `📋️registrar-handoff.md` in this folder (root `package.json` workspaces + `bun.lock`).

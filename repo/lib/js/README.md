# @repo/lib

Stateless TypeScript facade for the repo CLI (`repo/client/client` or `client.exe`). Lint scripts use it to query entities via GraphQL subprocess calls and emit breaches to `.repo/cache/breaches/`.

## Lint scripts

- **File**: `<name>.<ext>.lint.script.ts` next to `<name>.<ext>` — default export receives `FileLinter`.
- **Folder / bundle / technology**: `lint.script.ts` in a directory — runner resolves entity kind from `folder(path)` GraphQL.

## Cache

Each run writes `.repo/cache/breaches/<sanitized-entity-id>.json` with `{ entityId, script, breachs }`.

## Nx

The workspace registers `./repo/lib/js/nx-plugin.mjs`, which matches `**/*lint.script.ts` (including `lint.script.ts`). Nx project inference typically **only includes git-tracked files** in the graph; untracked lint scripts will not get `breach-*` targets until they are added to version control.

Run a script directly:

```bash
bun repo/lib/js/bin/lint.ts "path/to/foo.lint.script.ts"
```

## Env

- `REPO_CLI_BIN` — path to repo client binary (default: `repo/client/client.exe` on Windows, else `repo/client/client` under workspace root).
- `REPO_ROOT` — workspace root (default: detected from cwd).

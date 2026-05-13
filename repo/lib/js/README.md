# @repo/lib

Stateless TypeScript facade for the repo CLI (`repo/client/client` or `client.exe`). Lint scripts use it to query entities via GraphQL subprocess calls and emit breaches to `.repo/cache/breaches/`.

## Lint scripts

- **File**: `<name>.<ext>.lint.script.ts` next to `<name>.<ext>` — default export receives `FileLinter`.
- **Folder / bundle / technology**: `lint.script.ts` in a directory — runner resolves entity kind from `folder(path)` GraphQL.

## Cache

Each run writes `.repo/cache/breaches/<sanitized-entity-id>.json` with `{ entityId, script, breachs }`.

## Env

- `REPO_CLI_BIN` — path to repo client binary (default: `repo/client/client.exe` on Windows, else `repo/client/client` under workspace root).
- `REPO_ROOT` — workspace root (default: detected from cwd).

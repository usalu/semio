## Addendum: type check before releasing a landing lock (F65)

Before releasing ANY landing lock, run `repo typecheck <every path you touched>` from `/Users/ueli/Documents/semio`; it must exit 0. It checks the touched TypeScript sources and the files that import them directly (one incremental compiler call, no graph read; a repeat takes seconds, the first call that reaches a new large closure can take about a minute) and compares every touched hand-written `.d.mts` with its `.mjs`.

- Paths: files or folders, relative to the checkout or absolute. A touched `.mjs` stands for its `.d.mts`: when you add, remove or re-sign an export of a `.mjs`, update its `.d.mts` in the same landing (every export declared, nothing declared that the script lacks, same parameter count).
- Exit 0 clean, 1 type errors in a named file or an importer, or a declaration drift, 2 refused (bad path, compiler missing). Errors in other files the program reads are counted as "elsewhere" and do not fail the call; `--all-errors` counts them.
- `repo typecheck --declarations` compares every `.d.mts` pair of the library (about 1 s, no compiler); `--dry-run` lists what would be checked; `--json` prints one structured result (MCP tool `typecheck`, argument `files`).
- Lanes have so far checked with `bun build --no-bundle`, which does not type-check: that is no substitute. `bun test` does not type-check either.
- The build state lives in the machine cache (`SEMIO_TYPECHECK_CACHE` overrides it); nothing in the checkout is written. If a call reports "importers found, only the touched files are checked", the change is widely imported: also run `repo typecheck` on the importing folder you expect to be affected.

# Only Script Ts In Monorepo

## Production bundles

Every bundle uses a single `script.ts` with subcommands (`dev`, `build`, `test`, `policy`, `wasm`, …).

## Declarative routers (`@repo/lib` `bundle-script.ts`)

- Subcommands are **`class X extends BundleScript`** with `run(segments)`.
- Register with **`new ScriptRouter(import.meta.dir).register("dev", DevScript)`**.
- Entry: **`await runBundleScriptMain(router, import.meta.url)`** or **`runPolicyOnlyMain`** when only `policy` applies.
- Workspace root keeps **`ScriptRouter`** over workspace verbs (same `Script` base, imported from `bundle-script.ts`).
- Docs: `repo/lib/js/README.md`.
- Shared wasm: `runWasmPackWebBuild` in `bundle-script.ts`.
- Nested verbs: `dispatchSubcommand`.

## Native bootstrap

Production shells: `repo/native/bootstrap/script.ps1` and `script.sh`.  
Invoked via `bun ./script.ts setup native` and `bun ./script.ts start` with `SEMIO_REPO_ROOT` set to the workspace root.

## Neo4j migrations

Production router: `repo/lib/neo4j-migrate/script.ts`.  
Invoked via `bun ./script.ts migrate neo4j …` from the monorepo root.

## Ticket workspaces

Ticket folders may keep historical copies; **production `script.ts` files must not reference `.repo/🎫` paths**. One-off tasks stay in plain `.ts` modules beside a ticket `script.ts` when needed.

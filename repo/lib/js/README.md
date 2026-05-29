# @repo/lib (JS)

Repo policy lint facade and GraphQL CLI subprocess helpers.

## Bundle `script.ts` conventions

Every bundle or workspace router is a single `script.ts`. Implement commands as **`Script` subclasses** registered on a **`ScriptRouter`**, not long `if (command === …)` chains.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "./src/bundle-script.ts";

class TestScript extends BundleScript {
  run(segments: string[]) {
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url);
```

- **`BundleScript`**: `this.root` is the bundle directory; `this.repoRoot` is the monorepo root (`findRepoRoot`).
- **`runBundleScriptMain(router, import.meta.url)`**: runs `policy` when `export const policy` is present, then dispatches argv.
- **`runPolicyOnlyMain(import.meta.url)`**: policy-only bundles (no subcommands).
- **Workspace root** (`/script.ts`): same `Script` base class with `ScriptRouter` over workspace verbs (see root `script.ts`).
- **Ticket folders**: one `script.ts` router per ticket; task modules stay plain `.ts` beside it.
- Helpers: `runCmd`, `runBun`, `runBunx`, `runViteDev`, `runViteBuild`, `runVitest`, `devToolingEnv`, `spawnBunx`.

## Policy scripts (`script.ts` only)

- **File**: `export const policyFile = "index.ts"` plus `export const policy = defineLint(...)` in the bundle `script.ts`.
- **Folder / bundle / technology**: `export const policy = defineLint(...)` in `script.ts` at that directory — runner resolves entity kind from `folder(path)` GraphQL.

Run:

```bash
bun path/to/script.ts policy
bun repo/lib/js/bin/lint.ts path/to/script.ts
```

Nx registers `./repo/lib/js/nx-plugin.mjs`, which matches `**/script.ts` that export `policy` and adds cacheable `breach-*` targets (`bun "<script.ts>" policy`).

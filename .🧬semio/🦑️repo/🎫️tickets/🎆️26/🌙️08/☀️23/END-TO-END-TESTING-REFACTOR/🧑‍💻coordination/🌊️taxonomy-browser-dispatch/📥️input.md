# Browser Dispatch Relocation Input

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📋️project.json

SHA-256 `4ab233b0de3a0a7cf6955bc010ebae89dcf518992b5fcb698c3fd081f3820af5`

```text
{
  "name": "@semio-tech/browser-actor-import-test",
  "$schema": "../../../../../../../../node_modules/nx/schemas/project-schema.json",
  "targets": {
    "runtime-check": {
      "executor": "nx:run-commands",
      "options": { "cwd": "{projectRoot}", "command": "bun ./📜️script.ts runtime-check" },
      "cache": true,
      "parallelism": false,
      "outputs": ["{projectRoot}/dist/runtime-check"]
    },
    "pending-host-close-check": {
      "executor": "nx:run-commands",
      "options": { "cwd": "{projectRoot}", "command": "bun ./📜️script.ts pending-host-close-check" },
      "cache": true,
      "parallelism": false,
      "outputs": ["{projectRoot}/dist/pending-host-close-check"]
    }
  }
}

```
## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📜️script.ts

SHA-256 `6fa47a1e4bf27fa8883c7858daeacd4337c3ef900aa593bf927e8b1d22612e2d`

```text
#!/usr/bin/env bun
/** 🌊️ Dispatches the actor-import runtime contract from its canonical test case. */
import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { type ActorImportFactoryPort, testCanonicalActorAsyncImport } from "./🟦️.ts";

function repositoryRoot(start: string): string {
  let cursor = resolve(start);
  while (!existsSync(join(cursor, "nx.json"))) {
    const parent = dirname(cursor);
    if (parent === cursor) throw new Error("actor import test: repository root unavailable");
    cursor = parent;
  }
  return cursor;
}

if (import.meta.main) {
  const root = repositoryRoot(import.meta.dir);
  const bundle = await import(pathToFileURL(join(import.meta.dir, "..", "..", "📜️script.ts")).href) as { closedBrowserActorBundle: ActorImportFactoryPort };
  const command = process.argv[2] ?? "runtime-check";
  assert(["runtime-check", "pending-host-close-check"].includes(command), `actor import test: unknown command ${command}`);
  const evidenceRoot = join(import.meta.dir, "dist", command);
  await testCanonicalActorAsyncImport(root, evidenceRoot, undefined, bundle.closedBrowserActorBundle, true);
}

```
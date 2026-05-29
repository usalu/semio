#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts <dev|build|lint|test|policy> [args…]`. */
import { spawn } from "node:child_process";
import { join } from "node:path";
import type { FileLinter } from "../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../repo/lib/js/src/cli.ts";
import { dispatchPolicyArgv } from "../../repo/lib/js/src/policy-cli.ts";
import { defineLint } from "../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@ui/react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
}
const repoRoot = join(cwd, "..", "..", "..", "..");
const env = {
  ...process.env,
  WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
  CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
};

const childFor = (args: string[], root = cwd) => spawn("bunx", args, { stdio: "inherit", shell: true, env, cwd: root });
let child: ReturnType<typeof spawn>;

if (segs[0] === "dev") {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env.STORYBOOK_PORT ?? "6006";
  child = childFor(["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...segs.slice(1)], repoRoot);
} else if (segs[0] === "build") {
  child = childFor(["storybook", "build", "-c", ".storybook", ...segs.slice(1)], repoRoot);
} else if (segs[0] === "lint") {
  child = childFor(["eslint", "--max-warnings", "0", "--config", "eslint.config.ts", ".", ...segs.slice(1)]);
} else if (segs[0] === "test") {
  child = childFor(["vitest", "run", "--config", "vitest.config.ts", "--passWithNoTests", ...segs.slice(1)]);
} else {
  console.error("usage: bun ./script.ts <dev|build|lint|test> [args…]");
  process.exit(1);
}

child.on("exit", (c) => process.exit(c ?? 0));

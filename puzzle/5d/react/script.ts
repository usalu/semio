#!/usr/bin/env bun
/** 🧭 `@puzzle/5d-react` task router: `bun ./script.ts test|policy [args…]`. */
import { spawnSync } from "node:child_process";
import type { FileLinter } from "../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/src/cli.ts";
import { dispatchPolicyArgv } from "../../../repo/lib/js/src/policy-cli.ts";
import { defineLint } from "../../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@puzzle/5d-react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
}
const command = segs[0] ?? "test";
const extra = segs.slice(1);

const env = { ...process.env };
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

if (command === "test") {
	const result = spawnSync("bunx", ["vitest", "run", "--passWithNoTests", "--config", "vitest.config.ts", ...extra], {
		cwd,
		env,
		shell: true,
		stdio: "inherit",
	});
	process.exit(result.status ?? 1);
}

console.error("usage: bun ./script.ts test [args…]");
process.exit(1);

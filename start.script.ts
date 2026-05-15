#!/usr/bin/env bun
/** 🟢 IDE session hook: runs the platform-native setup check for local services. */
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dir;
process.chdir(root);

//#region 🧩Generate
function runWorkspaceGenerateFromLiveNeo4j(): void {
  const result = spawnSync(process.execPath, [join(root, "generate.script.ts")], {
    stdio: "inherit",
    cwd: root,
  });
  if (result.status !== 0) {
    console.log("[start] `bun run generate` did not refresh all `.repo/🛂` bundles (Neo4j may be offline or some databases are missing).");
  }
}
//#endregion 🧩Generate

if (!existsSync(join(root, "node_modules", "nx", "package.json"))) {
  console.log("[start] node_modules incomplete — run `bun install` and `bun ./setup.script.ts` (or platform setup script).");
  process.exit(0);
}

if (process.env.DEVCONTAINER === "true") {
  runWorkspaceGenerateFromLiveNeo4j();
  console.log("[start] Devcontainer session ready; .devcontainer/post-start.sh owns local services.");
  process.exit(0);
}

if (process.platform === "win32") {
  execFileSync(
    "powershell.exe",
    ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", join(root, "setup.windows.script.ps1"), "-SessionStart"],
    { stdio: "inherit", cwd: root },
  );
} else if (process.platform === "darwin") {
  execFileSync("bash", [join(root, "start.mac.sh")], { stdio: "inherit", cwd: root });
} else if (process.platform === "linux") {
  execFileSync("bash", [join(root, "start.linux.sh")], { stdio: "inherit", cwd: root });
} else {
  console.log(`[start] Unsupported native platform ${process.platform}; no session setup script was run.`);
}

runWorkspaceGenerateFromLiveNeo4j();

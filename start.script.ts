#!/usr/bin/env bun
/** 🟢 IDE session hook: runs the platform-native setup check for local services. */
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dir;
process.chdir(root);

if (!existsSync(join(root, "node_modules", "nx", "package.json"))) {
  console.log("[start] node_modules incomplete — run `bun install` and `bun ./setup.script.ts` (or platform setup script).");
  process.exit(0);
}

if (process.env.DEVCONTAINER === "true") {
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

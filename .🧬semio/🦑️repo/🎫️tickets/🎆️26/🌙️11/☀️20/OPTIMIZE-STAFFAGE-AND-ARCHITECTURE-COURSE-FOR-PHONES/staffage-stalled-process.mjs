import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const pid = Number(process.argv[2]);
if (!Number.isSafeInteger(pid) || pid < 1) throw new Error("Expected the Staffage test process ID");
const output = fileURLToPath(new URL("./🗑️generated/staffage-process-sample.txt", import.meta.url));
const result = spawnSync("/usr/bin/sample", [String(pid), "3", "10", "-file", output], { encoding: "utf8", timeout: 12000 });
console.log("[DEBUG]", result.status, result.error?.message ?? result.stderr);

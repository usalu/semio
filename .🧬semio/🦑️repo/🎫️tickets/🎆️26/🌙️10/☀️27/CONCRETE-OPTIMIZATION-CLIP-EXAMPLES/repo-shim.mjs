/** Stand-in for `repo tools exec --light node -- <args>` while the real repo binary is missing: runs node directly. */
import { spawnSync } from "node:child_process";

const args = process.argv.slice(2);
if (args.slice(0, 5).join(" ") !== "tools exec --light node --") {
  console.error("repo-shim: unsupported command " + args.join(" "));
  process.exit(2);
}
const result = spawnSync(process.execPath, args.slice(5), { stdio: "inherit", windowsHide: true });
process.exit(result.status ?? 1);

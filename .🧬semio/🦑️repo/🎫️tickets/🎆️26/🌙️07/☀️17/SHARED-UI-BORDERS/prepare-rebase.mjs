import fs from "node:fs";
import { execFileSync } from "node:child_process";
import { gzipSync } from "node:zlib";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const root = process.cwd();
const folder = fileURLToPath(new URL(".", import.meta.url));
const generated = resolve(folder, "🗑️generated");
const repo = "/home/clstb/.local/share/semio/launcher/bin/repo";
const git = (...args) => execFileSync(repo, ["tools", "exec", "--light", "git", "--", ...args], { encoding: "utf8", maxBuffer: 50000000 });
const originalHead = git("rev-parse", "HEAD").trim();
const target = "45ef25ecbd34d20ab312585ca5ba370b066c9117";
if (originalHead !== "15d755f053c3a94e974e919772f1cd9a3c7845ec") throw new Error("Branch changed since rebase inspection");
const pid = 2110900;
if (process.platform === "linux" && fs.existsSync(`/proc/${pid}/cmdline`)) {
  const command = fs.readFileSync(`/proc/${pid}/cmdline`, "utf8");
  if (fs.readlinkSync(`/proc/${pid}/cwd`) !== root || !command.includes("storybook") || !command.includes("6011")) throw new Error("Storybook process identity changed");
  process.kill(-pid, "SIGTERM");
}
const tracked = git("diff", "--name-only", "-z").split("\0").filter(Boolean);
const untracked = git("ls-files", "--others", "--exclude-standard", "-z").split("\0").filter(path => path && !path.includes("/🗑️generated/"));
const ignoredInputs = ["🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/🟦️.ts", "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/scenarios.feature"];
const paths = [...new Set([...tracked, ...untracked, ...ignoredInputs])].sort();
const files = paths.map(path => {
  if (!fs.existsSync(path)) return { path, deleted: true };
  const content = fs.readFileSync(path);
  return { path, content: content.toString("base64"), mode: fs.statSync(path).mode, sha256: createHash("sha256").update(content).digest("hex") };
});
fs.mkdirSync(generated, { recursive: true });
const snapshot = { originalHead, target, branch: git("branch", "--show-current").trim(), tracked, ignoredInputs, workingPatch: git("diff", "--binary"), indexPatch: git("diff", "--cached", "--binary"), files };
fs.writeFileSync(resolve(generated, "rebase-snapshot.json.gz"), gzipSync(JSON.stringify(snapshot)));
fs.writeFileSync(resolve(folder, "rebase.md"), `# Rebase onto Ueli 718\n\n- Original base: ${originalHead}\n- Target: ${target} (Ueli 718)\n- Branch: ${snapshot.branch}\n- Preserved ${files.length} modified/untracked/input files in a temporary compressed snapshot, including the ignored Storybook interaction helper.\n- Original index is ${snapshot.indexPatch ? "nonempty" : "empty"}. Changes remain uncommitted; no feature commit is implied.\n- Stopped this checkout's live Storybook on6011 before changing source base. Other checkout's6010 server is untouched.\n\nRebase and validation pending.\n`);
console.log(JSON.stringify({ originalHead, target, files: files.length, tracked: tracked.length, snapshot: resolve(generated, "rebase-snapshot.json.gz") }));

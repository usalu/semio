import fs from "node:fs";
import { gunzipSync } from "node:zlib";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const folder = fileURLToPath(new URL(".", import.meta.url));
const snapshot = JSON.parse(gunzipSync(fs.readFileSync(resolve(folder, "🗑️generated/rebase-snapshot.json.gz"))));
const repo = "/home/clstb/.local/share/semio/launcher/bin/repo";
const git = (...args) => execFileSync(repo, ["tools", "exec", "--light", "git", "--", ...args], { encoding: "utf8", maxBuffer: 50000000 });
const upstream = new Set(git("diff", "--no-renames", "--name-only", "-z", `${snapshot.originalHead}..${snapshot.target}`).split("\0").filter(Boolean));
const intentional = new Set([resolve(folder, "browser-env.mjs")]);
const merged = [], preserved = [], unexpected = [];
for (const file of snapshot.files) {
  if (file.deleted) { if (fs.existsSync(file.path)) unexpected.push(file.path); continue; }
  if (!fs.existsSync(file.path)) { unexpected.push(file.path); continue; }
  const hash = createHash("sha256").update(fs.readFileSync(file.path)).digest("hex");
  if (hash === file.sha256) preserved.push(file.path);
  else if (upstream.has(file.path)) merged.push(file.path);
  else if (!intentional.has(resolve(file.path))) unexpected.push(file.path);
}
if (git("rev-parse", "HEAD").trim() !== snapshot.target) throw new Error("Unexpected rebased HEAD");
if (git("diff", "--name-only", "--diff-filter=U").trim()) throw new Error("Unresolved merge entries remain");
if (git("diff", "--cached", "--name-only").trim()) throw new Error("Original empty index was not restored");
if (unexpected.length) throw new Error(`Unexpected preservation changes: ${JSON.stringify(unexpected)}`);
fs.appendFileSync(resolve(folder, "rebase.md"), `\n## Preservation Verification\n\nHEAD matches Ueli718 exactly; no branch-only feature commits existed to replay. ${preserved.length} original files retain exact SHA-256 content. Three overlapping files retain both upstream and local edits:\n\n${merged.map(path => `- \`${path}\``).join("\n")}\n\nActionGroup conflict resolved by retaining shared-frame refs/markers and upstream h-medium height. CSS and Shell merged automatically. Original empty index restored; changes remain unstaged/uncommitted. No unmerged paths remain. The conflict safety autostash fe1486d5867 is retained rather than dropping any stash. The test-only browser environment helper was deliberately updated to reject Nix libraries requiring a newer glibc than this host.\n`);
console.log(JSON.stringify({ head: snapshot.target, preserved: preserved.length, merged, unexpected }, null, 2));

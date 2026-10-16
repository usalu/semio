// 🧭️ Read-only: for each new Cargo.toml, count the workspaces that list its package as a member, and check its build-owners row.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, relative} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const GEN = process.argv[2]!;
const tracked = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files -- '*Cargo.toml'`.cwd(ROOT).text()).split("\n").filter(Boolean);
const untracked = (await Bun.$`git --no-optional-locks -c core.quotepath=false ls-files --others --exclude-standard -- '*Cargo.toml'`.cwd(ROOT).text()).split("\n").filter(Boolean);
const all = [...new Set([...tracked, ...untracked])].filter(p => !p.startsWith(".🧬semio/") && existsSync(join(ROOT, p)));
type WS = {dir: string; members: string[]; exclude: string[]; manifest: string};
const workspaces: WS[] = [];
const packageDirs = new Map<string, string>();
for (const m of all) {
  const parsed = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const dir = dirname(m) === "." ? "" : dirname(m);
  if (parsed.package) packageDirs.set(dir, parsed.package.name);
  if (parsed.workspace) workspaces.push({dir, members: parsed.workspace.members ?? [], exclude: parsed.workspace.exclude ?? [], manifest: m});
}
const matchPattern = (pattern: string, rel: string) => {
  if (!pattern.includes("*")) return pattern.replace(/\/$/, "") === rel;
  return new Bun.Glob(pattern).match(rel);
};
const membersOf = (ws: WS, dir: string): boolean => {
  const rel = ws.dir === "" ? dir : relative(ws.dir, dir);
  if (rel.startsWith("..")) return false;
  const inMembers = ws.members.some(p => matchPattern(p, rel));
  const excluded = ws.exclude.some(p => matchPattern(p, rel));
  return inMembers && !excluded;
};
const newList = readFileSync(join(GEN, "audit5r-new-cargo.txt"), "utf8").split("\n").filter(Boolean);
const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[];
const out: string[] = [];
let exactlyOne = 0, zero = 0, multi = 0, ownerMissing = 0;
for (const m of newList) {
  const dir = dirname(m);
  const wss = workspaces.filter(ws => membersOf(ws, dir));
  const names = wss.map(w => w.manifest);
  const parsed = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const pkg = parsed.package?.name ?? "(no package)";
  const hasOwner = owners.some(o => o.root === dir || (Array.isArray(o.packages) && o.packages.some((p: any) => (typeof p === "string" ? p : p.manifest ?? p.path ?? "") === m)));
  const ownerRowsForDir = owners.filter(o => o.root === dir || o.root === dirname(dir));
  if (wss.length === 1) exactlyOne++; else if (wss.length === 0) zero++; else multi++;
  if (!hasOwner) ownerMissing++;
  out.push([m, pkg, wss.length, names.join(" ; "), hasOwner ? "owner-row" : "NO-OWNER-ROW", ownerRowsForDir.length].join("\t"));
}
await Bun.write(join(GEN, "audit5r-new-cargo-membership.tsv"), out.join("\n") + "\n");
console.log("workspaces", workspaces.length, "new manifests", newList.length, "exactly one", exactlyOne, "zero", zero, "more than one", multi, "no owner row", ownerMissing);

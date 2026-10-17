// 🔎️ Read-only: manifests that depend on the package semio-framework (the facade), with the dependency key and table.
import {readFileSync, existsSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const git = async (args: string[]) => (await Bun.$`git -c core.quotepath=false -c core.fsmonitor=false ${args}`.cwd(ROOT).text()).split("\n").filter(p => p && !p.startsWith(".🧬semio/"));
const ms = [...new Set([...(await git(["ls-files", "--", "*Cargo.toml"])), ...(await git(["ls-files", "--others", "--exclude-standard", "--", "*Cargo.toml"]))])].filter(p => existsSync(`${ROOT}/${p}`));
const out: string[] = [];
const visit = (m: string, table: string, deps: any) => {
  for (const [k, v] of Object.entries<any>(deps ?? {})) {
    const pkg = typeof v === "object" && v && "package" in v ? v.package : k;
    if (pkg === "semio-framework") out.push([m, table, k, JSON.stringify(v)].join("\t"));
  }
};
for (const m of ms) {
  let d: any;
  try { d = Bun.TOML.parse(readFileSync(`${ROOT}/${m}`, "utf8")); } catch { continue; }
  for (const t of ["dependencies", "dev-dependencies", "build-dependencies"]) visit(m, t, d[t]);
  for (const [tg, tv] of Object.entries<any>(d.target ?? {})) for (const t of ["dependencies", "dev-dependencies"]) visit(m, `target.${tg}.${t}`, tv[t]);
  visit(m, "workspace.dependencies", d.workspace?.dependencies);
}
console.log(out.join("\n"));

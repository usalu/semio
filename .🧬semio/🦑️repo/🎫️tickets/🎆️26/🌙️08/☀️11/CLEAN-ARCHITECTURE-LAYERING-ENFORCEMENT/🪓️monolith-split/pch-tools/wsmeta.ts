// 📐️ Read-only: per artifact workspace under a prefix, shows exclude-patterns, declared members and whether standards crates are declared.
import {readFileSync, existsSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const out = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files -- ${process.argv[2]!}`.cwd(ROOT).text()).split("\n").filter(p => p.endsWith("/Cargo.toml") && existsSync(join(ROOT, p)));
for (const m of out) {
  const t = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  if (!t.workspace) continue;
  const meta = t.workspace.metadata?.semio?.repository;
  const members: string[] = t.workspace.members ?? [];
  console.log([m.replace(process.argv[2]!, "").slice(0, 70), meta ? "meta" : "nometa", `members=${members.length}`, `stdMembers=${members.filter(x => x.includes("🏅️standards")).length}`, `excl=${JSON.stringify(meta?.["exclude-patterns"] ?? t.workspace.exclude ?? []).slice(0, 120)}`].join("\t"));
}

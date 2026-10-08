import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const files = spawnSync("git", ["-c", "core.quotepath=false", "ls-files", "--", ":!.🧬semio", "*📋️project.json"], { cwd: "C:/git/semio", encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("
").filter(Boolean);
const rows: string[] = [];
for (const f of files) {
  let j: any;
  try { j = JSON.parse(readFileSync("C:/git/semio/" + f, "utf8")); } catch { continue; }
  const dash = j.metadata?.semio?.dashboard;
  const hits: string[] = [];
  for (const [t, v] of Object.entries<any>(j.targets ?? {})) {
    const d = v.metadata?.semio?.dashboard;
    if (d) hits.push(`target ${t}${d.ready ? " ready=" + JSON.stringify(d.ready) : ""}${d.requires ? " requires=" + JSON.stringify(d.requires) : ""}${v.continuous ? " continuous" : ""}`);
  }
  if (dash) for (const k of ["tools", "compounds", "groups"]) for (const x of dash[k] ?? []) hits.push(`${k.slice(0, -1)} ${x.id}${x.ready ? " ready=" + JSON.stringify(x.ready) : ""}`);
  if (hits.length) rows.push(`## ${j.name}  (${f})\n` + hits.map((h) => "  " + h).join("\n"));
}
console.log(rows.join("\n"));

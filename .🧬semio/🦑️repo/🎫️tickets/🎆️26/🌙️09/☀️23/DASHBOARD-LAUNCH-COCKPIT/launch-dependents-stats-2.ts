import { readFileSync, writeFileSync } from "node:fs";
const d = Bun.JSONC.parse(readFileSync(".vscode/launch.json", "utf8")) as any;
const rows = d.configurations as any[];
const cls = new Map<string, any[]>();
const add = (k: string, r: any) => { (cls.get(k) ?? cls.set(k, []).get(k)!).push(r); };
for (const r of rows) {
  const c = String(r.command ?? "");
  const env = r.env && Object.keys(r.env).length > 0;
  if (/^bun nx run \$\{input:[^}]+\}:\S+$/.test(c)) add("project-picker (bun nx run ${input:…}:target)", r);
  else if (/^bun nx run workspace:dev -- /.test(c)) add("workspace:dev playground variant", r);
  else if (/^bun nx run [^\s$]+:[^\s]+$/.test(c) && !env) add("plain nx target, no env", r);
  else if (/^bun nx run [^\s$]+:[^\s]+$/.test(c) && env) add("plain nx target WITH env", r);
  else if (/^bun nx run [^\s$]+:[^\s]+ /.test(c)) add("nx target with extra args", r);
  else if (/^bun \.\/📜️script\.ts/.test(c)) add("root script.ts verb", r);
  else if (/^bun nx /.test(c)) add("other bun nx", r);
  else if (/^bun /.test(c)) add("other bun", r);
  else add("other (" + c.split(" ")[0] + ")", r);
}
for (const [k, v] of [...cls].sort((a, b) => b[1].length - a[1].length)) console.log(String(v.length).padStart(5), k, " e.g. ", v.slice(0, 2).map(r => r.name + " => " + r.command).join(" || ").slice(0, 300));
console.log("with env:", rows.filter(r => r.env && Object.keys(r.env).length).length, " with ${input:", rows.filter(r => JSON.stringify(r).includes("${input:")).length, " cwd not workspaceFolder:", rows.filter(r => r.cwd && r.cwd !== "${workspaceFolder}").length);
const names = rows.map(r => r.name);
writeFileSync("/tmp/launch-names.txt", names.join("\n"));

import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { from: string; spec: string; names: string; kind: string }[];
const exempt = /(^|\/)(?:📜️script\.ts|🏗️builder\/🌐️vite\/🟦️\.ts|🧪️tests\/🎚️config\/🟦️\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s)$/u;
const live = rows.filter((r) => !exempt.test(r.from));
const by: Record<string, { n: number; test: number; names: Set<string>; files: Set<string> }> = {};
for (const r of live) {
  const k = r.kind + " " + r.spec.replace(/[^\x00-\x7f]/gu, "");
  const o = (by[k] ??= { n: 0, test: 0, names: new Set(), files: new Set() });
  o.n++; if (/(^|\/)🧪️tests\/|\.test\.|stories|🎨️stories/u.test(r.from)) o.test++;
  for (const n of r.names.split(",")) o.names.add(n.trim().replace(/^type /u, ""));
  o.files.add(r.from);
}
for (const [k, o] of Object.entries(by).sort((a, b) => b[1].n - a[1].n)) console.log(String(o.n).padStart(3), String(o.test).padStart(3), k, "|", [...o.names].slice(0, 14).join(", ").slice(0, 200));
console.log("total live", live.length, "exempt", rows.length - live.length);

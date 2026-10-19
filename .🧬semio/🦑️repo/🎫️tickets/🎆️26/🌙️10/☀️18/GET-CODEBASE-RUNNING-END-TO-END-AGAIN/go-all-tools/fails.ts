import { readFileSync } from "node:fs";
const lines = readFileSync(process.argv[2]!, "utf8").split("\n");
const out = new Map<string, string[]>();
const failed = new Set<string>();
const pkgOut: string[] = [];
for (const line of lines) {
  if (!line.startsWith("{")) { if (line.trim()) pkgOut.push(line); continue; }
  let e: { Action?: string; Test?: string; Package?: string; Output?: string };
  try { e = JSON.parse(line); } catch { continue; }
  const k = e.Test ?? `<pkg ${e.Package}>`;
  if (e.Action === "output") { if (!out.has(k)) out.set(k, []); out.get(k)!.push(e.Output ?? ""); }
  if (e.Action === "fail") failed.add(k);
}
for (const k of failed) { console.log(`=== FAIL ${k}`); console.log((out.get(k) ?? []).join("").split("\n").filter((l) => !/^=== (RUN|PAUSE|CONT)/.test(l)).slice(0, Number(process.argv[3] ?? 25)).join("\n")); }
if (pkgOut.length) console.log("NON-JSON:\n" + pkgOut.slice(0, 40).join("\n"));

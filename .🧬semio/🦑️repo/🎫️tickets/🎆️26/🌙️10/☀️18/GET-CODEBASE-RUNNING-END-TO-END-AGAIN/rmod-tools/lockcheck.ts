import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { walkFiles, root } from "./lib.ts";
import { cmpVer } from "./crates.ts";
const map = JSON.parse(readFileSync(import.meta.dir + "/crate-map.json", "utf8")).crates as Record<string, string>;
const olds = Object.keys(map), news = Object.values(map);
const parseEntry = (e: string) => { const m = e.match(/^"([^" ]+)(?: ([^" ]+))?(?: \(([^)]*)\))?"$/); return m ? { name: m[1]!, version: m[2] ?? "", source: m[3] ?? "" } : { name: e, version: "", source: "" }; };
const cmp = (a: any, b: any) => (a.name < b.name ? -1 : a.name > b.name ? 1 : a.version !== b.version ? cmpVer(a.version, b.version) : a.source < b.source ? -1 : a.source > b.source ? 1 : 0);
let locks = 0, unsortedPk = 0, unsortedDeps = 0, danglingDeps = 0, oldLeft = 0, newSeen = 0;
const bad: string[] = [];
for (const f of walkFiles((n) => n === "Cargo.lock")) {
  const t = readFileSync(f, "utf8");
  locks++;
  const blocks = t.split(/\n(?=\[\[package\]\]\n)/).filter((b) => b.startsWith("[[package]]"));
  const ids = blocks.map((b) => ({ name: b.match(/^name = "([^"]+)"/m)![1]!, version: b.match(/^version = "([^"]+)"/m)?.[1] ?? "", source: b.match(/^source = "([^"]+)"/m)?.[1] ?? "" }));
  const names = new Set(ids.map((i) => i.name));
  for (let i = 1; i < ids.length; i++) if (cmp(ids[i - 1], ids[i]) > 0) { unsortedPk++; bad.push(`${f.slice(root.length + 1)}: package order ${ids[i - 1]!.name} > ${ids[i]!.name}`); break; }
  for (const b of blocks) {
    const m = b.match(/dependencies = \[\n((?:  "[^\n]*",?\n)+)\]/);
    if (!m) continue;
    const items = m[1]!.trim().split("\n").map((l) => parseEntry(l.trim().replace(/,$/, "")));
    for (let i = 1; i < items.length; i++) if (cmp(items[i - 1], items[i]) > 0) { unsortedDeps++; bad.push(`${f.slice(root.length + 1)}: deps order in ${b.match(/^name = "([^"]+)"/m)![1]}`); break; }
    for (const it of items) if (!names.has(it.name)) { danglingDeps++; bad.push(`${f.slice(root.length + 1)}: dangling dep ${it.name}`); }
  }
  for (const o of olds) if (names.has(o) || t.includes(`"${o}"`)) oldLeft++;
  for (const n of news) if (names.has(n)) newSeen++;
}
console.log(`locks ${locks}; unsorted package order ${unsortedPk}; unsorted dependency lists ${unsortedDeps}; dangling deps ${danglingDeps}; old names left ${oldLeft}; new names present ${newSeen}`);
for (const b of bad.slice(0, 15)) console.log("  ", b);

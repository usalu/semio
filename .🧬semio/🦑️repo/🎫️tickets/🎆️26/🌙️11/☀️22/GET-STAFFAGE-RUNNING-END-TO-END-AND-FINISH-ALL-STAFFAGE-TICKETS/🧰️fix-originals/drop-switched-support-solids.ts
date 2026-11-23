/** 🪛️ Removes from `volumes.*.structure` the support solids whose anchor is not a permanent support of `structure.supports` (a foot a pose lifts), as a text edit that keeps the hand formatting of the file: `bun <this file> <asset json>`. Nothing is written unless the parsed result differs from the parsed input by exactly those solids. */
import { readFileSync, writeFileSync } from "node:fs";

type Solid = { id: string; part?: string; anchor?: string };
const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
const before = JSON.parse(text);
const permanent = new Set<string>(before.asset.structure.supports);
const lines = text.split("\n");
const remove = new Set<number>();
lines.forEach((line, index) => {
  const id = /^(\s*)"id": "([^"]+)",?$/.exec(line);
  if (id === null || !/^\s*\{$/.test(lines[index - 1]!)) return;
  const indent = lines[index - 1]!.length - 1;
  let end = index;
  while (!new RegExp(`^ {${indent}}\\},?$`).test(lines[end]!)) end++;
  const solid = JSON.parse(lines.slice(index - 1, end + 1).join("\n").replace(/,$/, "")) as Solid;
  if (solid.part !== "support" || permanent.has(solid.anchor!)) return;
  if (!lines[end]!.endsWith(",")) throw new Error(`${solid.id} is the last entry of its list: remove it by hand`);
  for (let i = index - 1; i <= end; i++) remove.add(i);
});
const next = lines.filter((_, index) => !remove.has(index)).join("\n");
const after = JSON.parse(next);
for (const [scale, volumes] of Object.entries<{ structure?: Solid[] }>(before.asset.volumes)) {
  if (!Array.isArray(volumes.structure)) continue;
  const dropped = volumes.structure.filter((solid) => solid.part === "support" && !permanent.has(solid.anchor!));
  volumes.structure = volumes.structure.filter((solid) => !dropped.includes(solid));
  console.log(`1:${scale} dropped ${dropped.map((solid) => solid.id).join(", ") || "nothing"}`);
}
if (JSON.stringify(before) !== JSON.stringify(after)) throw new Error(`${file}: the text edit does not equal the intended removal`);
writeFileSync(file, next);

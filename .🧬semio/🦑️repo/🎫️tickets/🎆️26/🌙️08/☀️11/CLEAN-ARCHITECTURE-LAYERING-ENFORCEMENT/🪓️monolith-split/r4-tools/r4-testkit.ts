// Source change: turns the cross-crate cfg(test) helpers listed by r4-xtest.ts (JSON rows) into `testkit` feature items of their owner crate.
import {readFileSync, writeFileSync} from "node:fs";
import {relative, resolve} from "node:path";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {useLeafNames, identifierSet} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
type Row = {user: string; userDir: string; owner: string; ownerDir: string; item: string; defFile: string};
const rows: Row[] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const ROOT = "/Users/ueli/Documents/semio";
const isGate = (a: string) => /cfg\(\s*test\s*\)/u.test(a);
const files = new Map<string, Set<string>>();
for (const row of rows) files.set(row.defFile, new Set([...(files.get(row.defFile) ?? []), row.item]));
for (const [file, seeds] of files) {
  const abs = resolve(ROOT, file);
  let text = readFileSync(abs, "utf8");
  const tree = parseRustSourceTree(text);
  const gated = tree.items.filter((it) => it.attributeTexts.some(isGate));
  const convert = new Set<number>();
  const idsOf = (i: number) => identifierSet(text.slice(gated[i]!.declStart, gated[i]!.end));
  const queue = gated.map((it, i) => (it.name && seeds.has(it.name) ? i : -1)).filter((i) => i >= 0);
  if (queue.length !== seeds.size) throw new Error(`${file}: seeds ${[...seeds]} not all found among ${gated.map((g) => g.name)}`);
  while (queue.length) {
    const i = queue.pop()!;
    if (convert.has(i)) continue;
    convert.add(i);
    const ids = idsOf(i);
    gated.forEach((other, j) => {
      if (convert.has(j)) return;
      if (other.kind === "use") { if (useLeafNames(text.slice(other.declStart, other.end)).some((n) => ids.has(n))) queue.push(j); }
      else if (other.name && ids.has(other.name)) queue.push(j);
    });
  }
  const edits = [...convert].map((i) => gated[i]!).sort((a, b) => b.start - a.start);
  for (const it of edits) {
    const head = text.slice(it.start, it.declStart).replace(/#\[cfg\(\s*test\s*\)\]/u, '#[cfg(feature = "testkit")]');
    text = text.slice(0, it.start) + head + text.slice(it.declStart);
  }
  writeFileSync(abs, text);
  console.log("converted", file.split("/").slice(-4).join("/"), [...convert].map((i) => gated[i]!.name ?? "use").join(","));
}
const owners = new Map<string, Row>();
for (const row of rows) owners.set(row.owner, row);
for (const row of owners.values()) {
  const p = resolve(row.ownerDir, "Cargo.toml");
  let t = readFileSync(p, "utf8");
  if (!/^\[features\]/mu.test(t)) t = t.replace(/^\[dependencies\]/mu, '[features]\ntestkit = []\n\n[dependencies]');
  else if (!/^testkit\s*=/mu.test(t)) t = t.replace(/^\[features\]\n/mu, '[features]\ntestkit = []\n');
  const selfRow = `${row.owner} = { path = ".", features = ["testkit"] }`;
  if (!t.includes(selfRow)) t = /^\[dev-dependencies\]/mu.test(t) ? t.replace(/^\[dev-dependencies\]\n/mu, `[dev-dependencies]\n${selfRow}\n`) : `${t.trimEnd()}\n\n[dev-dependencies]\n${selfRow}\n`;
  writeFileSync(p, t);
  console.log("owner manifest", relative(ROOT, p).split("/").slice(-5).join("/"));
}
for (const row of rows) {
  const p = resolve(row.userDir, "Cargo.toml");
  let t = readFileSync(p, "utf8");
  const rel = relative(row.userDir, row.ownerDir);
  const userRow = `${row.owner} = { path = "${rel}", features = ["testkit"] }`;
  if (t.includes(userRow)) continue;
  t = /^\[dev-dependencies\]/mu.test(t) ? t.replace(/^\[dev-dependencies\]\n/mu, `[dev-dependencies]\n${userRow}\n`) : `${t.trimEnd()}\n\n[dev-dependencies]\n${userRow}\n`;
  writeFileSync(p, t);
  console.log("user manifest", row.user, "<-", row.owner);
}

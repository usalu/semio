// 🚧️ Finds cfg(test) items (fn/const/static/struct/enum/use/mod with pub or pub(crate)) whose name is used by files of another planned crate: cfgtest-cross.ts <pkg>
import { readFileSync } from "node:fs";
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const plan = await planArtifactCrates(root, process.argv[2]!, { mutationFamilies: "off", tests: { fixtures: true, aggregate: true, hoist: true, detach: true } });
const crateOf = new Map<string, string>();
for (const c of plan.crates) for (const p of [...(c.lib?.paths ?? []), ...(c.test?.paths ?? [])]) if (p.endsWith(".rs") && !crateOf.has(p)) crateOf.set(p, c.name.replace("semio-s-artifact-stdio-", ""));
import { existsSync } from "node:fs";
const files = [...crateOf.keys()].filter((f) => existsSync(`${root}/${f}`));
const defs: { name: string; file: string; kind: string }[] = [];
for (const f of files) {
  const text = readFileSync(`${root}/${f}`, "utf8");
  for (const m of text.matchAll(/#\[cfg\(test\)\]\s*(?:\/\/[^\n]*\n\s*)*(?:#\[[^\]]*\]\s*)*pub(?:\([a-z]+\))?\s+(fn|const|static|struct|enum|type|use)\s+(?:\{[^}]*\}|(\w+)(?:::\w+)*)/gu)) {
    if (m[1] === "use") continue;
    defs.push({ name: m[2]!, file: f, kind: m[1]! });
  }
}
for (const d of defs) {
  const re = new RegExp(`\\b${d.name}\\b`, "u");
  const users = files.filter((f) => f !== d.file && crateOf.get(f) !== crateOf.get(d.file) && re.test(readFileSync(`${root}/${f}`, "utf8")));
  if (users.length) console.log(`${d.kind} ${d.name} @ ${crateOf.get(d.file)} ${d.file.split("/").slice(-4).join("/")} <- ${users.map((u) => crateOf.get(u)).join(",")} ${users[0]!.split("/").slice(-4).join("/")}`);
}
console.log("defs", defs.length);

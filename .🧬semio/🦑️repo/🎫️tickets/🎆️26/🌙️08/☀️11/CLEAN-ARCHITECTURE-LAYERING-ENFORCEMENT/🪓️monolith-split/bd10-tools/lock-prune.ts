// 🧰 bd10: checks a workspace Cargo.lock for dangling references and (with --write) drops the blocks no workspace member reaches.
import {readFileSync, writeFileSync} from "node:fs";
const [lockPath, ...roots] = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const write = process.argv.includes("--write");
const text = readFileSync(lockPath!, "utf8");
const parts = text.split(/\n(?=\[\[package\]\]\n)/u);
const header = parts[0]!.startsWith("[[package]]") ? "" : parts.shift()!;
type B = {name: string; version: string; text: string; deps: string[]};
const blocks: B[] = parts.map((t) => ({text: t, name: /name = "([^"]+)"/u.exec(t)![1]!, version: /version = "([^"]+)"/u.exec(t)![1]!, deps: [...(/dependencies = \[([\s\S]*?)\n\]/u.exec(t)?.[1] ?? "").matchAll(/"([^"]+)"/gu)].map((m) => m[1]!)}));
const resolveRef = (ref: string): B[] => {
  const [name, version] = ref.split(" ");
  return blocks.filter((b) => b.name === name && (!version || b.version === version));
};
const seen = new Set<B>(), dangling: string[] = [], queue: B[] = [];
for (const r of roots) { const hit = blocks.filter((b) => b.name === r && !/^source = /mu.test(b.text)); if (!hit.length) dangling.push("root " + r); queue.push(...hit); }
while (queue.length) {
  const b = queue.pop()!;
  if (seen.has(b)) continue;
  seen.add(b);
  for (const d of b.deps) { const hit = resolveRef(d); if (hit.length !== 1) dangling.push(`${b.name} -> ${d} (${hit.length})`); queue.push(...hit); }
}
const dropped = blocks.filter((b) => !seen.has(b));
console.log(`blocks ${blocks.length}, reachable ${seen.size}, unreachable ${dropped.length}, dangling ${dangling.length}`);
for (const d of dangling.slice(0, 40)) console.log("DANGLING", d);
for (const b of dropped.slice(0, 80)) console.log("UNREACHABLE", b.name, b.version, /^source = /mu.test(b.text) ? "registry" : "path");
if (write && !dangling.length) writeFileSync(lockPath!, `${header ? `${header}\n` : ""}${blocks.filter((b) => seen.has(b)).map((b) => b.text.trimEnd()).join("\n\n")}\n`);

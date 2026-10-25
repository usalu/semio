import { existsSync, readFileSync, renameSync, writeFileSync, statSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join, relative, resolve, basename } from "node:path";

const root = "/Users/ueli/Documents/semio";
const [, , listFile, mode = "dry"] = process.argv;
if (!listFile) throw new Error("usage: rename-leaf.ts <tsv: old\tnew (repo-relative)> [dry|apply]");
const pairs = readFileSync(listFile, "utf8").split("\n").filter(Boolean).map((line) => line.split("\t") as [string, string]);
const nfc = (value: string) => value.normalize("NFC");
const byOld = new Map(pairs.map(([from, to]) => [nfc(from), nfc(to)]));
const newTargets = new Map<string, string>();
for (const [from, to] of byOld) {
  if (newTargets.has(to)) throw new Error(`collision between ${from} and ${newTargets.get(to)} -> ${to}`);
  newTargets.set(to, from);
  if (existsSync(join(root, to))) throw new Error(`target exists: ${to}`);
}
const basenames = [...new Set([...byOld.keys()].map((path) => basename(path)))];
const candidates = new Set<string>();
for (const name of basenames) {
  const found = spawnSync("rg", ["-l", "-F", "--no-messages", "-g", "!node_modules", "-g", "!.git", "-g", "!**/🗑️generated/**", "-g", "!.🧬semio/🦑️repo/🎫️tickets/**", "-g", "!**/target/**", "-g", "!**/dist/**", "-g", "!bazel-*", "-g", "!.cache", name, "."], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
  for (const line of found.split("\n")) if (line) candidates.add(nfc(line.replace(/^\.\//, "")));
}
const literal = /(["'`])((?:(?!\1)[^\\\n]|\\.)*?)\1/gu;
const unresolved: string[] = [];
const edits = new Map<string, { text: string; count: number }>();
const oldAbs = new Map([...byOld.keys()].map((path) => [resolve(root, path), path]));
for (const file of candidates) {
  if (byOld.has(file)) continue;
  let text: string;
  try { text = readFileSync(join(root, file), "utf8"); } catch { continue; }
  let count = 0;
  const rewritten = text.replace(literal, (whole, quote: string, body: string) => {
    const normal = nfc(body);
    const name = basenames.find((candidate) => normal === candidate || normal.endsWith(`/${candidate}`));
    if (!name) return whole;
    let target: string | undefined;
    if (normal === name) target = [...byOld.keys()].find((path) => basename(path) === name && dirname(path) === dirname(file));
    else if (normal.startsWith("./") || normal.startsWith("../")) target = oldAbs.get(resolve(root, dirname(file), normal));
    else target = byOld.has(normal) ? normal : undefined;
    if (!target) { unresolved.push(`${file}: ${JSON.stringify(body)}`); return whole; }
    count += 1;
    const replacement = byOld.get(target)!;
    return `${quote}${body.slice(0, body.length - name.length)}${basename(replacement)}${quote}`;
  });
  if (count > 0) edits.set(file, { text: rewritten, count });
}
const recent = [...edits.keys(), ...byOld.keys()].filter((file) => Date.now() - statSync(join(root, file)).mtimeMs < 30 * 60 * 1000);
for (const file of recent) console.log("RECENT", file);
console.log(`pairs ${byOld.size} candidates ${candidates.size} files-to-edit ${edits.size} unresolved ${unresolved.length}`);
for (const line of unresolved.slice(0, 400)) console.log("UNRESOLVED", line);
if (mode === "apply" && recent.length > 0 && !process.env.ALLOW_RECENT) throw new Error("recent peer edits; rerun later or set ALLOW_RECENT");
if (mode === "apply") {
  for (const [file, { text }] of edits) writeFileSync(join(root, file), text);
  for (const [from, to] of byOld) renameSync(join(root, from), join(root, to));
  console.log("applied");
}

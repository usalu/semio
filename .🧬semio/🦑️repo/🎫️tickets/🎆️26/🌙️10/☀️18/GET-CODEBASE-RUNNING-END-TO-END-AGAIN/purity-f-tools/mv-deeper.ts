import { existsSync, readFileSync, renameSync, statSync, writeFileSync, mkdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { basename, dirname, join, relative, resolve, sep } from "node:path";

/**
 * usage: bun mv-deeper.ts <pairs.tsv: old<TAB>new (repo-relative)> [apply]
 * Moves each file to a new (deeper or sibling) location: relative literals inside the moved file are re-expressed from the new
 * directory, every literal elsewhere that resolves to an old path (relative, repo-relative or relative to an extra base) is
 * rewritten to the new path. Unresolvable mentions of the old basename are printed.
 */
const root = "/Users/ueli/Documents/semio";
const [, , listFile, mode = "dry"] = process.argv;
const nfc = (v: string) => v.normalize("NFC");
const pairs = readFileSync(listFile!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t").map(nfc) as [string, string]);
const byOld = new Map(pairs), oldAbs = new Map(pairs.map(([o, n]) => [resolve(root, o), n]));
const extraBases = (process.env.EXTRA_BASES ?? "").split(",").filter(Boolean).map(nfc);
for (const [o, n] of pairs) { if (!existsSync(join(root, o))) throw new Error(`missing ${o}`); if (existsSync(join(root, n))) throw new Error(`target exists ${n}`); }
const literal = /(["'`])((?:(?!\1)[^\\\n]|\\.)*?)\1/gu;
const names = [...new Set(pairs.map(([o]) => basename(o)))];
const found = new Set<string>();
for (const name of names) {
  const out = spawnSync("rg", ["-l", "-F", "--no-messages", "-g", "!node_modules", "-g", "!.git", "-g", "!**/🗑️generated/**", "-g", "!.🧬semio/**", "-g", "!**/target/**", "-g", "!**/dist/**", "-g", "!bazel-*", "-g", "!.cache", "-g", "!*.lock", name, "🧰️framework", "♻️mit-bestand", "🎓️teaching", "✏️s", "🌎️hub", "🏢️semio-tech", "📋️build-owners.json", "🔒️dependencies.json", "package.json"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
  for (const l of out.split("\n")) if (l) found.add(nfc(l));
}
const unresolved: string[] = [], edits = new Map<string, { text: string; count: number }>();
const moved = new Set(pairs.map(([o]) => o));
for (const file of found) {
  if (moved.has(file)) continue;
  let text: string; try { text = readFileSync(join(root, file), "utf8"); } catch { continue; }
  let count = 0;
  const out = text.replace(literal, (whole, q: string, body: string) => {
    const b = nfc(body);
    if (!names.some((n) => b.endsWith(n))) return whole;
    let target: string | undefined;
    const cands = [resolve(root, dirname(file), b), resolve(root, b), ...extraBases.map((e) => resolve(root, e, b))];
    for (const c of cands) { if (oldAbs.has(c)) { target = oldAbs.get(c); break; } }
    if (!target) { if (!/[\s]/.test(b) || b.includes("/")) unresolved.push(`${file}: ${JSON.stringify(body)}`); return whole; }
    count += 1;
    const abs = resolve(root, target);
    let repl: string;
    if (b.startsWith("./") || b.startsWith("../")) { const r = relative(dirname(join(root, file)), abs).split(sep).join("/"); repl = r.startsWith(".") ? r : "./" + r; }
    else if (resolve(root, b) === [...oldAbs.keys()].find((k) => k === resolve(root, b))) repl = target;
    else { const base = extraBases.map((e) => resolve(root, e)).find((e) => oldAbs.has(resolve(e, b))); repl = base ? relative(base, abs).split(sep).join("/") : target; }
    return `${q}${repl}${q}`;
  });
  if (count > 0) edits.set(file, { text: out, count });
}
const selfEdits = new Map<string, string>();
for (const [o, n] of pairs) {
  const text = readFileSync(join(root, o), "utf8"); const oldDir = dirname(join(root, o)), newDir = dirname(join(root, n));
  selfEdits.set(o, text.replace(literal, (whole, q: string, body: string) => {
    if (!(body.startsWith("./") || body.startsWith("../"))) return whole;
    const abs = resolve(oldDir, nfc(body));
    if (!existsSync(abs) && ![...oldAbs.keys()].includes(abs)) return whole;
    const target = oldAbs.has(abs) ? resolve(root, oldAbs.get(abs)!) : abs;
    const r = relative(newDir, target).split(sep).join("/");
    return `${q}${r.startsWith(".") ? r : "./" + r}${q}`;
  }));
}
console.log(`pairs ${pairs.length} candidate-files ${found.size} files-to-edit ${edits.size}`);
for (const [f, e] of edits) console.log("EDIT", e.count, f.slice(-110));
for (const u of unresolved) console.log("UNRESOLVED", u.slice(0, 260));
for (const [o, n] of pairs) if (selfEdits.get(o) !== readFileSync(join(root, o), "utf8")) console.log("SELF", n.slice(-100));
const recent = [...edits.keys(), ...moved].filter((f) => Date.now() - statSync(join(root, f)).mtimeMs < 20 * 60 * 1000);
for (const f of recent) console.log("RECENT", f.slice(-110));
if (mode === "apply") {
  if (recent.length > 0 && !process.env.ALLOW_RECENT) throw new Error("recent peer edits");
  for (const [f, e] of edits) writeFileSync(join(root, f), e.text);
  for (const [o, n] of pairs) { mkdirSync(dirname(join(root, n)), { recursive: true }); writeFileSync(join(root, o), selfEdits.get(o)!); renameSync(join(root, o), join(root, n)); }
  console.log("applied");
}

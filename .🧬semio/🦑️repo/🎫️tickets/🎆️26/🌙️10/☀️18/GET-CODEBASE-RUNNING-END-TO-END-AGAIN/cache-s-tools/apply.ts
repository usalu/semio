/** ✍️ Applies the cache decisions to every ✏️s manifest in place; --write persists, default is a dry run. Each file is re-read right before writing. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, manifests, facts } from "./lib.ts";
import { decide } from "./decide.ts";
import { scan, prop, setProps, apply, mergeNamedInput, type Edit } from "./jsonspan.ts";
import { extras } from "./extras.ts";
import { nativeName } from "./lib.ts";

const write = Bun.argv.includes("--write");
const backupPath = join(import.meta.dir, "..", "🗑️generated", "cache-s-backup.json");
const backup: Record<string, string> = write && existsSync(backupPath) ? JSON.parse(readFileSync(backupPath, "utf8")) : {};
const summary: Record<string, number> = {};
const rows: string[] = [];
const unconverted: string[] = [];
let changedFiles = 0;
for (const file of manifests()) {
  const path = join(ROOT, file);
  const text = readFileSync(path, "utf8");
  const p = facts(file, text);
  const tree = scan(text);
  const targets = prop(tree, "targets")?.value;
  if (!targets?.props) continue;
  const edits: Edit[] = [];
  for (const tp of targets.props) {
    const target = p.json.targets[tp.key];
    if (target.executor === undefined) {
      const command = target.options?.command;
      if (typeof command === "string" && /^bun \.\/📜️script\.ts\b/u.test(command)) {
        edits.push(...setProps(text, tp.value, [["executor", "repo:script"]], []));
        const options = prop(tp.value, "options")?.value;
        if (options && target.options.cwd === undefined) edits.push(...setProps(text, options, [["cwd", p.root]], []));
        summary["executor:repo:script"] = (summary["executor:repo:script"] ?? 0) + 1;
      } else unconverted.push(`${file}#${tp.key}: ${JSON.stringify(target.options?.command ?? target.options?.commands ?? null)}`);
    }
    const d = decide(p, tp.key, target);
    if (!d) continue;
    const entries: [string, unknown][] = [];
    if (d.cache !== undefined && target.cache !== d.cache) entries.push(["cache", d.cache]);
    if (d.outputs) entries.push(["outputs", d.outputs]);
    if (d.inputs) entries.push(["inputs", d.inputs]);
    if (!entries.length) continue;
    edits.push(...setProps(text, tp.value, entries, ["executor"]));
    summary[d.note.split(":")[0] + ":" + (d.cache ? "on" : "off")] = (summary[d.note.split(":")[0] + ":" + (d.cache ? "on" : "off")] ?? 0) + 1;
    rows.push(`${file}\t${tp.key}\t${entries.map(([k]) => k).join(",")}\t${d.note}`);
  }
  const i = p.root.indexOf("/📦️packages/");
  const owner = i >= 0 ? p.root.slice(0, i) : p.root;
  const cachedReader = Object.keys(p.json.targets ?? {}).some((n) => /^(?:verify|test|check)(?:-|$)/u.test(n) || nativeName(n));
  if (cachedReader && p.json.targets) {
    const e = extras(p.root, owner, p.native ? "native" : "script");
    if (e.globs.length) {
      const group = p.native ? "nativeSources" : "default";
      const before = edits.length;
      edits.push(...mergeNamedInput(text, tree, group, e.globs));
      if (edits.length > before) { summary["extras:" + group] = (summary["extras:" + group] ?? 0) + 1; rows.push(`${file}\t(project)\tnamedInputs.${group}\t+${e.globs.length}`); }
    }
  }
  if (!edits.length) continue;
  const next = apply(text, edits);
  const parsed = JSON.parse(next);
  if (Bun.argv.includes("--diff") && Bun.argv.includes(file)) { await Bun.write("/tmp/claude-501/after.json", next); await Bun.write("/tmp/claude-501/before.json", text); void parsed; }
  changedFiles++;
  if (write) {
    if (readFileSync(path, "utf8") !== text) { console.error("[skip] changed concurrently:", file); continue; }
    backup[file] ??= text;
    writeFileSync(path, next);
  }
}
console.log(JSON.stringify({ write, changedFiles, summary }, null, 1));
if (unconverted.length) console.log("UNCONVERTED\n" + unconverted.join("\n"));
if (Bun.argv.includes("--rows")) console.log(rows.join("\n"));

if (write) { mkdirSync(join(import.meta.dir, "..", "🗑️generated"), { recursive: true }); writeFileSync(backupPath, JSON.stringify(backup)); }

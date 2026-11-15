import {existsSync, readFileSync} from "node:fs";
import {dirname, join, relative} from "node:path";
import {brotliDecompressSync} from "node:zlib";

const findRoot = (directory: string): string => (existsSync(join(directory, "AGENTS.md")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd() : findRoot(dirname(directory)));
const root = process.env.SEMIO_WORKSPACE_ROOT ?? findRoot(import.meta.dir);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {openFactIndexV1} = await import(join(root, library, "🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const directory = join(root, ".cache/b/g/m/generations");
const pointer = JSON.parse(readFileSync(join(directory, "current.json"), "utf8")), manifest = JSON.parse(readFileSync(join(directory, "m", pointer.manifest + ".json"), "utf8"));
const raw = brotliDecompressSync(readFileSync(join(directory, "x", manifest.factIndex + ".facts.br")), {maximumOutputLength: 2 ** 31} as never);
const index: any = openFactIndexV1(raw, {maximumBytes: raw.length});
const minimum = Number(process.argv[2] ?? 3000);
const owners = new Map<number, Set<number>>();
for (const cls of index.classes) for (const id of cls.r) {
  const kind = index.rows[id].kind;
  if (kind !== "file" && kind !== "digest") continue;
  let set = owners.get(id);
  if (!set) owners.set(id, set = new Set());
  for (const o of cls.o) if (!index.owners[o].startsWith("@derived:")) set.add(o);
}
const stages: [string, string, string][] = [
  ["🎯️precision/", "project-assembly readProjectInputPrecisionV1 (PRECISION_FILES)", "input-precision"],
  ["🔌️managers/", "project-assembly readProjectInputPrecisionV1 (managerSourceFilesV1)", "package-managers"],
  ["🔌️providers/📋️project.json", "script-inputs providerProjectV1 (reads the providers project configuration)", "provider-project field"],
  ["Cargo.toml", "native cargo manifest walks (crate index, resolution)", "cargo manifests"],
];
const classOf = (path: string) => (/\.(?:ts|mjs|cjs|mts)$/u.test(path) ? "CODE" : path.includes("🧬️schema/") ? "SCHEMA" : "DATA");
const rows = [...owners].filter(([, set]) => set.size >= minimum).map(([id, set]) => {
  const row = index.rows[id], path = relative(root, row.path).replace(library, "L/");
  const stage = stages.find(([fragment]) => path.includes(fragment));
  return {kind: row.kind, holders: set.size, path, class: classOf(path), stage: stage?.[1] ?? "other", target: stage?.[2] ?? "-"};
}).sort((a, b) => b.holders - a.holders || a.path.localeCompare(b.path));
const summary = new Map<string, number>();
for (const row of rows) { const key = `${row.kind} ${row.target} ${row.class}`; summary.set(key, (summary.get(key) ?? 0) + 1); }
console.log(JSON.stringify({generation: pointer.generation, projects: index.owners.filter((o: string) => !o.startsWith("@derived:")).length, minimum, rows: rows.length, summary: Object.fromEntries(summary)}, null, 1));
if (process.argv.includes("--rows")) for (const row of rows.filter((r) => r.target !== "cargo manifests")) console.log(row.holders, row.kind, row.class, row.target, row.path.slice(0, 120));

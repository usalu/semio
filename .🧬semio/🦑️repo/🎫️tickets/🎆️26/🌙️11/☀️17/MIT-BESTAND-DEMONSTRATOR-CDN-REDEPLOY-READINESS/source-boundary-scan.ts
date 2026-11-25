import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

type Boundary = { maxSourceBytes: number; maxSourceFiles: number; maxActiveLines: number };
type Metric = keyof Boundary;
type Row = { status: "OVER" | "NEAR"; name: string; manifest: string; entry: string; values: Boundary; limits: Boundary; origin: string; worst: number };

const root = resolve(import.meta.dirname, "../../../../../../..");
const margin = Number(process.env.MARGIN ?? 0.03);
const policy: Boundary = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8")).storage.sourceBoundary;
const out = process.env.OUT ?? join(import.meta.dirname, "🔬️source-boundary-scan-1125.md");
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "target_family=unix", "unix"];
const skipped = new Set(["target", "node_modules", ".git", ".cache", ".🧬semio", "🎫️tickets"]);
const metrics: Metric[] = ["maxSourceBytes", "maxSourceFiles", "maxActiveLines"];
const manifests: string[] = [];
const walk = (directory: string): void => {
  for (const name of readdirSync(directory)) {
    if (skipped.has(name)) continue;
    const path = join(directory, name);
    const stat = statSync(path, { throwIfNoEntry: false });
    if (stat?.isDirectory()) walk(path);
    else if (name === "Cargo.toml") manifests.push(path);
  }
};
walk(root);
const read = async (path: string): Promise<Uint8Array | null> => { try { return readFileSync(path); } catch { return null; } };
const bodies = (text: string, header: string): string[] => [...text.matchAll(new RegExp(`^${header}[ \\t]*$([\\s\\S]*?)(?=^\\[|(?![\\s\\S]))`, "gm"))].map(match => match[1]!);
const keyOf = (body: string | undefined, key: string): string | undefined => new RegExp(`^\\s*${key}\\s*=\\s*"?([^"\\n]+?)"?\\s*$`, "m").exec(body ?? "")?.[1];
const entriesOf = (manifest: string, text: string): string[] => {
  const base = dirname(manifest), found = new Set<string>();
  const lib = bodies(text, String.raw`\[lib\]`)[0];
  const bins = bodies(text, String.raw`\[\[bin\]\]`);
  if (lib !== undefined) found.add(resolve(base, keyOf(lib, "path") ?? "src/lib.rs"));
  for (const bin of bins) { const path = keyOf(bin, "path"); if (path) found.add(resolve(base, path)); }
  if (lib === undefined && bins.length === 0) for (const guess of ["src/lib.rs", "src/main.rs"]) found.add(join(base, guess));
  return [...found].filter(path => statSync(path, { throwIfNoEntry: false })?.isFile());
};
const overrideOf = (text: string): Partial<Boundary> => {
  const body = bodies(text, String.raw`\[package\.metadata\.semio\.rust-source-boundary\]`)[0];
  const value = (key: Metric): number | undefined => { const raw = keyOf(body, key); return raw === undefined ? undefined : Number(raw.replaceAll("_", "")); };
  return { maxSourceBytes: value("maxSourceBytes"), maxSourceFiles: value("maxSourceFiles"), maxActiveLines: value("maxActiveLines") };
};
const rows: Row[] = [], refused: string[] = [];
let packages = 0;
for (const manifest of manifests) {
  const text = readFileSync(manifest, "utf8");
  const pkg = bodies(text, String.raw`\[package\]`)[0];
  const name = keyOf(pkg, "name");
  if (!pkg || !name) continue;
  packages++;
  const override = overrideOf(text);
  const limits = Object.fromEntries(metrics.map(metric => [metric, override[metric] ?? policy[metric]])) as Boundary;
  const origin = metrics.filter(metric => override[metric] !== undefined).map(metric => metric.replace("max", "")).join(",") || "policy";
  for (const entry of entriesOf(manifest, text)) {
    const label = relative(root, entry).replaceAll("\\", "/");
    try {
      const inputs = await readRustSourceClosureV1([entry], root, cfg, read);
      const values: Boundary = { maxSourceBytes: inputs.reduce((sum, input) => sum + input.bytes, 0), maxSourceFiles: inputs.length, maxActiveLines: inputs.reduce((sum, input) => sum + input.activeLines, 0) };
      const worst = Math.max(...metrics.map(metric => values[metric] / limits[metric]));
      if (worst >= 1 - margin) rows.push({ status: metrics.some(metric => values[metric] > limits[metric]) ? "OVER" : "NEAR", name, manifest: relative(root, manifest).replaceAll("\\", "/"), entry: label, values, limits, origin, worst });
    } catch (error) { refused.push(`${name} ${label}: ${String((error as Error).message ?? error).slice(0, 160)}`); }
  }
}
rows.sort((a, b) => (a.status === b.status ? b.worst - a.worst : a.status === "OVER" ? -1 : 1));
const cell = (row: Row, metric: Metric): string => `${row.values[metric]}/${row.limits[metric]} (${(100 * row.values[metric] / row.limits[metric]).toFixed(1)}%)`;
const table = [
  "| Status | Crate | Manifest | Entry | Source bytes | Source files | Active lines | Limit origin |",
  "| --- | --- | --- | --- | --- | --- | --- | --- |",
  ...rows.map(row => `| ${row.status} | ${row.name} | ${row.manifest} | ${row.entry} | ${cell(row, "maxSourceBytes")} | ${cell(row, "maxSourceFiles")} | ${cell(row, "maxActiveLines")} | ${row.origin} |`),
];
const over = rows.filter(row => row.status === "OVER");
const near = rows.length - over.length;
const report = [
  "# 🔬️ Rust Source Boundary Scan",
  "",
  `Scanned ${manifests.length} manifests, ${packages} packages with [package]. Policy storage.sourceBoundary: ${policy.maxSourceBytes} bytes, ${policy.maxSourceFiles} files, ${policy.maxActiveLines} active lines. Reported when any metric is within ${(margin * 100).toFixed(0)}% of or over its limit. Limit origin lists metrics overridden by the crate's [package.metadata.semio.rust-source-boundary], else policy.`,
  "",
  `Over limit: ${over.length}. Within ${(margin * 100).toFixed(0)}% of limit: ${near}. Unevaluable closures: ${refused.length}.`,
  "",
  ...(table.length > 2 ? table : ["No crate within margin."]),
  "",
  "## Unevaluable",
  "",
  ...(refused.length ? refused.map(line => `- ${line}`) : ["None."]),
  "",
].join("\n");
writeFileSync(out, report);
console.log(`scanned ${manifests.length} manifests, ${packages} packages; over ${over.length}, near ${near}, unevaluable ${refused.length}`);
for (const row of over) console.log("OVER", row.name, row.manifest, metrics.filter(metric => row.values[metric] > row.limits[metric]).map(metric => `${metric}=${row.values[metric]}/${row.limits[metric]}`).join(" "));
console.log(`report: ${out}`);

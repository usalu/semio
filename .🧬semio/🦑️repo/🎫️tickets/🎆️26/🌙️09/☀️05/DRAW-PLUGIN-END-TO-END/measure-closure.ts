import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const root = process.cwd();
const framework = (await import("node:fs")).readdirSync(root).find((name) => name.endsWith("framework"));
if (!framework) throw new Error("framework missing");
const modules = join(root, framework, (await import("node:fs")).readdirSync(join(root, framework)).find((name) => name.endsWith("modules"))!);
const processDir = join(modules, (await import("node:fs")).readdirSync(modules).find((name) => name.endsWith("process"))!);
async function findExport(dir: string): Promise<string> {
  const fs = await import("node:fs/promises");
  const pending = [dir];
  while (pending.length) {
    const current = pending.pop()!;
    for (const entry of await fs.readdir(current, { withFileTypes: true })) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) {
        if (entry.name !== "node_modules") pending.push(path);
        continue;
      }
      if (!entry.name.endsWith(".ts")) continue;
      const text = await fs.readFile(path, "utf8");
      if (text.includes("export async function readRustSourceClosureV1")) return path;
    }
  }
  throw new Error("closure reader missing");
}
const reader = await findExport(processDir);
const { readRustSourceClosureV1 } = await import(reader);
const fsSync = await import("node:fs");
const s = fsSync.readdirSync(root).find((name) => {
  const path = join(root, name);
  if (!fsSync.statSync(path).isDirectory() || name.startsWith(".") || name === "node_modules") return false;
  return fsSync.readdirSync(path).some((child) => child.endsWith("plugins") && fsSync.statSync(join(path, child)).isDirectory());
});
const pluginsName = fsSync.readdirSync(join(root, s!)).find((name) => fsSync.statSync(join(root, s!, name)).isDirectory() && name.endsWith("plugins"));
const plugins = join(root, s!, pluginsName!);
const draw = join(plugins, (await import("node:fs")).readdirSync(plugins).find((name) => name.endsWith("draw"))!);

function walkCargo(dir: string, names: Set<string>, found: Map<string, string>) {
  const fs = require("node:fs") as typeof import("node:fs");
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "node_modules" || entry.name === "target") continue;
    const path = join(dir, entry.name);
    if (entry.isDirectory()) walkCargo(path, names, found);
    else if (entry.name === "Cargo.toml") {
      const text = fs.readFileSync(path, "utf8");
      const match = text.match(/^name = "([^"]+)"/m);
      if (match && names.has(match[1]!)) found.set(match[1]!, path);
    }
  }
}
const wanted = new Set([
  "semio-s-artifact-draw-drawing-editor",
  "semio-s-artifact-draw-drawing-model",
  "semio-s-artifact-draw-drawing-model-tests",
  "semio-s-artifact-draw-drawing-model-tests-2",
]);
const found = new Map<string, string>();
walkCargo(draw, wanted, found);
const read = async (path: string) => {
  try { return await readFile(path); } catch { return null; }
};
for (const [name, cargo] of found) {
  const text = await readFile(cargo, "utf8");
  const pathLine = text.split("\n").find((line) => line.trim().startsWith("path =") && line.includes(".rs"));
  if (!pathLine) throw new Error("no lib path " + name);
  const relative = pathLine.split("=")[1]!.trim().replaceAll('"', "");
  const entry = join(dirname(cargo), relative);
  for (const cfg of [[], ["test"], ["test", "feature=\"testkit\""], ["test", "feature=\"testkit\"", "debug_assertions", "unix"]]) {
    const rows = await readRustSourceClosureV1([entry], root, cfg, read);
    const bytes = rows.reduce((sum: number, row: { bytes: number }) => sum + row.bytes, 0);
    const lines = rows.reduce((sum: number, row: { activeLines: number }) => sum + row.activeLines, 0);
    console.log(JSON.stringify({ name, cfg, files: rows.length, bytes, lines, overBytes: bytes - 262144, overFiles: rows.length - 32 }));
    if (cfg.length > 0) {
      const plain = await readRustSourceClosureV1([entry], root, [], read);
      const plainPaths = new Set(plain.map((row: { path: string }) => row.path));
      const extra = rows.filter((row: { path: string }) => !plainPaths.has(row.path)).sort((a: { bytes: number }, b: { bytes: number }) => b.bytes - a.bytes);
      for (const row of extra) console.log("  extra", row.bytes, row.path);
    }
  }
}

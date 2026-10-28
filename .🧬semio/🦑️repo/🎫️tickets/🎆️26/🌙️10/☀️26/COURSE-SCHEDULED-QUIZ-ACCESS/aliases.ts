import fs from "node:fs";
import path from "node:path";

export const root = process.cwd();
export const ticket = import.meta.dirname;
export const core = path.resolve(root, "🧰️framework/🛍️products/📚️course");
export const ui = path.resolve(core, "🎯️targets/⚛️react");
const entries = new Map<string, string>();
const target = (value: unknown): string | undefined => {
  if (typeof value === "string") return value;
  if (value && typeof value === "object") for (const condition of ["import", "node", "default"]) if (condition in value) return target(value[condition as keyof typeof value]);
};
const visit = (directory: string): void => {
  const manifest = path.join(directory, "package.json");
  if (fs.existsSync(manifest)) {
    const document = JSON.parse(fs.readFileSync(manifest, "utf8"));
    const exports = typeof document.exports === "string" ? { ".": document.exports } : document.exports;
    for (const [key, value] of Object.entries(exports ?? {})) {
      const source = target(value);
      if (typeof document.name === "string" && key.startsWith(".") && source && !key.includes("*")) entries.set(document.name + (key === "." ? "" : key.slice(1)), path.resolve(directory, source));
    }
  }
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) if (entry.isDirectory() && !entry.isSymbolicLink() && !/^(?:node_modules|dist|target|\.cache|\.git|\.config|\.🧬semio|🗑️generated)$/.test(entry.name) && !entry.name.startsWith(".")) visit(path.join(directory, entry.name));
};
visit(path.resolve(root, "🧰️framework"));
visit(path.resolve(root, "🎓️teaching/🏛️architecture/📚️course"));
export const paths = Object.fromEntries([...entries].map(([name, source]) => [name, [source]]));
export const aliases = [...entries].map(([name, replacement]) => ({ find: new RegExp("^" + name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "$"), replacement }));

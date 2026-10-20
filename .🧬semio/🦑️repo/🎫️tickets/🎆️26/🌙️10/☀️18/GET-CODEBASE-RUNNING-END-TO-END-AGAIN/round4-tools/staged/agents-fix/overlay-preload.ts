import fs from "node:fs";
import { mock } from "bun:test";
import path from "node:path";
import { fileURLToPath } from "node:url";
const root = process.env.AGENTS_FIX_ROOT ?? "/Users/ueli/Documents/semio";
const here = path.dirname(fileURLToPath(import.meta.url));
const manifest = JSON.parse(fs.readFileSync(path.join(here, "staged", "manifest.json"), "utf8")) as { files: { path: string; after: string }[] };
const overlay = new Map<string, string>();
for (const row of manifest.files) overlay.set(path.join(root, row.path), fs.readFileSync(path.join(here, "staged", row.after), "utf8"));
const loaders: Record<string, "ts" | "tsx" | "js" | "json"> = { ".ts": "ts", ".tsx": "tsx", ".mjs": "js", ".json": "json" };
Bun.plugin({
  name: "agents-fix-overlay",
  setup(build) {
    const escaped = [...overlay.keys()].map((file) => file.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
    build.onLoad({ filter: new RegExp("^(?:" + escaped.join("|") + ")$") }, (args) => {
      const contents = overlay.get(args.path)!;
      if (path.extname(args.path) === ".json") { const value = JSON.parse(contents); return { exports: { default: value, ...value }, loader: "object" as const }; }
      return { contents, loader: loaders[path.extname(args.path)]! };
    });
  },
});
const original = fs.readFileSync;
const key = (value: unknown): string | undefined => (typeof value === "string" ? value : value instanceof URL ? fileURLToPath(value) : undefined);
const patched = ((target: unknown, options?: unknown) => {
  const contents = overlay.get(key(target) ?? "");
  if (contents === undefined) return (original as (...args: unknown[]) => unknown)(target, options);
  const encoding = typeof options === "string" ? options : (options as { encoding?: string } | undefined)?.encoding;
  return encoding ? contents : Buffer.from(contents, "utf8");
}) as typeof fs.readFileSync;
const replaced = { ...fs, readFileSync: patched };
mock.module("node:fs", () => ({ ...replaced, default: replaced }));
mock.module("fs", () => ({ ...replaced, default: replaced }));

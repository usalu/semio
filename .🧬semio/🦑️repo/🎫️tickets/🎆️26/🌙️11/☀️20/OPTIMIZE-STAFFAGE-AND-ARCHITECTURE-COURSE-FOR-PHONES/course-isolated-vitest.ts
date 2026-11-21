import { readFile } from "node:fs/promises";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { startVitest } from "vitest/node";
import react from "@vitejs/plugin-react";
import { isolatedVitePackageResolver } from "./isolated-browser-runner.ts";

const course = new URL("../../../../../../../🧰️framework/🛍️products/📚️course/", import.meta.url);
const workspace = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const target = new URL("🎯️targets/⚛️react/", course);
const packageRoot = fileURLToPath(new URL("📦️packages/🟦️typescript", target));
const root = fileURLToPath(new URL("./🗑️generated/w", import.meta.url));
const canonical = await readFile(new URL("🧪️tests/🎚️config/🟦️.ts", target), "utf8");
const list = /\binclude:\s*\[([\s\S]*?)\]/u.exec(canonical)?.[1];
if (list === undefined) throw Error("Course React test roster is missing");
const include = [...list.matchAll(/"([^"]+)"/gu)].map((match) => relative(root, resolve(packageRoot, match[1]!)).split("\\").join("/"));
const cacheDir = fileURLToPath(new URL("./🗑️generated/vitest-course", import.meta.url));
const preload = new URL("./isolated-test-preload.mjs", import.meta.url).href;
const locales = fileURLToPath(new URL("🧪️tests/🌐️preloaded-locales/🟦️.ts", course));
const reactSetup = resolve(workspace, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts");
console.info("[DEBUG] running", include.length, "canonical course test entries in the isolated runner");
const ctx = await startVitest(
  "test",
  process.argv.slice(2),
  { root, include, setupFiles: [reactSetup, locales], environment: "jsdom", watch: false, config: false, maxWorkers: 2, execArgv: ["--import", preload], css: { include: [/🎨️\.css(?:\?|$)/u] } },
  { root, cacheDir, envFile: false, optimizeDeps: { noDiscovery: true, include: [] }, server: { watch: null, fs: { allow: [workspace, root] } }, plugins: [isolatedVitePackageResolver(), react()] },
);
await ctx?.close();

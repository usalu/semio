/** @emoji 🕵️ Prints how many repo modules Vite's config bundle pulls in — every one of them is a
 * `configFileDependencies` entry whose save restarts the whole dev server. */
import { loadConfigFromFile } from "vite";
const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const loaded = await loadConfigFromFile({ command: "serve", mode: "development" }, `${root}/⚙️vite.config.ts`, root, "info", undefined, "bundle");
const deps = loaded?.dependencies ?? [];
console.log(`[DEBUG] configFileDependencies=${deps.length}`);
const repo = "/Users/ueli/Documents/semio/";
const rel = deps.map((d) => d.replaceAll("\\", "/"));
console.log(`[DEBUG] sample:`);
for (const d of rel.slice(0, 20)) console.log("  " + d);
const big = ["ShellHost", "World3dHost", "backbone-worker", "🧵️backbone-worker"];
for (const needle of big) console.log(`[DEBUG] contains ${needle}: ${rel.filter((d) => d.includes(needle)).length}`);
import { writeFileSync } from "node:fs";
writeFileSync(process.env.OUT ?? "/dev/stdout", rel.join("\n") + "\n");

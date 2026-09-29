/** 📦️ Bundle composition of the site build: rendered bytes per origin (npm package or repo directory, three segments deep),
 * largest first. `bun site_infra_bundle.ts` from anywhere; nothing is written. */
import { join } from "node:path";
import { build } from "vite";

const root = join(import.meta.dir, "../../../../../../..");
const sizes = new Map<string, number>();
const origin = (id: string): string => {
  const path = id.replaceAll("\\", "/").replace(/^\0/u, "");
  const npm = path.match(/node_modules\/((?:@[^/]+\/)?[^/]+)/u);
  if (npm) return `npm:${npm[1]}`;
  return path.startsWith(root.replaceAll("\\", "/")) ? path.slice(root.length + 1).split("/").slice(0, 4).join("/") : path.slice(0, 80);
};
await build({
  configFile: join(root, "🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts"),
  logLevel: "warn",
  build: { write: false, ...(process.argv.includes("--pure-ui") ? { rollupOptions: { treeshake: { moduleSideEffects: (id: string) => /\.css(?:$|\?)/u.test(id) || !/🧰️framework\/🔨️modules\/(?:🖱️ui|🖼️assets)\//u.test(id.replaceAll("\\", "/")) } } } : {}) },
  plugins: [{
    name: "site-infra-sizes",
    generateBundle(_options, bundle) {
      for (const output of Object.values(bundle)) if (output.type === "chunk") for (const [id, module] of Object.entries(output.modules)) sizes.set(origin(id), (sizes.get(origin(id)) ?? 0) + module.renderedLength);
    },
  }],
});
const total = [...sizes.values()].reduce((sum, size) => sum + size, 0);
console.log(`[DEBUG] total rendered ${(total / 1024).toFixed(0)} KiB`);
for (const [key, size] of [...sizes].sort((left, right) => right[1] - left[1]).slice(0, 30)) console.log(`[DEBUG] ${(size / 1024).toFixed(1).padStart(8)} KiB  ${key}`);

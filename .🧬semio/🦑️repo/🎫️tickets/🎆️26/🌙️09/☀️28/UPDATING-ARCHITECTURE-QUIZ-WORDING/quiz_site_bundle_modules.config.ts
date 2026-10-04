/** 📦️ Builds the quiz site into this ticket's `🗑️generated` folder and prints the largest modules of the main chunk
 * (rendered length before minification), grouped per npm package or repo file — to see what the site bundle ships.
 * Usage: bun node_modules/vite/bin/vite.js build --config <this file> --configLoader bundle
 */

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { mergeConfig, type ConfigEnv, type Plugin, type UserConfig } from "vite";
import site from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));

const sizes: Plugin = {
  name: "quiz-site-bundle-modules",
  generateBundle(_options, bundle) {
    const groups = new Map<string, number>();
    for (const chunk of Object.values(bundle)) {
      if (chunk.type !== "chunk") continue;
      for (const [id, info] of Object.entries(chunk.modules)) {
        const key = id.includes("node_modules") ? id.replace(/^.*node_modules[\\/](@[^\\/]+[\\/][^\\/]+|[^\\/]+).*$/, "npm:$1") : id.replace(/^.*🧰️framework/, "fw").replace(/\?.*$/, "");
        groups.set(key, (groups.get(key) ?? 0) + info.renderedLength);
      }
    }
    for (const [key, size] of [...groups].sort((a, b) => b[1] - a[1]).slice(0, 45)) console.log(`[bundle] ${String(size).padStart(9)} ${key}`);
  },
};

export default (env: ConfigEnv): UserConfig => {
  const base = typeof site === "function" ? site(env) : site;
  return mergeConfig(base as UserConfig, { plugins: [sizes], build: { outDir: resolve(here, "🗑️generated", "react", "site-bundle"), emptyOutDir: true } });
};

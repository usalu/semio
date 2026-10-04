/** 💸️ C3 budget probe: the site's own Vite configuration with the quiz sources as they were before work package C3
 * (`🗑️generated/c3/pre-variant.json`, written by `c3_budget.ts`) loaded in place of the working tree's, so a release
 * build of that variant can be weighed beside the real one. Build from the site package
 * (`🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`):
 * `bun <repo>/node_modules/vite/bin/vite.js build --config <this file> --configLoader bundle --outDir <TK>/🗑️generated/c3/site-pre --emptyOutDir`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import site from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const variant: Readonly<Record<string, string>> = JSON.parse(readFileSync(resolve(here, "🗑️generated/c3/pre-variant.json"), "utf8"));
const served = new Set<string>();

export default async (env: { readonly command: string; readonly mode: string }) => {
  const base = typeof site === "function" ? await (site as (env: unknown) => unknown)(env) : await site;
  const config = base as { readonly plugins?: readonly unknown[] };
  return {
    ...config,
    plugins: [
      {
        name: "c3-pre-variant",
        enforce: "pre",
        load(id: string) {
          const path = id.split("?")[0]!.replaceAll("\\", "/");
          const text = variant[path];
          if (text === undefined) return null;
          served.add(path);
          return text;
        },
        buildEnd() {
          const missing = Object.keys(variant).filter((path) => !served.has(path));
          if (missing.length > 0) this.error(`not loaded from the variant: ${missing.join(", ")}`);
        },
      },
      ...(config.plugins ?? []),
    ],
  };
};

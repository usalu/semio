/** 💸️ C3b budget probe: the site's own Vite configuration with one variant of `🗑️generated/c3b/variants.json` (written
 * by `c3b_budget.ts`, chosen by the environment variable `C3B_VARIANT`) loaded in place of the working tree's sources,
 * and, beside the build, the rendered length of every module of every chunk (`<outDir>.modules.json`), so a variant's
 * entry script can be weighed and told apart module by module. Without `C3B_VARIANT` it is the site's configuration
 * plus that record. Build from the site package (`🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`):
 * `C3B_VARIANT=<name> bun <repo>/node_modules/vite/bin/vite.js build --config <this file> --configLoader bundle --outDir <TK>/🗑️generated/c3b/<name> --emptyOutDir`. */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import site from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const chosen = process.env.C3B_VARIANT;
const variant: Readonly<Record<string, string>> = chosen === undefined ? {} : (JSON.parse(readFileSync(resolve(here, "🗑️generated/c3b/variants.json"), "utf8")) as Record<string, Record<string, string>>)[chosen] ?? {};
if (chosen !== undefined && Object.keys(variant).length === 0) throw new Error(`no variant ${chosen}`);
const served = new Set<string>();

export default async (env: { readonly command: string; readonly mode: string }) => {
  const base = typeof site === "function" ? await (site as (env: unknown) => unknown)(env) : await site;
  const config = base as { readonly plugins?: readonly unknown[] };
  let outDir = "";
  return {
    ...config,
    plugins: [
      {
        name: "c3b-variant",
        enforce: "pre",
        configResolved(resolved: { readonly root: string; readonly build: { readonly outDir: string } }) {
          outDir = resolve(resolved.root, resolved.build.outDir);
        },
        load(id: string) {
          const path = id.split("?")[0]!.replaceAll("\\", "/");
          const text = variant[path];
          if (text === undefined || id.includes("?")) return null;
          served.add(path);
          return text;
        },
        buildEnd() {
          const missing = Object.keys(variant).filter((path) => !served.has(path));
          if (missing.length > 0) this.error(`not loaded from the variant: ${missing.join(", ")}`);
        },
        generateBundle(_options: unknown, bundle: Record<string, { readonly type: string; readonly isEntry?: boolean; readonly isDynamicEntry?: boolean; readonly modules?: Record<string, { readonly renderedLength: number }> }>) {
          const chunks = Object.entries(bundle)
            .filter(([, output]) => output.type === "chunk")
            .map(([file, output]) => ({ file, entry: output.isEntry === true, dynamic: output.isDynamicEntry === true, modules: Object.fromEntries(Object.entries(output.modules ?? {}).map(([id, module]) => [id.replaceAll("\\", "/"), module.renderedLength])) }));
          writeFileSync(`${outDir}.modules.json`, JSON.stringify(chunks, null, 1));
        },
      },
      ...(config.plugins ?? []),
    ],
  };
};

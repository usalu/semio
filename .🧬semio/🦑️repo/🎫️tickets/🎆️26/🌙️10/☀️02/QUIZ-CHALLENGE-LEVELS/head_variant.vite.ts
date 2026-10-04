/** 🕰️ `site_chunk_modules.vite.ts` with the sources of `🗑️generated/site-final/head-variant.json` (written by
 * `head_variant.ts`) loaded at their `HEAD` text, so the entry before this ticket's day of work can be weighed beside the
 * tree's. Build from the site package:
 * `CHUNK_LABEL=head bun <repo>/node_modules/vite/bin/vite.js build --config <this file> --configLoader bundle --outDir <dir> --emptyOutDir`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import measured from "./site_chunk_modules.vite.ts";

const here = dirname(fileURLToPath(import.meta.url));
const variant: Readonly<Record<string, string>> = JSON.parse(readFileSync(resolve(here, `🗑️generated/site-final/${process.env.VARIANT ?? "head-variant.json"}`), "utf8"));
const served = new Set<string>();

export default async (env: { readonly command: string; readonly mode: string }) => {
  const config = (await measured(env)) as { readonly plugins?: readonly unknown[]; readonly build?: Record<string, unknown> };
  const pure = process.env.PURE_QUIZ_REACT === "on";
  return {
    ...config,
    ...(pure ? { build: { ...config.build, rollupOptions: { ...(config.build?.rollupOptions as object), treeshake: { moduleSideEffects: (id: string) => !(id.includes("🎯️targets/⚛️react/") && id.includes("❓️quiz") && /\.tsx?$/u.test(id.split("?")[0]!)) } } } } : {}),
    plugins: [
      {
        name: "head-variant",
        enforce: "pre",
        load(id: string) {
          const path = id.split("?")[0]!.replaceAll("\\", "/");
          const text = variant[path];
          if (text === undefined) return null;
          served.add(path);
          return text;
        },
        buildEnd() {
          console.log(`[variant] served ${served.size} of ${Object.keys(variant).length}: unserved ${Object.keys(variant).filter((path) => !served.has(path)).join(", ")}`);
        },
      },
      ...(config.plugins ?? []),
    ],
  };
};

/** 🧩️ The site's own Vite configuration plus a report of every chunk's modules (rendered bytes and gzip of the rendered
 * code, grouped by a directory prefix), written to `🗑️generated/site-final/<label>-modules.txt`. Build from the site
 * package (`🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`):
 * `CHUNK_LABEL=<label> bun <repo>/node_modules/vite/bin/vite.js build --config <this file> --configLoader bundle --outDir <dir> --emptyOutDir`. */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";
import site from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const depth = Number(process.env.CHUNK_DEPTH ?? 9);
const group = (id: string): string => {
  const clean = id.replace(/\\/gu, "/").replace(/^.*?\/semio\//u, "").replace(/^\0/u, "").replace(/^.*node_modules\//u, "node_modules/");
  const parts = clean.split("?")[0]!.split("/");
  return clean.startsWith("node_modules/") ? parts.slice(0, parts[1]!.startsWith("@") ? 3 : 2).join("/") : parts.slice(0, depth).join("/");
};

export default async (env: { readonly command: string; readonly mode: string }) => {
  const base = typeof site === "function" ? await (site as (env: unknown) => unknown)(env) : await site;
  const config = base as { readonly plugins?: readonly unknown[] };
  return {
    ...config,
    plugins: [
      ...(config.plugins ?? []),
      {
        name: "site-chunk-modules",
        generateBundle(_options: unknown, bundle: Record<string, { type: string; fileName: string; isEntry?: boolean; isDynamicEntry?: boolean; code?: string; modules?: Record<string, { renderedLength: number; code?: string | null }> }>) {
          const lines: string[] = [];
          for (const chunk of Object.values(bundle).filter((item) => item.type === "chunk")) {
            const groups = new Map<string, { raw: number; code: string }>();
            for (const [id, module] of Object.entries(chunk.modules ?? {})) {
              const key = group(id);
              const entry = groups.get(key) ?? { raw: 0, code: "" };
              entry.raw += module.renderedLength;
              entry.code += module.code ?? "";
              groups.set(key, entry);
            }
            lines.push(`== ${chunk.fileName} entry=${Boolean(chunk.isEntry)} dynamic=${Boolean(chunk.isDynamicEntry)} raw=${chunk.code?.length} gzip=${gzipSync(chunk.code ?? "").length}`);
            for (const [key, entry] of [...groups].sort((a, b) => b[1].raw - a[1].raw)) lines.push(`${String(entry.raw).padStart(9)} ${String(gzipSync(entry.code).length).padStart(8)}gz  ${key}`);
          }
          mkdirSync(resolve(here, "🗑️generated/site-final"), { recursive: true });
          writeFileSync(resolve(here, `🗑️generated/site-final/${process.env.CHUNK_LABEL ?? "now"}-modules.txt`), `${lines.join("\n")}\n`);
        },
      },
    ],
  };
};

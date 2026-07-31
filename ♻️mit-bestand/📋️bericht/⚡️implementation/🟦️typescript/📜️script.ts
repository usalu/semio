#!/usr/bin/env bun
/** 🏚️ `@semio-tech/mit-bestand-bericht` router: `bun ./📜️script.ts build|watch|latex`. */
import { existsSync, statSync, watch } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { buildPrintDocument, fetchPrintFonts } from "../../../../🧰️framework/🛍️product/📓️print/⚡️implementation/🟦️typescript/📜️script.ts";

const berichtRoot = import.meta.dir;
const defaultTex = join(berichtRoot, "zwischenbericht/🖋️zwischenbericht.tex");

function resolveTexPath(segments: string[]): string {
  const raw = segments[0] ?? defaultTex;
  const abs = resolve(raw.endsWith(".tex") ? raw : `${raw}.tex`);
  if (!existsSync(abs)) throw new Error(`missing tex file: ${abs}`);
  return abs;
}

function resolveOutDir(texAbs: string, segments: string[]): string {
  if (segments[1]) return resolve(segments[1]);
  return join(dirname(texAbs), "dist");
}

async function buildDocument(segments: string[]): Promise<void> {
  await fetchPrintFonts();
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  await buildPrintDocument(texAbs, outDir);
}

async function watchDocument(segments: string[]): Promise<void> {
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  const roots = [dirname(texAbs), join(berichtRoot, "../../framework/print/tex")];
  const mtimes = new Map<string, number>();
  const rebuild = async () => {
    try {
      await buildDocument([texAbs, outDir]);
    } catch (error) {
      console.error("[DEBUG] mit-bestand/bericht watch rebuild failed:", error);
    }
  };
  await rebuild();
  for (const root of roots) {
    if (!existsSync(root)) continue;
    watch(root, { recursive: true }, (_event, file) => {
      if (!file) return;
      const abs = join(root, file);
      if (abs.includes(".semio-dark") || /-dark\.tex$/i.test(abs)) return;
      if (!/\.(tex|sty|cls|ttf|json)$/i.test(abs)) return;
      try {
        const mtime = statSync(abs).mtimeMs;
        if (mtimes.get(abs) === mtime) return;
        mtimes.set(abs, mtime);
        void rebuild();
      } catch {
        /* ignore */
      }
    });
  }
  console.log(`[DEBUG] mit-bestand/bericht watching ${basename(texAbs)}`);
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDocument(segments);
  }
}

class WatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await watchDocument(segments);
  }
}

class LatexScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDocument(segments);
  }
}

const router = new ScriptRouter(berichtRoot).register("build", BuildScript).register("watch", WatchScript).register("latex", LatexScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });

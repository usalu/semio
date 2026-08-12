#!/usr/bin/env bun
/** 🏚️ `@semio-tech/mit-bestand-bericht` router: `bun ./script.ts build|watch|latex`. */
import { existsSync, statSync, watch } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/index.ts";
import { buildPrintDocument, fetchPrintFonts } from "../../print/script.ts";

const berichtRoot = import.meta.dir;

//#region Documents
/** 📚 The Zukunft Bau report family of `Entwerfen mit Bestand`, keyed by document name.
 * @see https://www.zukunftbau.de/programme/forschungsfoerderung */
const DOCUMENTS = {
  zwischenbericht: "zwischenbericht/zwischenbericht.tex",
  forschungsbericht: "forschungsbericht/forschungsbericht.tex",
  kompaktbericht: "kompaktbericht/kompaktbericht.tex",
} as const;

function documentTexPath(name: string): string | undefined {
  const relative = DOCUMENTS[name as keyof typeof DOCUMENTS];
  return relative ? join(berichtRoot, relative) : undefined;
}
//#endregion

function resolveTexPath(segments: string[]): string {
  const raw = segments[0];
  if (raw === undefined) throw new Error(`missing document: pass one of ${Object.keys(DOCUMENTS).join(", ")} or a .tex path`);
  const abs = documentTexPath(raw) ?? resolve(raw.endsWith(".tex") ? raw : `${raw}.tex`);
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

async function buildDocuments(segments: string[]): Promise<void> {
  if (segments.length > 0) return buildDocument(segments);
  for (const name of Object.keys(DOCUMENTS)) await buildDocument([name]);
}

async function watchDocument(segments: string[]): Promise<void> {
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  const roots = [dirname(texAbs), join(berichtRoot, "../../print/tex")];
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
    await buildDocuments(segments);
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

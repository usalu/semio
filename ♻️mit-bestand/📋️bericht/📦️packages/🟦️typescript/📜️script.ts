#!/usr/bin/env bun
/** 🏚️ `@semio-tech/mit-bestand-bericht` router: `bun ./📜️script.ts build|watch|latex`. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, extname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { provisionPrintFonts } from "../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts";
import { buildPrintDocument } from "../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

const packageRoot = import.meta.dir;
const ownerRoot = join(packageRoot, "../..");
const defaultTex = join(ownerRoot, "📋️zwischenbericht/📋️zwischenbericht.tex");
const emojiSegmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });

function compilerName(name: string): string {
  const first = emojiSegmenter.segment(name)[Symbol.iterator]().next().value?.segment ?? "";
  return /\p{Extended_Pictographic}|\p{Regional_Indicator}/u.test(first) ? name.slice(first.length) : name;
}

function stageReportSources(texAbs: string, outDir: string): string {
  const sourceRoot = dirname(texAbs);
  const stageRoot = join(outDir, "source");
  rmSync(stageRoot, { recursive: true, force: true });
  const nodes: { readonly source: string; readonly sourceRelative: string; readonly stagedRelative: string; readonly file: boolean }[] = [];
  const visit = (sourceRelative: string, stagedRelative: string): void => {
    const source = join(sourceRoot, sourceRelative);
    const file = statSync(source).isFile();
    nodes.push({ source, sourceRelative, stagedRelative, file });
    if (file) return;
    for (const entry of readdirSync(source, { withFileTypes: true })) {
      if (entry.name === ".DS_Store") continue;
      visit(join(sourceRelative, entry.name), join(stagedRelative, compilerName(entry.name)));
    }
  };
  for (const entry of [basename(texAbs), "📚️references.bib", "🖼️asset", "📎️anhang"]) visit(entry, compilerName(entry));
  const replacements = nodes
    .flatMap((node) => {
      const path = [node.sourceRelative.replaceAll("\\", "/"), node.stagedRelative.replaceAll("\\", "/")] as const;
      return node.file && extname(path[0]) === ".tex" ? [path, [path[0].slice(0, -4), path[1].slice(0, -4)] as const] : [path];
    })
    .sort((left, right) => right[0].length - left[0].length);
  for (const node of nodes) {
    const destination = join(stageRoot, node.stagedRelative);
    if (!node.file) {
      mkdirSync(destination, { recursive: true });
      continue;
    }
    mkdirSync(dirname(destination), { recursive: true });
    if (![".tex", ".bib"].includes(extname(node.source))) {
      copyFileSync(node.source, destination);
      continue;
    }
    let content = readFileSync(node.source, "utf8");
    for (const [source, staged] of replacements) content = content.replaceAll(source, staged);
    writeFileSync(destination, content, "utf8");
  }
  return join(stageRoot, compilerName(basename(texAbs)));
}

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
  await provisionPrintFonts();
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  mkdirSync(outDir, { recursive: true });
  const stagedTex = stageReportSources(texAbs, outDir);
  await buildPrintDocument(stagedTex, outDir, dirname(stagedTex));
}

async function watchDocument(segments: string[]): Promise<void> {
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  const roots = [dirname(texAbs), join(packageRoot, "../../../../🧰️framework/🛍️products/📓️print/🖋️latex")];
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

const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("watch", WatchScript).register("latex", LatexScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });

import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";

const workspace = resolve(import.meta.dir, "../../../../../../..");
const print = join(workspace, "🧰️framework/🛍️products/📓️print");
const lane = process.env.SEMIO_PRINT_LANE;
if (lane && !/^[a-z]+$/.test(lane)) throw Error("Invalid Print scratch lane");
const scratch = join(import.meta.dir, "🗑️generated/print", lane ?? "");
const printTools = join(workspace, ".cache/b/t/m/print");
const compiler = process.env.SEMIO_PRINT_COMPILER ?? readdirSync(join(printTools, "tectonic"))
  .map(platform => join(printTools, "tectonic", platform, process.platform === "win32" ? "tectonic.exe" : "tectonic"))
  .find(existsSync);
if (!compiler) throw new Error("The pinned Print compiler is missing");
const bundle = pathToFileURL(join(printTools, "tex")).href;
const catalog = JSON.parse(readFileSync(join(print, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8"));
const segmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });
const name = (part: string): string => {
  if (part === "📐️.tex") return "_.tex";
  const first = segmenter.segment(part)[Symbol.iterator]().next().value?.segment ?? "";
  return /\p{Extended_Pictographic}|\p{Regional_Indicator}/u.test(first) ? part.slice(first.length) : part;
};
const staged = (path: string) => path.split("/").map(name).join("/");
const rewrite = (text: string) => text.replaceAll("📐️.tex", "_.tex").replace(/(?:\p{Extended_Pictographic}|\p{Regional_Indicator})\uFE0F?(?=[A-Za-z0-9_.-])/gu, "");
function stage(source: string, target: string): void {
  if (target.length > 256) throw new Error("Invalid staged path: " + target);
  if (statSync(source).isDirectory()) {
    mkdirSync(target, { recursive: true });
    for (const entry of readdirSync(source)) stage(join(source, entry), join(target, name(entry)));
    return;
  }
  mkdirSync(dirname(target), { recursive: true });
  if (/\.(tex|sty|cls|bib)$/.test(source)) writeFileSync(target, rewrite(readFileSync(source, "utf8")));
  else cpSync(source, target);
}
const library = join(scratch, "lib");
for (const path of [...catalog.librarySources, "🖼️assets/🔤️font"]) stage(join(print, path), join(library, staged(path)));
const fonts = join(library, "assets/font");
const search = [join(library, "latex"), library, ...readdirSync(fonts).map(font => join(fonts, font))];
const identifiers = process.argv.slice(2);
const passCount = Number(process.env.SEMIO_PRINT_PASSES ?? "3");
for (const id of identifiers) {
  const document = catalog.documents.find((entry: { id: string }) => entry.id === id || (id === "report-dark" && entry.id === "report"));
  const source = document ? join(print, document.texPath) : resolve(id);
  const work = join(scratch, document ? id : basename(source, ".tex"));
  mkdirSync(work, { recursive: true });
  const stagedTex = join(work, document ? staged(document.texPath) : name(basename(source)));
  if (document) {
    for (const entry of document.sources) {
      stage(join(print, entry), join(work, staged(entry)));
    }
    if (id === "report-dark") {
      writeFileSync(stagedTex, readFileSync(stagedTex, "utf8").replace(/\btheme\s*=\s*light\b/, "theme=dark"));
    }
  } else stage(source, stagedTex);
  const tex = basename(stagedTex), cwd = dirname(stagedTex);
  for (let pass = 0; pass < passCount; pass++) {
    const result = spawnSync(compiler, ["--bundle", bundle, "--keep-logs", "--keep-intermediates", "--reruns", "0", "-Z", "deterministic-mode", ...[...search, work, cwd].flatMap(path => ["-Z", "search-path=" + path]), "--outdir", work, tex], {
      cwd, encoding: "utf8", timeout: 180000, env: { ...process.env, SOURCE_DATE_EPOCH: String(catalog.sourceDateEpoch), TZ: "UTC" },
    });
    if (result.status !== 0) throw Error(id + " pass " + (pass + 1) + ": " + (result.error ?? result.stdout + result.stderr));
  }
  const base = basename(tex, ".tex");
  const log = readFileSync(join(work, base + ".log"), "utf8");
  const info = spawnSync("pdfinfo", [base + ".pdf"], { cwd: work, encoding: "utf8" });
  const raster = spawnSync("pdftoppm", ["-r", "105", "-png", base + ".pdf", "p"], { cwd: work, encoding: "utf8" });
  if (info.status !== 0 || raster.status !== 0) throw Error(id + ": " + (info.error ?? info.stderr ?? "") + (raster.error ?? raster.stderr ?? ""));
  console.log(`[DEBUG] ${id}: ${info.stdout.match(/^Pages:\s*(\d+)/m)?.[1]} pages, ${(log.match(/^Overfull /gm) ?? []).length} overfull boxes`);
}

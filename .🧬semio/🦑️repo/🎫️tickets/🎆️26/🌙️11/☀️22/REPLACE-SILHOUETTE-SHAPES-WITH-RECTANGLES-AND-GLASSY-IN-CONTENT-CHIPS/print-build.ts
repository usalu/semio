/**
 * 🖨️ Portable Print fixture build for macOS, Linux and Windows: stages the Print library and one
 * document the way `📥️source-staging` does (leading pictographs stripped from every path segment and
 * from the references inside TeX sources), compiles with the pinned tectonic and its cached bundle,
 * and rasterises pages (PDFKit on macOS, pdftoppm or mutool elsewhere).
 *
 * usage: node print-build.ts <source dir (absolute or relative to the repo)> <main tex> <name> [pages=1,2] [scale=1.6] [theme]
 * env:   SEMIO_PRINT_EXTRA (path-delimited absolute files copied next to the main tex), SEMIO_REPO (default: three levels above the tickets folder), SEMIO_PRINT_SCRATCH (ASCII scratch directory)
 * @see https://tectonic-typesetting.github.io/book/latest/
 */
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";

const here = dirname(fileURLToPath(import.meta.url));
const repo = process.env.SEMIO_REPO ?? resolve(here, "../../../../../../..");
const scratch = process.env.SEMIO_PRINT_SCRATCH ?? join(tmpdir(), "semio-print");
const [source, main, name, pages = "1", scale = "1.6", theme] = process.argv.slice(2);
if (!source || !main || !name) throw Error("usage: node print-build.ts <source dir> <main tex> <name> [pages] [scale] [light|dark]");
const print = join(repo, "🧰️framework/🛍️products/📓️print");
const cache = join(repo, ".cache/b");
const triples: Record<string, string> = { "darwin-arm64": "aarch64-apple-darwin", "darwin-x64": "x86_64-apple-darwin", "linux-x64": "x86_64-unknown-linux-gnu", "linux-arm64": "aarch64-unknown-linux-gnu", "win32-x64": "x86_64-pc-windows-msvc", "win32-arm64": "aarch64-pc-windows-msvc" };
const triple = triples[`${process.platform}-${process.arch}`] ?? "";
const tectonic = join(cache, "t/m/print/tectonic", triple, process.platform === "win32" ? "tectonic.exe" : "tectonic");
const bundle = join(cache, "t/m/print/tex");
const fonts = join(cache, "p/m/print/prototype/fonts");
for (const required of [tectonic, bundle, fonts]) if (!existsSync(required)) throw Error(`missing pinned Print material: ${required}`);

const segmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });
const pictograph = /\p{Extended_Pictographic}|\p{Regional_Indicator}/u;
const staged = (part: string): string => {
  if (part === "📐️.tex") return "_.tex";
  const first = segmenter.segment(part)[Symbol.iterator]().next().value?.segment ?? "";
  return pictograph.test(first) ? part.slice(first.length) : part;
};
const rewrite = (body: string): string =>
  body.replaceAll("📐️.tex", "_.tex").replace(/(?:\.\.\/)+(?:\p{Extended_Pictographic}|\p{Regional_Indicator})\uFE0F?assets\//gu, "assets/").replace(/(?:\p{Extended_Pictographic}|\p{Regional_Indicator})️?(?=[A-Za-z0-9_.-])/gu, "");
const darkened = (body: string): string => body.replace(/theme\s*=\s*light/, "theme=dark").replace(/(asset\/logo\/[^,\s}\]]+?)(?<!-dark)(\.(?:png|jpe?g|webp|pdf))(?=[,\s}\]])/gi, "$1-dark$2");
const text = /\.(tex|sty|cls|bib)$/;
const copy = (from: string, to: string): void => {
  if (statSync(from).isDirectory()) {
    mkdirSync(to, { recursive: true });
    for (const child of readdirSync(from)) if (!["node_modules", "dist"].includes(child)) copy(join(from, child), join(to, staged(child)));
  } else if (text.test(from)) writeFileSync(to, rewrite(readFileSync(from, "utf8")));
  else cpSync(from, to);
};

const work = join(scratch, "work", name), out = join(scratch, "out", name);
rmSync(work, { recursive: true, force: true });
rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
copy(join(print, "🖋️latex"), join(work, "semio-latex"));
copy(join(print, "🖼️assets"), join(work, "assets"));
copy(join(print, "🔨️modules/🕸️graph"), join(work, "modules/graph"));
copy(isAbsolute(source) ? source : join(repo, source), work);
for (const extra of (process.env.SEMIO_PRINT_EXTRA ?? "").split(process.platform === "win32" ? ";" : ":").filter(Boolean)) cpSync(extra, join(work, staged(basename(extra))));
const mainStaged = staged(main);
if (theme === "dark") writeFileSync(join(work, mainStaged), darkened(readFileSync(join(work, mainStaged), "utf8")));

const fontRoot = join(work, "assets/font");
const search = [join(work, "semio-latex"), work, fonts, ...readdirSync(fontRoot).map((family) => join(fontRoot, family)), join(work, "assets/logo")];
const separator = process.platform === "win32" ? ";" : ":";
const env = {
  ...process.env,
  HOME: scratch, USERPROFILE: scratch, TECTONIC_CACHE_DIR: join(scratch, "cache"),
  LOCALAPPDATA: join(scratch, "AppData", "Local"), APPDATA: join(scratch, "AppData", "Roaming"),
  SOURCE_DATE_EPOCH: "1782864000", TZ: "UTC",
  TEXINPUTS: search.map((path) => `${path}//${separator}${path}${separator}`).join(""),
};
mkdirSync(join(scratch, "AppData", "Local"), { recursive: true });
mkdirSync(join(scratch, "AppData", "Roaming"), { recursive: true });
const passes = Number(process.env.SEMIO_PRINT_PASSES ?? "3");
let result: ReturnType<typeof spawnSync<string>> | undefined;
for (let pass = 0; pass < passes; pass++) {
  result = spawnSync(tectonic, ["--bundle", pathToFileURL(bundle).href, "--keep-logs", "--keep-intermediates", "--reruns", "0", "-Z", "deterministic-mode", ...search.flatMap((path) => ["-Z", `search-path=${path}`]), "--outdir", work, mainStaged], { cwd: work, encoding: "utf8", timeout: Number(process.env.SEMIO_PRINT_TIMEOUT ?? "300000"), env });
  if (result.status !== 0) break;
}
const stem = mainStaged.replace(/\.tex$/, "");
for (const extension of [".pdf", ".log"]) if (existsSync(join(work, stem + extension))) cpSync(join(work, stem + extension), join(out, stem + extension));
const lines = ((result?.stdout ?? "") + (result?.stderr ?? "")).split("\n").filter((line) => /^(error|warning: .*(Overfull|Undefined)|!)/.test(line));
console.log(lines.slice(0, 20).join("\n") || "compiled without errors");

const pdf = join(out, stem + ".pdf");
if (!existsSync(pdf)) throw Error(`no pdf: see ${join(out, stem + ".log")}`);
const rasterise = (page: number, png: string): void => {
  if (process.platform === "darwin") {
    const swift = process.env.SEMIO_PDF2PNG ?? join(here, "print-lane-pdf2png.swift");
    const run = spawnSync("swift", [swift, pdf, String(page), png, scale], { encoding: "utf8" });
    if (run.status !== 0) throw Error(run.stderr);
  } else {
    const dpi = String(Math.round(72 * Number(scale)));
    const run = spawnSync("pdftoppm", ["-r", dpi, "-f", String(page), "-l", String(page), "-singlefile", "-png", pdf, png.replace(/\.png$/, "")], { encoding: "utf8" });
    if (run.status !== 0) throw Error(`pdftoppm required on this platform: ${run.stderr}`);
  }
};
for (const page of pages.split(",").map(Number)) {
  const png = join(out, `${stem}-p${page}.png`);
  rasterise(page, png);
  console.log(png);
}

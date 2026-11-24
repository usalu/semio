/**
 * 🖨️ Ticket-local Print build: stages the Print library and one document the way
 * `📥️source-staging` does (leading pictographs stripped from every path segment and
 * from the references inside TeX sources), compiles with the pinned tectonic and
 * rasterizes the pages with pdftoppm.
 *
 * usage: bun build.ts <source dir relative to repo | absolute dir> <main tex> <name> [dpi]
 * env:   SEMIO_PRINT_SCRATCH  ASCII scratch directory
 */
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { spawnSync } from "node:child_process";

const repo = "C:/semio";
const scratch = process.env.SEMIO_PRINT_SCRATCH;
if (!scratch) throw Error("SEMIO_PRINT_SCRATCH required");
const [source, main, name, dpi = "80"] = process.argv.slice(2);
const print = join(repo, "🧰️framework/🛍️products/📓️print");
const tectonic = join(repo, ".cache/b/t/m/print/tectonic/x86_64-pc-windows-msvc/tectonic.exe");
const segmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });
const staged = (part: string): string => {
  if (part === "📐️.tex") return "_.tex";
  const first = segmenter.segment(part)[Symbol.iterator]().next().value?.segment ?? "";
  return /\p{Extended_Pictographic}|\p{Regional_Indicator}/u.test(first) ? part.slice(first.length) : part;
};
const text = /\.(tex|sty|cls|bib)$/;
const rewrite = (body: string): string =>
  body.replaceAll("📐️.tex", "_.tex").replace(/(?:\p{Extended_Pictographic}|\p{Regional_Indicator})\uFE0F?(?=[A-Za-z0-9_.-])/gu, "");
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
copy(process.env.SEMIO_PRINT_LATEX ?? join(print, "🖋️latex"), join(work, "semio-latex"));
copy(join(print, "🖼️assets"), join(work, "assets"));
copy(join(print, "🔨️modules/🕸️graph"), join(work, "modules/graph"));
copy(isAbsolute(source) ? source : join(repo, source), work);
const fonts = join(work, "assets/font");
const search = [join(work, "semio-latex"), work, ...readdirSync(fonts).map(family => join(fonts, family)), join(work, "assets/logo")];
const passes = Number(process.env.SEMIO_PRINT_PASSES ?? "3");
let result = { stdout: "", stderr: "" } as ReturnType<typeof spawnSync<string>>;
for (let pass = 0; pass < passes; pass++) {
  result = spawnSync(tectonic, ["--keep-logs", "--keep-intermediates", "--reruns", "0", "-Z", "deterministic-mode", ...search.flatMap(path => ["-Z", `search-path=${path}`]), "--outdir", work, staged(main)], {
    cwd: work,
    encoding: "utf8",
    timeout: Number(process.env.SEMIO_PRINT_TIMEOUT ?? "240000"),
    env: { ...process.env, SOURCE_DATE_EPOCH: "1782864000", TZ: "UTC" },
  });
  if (result.status !== 0) break;
}
for (const extension of [".pdf", ".log"]) {
  const produced = join(work, staged(main).replace(/\.tex$/, extension));
  if (existsSync(produced)) cpSync(produced, join(out, staged(main).replace(/\.tex$/, extension)));
}
const lines = (result.stdout + result.stderr).split("\n").filter(line => /^(error|warning: .*(Overfull|Undefined)|!)/.test(line));
console.log(lines.slice(0, 20).join("\n") || "compiled without errors");
const pdf = join(out, staged(main).replace(/\.tex$/, ".pdf"));
if (existsSync(pdf)) {
  spawnSync("pdftoppm", ["-r", dpi, "-png", pdf, join(out, "p")], { cwd: out });
  console.log(readdirSync(out).filter(file => file.endsWith(".png")).join(" "));
}

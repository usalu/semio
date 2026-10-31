import { mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
let root = ticket;
while (!statSync(join(root, "🧰️framework"), { throwIfNoEntry: false })) root = dirname(root);
const print = join(root, "🧰️framework/🛍️products/📓️print");
const { printCompilerName, printSourceReferenceMapV1, printSourceReferenceTextV1 } = await import(pathToFileURL(join(print, "🔨️modules/📥️source-staging/🟦️.ts")).href);
const catalog = JSON.parse(readFileSync(join(print, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8"));
const assignment = join(root, "🎓️teaching/🏛️architecture/🏙️typology/📝️assignment");
const stage = process.env.SEMIO_PROBE_STAGE ?? join(ticket, "🗑️generated/stage");
const bundle = join(root, ".🧬semio/🦑️repo/⚡️cache/tools/tectonic/bundles/2f375cdacdf7f982138c778c3e12d8cd11e4aa3af3d854deb1467d560e8d71b0");
const tectonic = join(root, ".🧬semio/🦑️repo/⚡️cache/tools/tectonic/0.16.9/x86_64-pc-windows-msvc/tectonic.exe");
rmSync(stage, { recursive: true, force: true });

/** 📦️ Copies one tree with compiler names and projected TeX references, as Print staging does. */
function copy(from: string, to: string, references: ReadonlyMap<string, string>): void {
  if (statSync(from).isDirectory()) {
    for (const name of readdirSync(from)) copy(join(from, name), join(to, printCompilerName(name)), references);
    return;
  }
  mkdirSync(dirname(to), { recursive: true });
  const bytes = readFileSync(from);
  writeFileSync(to, /\.(tex|sty|cls|bib)$/.test(from) ? printSourceReferenceTextV1(bytes.toString("utf8"), references) : bytes);
}

const library = join(stage, "library"), libraryReferences = printSourceReferenceMapV1(catalog.librarySources);
for (const source of catalog.librarySources) copy(join(print, source), join(library, ...source.split("/").map(printCompilerName)), libraryReferences);
const fonts = join(stage, "fonts");
for (const directory of readdirSync(join(print, "🖼️assets/🔤️font"))) for (const file of readdirSync(join(print, "🖼️assets/🔤️font", directory))) if (/\.(ttf|otf)$/i.test(file)) copy(join(print, "🖼️assets/🔤️font", directory, file), join(fonts, printCompilerName(file)), libraryReferences);
const sources = readdirSync(assignment).filter(name => /\.(tex|png|jpg)$/.test(name));
const source = join(stage, "source");
for (const name of sources) copy(join(assignment, name), join(source, name), printSourceReferenceMapV1(sources));
for (const language of process.argv.slice(2).length ? process.argv.slice(2) : ["en", "de"]) {
  const output = join(stage, "compiled", language);
  mkdirSync(output, { recursive: true });
  const search = [join(library, "semio-latex"), library, source, output, fonts];
  const env = { ...process.env, SOURCE_DATE_EPOCH: String(catalog.sourceDateEpoch), TZ: "UTC", TECTONIC_CACHE_DIR: join(stage, "cache"), TEXINPUTS: search.map(path => path + "//").join(";") + ";", TTFONTS: fonts + ";", OPENTYPEFONTS: fonts + ";", OSFONTDIR: fonts };
  const result = process.env.SEMIO_PROBE_ENGINE === "xelatex"
    ? [1, 2, 3].map(() => spawnSync("xelatex", ["-interaction=nonstopmode", "-halt-on-error", "-output-directory=" + output, language + ".tex"], { cwd: source, encoding: "utf8", env })).at(-1)!
    : spawnSync(tectonic, ["--bundle", pathToFileURL(bundle).href, "--keep-logs", "--keep-intermediates", "-Z", "deterministic-mode", ...search.flatMap(path => ["-Z", "search-path=" + path]), "--outdir", output, language + ".tex"], { cwd: source, encoding: "utf8", env });
  console.log("[DEBUG]", language, result.status, (result.stdout + result.stderr).split("\n").filter(line => /error|^!|overfull|undefined/i.test(line)).slice(0, 40).join("\n"));
}

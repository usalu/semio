import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = path.dirname(fileURLToPath(import.meta.url));
let workspace = ticket;
while (!fs.existsSync(path.join(workspace, "🧰️framework/🛍️products/📓️print"))) {
  const parent = path.dirname(workspace);
  if (parent === workspace) throw Error("Repository Print product required");
  workspace = parent;
}
const product = path.join(workspace, "🧰️framework/🛍️products/📓️print");
const course = path.join(workspace, "🎓️teaching/🏛️architecture/📐️parametric/📝️assignment");
const generated = path.join(ticket, "🗑️generated");
const output = path.join(generated, "course-qa");
const { printCompilerName, printSourceReferenceMapV1, printSourceReferenceTextV1 } = await import(pathToFileURL(path.join(product, "🔨️modules/📥️source-staging/🟦️.ts")).href);
const { printLibrarySources } = await import(pathToFileURL(path.join(product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts")).href);
const { renderPrintSvgResourceV1 } = await import(pathToFileURL(path.join(product, "🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts")).href);
const signal = AbortSignal.timeout(120000);
const bindings: Record<string, string> = {};
const digest = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const safe = (name: string) => {
  const target = path.resolve(output, name);
  if (!target.startsWith(output + path.sep) || target.length > 256) throw Error("Bounded ticket-generated QA destination required");
  return target;
};
const write = (name: string, bytes: Uint8Array | string) => {
  const target = safe(name);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes);
};
const read = (source: string) => {
  const bytes = fs.readFileSync(source);
  bindings[path.relative(workspace, source).replaceAll("\\", "/")] = digest(bytes);
  return bytes;
};
/** 📚 Projects the actual Print finite source inventory into disposable compiler inputs. */
function stage(root: string, entries: readonly string[], directory: string): void {
  const files: string[] = [], names: string[] = [];
  const visit = (name: string) => {
    signal.throwIfAborted();
    const source = path.join(root, name), facts = fs.lstatSync(source);
    if (facts.isSymbolicLink()) throw Error("Physical QA input required");
    names.push(name);
    if (facts.isDirectory()) for (const child of fs.readdirSync(source)) {
      if (![".DS_Store", "dist", ".semio-dark"].includes(child)) visit(name + "/" + child);
    }
    else if (facts.isFile()) files.push(name);
    else throw Error("Ordinary QA input required");
  };
  entries.forEach(visit);
  const references = printSourceReferenceMapV1(names);
  for (const name of files) {
    const bytes = read(path.join(root, name));
    const target = directory + "/" + name.split("/").map(printCompilerName).join("/");
    write(target, /\.(tex|bib|sty|cls)$/.test(name) ? printSourceReferenceTextV1(bytes.toString("utf8"), references) : bytes);
  }
}
fs.mkdirSync(output, { recursive: true });
stage(product, printLibrarySources(), "library");
const canonical = ["en.tex", "de.tex", "en.content.tex", "de.content.tex", "geometry.tex"];
const shared = printLibrarySources().flatMap((entry: string) => {
  const result: string[] = [];
  const visit = (name: string) => { result.push(name); if (fs.statSync(path.join(product, name)).isDirectory()) for (const child of fs.readdirSync(path.join(product, name))) visit(name + "/" + child); };
  visit(entry); return result;
});
const references = printSourceReferenceMapV1([...shared, ...canonical]);
for (const name of canonical) write("source/" + name, printSourceReferenceTextV1(read(path.join(course, name)).toString("utf8"), references));
const plan = JSON.parse(read(path.join(course, "icons.json")).toString("utf8"));
for (const row of plan.resources) {
  signal.throwIfAborted();
  const bytes = read(path.join(workspace, row.source));
  if (digest(bytes) !== row.sha256) throw Error("Canonical SVG hash changed");
  write("source/" + row.output, await renderPrintSvgResourceV1(bytes, row.width, signal));
}
const fonts = JSON.parse(read(path.join(product, "🔨️modules/🔤print-font-catalog/🔣️.json")).toString("utf8"));
for (const font of fonts) write("fonts/" + font.texFilename, read(path.join(product, "🖼️assets/🔤️font", font.directory, font.filename)));
for (const language of ["en", "de"]) fs.mkdirSync(safe("compiled/" + language), { recursive: true });
const searches = [safe("library/semio-latex"), safe("library"), safe("source"), safe("fonts")];
const bundle = path.join(workspace, ".🧬semio/🦑️repo/⚡️cache/tools/tectonic/bundles/4eaf526eacd253559604ec9a4aaf9b4319abaff487bd7116e611e8537b36a8bf");
const argumentsFor = (language: string) => ["--bundle", pathToFileURL(bundle).href, "--keep-logs", "--keep-intermediates", "-Z", "deterministic-mode", ...searches.flatMap(directory => ["-Z", "search-path=" + directory]), "--outdir", safe("compiled/" + language), safe("source/" + language + ".tex")];
write("compiler-arguments.json", JSON.stringify({ en: argumentsFor("en"), de: argumentsFor("de"), environment: { TECTONIC_CACHE_DIR: safe("tectonic-cache") } }, null, 2));
write("source-bindings.json", JSON.stringify(bindings, null, 2));
for (const [name, expected] of Object.entries(bindings)) if (digest(fs.readFileSync(path.join(workspace, name))) !== expected) throw Error("Consumed source changed during QA staging");
console.log(JSON.stringify({ output, sourceFiles: canonical.length, resources: plan.resources.length, fonts: fonts.length, bindings: Object.keys(bindings).length }));

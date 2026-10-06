/** 🧭️ Probes the existing mathematics station parser with retained valid protected caption tokens. */
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { getDocument } from "pdfjs-dist/legacy/build/pdf.mjs";
import { scaleLinear } from "d3-scale";
import fixture from "./🔣️.json";

let workspace = import.meta.dir;
while (!existsSync(join(workspace, "nx.json"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw Error("Workspace root missing");
  workspace = parent;
}
const ticket = dirname(dirname(import.meta.dir));
const output = join(ticket, "🗑️generated", "native-math-custom-caption-probe");
const { compileVizProbeDocument } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts")).href);
console.log("[DEBUG] Current unchanged-source mathematical protected station probe START");
const records = await compileVizProbeDocument({ case: fixture.id, scenario: fixture.id, documentClass: "semio", documentClassOptions: `type=paper,language=en,theme=${fixture.theme}`, packages: ["semio-viz"], geometry: true, preamble: ["\\title{Mathematical Station Captions}", "\\author{Semio}", "\\date{}"], body: [{ raw: `\\clearpage\\SemioVizProbeBegin{${fixture.id}}{${fixture.id}}\\begin{VizFigure}[title={${fixture.id}},width=80,height=40]\\SemioVizChart{queueing-network-diagram}[${fixture.options}]\\end{VizFigure}` }] }, { workDir: output, scenario: undefined, keepWorkDir: true });
const out = join(output, "🧪️probe-out"), pdfFile = readdirSync(out).find(name => name.endsWith(".pdf"));
if (!pdfFile) throw Error("Actual PDF missing");
const pdf = await getDocument({ data: new Uint8Array(readFileSync(join(out, pdfFile))) }).promise;
let text = "";
try {
  for (let page = 1; page <= pdf.numPages; page++) text += (await (await pdf.getPage(page)).getTextContent()).items.flatMap(item => "str" in item ? [item.str] : []).join(" ");
} finally { await pdf.destroy(); }
for (const glyph of fixture.mathGlyphs) if (!text.includes(glyph)) throw Error(`Actual caption glyph missing: ${glyph}`);
if (/[\uFFFD\uFFFF]/u.test(text)) throw Error("Invalid actual caption glyph");
const nodes = records.filter(record => record.scenario === fixture.id && record.key === "diagram/node");
if (nodes.length !== fixture.positions.length) throw Error("Actual station count differs");
const projection = scaleLinear([0, 1], [0, fixture.unit]);
for (const [index, node] of nodes.entries()) if (Math.abs(Number(node.values[0]) - projection(fixture.positions[index]!)) > 1e-5) throw Error(`Actual node ${index} differs from independent D3 position`);
writeFileSync(join(output, "actual-admission.json"), JSON.stringify({ text, records, mathematicalGlyphs: fixture.mathGlyphs.length, d3Projections: nodes.length }, null, 2));
console.log(`[DEBUG] Mathematical protected station probe PASS: ${fixture.mathGlyphs.length} actual glyphs, ${nodes.length} independent D3 positions`);

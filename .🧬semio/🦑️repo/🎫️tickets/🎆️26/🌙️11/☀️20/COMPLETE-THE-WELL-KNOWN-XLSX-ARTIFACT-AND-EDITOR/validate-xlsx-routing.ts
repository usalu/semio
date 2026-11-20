import { strict as assert } from "node:assert";
import { createRequire } from "node:module";
import { join } from "node:path";

const root = process.cwd();
const load = async (file: string) => await import(join(root, file));
const json = async (file: string) => await Bun.file(join(root, file)).json();
const hub = "🌎️hub/🧩️compositions/🗄️stdio";
const source = "✏️s/🔌️plugins/🗄️stdio";
const shell = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const { stdioFormatClaimsTextV1 } = await load(`${source}/📇️registry/🧬️contract/🧭️format-claims/📜️generate.ts`);
const { routePickedFileV1 } = await load(`${shell}/🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts`);
const { formats } = JSON.parse(stdioFormatClaimsTextV1(root)) as {
  formats: { kindId: string; shortId: string; artifactRef: string; mimes: string[]; extensions: string[] }[];
};
const xlsx = formats.find(row => row.shortId === "xlsx");
assert.ok(xlsx);
const manifest = await json(`${hub}/🧩️extensions/💼️office/🧩️component/🔣️.json`);
const apps = manifest.manifest?.apps ?? manifest.plugin?.apps ?? manifest.apps;
const editor = apps?.find((app: { id: string }) => app.id === `${xlsx.artifactRef}#editor`);
assert.ok(editor, "office component must publish the editor claimed by natural file routing");
const catalog = { descriptors: () => formats, artifactRefOf: (kindId: string) => formats.find(row => row.kindId === kindId)?.artifactRef };
const noReads = () => { throw new Error("a single XLSX representation must not sniff the ZIP head"); };
const mime = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
for (const [picked, basis] of [[{ name: "Budget.XLSX", type: "application/zip" }, "extension"], [{ name: "Budget", type: mime }, "mime"]] as const) {
  const route = await routePickedFileV1(catalog, picked, noReads);
  assert.equal(route.kind, "routed");
  assert.equal(route.artifactRef, "s.stdio.xlsx@ecma-376/*");
  assert.equal(route.resolution.basis, basis);
  assert.equal(route.resolution.format.kindId, xlsx.kindId);
}
for (const [actionId, direction] of [["openArtifactFile", "importFormats"], ["saveArtifactFile", "exportFormats"]]) {
  const actions = [...(editor.actions ?? []), ...editor.windowKinds.flatMap((kind: { actions?: object[] }) => kind.actions ?? [])];
  const action = actions.find(candidate => candidate.id === actionId);
  assert.ok(action);
  const defaults = Object.fromEntries(action.args.map((argument: {id: string; default: unknown }) => [argument.id, argument.default]));
  assert.deepEqual(defaults, { formatKind: "s.stdio.xlsx@ecma-376", extension: ".xlsx", mediaType: mime, binary: true });
  assert.ok(editor.io[direction].includes(defaults.formatKind));
}
const dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests";
const row = "stdio-office/xlsx";
const matrix = await json("🌎️hub/🧪️tests/🧫️fixtures/🚪️io-matrix.json");
const programs = await json(`${dev}/🧫️fixtures/🧮️program-matrix.json`);
assert.equal(matrix.programs[row].imports[0].editedCell, "A1");
assert.equal(programs.kindVerbs[row], "set-cell");
assert.equal(programs.kindEdits[`${row}.set-cell`].control, "input[aria-label=\"A1\"]");
const transpiler = new Bun.Transpiler({ loader: "ts" });
assert.ok(transpiler.transformSync(await Bun.file(join(root, "🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts")).text()).length > 0);
const { formatOfFileName, judgeFile, readIoMatrixPins, readXlsx } = await load("🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts");
assert.equal(formatOfFileName("Budget.XLSX"), "xlsx");
assert.deepEqual(readIoMatrixPins().programs[row], matrix.programs[row]);
console.log("XLSX declaration, extension/MIME routing and office Open/Save File agree");
const osDev = dev.slice(0, -"/🧪️tests".length);
const { devServeCommandV1 } = await load(`${osDev}/🚀️local-hub/🏃️execution/🟦️.ts`);
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
const Ajv = require("ajv").default;
const contract = await json(`${osDev}/🧬️schema/🔣️.json`);
const { serves } = await json(`${dev}/🧫️fixtures/🚀️local-hub.json`);
const ajv = new Ajv({ strict: true }).addSchema(contract);
const validate = ajv.getSchema(`${contract.$id}#/$defs/DevServeSpawnRequestV1`);
for (const { request, command } of serves.spawns) {
  assert.equal(validate(request), true);
  assert.deepEqual(devServeCommandV1(request), command);
}
console.log("Four managed browser serve plans match the neutral fixture and Ajv schema");
const flags = process.argv.slice(2);
if (flags.includes("--oracle")) {
  const { ensureRepositoryBrowserPathV1 } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts");
  ensureRepositoryBrowserPathV1(root, process.env);
  const { chromium } = require("playwright");
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    const corpus = await json(`${source}/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/🔣️.json`);
    const fixtureWorkbook = corpus.artifacts.find((entry: { artifact: string }) => entry.artifact === "xlsx").sample;
    const demoWorkbook = `${source}/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/📕️example.xlsx`;
    for (const workbook of [fixtureWorkbook, demoWorkbook]) {
      const original = new Uint8Array(await Bun.file(join(root, workbook)).arrayBuffer());
      const valid = await judgeFile(page, "Budget.XLSX", original, "any");
      assert.equal(valid.ok, true, JSON.stringify(valid));
      const parsed = await readXlsx(page, original, "A1");
      assert.equal(parsed.valid, true);
      assert.ok(parsed.sheetNames[0]);
      assert.ok(parsed.text);
      assert.equal((await judgeFile(page, "truncated.xlsx", original.slice(0, 20), "xlsx")).ok, false);
    }
    const demo = await readXlsx(page, await Bun.file(join(root, demoWorkbook)).bytes(), "A1");
    assert.deepEqual(demo.sheetNames, ["Sheet1", "Totals"]);
    assert.equal(demo.text, "Name");
    console.log("Hub oracle accepts the XLSX acceptance fixture and native demo, and rejects truncation in Chromium");
  } finally {
    await browser.close();
  }
}
for (const url of flags.filter(flag => flag !== "--oracle")) {
  const response = await fetch(url, { signal: AbortSignal.timeout(5_000) });
  assert.ok(response.ok, `local browser host returned ${response.status}`);
}

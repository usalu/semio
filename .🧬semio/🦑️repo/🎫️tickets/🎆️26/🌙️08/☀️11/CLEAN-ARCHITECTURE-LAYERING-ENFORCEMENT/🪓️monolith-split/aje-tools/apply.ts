/** 🧱️ Applies the asset and JavaScript edge batch: package-owned facades with explicit exports, importer rewrites, manifest dependencies, one new package. Dry-run by default; re-reads every file right before writing and asserts every anchor is unique.
 * Usage: bun apply.ts [--write] [--snapshot <out.json>]
 */
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { segName } from "../psc3-tools/model.ts";

const ROOT = "/Users/ueli/Documents/semio/";
const args = process.argv.slice(2);
const write = args.includes("--write");
const snapIdx = args.indexOf("--snapshot");
const snapshotPath = snapIdx >= 0 ? args[snapIdx + 1] : undefined;
const nfc = (s: string) => s.normalize("NFC");
const snapshot: Record<string, string | null> = {};
const problems: string[] = [];
const log: string[] = [];

const FEM2D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d";
const FEM3D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d";
const BLOCK2D = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d";
const BLOCK3D = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d";
const BLOCK5D = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d";
const OS = "🧰️framework/🛍️products/💻️os";
const ASSETS = "🧰️framework/🔨️modules/🖼️assets";
const QUIZ_REACT = "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
const REPO_TEST = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";
const REGISTRY = "✏️s/🔌️plugins/🗄️stdio/📇️registry";
const TS = "/📦️packages/🟦️typescript";
const FEM_STD = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo";
const BLOCK_STD = "🏅️standards/🔖️1/🪆️subsets/✳️any";
const LEFT = "📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest";
const RIGHT = "📚️examples/➡️hexagonal-cut-concrete-forest-right/🖼️assets/➡️hexagonal-cut-concrete-forest";
const NAKAGIN = "📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule";
const FLOW_TESTS = "🔨️modules/🌊️flow/🕸️wasm/🧪️tests";
const FLOW_TESTING = "🔨️modules/🌊️flow/🕸️wasm/🧪️testing";

type Shape = { kind: "default"; name: string } | { kind: "named"; names: string[] } | { kind: "side" };
interface Row { pkg: string; pkgDir: string; facade: string; key: string; target: string; query?: string; shape: Shape; what: string; referenceVite?: boolean }

const rows: Row[] = [
  { pkg: "@semio-tech/fem-2d", pkgDir: FEM2D + TS, facade: `${FEM_STD}/🟦️.ts`, key: "", target: `${FEM2D}/${FEM_STD}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "fem2dDemoDsl" }, what: "the shipped FEM 2D demo document text" },
  { pkg: "@semio-tech/fem-3d", pkgDir: FEM3D + TS, facade: `${FEM_STD}/🟦️.ts`, key: "", target: `${FEM3D}/${FEM_STD}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "fem3dDemoDsl" }, what: "the shipped FEM 3D demo document text" },
  { pkg: "@semio-tech/block-2d", pkgDir: BLOCK2D + TS, facade: `${LEFT}/🟦️.ts`, key: "", target: `${BLOCK2D}/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block2dConcreteForestLeftDsl" }, what: "the shipped Block 2D concrete forest left document text" },
  { pkg: "@semio-tech/block-2d", pkgDir: BLOCK2D + TS, facade: `${RIGHT}/🟦️.ts`, key: "", target: `${BLOCK2D}/${BLOCK_STD}/${RIGHT}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block2dConcreteForestRightDsl" }, what: "the shipped Block 2D concrete forest right document text" },
  { pkg: "@semio-tech/block-3d", pkgDir: BLOCK3D + TS, facade: `${LEFT}/🟦️.ts`, key: "", target: `${BLOCK3D}/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block3dConcreteForestLeftDsl" }, what: "the shipped Block 3D concrete forest left document text" },
  { pkg: "@semio-tech/block-3d", pkgDir: BLOCK3D + TS, facade: `${NAKAGIN}/🟦️.ts`, key: "", target: `${BLOCK3D}/${BLOCK_STD}/${NAKAGIN}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block3dNakaginCapsuleDsl" }, what: "the shipped Block 3D Nakagin capsule document text" },
  { pkg: "@semio-tech/block-5d", pkgDir: BLOCK5D + TS, facade: `${LEFT}/🟦️.ts`, key: "", target: `${BLOCK5D}/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block5dConcreteForestLeftDsl" }, what: "the shipped Block 5D concrete forest left document text" },
  { pkg: "@semio-tech/block-5d", pkgDir: BLOCK5D + TS, facade: `${NAKAGIN}/🟦️.ts`, key: "", target: `${BLOCK5D}/${BLOCK_STD}/${NAKAGIN}/🗣️.dsl.semio`, query: "?raw", shape: { kind: "default", name: "block5dNakaginCapsuleDsl" }, what: "the shipped Block 5D Nakagin capsule document text" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: "🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🟦️.ts", key: "", target: `${OS}/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`, query: "?url", shape: { kind: "default", name: "abbauAufbauMasterarbeitGrundrissUrl" }, what: "the url of the Abbau-Aufbau master thesis floor plan scan" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: "🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🟦️.ts", key: "", target: `${OS}/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png`, query: "?url", shape: { kind: "default", name: "rathausAhlenGrundrissUrl" }, what: "the url of the Rathaus Ahlen floor plan" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule-j/🟦️.ts", key: "", target: `${ASSETS}/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb`, query: "?url", shape: { kind: "default", name: "capsuleJGlbUrl" }, what: "the url of the capsule J mesh" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline/🟦️.ts", key: "", target: `${ASSETS}/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, query: "?inline", shape: { kind: "default", name: "antaLatinOutlineFontDataUrl" }, what: "the inlined Anta latin outline font data url" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline/🟦️.ts", key: "", target: `${ASSETS}/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf`, query: "?inline", shape: { kind: "default", name: "antaLatinExtOutlineFontDataUrl" }, what: "the inlined Anta latin extended outline font data url" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline/🟦️.ts", key: "", target: `${ASSETS}/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf`, query: "?inline", shape: { kind: "default", name: "antaMathOutlineFontDataUrl" }, what: "the inlined Anta math outline font data url" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline/🟦️.ts", key: "", target: `${ASSETS}/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf`, query: "?inline", shape: { kind: "default", name: "antaSymbolsOutlineFontDataUrl" }, what: "the inlined Anta symbols outline font data url" },
  { pkg: "@semio-tech/assets", pkgDir: ASSETS + TS, facade: "👆️cursor/🖱️default/☀️light/🟦️.ts", key: "", target: `${ASSETS}/👆️cursor/🖱️default/☀️light.svg`, query: "?raw", shape: { kind: "default", name: "defaultCursorLightSvg" }, what: "the default light cursor svg text" },
  { pkg: "@semio-tech/quiz-react", pkgDir: QUIZ_REACT + TS, facade: "🎨️stylesheet/🟦️.ts", key: "", target: `${QUIZ_REACT}/🎨️.css`, query: "?raw", shape: { kind: "default", name: "quizStylesheet" }, what: "the quiz stylesheet source text" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: `${FLOW_TESTING}/🖥️host/🟦️.ts`, key: "", target: `${OS}/${FLOW_TESTS}/🖥️host/🟨️.js`, shape: { kind: "side" }, what: "the flow browser host test: importing it runs the test" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: `${FLOW_TESTING}/🎬️draw-list/🟦️.ts`, key: "", target: `${OS}/${FLOW_TESTS}/🎬️draw-list/🟨️.js`, shape: { kind: "side" }, what: "the flow draw list replay test: importing it runs the test" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: `${FLOW_TESTING}/⏱️consumed-browser-clock/🟦️.ts`, key: "", target: `${OS}/${FLOW_TESTS}/⏱️consumed-browser-clock/🟨️.js`, shape: { kind: "named", names: ["testFlowBrowserClock"] }, what: "the flow browser clock test" },
  { pkg: "@semio-tech/framework-os", pkgDir: OS + TS, facade: `${FLOW_TESTING}/🧬️schema-oracle/🟦️.ts`, key: "", target: `${OS}/${FLOW_TESTS}/🧬️schema-oracle/🟨️.js`, shape: { kind: "side" }, what: "the flow ABI schema oracle test: importing it runs the test" },
  { pkg: "@semio-tech/repo-test", pkgDir: REPO_TEST + TS, facade: "🕸️dependencies/🟦️.ts", key: "", target: `${REPO_TEST}/🕸️dependencies/🟨️.mjs`, shape: { kind: "named", names: ["packagesForOwner"] }, what: "the test dependency selection helper" },
  { pkg: "@semio-tech/stdio-registry-ts", pkgDir: REGISTRY + TS, facade: "🧬️schema/🟦️.ts", key: "", target: `${REGISTRY}/🧬️schema/🔣️.json`, shape: { kind: "default", name: "stdioRegistrySchema" }, what: "the stdio registry schema" },
  { pkg: "@semio-tech/stdio-registry-ts", pkgDir: REGISTRY + TS, facade: "🧬️contract/📐️part21/🧪️testing/🧫️fixtures/🚦️sqlite-cohort/🟦️.ts", key: "", target: `${REGISTRY}/🧬️contract/📐️part21/🧪️tests/🧫️fixtures/🚦️sqlite-cohort/🔣️.json`, shape: { kind: "default", name: "part21SqliteCohortFixture" }, what: "the part 21 sqlite cohort fixture" },
];

const keyOf = (r: Row): string => {
  const segs = dirname(r.facade).split("/").map(s => s.replace(/^🧪️testing$/u, "testing")).map(segName).filter(Boolean);
  return "./" + segs.join("/");
};
for (const r of rows) r.key = keyOf(r);

const read = (rel: string): string | null => { const abs = join(ROOT, rel); return existsSync(abs) ? readFileSync(abs, "utf8") : null; };
const saved = (rel: string, text: string | null) => { if (!(rel in snapshot)) snapshot[rel] = text; };
const put = (rel: string, text: string, before: string | null) => {
  saved(rel, before);
  if (!write) return;
  const abs = join(ROOT, rel);
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, text);
};
const emojiOf = (seg: string) => seg.match(/^(?:\p{Extended_Pictographic}(?:\u{FE0F}|\u{200D}\p{Extended_Pictographic}\u{FE0F}?)*)/u)?.[0] ?? "🔌️";

const bytes = (s: string) => Buffer.byteLength(s, "utf8");
const facadeText = (r: Row): string => {
  const facadeRel = r.pkgDir + "/" + r.facade;
  const spec = (() => { const x = relative(join(ROOT, dirname(facadeRel)), join(ROOT, r.target)).normalize("NFC"); return (x.startsWith(".") ? x : "./" + x) + (r.query ?? ""); })();
  const segs = r.facade.split("/");
  const header = `/** ${emojiOf(segs[segs.length - 2])} Public entry \`${r.pkg}${r.key.slice(1)}\`: ${r.what}. */\n`;
  const ref = r.query ? `/// <reference types="vite/client" />\n` : "";
  const body = r.shape.kind === "default" ? `export { default as ${r.shape.name} } from "${spec}";\n` : r.shape.kind === "named" ? `export { ${r.shape.names.join(", ")} } from "${spec}";\n` : `import "${spec}";\n`;
  return header + ref + body;
};

const sortBytes = (a: string, b: string) => Buffer.from(a).compare(Buffer.from(b));
const manifests = new Map<string, any>();
const NEW_PACKAGE_DIR = REGISTRY + TS;
const manifestOf = (dir: string) => {
  let m = manifests.get(dir);
  if (!m) {
    const text = read(dir + "/package.json");
    if (text === null) {
      if (dir !== NEW_PACKAGE_DIR) throw new Error("no manifest " + dir);
      const doc = { name: "@semio-tech/stdio-registry-ts", private: true, type: "module", exports: {} };
      m = { text: JSON.stringify(doc, null, 2) + "\n", doc, isNew: true };
    } else {
      const doc = JSON.parse(text);
      if (JSON.stringify(doc, null, 2) + (text.endsWith("\n") ? "\n" : "") !== text) throw new Error("manifest not round-trip stable " + dir);
      m = { text, doc, isNew: false };
    }
    manifests.set(dir, m);
  }
  return m;
};
const addExport = (dir: string, key: string, target: string) => {
  const m = manifestOf(dir);
  const exp = m.doc.exports ?? (m.doc.exports = {});
  if (key in exp) { if (exp[key] !== target) problems.push(`exports ${dir} ${key} already ${JSON.stringify(exp[key])}`); return; }
  const existing = Object.keys(exp);
  const fresh = { ...exp, [key]: target };
  const ordered = [...existing, ...Object.keys(fresh).filter(k => !existing.includes(k)).sort(sortBytes)];
  m.doc.exports = Object.fromEntries(ordered.map(k => [k, fresh[k]]));
};
const addDep = (dir: string, section: "dependencies" | "devDependencies", name: string) => {
  const m = manifestOf(dir);
  for (const s of ["dependencies", "devDependencies", "peerDependencies"]) if (m.doc[s]?.[name]) return;
  if (m.doc[section]) { m.doc[section] = { ...m.doc[section], [name]: "workspace:*" }; return; }
  const entries = Object.entries(m.doc);
  const afterDeps = entries.findIndex(([k]) => k === "dependencies");
  const afterExp = entries.findIndex(([k]) => k === "exports");
  const at = afterDeps >= 0 ? afterDeps + 1 : afterExp >= 0 ? afterExp + 1 : entries.length;
  entries.splice(at, 0, [section, { [name]: "workspace:*" }]);
  m.doc = Object.fromEntries(entries);
};

const edits: { file: string; from: string; to: string }[] = [];
const edit = (file: string, from: string, to: string) => edits.push({ file, from, to });


edit("✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts", `import fem2dDemoDsl from "../../../../🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";`, `import { fem2dDemoDsl } from "@semio-tech/fem-2d/standards/1/subsets/any/assets/demo";`);
edit("✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts", `import fem3dDemoDsl from "../../../../🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";`, `import { fem3dDemoDsl } from "@semio-tech/fem-3d/standards/1/subsets/any/assets/demo";`);

const BS = "✏️s/🧑‍💻dev/🧱️block/📖️stories/🧭️coordination/🟦️.ts";
const blockSpec = (pkg: string, example: string, asset: string) => `${pkg}/examples/${segName(example)}/assets/${segName(asset)}`;
edit(BS, `import block2dLeftDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/◻️2d/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio?raw";`, `import { block2dConcreteForestLeftDsl as block2dLeftDsl } from "@semio-tech/block-2d/examples/hexagonal-cut-concrete-forest-left/assets/hexagonal-cut-concrete-forest";`);
edit(BS, `import block2dRightDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/◻️2d/${BLOCK_STD}/${RIGHT}/🗣️.dsl.semio?raw";`, `import { block2dConcreteForestRightDsl as block2dRightDsl } from "@semio-tech/block-2d/examples/hexagonal-cut-concrete-forest-right/assets/hexagonal-cut-concrete-forest";`);
edit(BS, `import block3dLeftDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio?raw";`, `import { block3dConcreteForestLeftDsl as block3dLeftDsl } from "@semio-tech/block-3d/examples/hexagonal-cut-concrete-forest-left/assets/hexagonal-cut-concrete-forest";`);
edit(BS, `import block3dNakaginDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/${BLOCK_STD}/${NAKAGIN}/🗣️.dsl.semio?raw";`, `import { block3dNakaginCapsuleDsl as block3dNakaginDsl } from "@semio-tech/block-3d/examples/nakagin-capsule/assets/nakagin-capsule";`);
edit(BS, `import block5dLeftDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/${BLOCK_STD}/${LEFT}/🗣️.dsl.semio?raw";`, `import { block5dConcreteForestLeftDsl as block5dLeftDsl } from "@semio-tech/block-5d/examples/hexagonal-cut-concrete-forest-left/assets/hexagonal-cut-concrete-forest";`);
edit(BS, `import block5dNakaginDsl from "../../../../🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/${BLOCK_STD}/${NAKAGIN}/🗣️.dsl.semio?raw";`, `import { block5dNakaginCapsuleDsl as block5dNakaginDsl } from "@semio-tech/block-5d/examples/nakagin-capsule/assets/nakagin-capsule";`);
edit(BS, `import capsuleJGlbUrl from "../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb?url";`, `import { capsuleJGlbUrl } from "@semio-tech/assets/metabolism/representation/capsules/j/capsule-j";`);

const PS = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📖️stories/🎭️3d-world/🧪️.story.tsx";
edit(PS, `import abbauAufbauReferenceUrl from "../../../../../../../${OS}/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";`, `import { abbauAufbauMasterarbeitGrundrissUrl as abbauAufbauReferenceUrl } from "@semio-tech/framework-os/modules/infinite/assets/abbau-aufbau-masterarbeit-grundriss";`);
edit(PS, `import rathausAhlenReferenceUrl from "../../../../../../../${OS}/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png";`, `import { rathausAhlenGrundrissUrl as rathausAhlenReferenceUrl } from "@semio-tech/framework-os/modules/infinite/assets/rathaus-ahlen-grundriss";`);

const FS = `${OS}/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`;
edit(FS, `    await import("../../../🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js");`, `    await import("@semio-tech/framework-os/modules/flow/wasm/testing/schema-oracle");`);
edit(FS, `    await import("../../../🕸️wasm/🧪️tests/🖥️host/🟨️.js");`, `    await import("@semio-tech/framework-os/modules/flow/wasm/testing/host");`);
edit(FS, `    await import("../../../🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js");`, `    await import("@semio-tech/framework-os/modules/flow/wasm/testing/draw-list");`);
edit(FS, `    const { testFlowBrowserClock } = await import("../../../🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js");`, `    const { testFlowBrowserClock } = await import("@semio-tech/framework-os/modules/flow/wasm/testing/consumed-browser-clock");`);

edit(`${REGISTRY.replace("🗄️stdio/📇️registry", "🗄️stdio")}/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`, `import cohort from "../../../../../../../../../../📇️registry/🧬️contract/📐️part21/🧪️tests/🧫️fixtures/🚦️sqlite-cohort/🔣️.json";`, `import { part21SqliteCohortFixture as cohort } from "@semio-tech/stdio-registry-ts/contract/part21/testing/fixtures/sqlite-cohort";`);
edit(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧪️tests/🧪️fixture-corpus/🟦️.ts`, `import registrySchema from "../../../../../../../../../../📇️registry/🧬️schema/🔣️.json";`, `import { stdioRegistrySchema as registrySchema } from "@semio-tech/stdio-registry-ts/schema";`);

edit("✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔗️browser-contribution/🟦️.ts", `import { linkedSessionEngines } from "@semio-tech/framework-os-dev/engine/selection";\nimport { declaredBrowserSessionEnginesV1 } from "../../../../../${OS}/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs";`, `import { declaredBrowserSessionEnginesV1, linkedSessionEngines } from "@semio-tech/framework-os-dev/engine/selection";`);
edit("🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts", `import { packagesForOwner } from "../../../../../../${REPO_TEST}/🕸️dependencies/🟨️.mjs";`, `import { packagesForOwner } from "@semio-tech/repo-test/dependencies";`);

for (const stylesheetTest of ["🌗️contrast-states", "🏠️home-grid", "💭️crowd-client", "📐️adaptive-layout", "📡️presence-client", "📱️home-text-and-viewport", "🚶️journey-and-tabs"]) edit(`🧰️framework/🛍️products/❓️quiz/🧪️tests/${stylesheetTest}/🟦️.tsx`, `import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";`, `import { quizStylesheet as stylesheet } from "@semio-tech/quiz-react/stylesheet";`);
const RG = "🧰️framework/🛍️products/❓️quiz/🧪️tests/🕷️radar-geometry/🟦️.tsx";
edit(RG, `import antaLatin from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf?inline";`, `import { antaLatinOutlineFontDataUrl as antaLatin } from "@semio-tech/assets/fonts/anta/latin/regular/outline";`);
edit(RG, `import antaExtended from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf?inline";`, `import { antaLatinExtOutlineFontDataUrl as antaExtended } from "@semio-tech/assets/fonts/anta/latin-ext/regular/outline";`);
edit(RG, `import antaMath from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf?inline";`, `import { antaMathOutlineFontDataUrl as antaMath } from "@semio-tech/assets/fonts/anta/math/regular/outline";`);
edit(RG, `import antaSymbols from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf?inline";`, `import { antaSymbolsOutlineFontDataUrl as antaSymbols } from "@semio-tech/assets/fonts/anta/symbols/regular/outline";`);
edit("🧰️framework/🛍️products/❓️quiz/🧪️tests/📡️presence-client/🟦️.tsx", `import semioCursor from "../../../../🔨️modules/🖼️assets/👆️cursor/🖱️default/☀️light.svg?raw";`, `import { defaultCursorLightSvg as semioCursor } from "@semio-tech/assets/cursor/default/light";`);

edit(`${QUIZ_REACT}/🧪️tests/🎚️config/🟦️.ts`, `      { find: "@semio-tech/quiz-react", replacement: resolve(root, "🟦️.tsx") },`, `      { find: /^@semio-tech\\/quiz-react$/u, replacement: resolve(root, "🟦️.tsx") },`);

const FLOW_PROJECT = `${OS}/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json`;

for (const r of rows) {
  const facadeRel = r.pkgDir + "/" + r.facade;
  if (bytes(facadeRel) > 240) problems.push(`path over 240 bytes (${bytes(facadeRel)}): ${facadeRel}`);
  if (!existsSync(join(ROOT, r.target))) problems.push("target missing " + r.target);
}

const created: string[] = [], editedFiles = new Set<string>();

const keyAdds = new Map<string, { pkgDir: string; key: string; target: string }[]>();
const facadeFiles = new Map<string, string>();
for (const r of rows) {
  const facadeRel = r.pkgDir + "/" + r.facade;
  if (facadeFiles.has(facadeRel)) problems.push("duplicate facade " + facadeRel);
  facadeFiles.set(facadeRel, facadeText(r));
  const l = keyAdds.get(r.pkgDir) ?? [];
  l.push({ pkgDir: r.pkgDir, key: r.key, target: "./" + r.facade });
  keyAdds.set(r.pkgDir, l);
}
for (const [facadeRel, text] of facadeFiles) {
  const before = read(facadeRel);
  if (before !== null) { problems.push("facade exists " + facadeRel); continue; }
  put(facadeRel, text, null);
  created.push(facadeRel);
}

for (const [dir, adds] of keyAdds) for (const a of adds) addExport(dir, a.key, a.target);

const devSelection = `${OS}/🔨️modules/🧑‍💻dev${TS}/⚙️engine/🧭️selection/🟦️.ts`;
{
  const before = read(devSelection)!;
  const add = `export { declaredBrowserSessionEnginesV1 } from "../../../../⚙️engine/🧭️selection/🟨️.mjs";\n`;
  if (before.includes("declaredBrowserSessionEnginesV1")) problems.push("selection facade already exports it");
  else { put(devSelection, before + add, before); editedFiles.add(devSelection); }
}

addDep("✏️s/🧑‍💻dev/🧱️block" + TS, "devDependencies", "@semio-tech/block-2d");
addDep("✏️s/🧑‍💻dev/🧱️block" + TS, "devDependencies", "@semio-tech/block-3d");
addDep("✏️s/🧑‍💻dev/🧱️block" + TS, "devDependencies", "@semio-tech/block-5d");
addDep("✏️s/🧑‍💻dev/🧱️block" + TS, "devDependencies", "@semio-tech/assets");
addDep("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" + TS, "devDependencies", "@semio-tech/framework-os");
addDep(`${OS}/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust`, "devDependencies", "@semio-tech/framework-os");
addDep("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc" + TS, "devDependencies", "@semio-tech/stdio-registry-ts");
addDep("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf" + TS, "devDependencies", "@semio-tech/stdio-registry-ts");
addDep("🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles", "devDependencies", "@semio-tech/repo-test");
addDep(`${OS}/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust`, "devDependencies", "@semio-tech/framework-os");
addDep("🧰️framework/🛍️products/❓️quiz" + TS, "devDependencies", "@semio-tech/assets");

const byFile = new Map<string, { from: string; to: string }[]>();
for (const e of edits) { const l = byFile.get(e.file) ?? []; l.push(e); byFile.set(e.file, l); }
for (const [file, list] of byFile) {
  let text = read(file);
  if (text === null) { problems.push("missing importer " + file); continue; }
  const before = text;
  for (const e of list) {
    const n = text.split(e.from).length - 1;
    if (n !== 1) { problems.push(`anchor count ${n} in ${file}: ${e.from.slice(0, 110)}`); continue; }
    text = text.replace(e.from, () => e.to);
  }
  if (text !== before) { put(file, text, before); editedFiles.add(file); }
}

{
  const text = read(FLOW_PROJECT);
  if (text === null) problems.push("missing flow project.json");
  else {
    let next = text;
    for (const [js, facade] of [["⏱️consumed-browser-clock", "⏱️consumed-browser-clock"], ["🖥️host", "🖥️host"], ["🎬️draw-list", "🎬️draw-list"], ["🧬️schema-oracle", "🧬️schema-oracle"]] as const) {
      const line = (indent: string, path: string, comma: boolean) => `${indent}"{workspaceRoot}/${path}"${comma ? "," : ""}`;
      const jsPath = `${OS}/${FLOW_TESTS}/${js}/🟨️.js`;
      const facadePath = `${OS}/📦️packages/🟦️typescript/${FLOW_TESTING}/${facade}/🟦️.ts`;
      const re = new RegExp(`^( *)"\\{workspaceRoot\\}/${jsPath.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}"(,?)$`, "gmu");
      const hits = [...next.matchAll(re)];
      if (!hits.length) { problems.push("no project.json input for " + jsPath); continue; }
      next = next.replace(re, (_m, indent: string, comma: string) => `${indent}"{workspaceRoot}/${jsPath}",\n${indent}"{workspaceRoot}/${facadePath}"${comma}`);
      void line;
    }
    if (next !== text) { put(FLOW_PROJECT, next, text); editedFiles.add(FLOW_PROJECT); }
  }
}

for (const [dir, m] of manifests) {
  const out = JSON.stringify(m.doc, null, 2) + (m.text.endsWith("\n") ? "\n" : "");
  if (out === m.text && !m.isNew) continue;
  const rel = dir + "/package.json";
  put(rel, out, m.isNew ? null : m.text);
  (m.isNew ? created : [...editedFiles]);
  if (m.isNew) created.push(rel); else editedFiles.add(rel);
}

console.log(`${write ? "WROTE" : "DRY"}: rows=${rows.length} facades=${facadeFiles.size} created=${created.length} editedFiles=${editedFiles.size} manifests=${manifests.size} edits=${edits.length} problems=${problems.length}`);
for (const p of problems) console.log("PROBLEM", p);
for (const r of rows) console.log(`KEY ${r.pkg}${r.key.slice(1)}  <- ${r.pkgDir.split("/").slice(-2).join("/")}/${r.facade.slice(-70)}`);
if (write && snapshotPath) writeFileSync(snapshotPath, JSON.stringify(snapshot));
for (const l of log) console.log(l);
if (process.env.LIST_DIRS) for (const f of facadeFiles.keys()) console.log("DIR " + dirname(f));

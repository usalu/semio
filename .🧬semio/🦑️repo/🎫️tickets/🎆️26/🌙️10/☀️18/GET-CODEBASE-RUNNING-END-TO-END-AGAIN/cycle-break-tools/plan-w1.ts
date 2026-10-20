/** 🧾 Wave 1 plan: typed asset modules replace the cross-package `?raw`/`?url` imports (stages exports, declarations and statement rewrites). */
import { bindingOf, declares, model, read } from "./lib.ts";
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";

const plan = new Plan();
const WAVE = "w1-query-imports";
const PZ = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts";
const addKey = (packageJson: string, key: string, value: string, why: string): void => {
  const pkg = model.byJson.get(packageJson);
  if (!pkg) throw Error("unknown manifest " + packageJson);
  if (pkg.exports[key] !== undefined) return;
  plan.manifest({ packageJson, package: pkg.name, field: "exports", key, value, op: "add", why, wave: WAVE });
};
const declareDep = (importerFile: string, name: string, why: string, optionalField: "dependencies" | "devDependencies" = "devDependencies"): void => {
  const m = bindingOf(importerFile);
  if (m.name === name || m.packageJson === "package.json" || declares(m, name)) return;
  plan.manifest({ packageJson: m.packageJson, package: m.name, field: optionalField, key: name, value: "workspace:*", op: "add", why, wave: WAVE });
};
const rewrite = (file: string, anchor: string | RegExp, to: string, why: string): void => {
  const text = read(file);
  const from = typeof anchor === "string" ? anchor : (text.match(anchor)?.[0] ?? "");
  if (!from || !text.includes(from)) throw Error(`statement not found in ${file}: ${String(anchor).slice(0, 80)}`);
  if (text.indexOf(from) !== text.lastIndexOf(from)) throw Error(`statement not unique in ${file}`);
  plan.edit({ file, from, to, mode: "text", why, wave: WAVE });
};

const dims = [
  { dim: "◻️2d", pkg: "@semio-tech/puzzle-2d", pj: `${PZ}/◻️2d/📦️packages/🟦️typescript/package.json`, pre: "puzzle2d" },
  { dim: "🧊️3d", pkg: "@semio-tech/puzzle-3d-ts", pj: `${PZ}/🧊️3d/📦️packages/🟦️typescript/package.json`, pre: "puzzle3d" },
  { dim: "🖐️5d", pkg: "@semio-tech/puzzle-5d-ts", pj: `${PZ}/🖐️5d/📦️packages/🟦️typescript/package.json`, pre: "puzzle5d" },
];
const examples = [
  { dir: "🌲️concrete-forest", asset: "🌲️forest", slug: "concrete-forest", aslug: "forest", name: "ConcreteForestDsl", dims: ["◻️2d", "🧊️3d", "🖐️5d"] },
  { dir: "🏗️nakagin-capsule-tower", asset: "🏢️tower", slug: "nakagin-capsule-tower", aslug: "tower", name: "NakaginCapsuleTowerDsl", dims: ["◻️2d", "🧊️3d", "🖐️5d"] },
  { dir: "🌙️capsule-dream", asset: "🌙️dream", slug: "capsule-dream", aslug: "dream", name: "CapsuleDreamDsl", dims: ["🖐️5d"] },
];
const d5 = dims[2]!;
const manifest5 = { name: d5.pkg, private: true, type: "module", sideEffects: false, exports: {} as Record<string, string>, license: "LGPL-3.0-or-later", bundleKind: "library" };
for (const d of dims) for (const e of examples) if (e.dims.includes(d.dim)) {
  const key = `./examples/${e.slug}/assets/${e.aslug}`, value = `./📚️examples/${e.dir}/🖼️assets/${e.asset}/🟦️.ts`;
  if (d.dim === "🖐️5d") manifest5.exports[key] = value; else addKey(d.pj, key, value, `typed DSL text asset of ${d.pkg} for the puzzle stories`);
}
plan.newFile({ path: d5.pj, content: JSON.stringify(manifest5, null, 2) + "\n", why: "puzzle 5d artifact had no TypeScript package; the typed example assets need an owner", wave: WAVE });

const stories: { dir: string; dim: string; imports: [string, string, string][] }[] = [
  { dir: "🎭️2d-fixtures", dim: "◻️2d", imports: [["concreteForestFixtureDsl", "🌲️concrete-forest", "🌲️forest"], ["nakaginCapsuleTowerFixtureDsl", "🏗️nakagin-capsule-tower", "🏢️tower"]] },
  { dir: "🎭️3d-world", dim: "🧊️3d", imports: [["concreteForestFixtureDsl", "🌲️concrete-forest", "🌲️forest"], ["nakaginCapsuleTowerFixtureDsl", "🏗️nakagin-capsule-tower", "🏢️tower"]] },
  { dir: "🎭️5d-timeline", dim: "🖐️5d", imports: [["concreteForestFixtureDsl", "🌲️concrete-forest", "🌲️forest"], ["nakaginCapsuleTowerFixtureDsl", "🏗️nakagin-capsule-tower", "🏢️tower"], ["capsuleDreamFixtureDsl", "🌙️capsule-dream", "🌙️dream"]] },
];
for (const s of stories) {
  const file = `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/${s.dir}/🧪️.story.tsx`;
  const d = dims.find(x => x.dim === s.dim)!;
  for (const [local, exDir, asset] of s.imports) {
    const e = examples.find(x => x.dir === exDir)!;
    const spec = `../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/${s.dim}/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/${exDir}/🖼️assets/${asset}/🗣️.dsl.semio?raw`;
    rewrite(file, `import ${local} from "${spec}";`, `import { ${d.pre}${e.name} as ${local} } from "${d.pkg}/examples/${e.slug}/assets/${e.aslug}";`, "typed asset module of the owning puzzle artifact package");
    declareDep(file, d.pkg, "story imports the typed DSL asset of this puzzle artifact", "devDependencies");
  }
}

const A = "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/package.json";
addKey(A, "./logos/emblem/dark-round", "./🪧️logos/🛡️emblem/🌘️dark-round/🟦️.ts", "typed url asset of the dark round emblem for the teaching course");
const courseFile = "🎓️teaching/🏛️architecture/📚️course/🟦️.ts";
rewrite(courseFile, `import logo from "../../../🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg?url";`, `import { darkRoundEmblemSvgUrl as logo } from "@semio-tech/assets/logos/emblem/dark-round";`, "typed url asset module of the assets package");
declareDep(courseFile, "@semio-tech/assets", "course entry imports the emblem url asset", "dependencies");

const S = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json";
addKey(S, "./palette/text", "./🎨️palette/🟦️.ts", "typed stylesheet text of the palette for contrast laws");
addKey(S, "./chrome/text", "./🖌️ui/🟦️.ts", "typed stylesheet text of the ui chrome for contrast laws");
const C = "🧰️framework/🛍️products/📚️course/🧪️tests";
rewrite(`${C}/🌗️contrast-states/🟦️.tsx`, `import palette from "../../../../🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css?raw";`, `import { paletteCssText as palette } from "@semio-tech/ui-styling/palette/text";`, "typed stylesheet text module of ui-styling");
rewrite(`${C}/🌗️contrast-states/🟦️.tsx`, `import chrome from "../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css?raw";`, `import { chromeCssText as chrome } from "@semio-tech/ui-styling/chrome/text";`, "typed stylesheet text module of ui-styling");
rewrite(`${C}/💭️crowd-client/🟦️.tsx`, `import palette from "../../../../🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css?raw";`, `import { paletteCssText as palette } from "@semio-tech/ui-styling/palette/text";`, "typed stylesheet text module of ui-styling");
declareDep(`${C}/🌗️contrast-states/🟦️.tsx`, "@semio-tech/ui-styling", "course contrast law reads the palette and chrome text", "devDependencies");

const W = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/package.json";
addKey(W, "./testing/source-text/host-io", "./🧪️testing/📖️source-text/🚪️host-io/🟦️.ts", "typed source text of the wgpu host-io module for the renderer parity law");
addKey(W, "./testing/source-text/renderer-boot", "./🧪️testing/📖️source-text/🎬️renderer-boot/🟦️.ts", "typed source text of the wgpu renderer-boot module for the renderer parity law");
const NG = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/package.json";
addKey(NG, "./testing/source-text/wasm-graph-surface", "./🧪️testing/📖️source-text/🧭️wasm-graph-surface/🟦️.ts", "typed source text of the wasm graph surface element for the pointer-cancellation law");
const UI = "🧰️framework/🔨️modules/🖱️ui";
const oldSourceText = `${UI}/🎯️targets/⚛️react/📦️packages/🟦️typescript/🌐️i18n/📖️source-text/🟦️.ts`;
plan.move(oldSourceText, `${UI}/🌐️i18n/📖️source-text/🟦️.ts`, WAVE, "the typed source text lives with the i18n bundles it reads: the ui owns them since the i18n moved from the react target into the ui");
rewrite(oldSourceText, "`@semio-tech/ui-react/i18n/source-text`: the ui-react i18n bundle module source text", "`@semio-tech/ui/i18n/source-text`: the ui i18n bundle module source text", "docstring follows the owning package");
rewrite(oldSourceText, '"../../../../../../🌐️i18n/🟦️.ts?raw"', '"../🟦️.ts?raw"', "the typed source text sits beside the bundle it reads");
const RETAINED = "🧬️contract/🧵️retained";
plan.newFile({
  path: `${UI}/${RETAINED}/🧪️testing/📖️source-text/🟦️.ts`,
  content: `/** 📖️ Public entry \`@semio-tech/ui-contract/testing/source-text/retained-fixtures\`: typed loaders of the retained resident and owned scene JSON fixture texts for the ui fixture corpus. */\n/// <reference types="vite/client" />\nexport const loadRetainedOwnedSceneFixtureText = (): Promise<string> => import("../../🧪️tests/🧫️fixtures/🎬️owned-scene/🔣️.json?raw").then(module => module.default);\nexport const loadRetainedResidentReaderFixtureText = (): Promise<string> => import("../../💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json?raw").then(module => module.default);\nexport const loadRetainedResidentPageFixtureText = (): Promise<string> => import("../../💾️resident/📃️page/🧪️tests/🧫️fixtures/🔣️.json?raw").then(module => module.default);\n`,
  why: "the ui fixture corpus read the ui-contract JSON fixtures by relative ?raw imports across the package boundary", wave: WAVE,
});
const uiFixtures = `${UI}/🧪️tests/🧫️fixtures/🟦️.ts`;
const fixtureText = read(uiFixtures);
const loaderLines = fixtureText.split("\n").filter(l => /^export const loadRetained(OwnedScene|ResidentReader|ResidentPage)FixtureText = /u.test(l));
if (loaderLines.length !== 3) throw Error("ui fixture loaders changed");
rewrite(uiFixtures, loaderLines.join("\n"), 'export { loadRetainedOwnedSceneFixtureText, loadRetainedResidentPageFixtureText, loadRetainedResidentReaderFixtureText } from "@semio-tech/ui-contract/testing/source-text/retained-fixtures";', "the loaders live with the fixtures they read");
const T = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests";
rewrite(`${T}/🔁️refresh-coalescing/🟦️.ts`, `const { default: hostIo } = await import("../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts?raw");`, `const { wgpuHostIoSourceText: hostIo } = await import("@semio-tech/framework-renderer-wgpu/testing/source-text/host-io");`, "typed source text module of the wgpu package");
rewrite(`${T}/🛑️scene-pointer-cancellation/🟦️.ts`, `import nodeGraphSource from "../../🧱️elements/🕸️NodeGraph/🧭️wasm-graph-surface/🟦️.tsx?raw";`, `import { wasmGraphSurfaceSourceText as nodeGraphSource } from "@semio-tech/framework-os-node-graph-rs/testing/source-text/wasm-graph-surface";`, "typed source text module of the node graph package");
rewrite(`${T}/🩺️window-fault/🟦️.ts`, /import uiBundleSource from "[^"]*🌐️i18n\/🟦️\.ts\?raw";/u, `import { uiI18nBundleSourceText as uiBundleSource } from "@semio-tech/ui/i18n/source-text";`, "typed source text module of the ui");
rewrite(`${T}/🪆️embedded-mount/🟦️.ts`, `import wgpuBootSource from "../../🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts?raw";`, `import { wgpuRendererBootSourceText as wgpuBootSource } from "@semio-tech/framework-renderer-wgpu/testing/source-text/renderer-boot";`, "typed source text module of the wgpu package");
const tf = `${T}/🔁️refresh-coalescing/🟦️.ts`;
declareDep(tf, "@semio-tech/framework-renderer-wgpu", "renderer parity laws read the wgpu source text modules", "devDependencies");
declareDep(`${T}/🛑️scene-pointer-cancellation/🟦️.ts`, "@semio-tech/framework-os-node-graph-rs", "pointer law reads the node graph source text module", "devDependencies");
declareDep(`${T}/🩺️window-fault/🟦️.ts`, "@semio-tech/ui", "window-fault law reads the ui i18n source text module", "devDependencies");

finish(plan, "w1", process.argv);

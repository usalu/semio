/** 🎛️ D7: the number facets of an action argument are a ui-contract projection; they leave the manifest so the manifest stops importing the ui contract. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";
import { ROOT } from "./lib.ts";
const W = "d7-action-arg-facets";
const M = "🧰️framework/🔨️modules";
const SRC = `${M}/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts`;
const DST = `${M}/🖱️ui/🧬️contract/🎛️action-arg-facets/🟦️.ts`;
const text = readFileSync(resolve(ROOT, SRC), "utf8");
const start = text.indexOf("//#region 🔖️ActionArgFacets");
const endMark = "//#endregion 🔖️ActionArgFacets\n";
const end = text.indexOf(endMark) + endMark.length;
if (start < 0 || end < endMark.length) throw Error("region not found");
const region = text.slice(start, end);
const body = region.replace("//#region 🔖️ActionArgFacets\n", "").replace(endMark, "").trimEnd() + "\n";
const content = `/** 🎛️ The number facets of an action argument as the UI contract needs them: the one mapping every renderer shares, projected from the manifest's argument definition. */
import { type ShellLocale } from "@semio-tech/framework/manifest/generated/ui-axes";
import { argControl, type ActionArgDef } from "../../../🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";
import type { SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale } from "../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";
import { UI_NUMBER_PRECISION_MAX, uiNumberCrossedBound, uiNumberDisplayText } from "../🧩️component/🟦️.ts";

${body}`;
const plan = new Plan();
plan.newFile({ path: DST, content, wave: W, why: "ui-contract projection of the manifest argument definition" });
plan.edit({ file: SRC, from: region, to: "", mode: "text", wave: W, why: "the facets move to the ui contract" });
plan.edit({ file: SRC, from: `import { UI_NUMBER_PRECISION_MAX, uiNumberCrossedBound, uiNumberDisplayText } from "../../🖱️ui/🧬️contract/🧩️component/🟦️.ts";\n`, to: "", mode: "text", wave: W, why: "the manifest no longer imports the ui contract" });
plan.edit({ file: SRC, from: `import type { SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale } from "../🏷️ui-contract-and-canvas-types/🟦️.ts";\n`, to: "", mode: "text", wave: W, why: "the facet types moved with the facets" });
plan.edit({ file: "🧰️framework/📦️packages/🟦️typescript/🟦️.ts", from: `export * from "../../🔨️modules/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";\n`, to: `export * from "../../🔨️modules/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";\nexport * from "../../🔨️modules/🖱️ui/🧬️contract/🎛️action-arg-facets/🟦️.ts";\n`, mode: "text", wave: W, why: "the root barrel keeps exporting the facets" });
const T = `${M}/🛂️manifest/🧪️tests/🧪️number-facets/🟦️.ts`;
plan.edit({ file: T, from: `import { actionArgNumberFacets, actionArgUnitSymbol, type ActionArgDef, type ActionArgNumberFacets } from "../../🎬️action-types-and-semantics/🟦️.ts";`, to: `import type { ActionArgDef } from "../../🎬️action-types-and-semantics/🟦️.ts";\nimport { actionArgNumberFacets, actionArgUnitSymbol, type ActionArgNumberFacets } from "../../../🖱️ui/🧬️contract/🎛️action-arg-facets/🟦️.ts";`, mode: "text", wave: W, why: "the facets live in the ui contract now" });
finish(plan, "d7", process.argv);

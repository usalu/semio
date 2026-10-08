import brepPrimitive from "./🔣️brep-primitive.json";
import brepCurve from "./🔣️brep-curve.json";
import brepSurface from "./🔣️brep-surface.json";
import brepSolid from "./🔣️brep-solid.json";
import brepBoolean from "./🔣️brep-boolean.json";
import brepFeature from "./🔣️brep-feature.json";
import brepTransform from "./🔣️brep-transform.json";
import brepIntersect from "./🔣️brep-intersect.json";
import brepEvaluate from "./🔣️brep-evaluate.json";
import brepTopology from "./🔣️brep-topology.json";
import brepInterchange from "./🔣️brep-interchange.json";
import meshPrimitive from "./🔣️mesh-primitive.json";
import meshConvert from "./🔣️mesh-convert.json";
import meshTransform from "./🔣️mesh-transform.json";
import meshComponent from "./🔣️mesh-component.json";
import meshEdit from "./🔣️mesh-edit.json";
import meshRepair from "./🔣️mesh-repair.json";
import meshInspect from "./🔣️mesh-inspect.json";
import meshInterchange from "./🔣️mesh-interchange.json";
import meshShading from "./🔣️mesh-shading.json";
import meshUv from "./🔣️mesh-uv.json";
import analysisMeasure from "./🔣️analysis-measure.json";
import analysisCheck from "./🔣️analysis-check.json";
import mathValues from "./🔣️math-values.json";
import mathArithmetic from "./🔣️math-arithmetic.json";
import mathVector from "./🔣️math-vector.json";
import mathList from "./🔣️math-list.json";

/** 🗣️ A user-facing string in every supported language, English first and German second. */
export type CatalogueText = { readonly en: string; readonly de: string };

/** 🏷️ The fidelity of a kind: the kernel's operation quality for B-Rep kinds, the mesh fidelity for mesh kinds. */
export type CatalogueQuality = "exact-analytic" | "exact-numerical" | "approximate" | "mesh-derived-brep" | "polygon-mesh" | "tessellated-mesh";

/** 🔌️ The value type a port carries. */
export type CataloguePortType = "number" | "integer" | "angle" | "length" | "boolean" | "text" | "enum" | "vector" | "point" | "plane" | "shape" | "shapes" | "mesh" | "selection" | "any";

/** 🧱️ The topological kind of a B-Rep shape. */
export type CatalogueShapeKind = "solid" | "shell" | "face" | "wire" | "edge" | "curve" | "surface" | "vertex" | "compound";

/** 🧲️ The sub-elements a selection port holds, taken from the shape or mesh on its source port; `mode` follows the enum port named by `modeFrom`. */
export type CatalogueSelection = { readonly component: "face" | "edge" | "vertex" | "mode"; readonly source: string; readonly multiple: boolean; readonly modeFrom?: string };

/** 🔘️ One choice of an enum port. */
export type CatalogueOption = { readonly value: string; readonly label: CatalogueText };

/** 🔌️ One input or output of a widget kind, with everything an inspector control needs. */
export type CataloguePort = {
  readonly name: string;
  readonly label: CatalogueText;
  readonly description: CatalogueText;
  readonly type: CataloguePortType;
  readonly shapeKinds?: readonly CatalogueShapeKind[];
  readonly selection?: CatalogueSelection;
  readonly list?: boolean;
  readonly minItems?: number;
  readonly maxItems?: number;
  readonly default?: unknown;
  readonly min?: number;
  readonly exclusiveMin?: boolean;
  readonly max?: number;
  readonly step?: number;
  readonly unit?: string;
  readonly options?: readonly CatalogueOption[];
  readonly optional?: boolean;
};

/** 👆️ A picked component that seeds a port when the kind is applied to the pick. */
export type CataloguePick = { readonly component: "shape" | "mesh" | "face" | "edge" | "vertex"; readonly port: string };

/** 🧭️ A gumball motion mapped to the port it drives, with the ports that fix its axis and origin. */
export type CatalogueGumball = { readonly motion: "translate" | "rotate" | "scale"; readonly port: string; readonly axisPort?: string; readonly originPort?: string; readonly along?: "normal" };

/** 🕹️ How a kind reacts to viewport picks and gumball motions. */
export type CatalogueInteraction = { readonly pick?: readonly CataloguePick[]; readonly gumball?: readonly CatalogueGumball[] };

/** 🧩️ One geometry widget kind: its identity, texts, typed ports, fidelity and interaction hints. */
export type CatalogueKind = {
  readonly id: string;
  readonly category: string;
  readonly emoji: string;
  readonly label: CatalogueText;
  readonly description: CatalogueText;
  readonly inputs: readonly CataloguePort[];
  readonly outputs: readonly CataloguePort[];
  readonly quality: CatalogueQuality;
  readonly interaction?: CatalogueInteraction;
  readonly preview: boolean;
};

/** 🗂️ A palette category with its sort position and texts. */
export type CatalogueCategory = { readonly id: string; readonly emoji: string; readonly order: number; readonly label: CatalogueText; readonly description: CatalogueText };

/** 📄️ One category file: the category and every kind it owns. */
export type CatalogueCategoryFile = { readonly category: CatalogueCategory; readonly kinds: readonly CatalogueKind[] };

/** 📚️ Every bundled category file, ordered by palette position. */
export const CATALOGUE_CATEGORY_FILES: readonly CatalogueCategoryFile[] = ([
  brepPrimitive, brepCurve, brepSurface, brepSolid, brepBoolean, brepFeature, brepTransform, brepIntersect, brepEvaluate, brepTopology, brepInterchange,
  meshPrimitive, meshConvert, meshTransform, meshComponent, meshEdit, meshRepair, meshInspect, meshInterchange, meshShading, meshUv,
  analysisMeasure, analysisCheck, mathValues, mathArithmetic, mathVector, mathList,
] as unknown as CatalogueCategoryFile[]).sort((left, right) => left.category.order - right.category.order || (left.category.id < right.category.id ? -1 : 1));

/** 📦️ The catalogue: category files in palette order with lookups by kind id and port name. */
export type Catalogue = {
  readonly categories: readonly CatalogueCategoryFile[];
  readonly kinds: readonly CatalogueKind[];
  readonly kind: (id: string) => CatalogueKind | undefined;
  readonly category: (id: string) => CatalogueCategoryFile | undefined;
  readonly port: (kindId: string, name: string) => CataloguePort | undefined;
};

/** 🏗️ Indexes category files; a duplicate kind id is refused. */
export function buildCatalogue(files: readonly CatalogueCategoryFile[]): Catalogue {
  const index = new Map<string, CatalogueKind>();
  for (const file of files) for (const kind of file.kinds) {
    if (index.has(kind.id)) throw new Error(`geometry catalogue: duplicate kind id ${kind.id}`);
    index.set(kind.id, kind);
  }
  const byCategory = new Map(files.map(file => [file.category.id, file]));
  return {
    categories: files,
    kinds: [...index.values()],
    kind: id => index.get(id),
    category: id => byCategory.get(id),
    port: (kindId, name) => { const kind = index.get(kindId); return kind?.inputs.find(port => port.name === name) ?? kind?.outputs.find(port => port.name === name); },
  };
}

let bundled: Catalogue | undefined;

/** 📦️ The process-wide bundled catalogue, built once. */
export function catalogue(): Catalogue {
  return bundled ??= buildCatalogue(CATALOGUE_CATEGORY_FILES);
}

/** 🌐️ The text for a BCP 47 locale such as `de` or `de-CH`; an unsupported language reads as the first language, English. */
export function resolveCatalogueText(text: CatalogueText, locale: string): string {
  return locale.split(/[-_]/)[0]?.toLowerCase() === "de" ? text.de : text.en;
}

//#region 🔖️Findings
/** 🔎️ One violated catalogue law: its code, the kind or category that owns it and the port it concerns. */
export type CatalogueFinding = { readonly code: string; readonly owner: string; readonly port?: string };

const blank = (text: CatalogueText): boolean => text.en.trim() === "" || text.de.trim() === "";
const identifier = (name: string): boolean => /^[a-z][A-Za-z0-9]*$/.test(name);
const isNumber = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);
const isWhole = (value: unknown): boolean => isNumber(value) && Number.isInteger(value);
const isTriple = (value: unknown): boolean => Array.isArray(value) && value.length === 3 && value.every(isNumber);
const NUMERIC: readonly CataloguePortType[] = ["number", "integer", "angle", "length"];

function idMatches(category: string, id: string): boolean {
  const namespace = category.split(".")[0] ?? "";
  const prefix = namespace === "brep" || namespace === "mesh" ? category : namespace === "math" || namespace === "analysis" ? namespace : undefined;
  return prefix !== undefined && id.startsWith(`${prefix}.`) && identifier(id.slice(prefix.length + 1));
}

function elementForm(type: CataloguePortType, value: unknown): boolean {
  switch (type) {
    case "number": case "length": case "angle": return isNumber(value);
    case "integer": return isWhole(value);
    case "boolean": return typeof value === "boolean";
    case "text": case "enum": return typeof value === "string";
    case "vector": case "point": return isTriple(value);
    case "plane": { const keys = value !== null && typeof value === "object" && !Array.isArray(value) ? Object.keys(value) : []; const plane = value as Record<string, unknown>; return keys.length === 2 && isTriple(plane.origin) && isTriple(plane.normal); }
    default: return false;
  }
}

const selectionForm = (value: unknown): boolean => Array.isArray(value) && value.every(item => typeof item === "string" || (isWhole(item) && (item as number) >= 0));

function inRange(port: CataloguePort, value: number): boolean {
  const above = port.min === undefined ? true : port.exclusiveMin ? value > port.min : value >= port.min;
  return above && (port.max === undefined || value <= port.max);
}

function checkTexts(out: CatalogueFinding[], owner: string, port: string | undefined, label: CatalogueText, description: CatalogueText): void {
  if (blank(label) || blank(description)) out.push({ code: "text-missing", owner, port });
  else if (description.en === description.de) out.push({ code: "text-identical", owner, port });
}

function checkDefault(out: CatalogueFinding[], owner: string, port: CataloguePort, value: unknown): void {
  const at = port.name;
  if (port.type === "selection") {
    if (!selectionForm(value)) { out.push({ code: "default-type-mismatch", owner, port: at }); return; }
    const items = value as unknown[];
    if (items.length > 0 && port.minItems !== undefined && items.length < port.minItems || port.maxItems !== undefined && items.length > port.maxItems) out.push({ code: "default-length-out-of-range", owner, port: at });
    return;
  }
  let elements: unknown[] = [value];
  if (port.list) {
    if (!Array.isArray(value)) { out.push({ code: "default-type-mismatch", owner, port: at }); return; }
    if (port.minItems !== undefined && value.length < port.minItems || port.maxItems !== undefined && value.length > port.maxItems) out.push({ code: "default-length-out-of-range", owner, port: at });
    elements = value;
  }
  if (elements.some(element => !elementForm(port.type, element))) { out.push({ code: "default-type-mismatch", owner, port: at }); return; }
  if (NUMERIC.includes(port.type) && elements.some(element => !inRange(port, element as number))) out.push({ code: "default-out-of-range", owner, port: at });
  if (port.type === "enum" && !(port.options ?? []).some(option => option.value === value)) out.push({ code: "enum-default-not-option", owner, port: at });
}

function checkSelection(out: CatalogueFinding[], kind: CatalogueKind, port: CataloguePort): void {
  const at = port.name;
  const selection = port.selection;
  if (selection === undefined) { if (port.type === "selection") out.push({ code: "selection-source-invalid", owner: kind.id, port: at }); return; }
  if (port.type !== "selection") { out.push({ code: "selection-source-invalid", owner: kind.id, port: at }); return; }
  const source = kind.inputs.find(candidate => candidate.name === selection.source);
  if (source === undefined || source.name === port.name || !["shape", "shapes", "mesh"].includes(source.type)) out.push({ code: "selection-source-invalid", owner: kind.id, port: at });
  const mode = selection.modeFrom === undefined ? undefined : kind.inputs.find(candidate => candidate.name === selection.modeFrom);
  const modeOk = selection.component === "mode"
    ? mode !== undefined && mode.type === "enum" && (mode.options ?? []).every(option => ["vertex", "edge", "face"].includes(option.value))
    : selection.modeFrom === undefined;
  if (!modeOk) out.push({ code: "selection-mode-invalid", owner: kind.id, port: at });
  if (!selection.multiple && port.maxItems !== 1) out.push({ code: "selection-multiplicity-mismatch", owner: kind.id, port: at });
}

function checkPort(out: CatalogueFinding[], kind: CatalogueKind, port: CataloguePort, input: boolean): void {
  const owner = kind.id, at = port.name, type = port.type;
  checkTexts(out, owner, at, port.label, port.description);
  const shaped = type === "shape" || type === "shapes";
  if (port.shapeKinds !== undefined && (!shaped || port.shapeKinds.length === 0)) out.push({ code: "shape-kinds-misplaced", owner, port: at });
  if (port.list && ["shape", "shapes", "mesh", "selection", "enum"].includes(type)) out.push({ code: "list-misplaced", owner, port: at });
  const counted = port.list === true || type === "shapes" || type === "selection";
  const counts = port.minItems !== undefined || port.maxItems !== undefined;
  if ((!counted && counts) || (port.minItems !== undefined && port.maxItems !== undefined && port.minItems > port.maxItems)) out.push({ code: "item-bounds-misplaced", owner, port: at });
  const constrained = port.min !== undefined || port.max !== undefined || port.step !== undefined || port.exclusiveMin === true;
  if (constrained && (!NUMERIC.includes(type) || !input)) out.push({ code: "constraint-misplaced", owner, port: at });
  const inverted = port.min !== undefined && port.max !== undefined && (port.min > port.max || (port.min === port.max && port.exclusiveMin === true));
  if (inverted || (port.step !== undefined && !(port.step > 0 && Number.isFinite(port.step))) || (port.exclusiveMin === true && port.min === undefined)) out.push({ code: "bounds-inverted", owner, port: at });
  const options = port.options ?? [];
  if (options.some(option => blank(option.label))) out.push({ code: "text-missing", owner, port: at });
  if ((type === "enum") !== (port.options !== undefined) || (type === "enum" && (options.length < 2 || new Set(options.map(option => option.value)).size !== options.length))) out.push({ code: "enum-options-invalid", owner, port: at });
  if (!input) {
    if (port.default !== undefined) out.push({ code: "default-misplaced", owner, port: at });
    if (port.selection !== undefined) out.push({ code: "selection-on-output", owner, port: at });
    return;
  }
  const valueless = type === "shape" || type === "shapes" || type === "mesh";
  if (port.default !== undefined) {
    if (valueless) out.push({ code: "default-misplaced", owner, port: at });
    else checkDefault(out, owner, port, port.default);
  } else if (!valueless && port.optional !== true) out.push({ code: "default-missing", owner, port: at });
  checkSelection(out, kind, port);
}

function selectionAccepts(kind: CatalogueKind, port: CataloguePort, component: CataloguePick["component"]): boolean {
  if (component === "shape") return port.type === "shape" || port.type === "shapes";
  if (component === "mesh") return port.type === "mesh";
  if (port.type === "selection" && port.selection !== undefined) {
    if (port.selection.component === component) return true;
    const mode = port.selection.component === "mode" && port.selection.modeFrom !== undefined ? kind.inputs.find(candidate => candidate.name === port.selection?.modeFrom) : undefined;
    return mode?.options?.some(option => option.value === component) ?? false;
  }
  return (port.type === "shape" || port.type === "shapes") && (port.shapeKinds?.includes(component) ?? false);
}

function checkInteraction(out: CatalogueFinding[], kind: CatalogueKind): void {
  const owner = kind.id;
  const input = (name: string | undefined): CataloguePort | undefined => kind.inputs.find(port => port.name === name);
  for (const pick of kind.interaction?.pick ?? []) {
    const port = input(pick.port);
    if (port === undefined) out.push({ code: "pick-port-missing", owner, port: pick.port });
    else if (!selectionAccepts(kind, port, pick.component)) out.push({ code: "pick-component-mismatch", owner, port: pick.port });
  }
  for (const gumball of kind.interaction?.gumball ?? []) {
    const port = input(gumball.port);
    if (port === undefined) { out.push({ code: "gumball-port-missing", owner, port: gumball.port }); continue; }
    const accepted: readonly CataloguePortType[] = gumball.motion === "translate" ? ["vector", "point", "plane", "length", "number"] : gumball.motion === "rotate" ? ["angle", "plane"] : ["vector", "number", "length"];
    if (!accepted.includes(port.type)) out.push({ code: "gumball-type-mismatch", owner, port: port.name });
    const scalar = port.type === "length" || port.type === "number";
    if (gumball.motion === "translate" ? scalar !== (gumball.along !== undefined) : gumball.along !== undefined) out.push({ code: "gumball-along-invalid", owner, port: port.name });
    const axis = gumball.axisPort === undefined ? undefined : input(gumball.axisPort);
    const axisOk = gumball.motion === "rotate" && port.type === "angle" ? axis?.type === "vector" : gumball.axisPort === undefined;
    const origin = gumball.originPort === undefined ? undefined : input(gumball.originPort);
    const originOk = (gumball.motion === "rotate" && port.type === "angle") || gumball.motion === "scale" ? gumball.originPort === undefined || origin?.type === "point" : gumball.originPort === undefined;
    if (!axisOk || !originOk) out.push({ code: "gumball-axis-invalid", owner, port: port.name });
  }
}

function checkKind(out: CatalogueFinding[], file: CatalogueCategoryFile, kind: CatalogueKind): void {
  const owner = kind.id;
  if (kind.category !== file.category.id) out.push({ code: "kind-category-mismatch", owner });
  if (!idMatches(file.category.id, kind.id)) out.push({ code: "kind-id-malformed", owner });
  if (kind.emoji.trim() === "") out.push({ code: "text-missing", owner });
  checkTexts(out, owner, undefined, kind.label, kind.description);
  for (const [ports, input] of [[kind.inputs, true], [kind.outputs, false]] as const) {
    const seen = new Set<string>();
    const reported = new Set<string>();
    for (const port of ports) {
      if (seen.has(port.name) && !reported.has(port.name)) { out.push({ code: "duplicate-port-name", owner, port: port.name }); reported.add(port.name); }
      seen.add(port.name);
    }
    for (const port of ports) checkPort(out, kind, port, input);
  }
  if (kind.outputs.length === 0) out.push({ code: "no-outputs", owner });
  checkInteraction(out, kind);
}

/** 🔎️ Checks the catalogue laws the JSON schema cannot express: ids, texts, defaults within bounds, enum options, selection sources, interaction targets and unique emojis. An empty answer means the files are sound. */
export function checkCatalogue(files: readonly CatalogueCategoryFile[]): CatalogueFinding[] {
  const out: CatalogueFinding[] = [];
  const kindIds = new Set<string>(), categoryIds = new Set<string>();
  for (const file of files) {
    checkTexts(out, file.category.id, undefined, file.category.label, file.category.description);
    if (categoryIds.has(file.category.id)) out.push({ code: "duplicate-category-id", owner: file.category.id });
    categoryIds.add(file.category.id);
    const emojis = new Set<string>();
    for (const kind of file.kinds) {
      if (kindIds.has(kind.id)) out.push({ code: "duplicate-kind-id", owner: kind.id });
      kindIds.add(kind.id);
      if (emojis.has(kind.emoji)) out.push({ code: "emoji-duplicate", owner: kind.id });
      emojis.add(kind.emoji);
      checkKind(out, file, kind);
    }
  }
  return out;
}
//#endregion 🔖️Findings

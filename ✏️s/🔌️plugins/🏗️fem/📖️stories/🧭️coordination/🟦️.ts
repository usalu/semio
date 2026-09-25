/// <reference types="vite/client" />
// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🏗️fem/📖️stories/🧭️coordination/🟦️.ts
// Specs: The `🏗️fem` scope's story coordination: the story-local reader of the two fem DSL dialects and the scene projections, bilingual labels and command emulators built on it — everything the `🎭️*` stories need that is not itself a story.
// Specs: Story-local reader for the two `🏗️fem` DSL dialects (`fem.fem2d.dsl`, `fem.fem3d.dsl` — the
// `🗣️.dsl.semio` example assets under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/**/📚️examples/🎬️demo/🖼️assets/`).
// Summary: Shared by every `stories/fem/**` story file so the fixture data in the stories is the REAL
// shipped example document, not hand-authored story data — the same discipline `../block/dsl.ts` follows.
// fem's `📦️packages/🦀️rust` crate is a WASM *component*, not a `wasm-bindgen` module, so there is no
// browser-callable `parse_dsl` to reuse; this is a small TypeScript reader of the same text grammar and
// deliberately a READER only — it never re-emits DSL, so it can never drift into a second authority.
// Grammar (as emitted by `🧬️schema/📸️snapshot/📝️text/🦀️.rs` via the `dsl::DslArtifact` derive): a
// `semio <dialect> v<n>` banner, top-level `key=value` lines, `name { … }` statement blocks whose lines are
// each `<keyword> key=value …` records (`elements`) or a bare assignment run (`analysis`), and
// `name [col:TYPE …] { rows }` tables. fem rows go beyond block's flat rows in three ways this reader
// handles: quantity-suffixed scalars (`-4m`, `210000000000Pa`, `7850kg/m3`, `0.001m2`), a `BLOCK` column
// holding a nested multi-line `{ … }` statement run (`load-cases.loads`) and a `MAP` column holding a
// nested `{ k=v … }` run (fem3d `combinations.terms`) — so rows are read from a brace-aware TOKEN stream
// chunked by column count, not line by line.
//
// Specs: The `🏗️fem` scope's shared story fixtures, scene-node projections, window-render summaries,
// bilingual (en/de) labels and story-local command emulators — everything the
// `stories/fem/**/*.stories.tsx` files need that is NOT itself a story.
// Summary: Lives beside the story files rather than inside them because Storybook's CSF indexer treats
// EVERY named export of a `*.stories.*` module as a story, so a shared helper exported from a story file
// would be indexed as a broken story (same split `../block/scene.ts` uses). The real shipped example
// documents (`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, one per artifact) are `?raw`-imported here and
// parsed by `./dsl.ts`; the scene projections are line-for-line ports of the windows' own Rust builders —
// `fem2d_structure_layers`/`screen_2d`/`vector_layer`
// (`🗿️artifacts/◻️2d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`) and
// `fem3d_structural_instances`/`quat_z_to`/`quat_roll_z`/`quat_mul`/`mesh_box`
// (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` + `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`).
// Two halves of those windows are deliberately NOT reproduced because they are solver/mesher output the
// browser cannot compute without fem's plugin wasm: fem2d's `mesh-edge-*` overlay
// (`fem2d_region_triangles` → `fem2d_mesh_preview`) and fem3d's `solid-*` boundary meshes
// (`fem3d_solid_mesh_entries` → `fem3d_mesh_preview`), plus every results-window layer derived from
// `fem2d_solve_all`/`fem3d_solve_all`. Each is reported as a counted omission in the story's debug
// readout rather than faked — see `femStoryOmissions`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { parseViewport2d, type Viewport2d } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
import fem2dDemoDsl from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";
import fem3dDemoDsl from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";

//#region 🔖️Model
/** @emoji 🗂️ One parsed fem DSL document: the banner dialect/version, top-level scalars, `name { … }` statement blocks and `name [cols] { rows }` tables. */
export type FemDslDocument = {
  readonly dialect: string;
  readonly version: string;
  readonly scalars: Readonly<Record<string, string>>;
  readonly blocks: Readonly<Record<string, readonly FemDslStatement[]>>;
  readonly tables: Readonly<Record<string, readonly Readonly<Record<string, unknown>>[]>>;
};

/** @emoji 📄️ One line of a statement block: an optional leading keyword (`beam`, `nodal`, `area`, …) plus its `key=value` fields. */
export type FemDslStatement = { readonly keyword: string | undefined; readonly fields: Readonly<Record<string, unknown>> };
//#endregion 🔖️Model

//#region 🔖️Tokenizer
const CLOSERS: Readonly<Record<string, string>> = { "[": "]", "{": "}" };

/** @emoji ✂️ Splits DSL text into column tokens: a quoted string (re-emitted as JSON so the reader can tell it from a bare word), a balanced `[ … ]` or `{ … }` group (newlines included), or a bare word. Never splits inside quotes or a group. */
function tokenize(text: string): string[] {
  const tokens: string[] = [];
  let index = 0;
  while (index < text.length) {
    const char = text[index]!;
    if (char === " " || char === "\t" || char === "\n" || char === "\r") {
      index += 1;
      continue;
    }
    if (char === '"') {
      let end = index + 1;
      let value = "";
      while (end < text.length && text[end] !== '"') {
        if (text[end] === "\\" && end + 1 < text.length) {
          value += text[end + 1];
          end += 2;
          continue;
        }
        value += text[end];
        end += 1;
      }
      tokens.push(JSON.stringify(value));
      index = end + 1;
      continue;
    }
    const closer = CLOSERS[char];
    if (closer !== undefined) {
      let depth = 0;
      let end = index;
      while (end < text.length) {
        if (text[end] === char) depth += 1;
        else if (text[end] === closer) {
          depth -= 1;
          if (depth === 0) break;
        }
        end += 1;
      }
      tokens.push(text.slice(index, end + 1));
      index = end + 1;
      continue;
    }
    let end = index;
    while (end < text.length && !" \t\n\r".includes(text[end]!)) end += 1;
    tokens.push(text.slice(index, end));
    index = end;
  }
  return tokens;
}
//#endregion 🔖️Tokenizer

//#region 🔖️Values
const NUMBER_PATTERN = /^[-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?$/;
const QUANTITY_PATTERN = /^([-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?)([A-Za-z°µ][A-Za-z0-9/^·]*)$/;
const COORDINATE_PATTERN = /^[-+]?\d+(?:\.\d+)?(?:,[-+]?\d+(?:\.\d+)?)+$/;
const TEXTUAL_COLUMN_TYPES: ReadonlySet<string> = new Set(["TEXT", "ID", "BOOL"]);

/** @emoji 🔤️ `some-key` → `someKey`, the record key every projection below reads. */
function camel(key: string): string {
  return key.replace(/-([a-z0-9])/g, (_, letter: string) => letter.toUpperCase());
}

/** @emoji 🧩️ One `key=value key=value …` run (possibly multi-line) as a typed record; quoted values keep their spaces. */
function parseAssignments(text: string): Record<string, unknown> {
  const record: Record<string, unknown> = {};
  for (const token of tokenize(text)) {
    const equals = token.indexOf("=");
    if (equals <= 0) continue;
    record[camel(token.slice(0, equals))] = coerceFemDslValue(token.slice(equals + 1));
  }
  return record;
}

/** @emoji 📄️ A `{ … }` statement run as one record per line — the shape `#[dsl(statements, block)]` fields (`FemLoadCase::loads`) and `elements { … }` both print. */
function parseStatements(body: string): FemDslStatement[] {
  const statements: FemDslStatement[] = [];
  for (const rawLine of body.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (line === "" || line.startsWith("#")) continue;
    const tokens = tokenize(line);
    if (tokens.length === 0) continue;
    const leading = tokens[0]!;
    const keyword = leading.includes("=") ? undefined : leading;
    const fields: Record<string, unknown> = {};
    for (const token of tokens.slice(keyword === undefined ? 0 : 1)) {
      const equals = token.indexOf("=");
      if (equals <= 0) continue;
      fields[camel(token.slice(0, equals))] = coerceFemDslValue(token.slice(equals + 1));
    }
    statements.push({ keyword, fields });
  }
  return statements;
}

/** @emoji 🔢️ Coerces one token to its typed value: `_` is the DSL's "absent" marker, quantities drop their unit suffix (`-4m` → `-4`, `210000000000Pa` → `2.1e11`), `x,y` coordinate pairs become number tuples, `[ … ]` becomes a list and `{ … }` a MAP record or a statement list. A `TEXT`/`ID`/`BOOL` column never has its unit stripped, so an id like `chs76` stays a string. */
export function coerceFemDslValue(token: string, columnType?: string): unknown {
  if (token === "_") return undefined;
  if (token.startsWith('"')) return JSON.parse(token) as string;
  if (token.startsWith("[")) {
    const inner = token.slice(1, -1).trim();
    return inner === "" ? [] : tokenize(inner).map((entry) => coerceFemDslValue(entry));
  }
  if (token.startsWith("{")) {
    const inner = token.slice(1, -1);
    return columnType === "MAP" ? parseAssignments(inner) : parseStatements(inner);
  }
  if (token === "true") return true;
  if (token === "false") return false;
  if (NUMBER_PATTERN.test(token)) return Number(token);
  if (COORDINATE_PATTERN.test(token)) return token.split(",").map(Number);
  if (!TEXTUAL_COLUMN_TYPES.has(columnType ?? "")) {
    const quantity = QUANTITY_PATTERN.exec(token);
    if (quantity) return Number(quantity[1]);
  }
  return token;
}

/** @emoji 🔤️ Column header `name:TYPE` → the camelCased record key plus the declared type. */
function parseColumnHeader(header: string): { readonly key: string; readonly type: string } {
  const colon = header.lastIndexOf(":");
  return { key: camel(colon > 0 ? header.slice(0, colon) : header), type: colon > 0 ? header.slice(colon + 1) : "TEXT" };
}
//#endregion 🔖️Values

//#region 🔖️Parser
const TABLE_HEADER = /^([a-z0-9-]+)\s*\[([^\]]*)\]\s*\{$/;
const BLOCK_HEADER = /^([a-z0-9-]+)\s*\{$/;

/** @emoji 🧮️ Net brace depth a line contributes, ignoring braces inside a quoted string. */
function braceDelta(line: string): number {
  let delta = 0;
  let quoted = false;
  for (const char of line) {
    if (char === '"') quoted = !quoted;
    else if (!quoted && char === "{") delta += 1;
    else if (!quoted && char === "}") delta -= 1;
  }
  return delta;
}

/** @emoji 📖️ Parses one `🗣️.dsl.semio` fem document. Throws on a missing/foreign banner rather than returning a half-read document — a story rendering nothing is far harder to diagnose than a thrown fixture error. */
export function parseFemDsl(text: string): FemDslDocument {
  const lines = text.split(/\r?\n/);
  const banner = /^semio\s+(\S+)\s+v(\S+)\s*$/.exec(lines[0] ?? "");
  if (!banner) throw new Error(`[fem-dsl] missing "semio <dialect> v<n>" banner: ${JSON.stringify(lines[0] ?? "")}`);
  const scalars: Record<string, string> = {};
  const blocks: Record<string, FemDslStatement[]> = {};
  const tables: Record<string, Record<string, unknown>[]> = {};

  let index = 1;
  const readBody = (): string => {
    const body: string[] = [];
    let depth = 1;
    while (index < lines.length) {
      const line = lines[index] ?? "";
      index += 1;
      depth += braceDelta(line);
      if (depth === 0) break;
      body.push(line);
    }
    return body.join("\n");
  };

  while (index < lines.length) {
    const line = (lines[index] ?? "").trim();
    index += 1;
    if (line === "" || line.startsWith("#")) continue;

    const table = TABLE_HEADER.exec(line);
    if (table) {
      const columns = table[2]!.trim().split(/\s+/).filter(Boolean).map(parseColumnHeader);
      const tokens = tokenize(readBody());
      const rows: Record<string, unknown>[] = [];
      for (let offset = 0; offset + columns.length <= tokens.length; offset += columns.length) {
        const row: Record<string, unknown> = {};
        columns.forEach((column, position) => {
          row[column.key] = coerceFemDslValue(tokens[offset + position]!, column.type);
        });
        rows.push(row);
      }
      tables[table[1]!] = rows;
      continue;
    }

    const block = BLOCK_HEADER.exec(line);
    if (block) {
      blocks[block[1]!] = parseStatements(readBody());
      continue;
    }

    const equals = line.indexOf("=");
    if (equals > 0) scalars[line.slice(0, equals)] = line.slice(equals + 1);
  }

  return { dialect: banner[1]!, version: banner[2]!, scalars, blocks, tables };
}
//#endregion 🔖️Parser

//#region 🔖️SnapshotModel
export type FemVec2 = readonly [number, number];
export type FemSupport = { readonly id: string; readonly nodeId: string; readonly fixed: readonly string[] };
export type FemMaterial = { readonly id: string; readonly name: string; readonly e: number; readonly nu: number; readonly rho: number };
export type FemSection = { readonly id: string; readonly name: string; readonly area: number; readonly iy: number };
export type FemAnalysisSettings = { readonly modalCount: number; readonly bucklingCount: number; readonly deformationScale: number };

/** @emoji 🏋️ One applied load, flattened across `FemLoad`'s three variants (`◻️2d/🦀️.rs`'s `Nodal`/`MemberUdl`/`Area`). */
export type FemLoad = {
  readonly kind: string;
  readonly id: string;
  readonly nodeId?: string;
  readonly dof?: string;
  readonly value?: number;
  readonly elementId?: string;
  readonly wx?: number;
  readonly wy?: number;
  readonly regionId?: string;
  readonly solidId?: string;
  readonly pressure?: number;
};

export type FemLoadCase = { readonly id: string; readonly name: string; readonly loads: readonly FemLoad[]; readonly selfWeight: boolean };
export type FemCombination = { readonly id: string; readonly name: string };

export type Fem2dNode = { readonly id: string; readonly x: number; readonly y: number };
export type Fem2dElement = { readonly kind: string; readonly id: string; readonly start: string; readonly end: string; readonly materialId: string; readonly sectionId: string };
export type Fem2dRegion = { readonly id: string; readonly name: string; readonly outline: readonly FemVec2[]; readonly holes: readonly FemVec2[]; readonly thickness: number; readonly materialId: string; readonly meshSize: number };

/** @emoji ◻️ The subset of `Fem2dSnapshot` (`🗿️artifacts/◻️2d/🦀️.rs`) the 2D stories render. */
export type Fem2dSnapshot = {
  readonly nodes: readonly Fem2dNode[];
  readonly elements: readonly Fem2dElement[];
  readonly supports: readonly FemSupport[];
  readonly regions: readonly Fem2dRegion[];
  readonly materials: readonly FemMaterial[];
  readonly sections: readonly FemSection[];
  readonly loadCases: readonly FemLoadCase[];
  readonly combinations: readonly FemCombination[];
  readonly analysis: FemAnalysisSettings;
};

export type Fem3dNode = { readonly id: string; readonly x: number; readonly y: number; readonly z: number };
export type Fem3dElement = { readonly kind: string; readonly id: string; readonly start: string; readonly end: string; readonly materialId: string; readonly sectionId: string; readonly roll: number };
export type Fem3dSolid = { readonly id: string; readonly name: string; readonly outline: readonly FemVec2[]; readonly holes: readonly FemVec2[]; readonly baseZ: number; readonly height: number; readonly layers: number; readonly meshSize: number; readonly materialId: string; readonly axis: "x" | "y" | "z" };

/** @emoji 🧊️ The subset of `Fem3dSnapshot` (`🗿️artifacts/🧊️3d/🦀️.rs`) the 3D stories render. */
export type Fem3dSnapshot = {
  readonly nodes: readonly Fem3dNode[];
  readonly elements: readonly Fem3dElement[];
  readonly supports: readonly FemSupport[];
  readonly solids: readonly Fem3dSolid[];
  readonly materials: readonly FemMaterial[];
  readonly sections: readonly FemSection[];
  readonly loadCases: readonly FemLoadCase[];
  readonly combinations: readonly FemCombination[];
  readonly analysis: FemAnalysisSettings;
};
//#endregion 🔖️SnapshotModel

//#region 🔖️Projections
function text(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function num(value: unknown, fallback = 0): number {
  return typeof value === "number" ? value : fallback;
}

function flag(value: unknown, fallback = false): boolean {
  return typeof value === "boolean" ? value : fallback;
}

function strings(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.map((entry) => String(entry)) : [];
}

function polygon(value: unknown): readonly FemVec2[] {
  return Array.isArray(value) ? value.filter((entry): entry is number[] => Array.isArray(entry) && entry.length >= 2 && entry.every((n) => typeof n === "number")).map((entry) => [entry[0]!, entry[1]!] as const) : [];
}

function analysis(statements: readonly FemDslStatement[] | undefined): FemAnalysisSettings {
  const fields = statements?.[0]?.fields ?? {};
  return { modalCount: num(fields.modalCount, 3), bucklingCount: num(fields.bucklingCount, 3), deformationScale: num(fields.deformationScale, 1) };
}

function supports(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemSupport[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), nodeId: text(row.nodeId), fixed: strings(row.fixed) }));
}

function materials(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemMaterial[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), e: num(row.e), nu: num(row.nu), rho: num(row.rho) }));
}

function sections(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemSection[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), area: num(row.area), iy: num(row.iy) }));
}

function combinations(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemCombination[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name) }));
}

/** @emoji 🏋️ A `loads:BLOCK` cell's statement run → the flattened `FemLoad` rows the structure layers draw vectors for. The `member-udl` keyword is accepted in both its DSL (`member-udl`) and camelCase spellings. */
function loads(cell: unknown): readonly FemLoad[] {
  if (!Array.isArray(cell)) return [];
  return (cell as readonly FemDslStatement[]).map((statement) => {
    const keyword = statement.keyword ?? "";
    const fields = statement.fields;
    return {
      kind: keyword === "member-udl" ? "memberUdl" : keyword,
      id: text(fields.id),
      ...(fields.nodeId === undefined ? {} : { nodeId: text(fields.nodeId) }),
      ...(fields.dof === undefined ? {} : { dof: text(fields.dof) }),
      ...(fields.value === undefined ? {} : { value: num(fields.value) }),
      ...(fields.elementId === undefined ? {} : { elementId: text(fields.elementId) }),
      ...(fields.wx === undefined ? {} : { wx: num(fields.wx) }),
      ...(fields.wy === undefined ? {} : { wy: num(fields.wy) }),
      ...(fields.regionId === undefined ? {} : { regionId: text(fields.regionId) }),
      ...(fields.solidId === undefined ? {} : { solidId: text(fields.solidId) }),
      ...(fields.pressure === undefined ? {} : { pressure: num(fields.pressure) }),
    };
  });
}

function loadCases(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly FemLoadCase[] {
  return (rows ?? []).map((row) => ({ id: text(row.id), name: text(row.name), loads: loads(row.loads), selfWeight: flag(row.selfWeight) }));
}

/** @emoji ◻️ `fem.fem2d.dsl` → the `Fem2dSnapshot` slice the 2D stories render. */
export function parseFem2dDsl(dslText: string): Fem2dSnapshot {
  const doc = parseFemDsl(dslText);
  return {
    nodes: (doc.tables.nodes ?? []).map((row) => ({ id: text(row.id), x: num(row.x), y: num(row.y) })),
    elements: (doc.blocks.elements ?? []).map((statement) => ({
      kind: statement.keyword ?? "beam",
      id: text(statement.fields.id),
      start: text(statement.fields.start),
      end: text(statement.fields.end),
      materialId: text(statement.fields.materialId),
      sectionId: text(statement.fields.sectionId),
    })),
    supports: supports(doc.tables.supports),
    regions: (doc.tables.regions ?? []).map((row) => ({
      id: text(row.id),
      name: text(row.name),
      outline: polygon(row.outline),
      holes: polygon(row.holes),
      thickness: num(row.thickness),
      materialId: text(row.materialId),
      meshSize: num(row.meshSize, 1),
    })),
    materials: materials(doc.tables.materials),
    sections: sections(doc.tables.sections),
    loadCases: loadCases(doc.tables["load-cases"]),
    combinations: combinations(doc.tables.combinations),
    analysis: analysis(doc.blocks.analysis),
  };
}

/** @emoji 🧊️ `fem.fem3d.dsl` → the `Fem3dSnapshot` slice the 3D stories render. */
export function parseFem3dDsl(dslText: string): Fem3dSnapshot {
  const doc = parseFemDsl(dslText);
  return {
    nodes: (doc.tables.nodes ?? []).map((row) => ({ id: text(row.id), x: num(row.x), y: num(row.y), z: num(row.z) })),
    elements: (doc.blocks.elements ?? []).map((statement) => ({
      kind: statement.keyword ?? "frame",
      id: text(statement.fields.id),
      start: text(statement.fields.start),
      end: text(statement.fields.end),
      materialId: text(statement.fields.materialId),
      sectionId: text(statement.fields.sectionId),
      roll: num(statement.fields.roll),
    })),
    supports: supports(doc.tables.supports),
    solids: (doc.tables.solids ?? []).map((row) => ({
      id: text(row.id),
      name: text(row.name),
      outline: polygon(row.outline),
      holes: polygon(row.holes),
      baseZ: num(row.baseZ),
      height: num(row.height),
      layers: num(row.layers, 1),
      meshSize: num(row.meshSize, 1),
      materialId: text(row.materialId),
      axis: (text(row.axis) === "x" || text(row.axis) === "y" ? text(row.axis) : "z") as "x" | "y" | "z",
    })),
    materials: materials(doc.tables.materials),
    sections: sections(doc.tables.sections),
    loadCases: loadCases(doc.tables["load-cases"]),
    combinations: combinations(doc.tables.combinations),
    analysis: analysis(doc.blocks.analysis),
  };
}

/** @emoji 🕳️ `crate::artifacts::fem2d::schema::empty_fem2d_snapshot()`'s story twin — what `setActiveExample` loads for any id other than the bundled example's own. */
export const EMPTY_FEM2D_SNAPSHOT: Fem2dSnapshot = { nodes: [], elements: [], supports: [], regions: [], materials: [], sections: [], loadCases: [], combinations: [], analysis: { modalCount: 3, bucklingCount: 3, deformationScale: 1 } };

/** @emoji 🕳️ `Fem3dSnapshot::default()`'s story twin — what fem3d's `setActiveExample` loads for any id other than `"default"`. */
export const EMPTY_FEM3D_SNAPSHOT: Fem3dSnapshot = { nodes: [], elements: [], supports: [], solids: [], materials: [], sections: [], loadCases: [], combinations: [], analysis: { modalCount: 3, bucklingCount: 3, deformationScale: 1 } };
//#endregion 🔖️Projections

//#region 🔖️Locale
/** 🗣️ Explicit OS locale supplied to the story host; artifact commands do not mutate it. */
export type FemStoryLocale = "en-US" | "de-DE";

export const FEM_STORY_LOCALES: readonly FemStoryLocale[] = ["en-US", "de-DE"];

export type FemStoryTextKey =
  | "model"
  | "results"
  | "viewer"
  | "language"
  | "example"
  | "nodes"
  | "members"
  | "supports"
  | "regions"
  | "solids"
  | "materials"
  | "sections"
  | "loadCases"
  | "combinations"
  | "deformationScale"
  | "activeCase"
  | "noLoadCase"
  | "displayMode"
  | "addNode"
  | "loadExample"
  | "clearExample"
  | "omitted";

const FEM_STORY_TEXT: Readonly<Record<FemStoryTextKey, Readonly<Record<FemStoryLocale, string>>>> = {
  model: { "en-US": "Model", "de-DE": "Modell" },
  results: { "en-US": "Results", "de-DE": "Ergebnisse" },
  viewer: { "en-US": "Viewer", "de-DE": "Betrachter" },
  language: { "en-US": "Language", "de-DE": "Sprache" },
  example: { "en-US": "Example", "de-DE": "Beispiel" },
  nodes: { "en-US": "Nodes", "de-DE": "Knoten" },
  members: { "en-US": "Members", "de-DE": "Stäbe" },
  supports: { "en-US": "Supports", "de-DE": "Auflager" },
  regions: { "en-US": "Regions", "de-DE": "Flächen" },
  solids: { "en-US": "Solids", "de-DE": "Volumen" },
  materials: { "en-US": "Materials", "de-DE": "Materialien" },
  sections: { "en-US": "Sections", "de-DE": "Querschnitte" },
  loadCases: { "en-US": "Load cases", "de-DE": "Lastfälle" },
  combinations: { "en-US": "Combinations", "de-DE": "Kombinationen" },
  deformationScale: { "en-US": "Deformation scale", "de-DE": "Verformungsmaßstab" },
  activeCase: { "en-US": "Active case", "de-DE": "Aktiver Lastfall" },
  noLoadCase: { "en-US": "No load case defined", "de-DE": "Kein Lastfall definiert" },
  displayMode: { "en-US": "Display mode", "de-DE": "Anzeigemodus" },
  addNode: { "en-US": "Add node", "de-DE": "Knoten hinzufügen" },
  loadExample: { "en-US": "Load example", "de-DE": "Beispiel laden" },
  clearExample: { "en-US": "Clear document", "de-DE": "Dokument leeren" },
  omitted: { "en-US": "Not rendered here", "de-DE": "Hier nicht gezeichnet" },
};

/** @emoji 🗣️ One label in the requested locale. Throws on an unknown key so a typo surfaces at render time instead of printing `undefined` into the panel. */
export function femStoryLabel(key: FemStoryTextKey, locale: FemStoryLocale): string {
  const entry = FEM_STORY_TEXT[key];
  if (!entry) throw new Error(`[fem-story] unknown label key ${JSON.stringify(key)}`);
  return entry[locale];
}
//#endregion 🔖️Locale

//#region 🔖️Fixtures
/** @emoji 🎬️ The single `📚️examples/*` unit each fem subset registers (`🌐️any/🦀️.rs`'s `examples()`), keyed by the id its `ExampleSource` carries. */
export const FEM2D_STORY_EXAMPLE_ID = "demo";

/** 🎬️ The registered example and an explicit empty-document selection. */
export const FEM3D_STORY_EXAMPLE_ID = "demo";
export const FEM3D_CLEARED_EXAMPLE_ID = "cleared";

export const FEM2D_DEMO_SNAPSHOT: Fem2dSnapshot = parseFem2dDsl(fem2dDemoDsl);
export const FEM3D_DEMO_SNAPSHOT: Fem3dSnapshot = parseFem3dDsl(fem3dDemoDsl);

/** @emoji 📨️ `ActionDescriptor.args` is declared `unknown`, so every story reducer narrows it here once instead of casting at each read. A non-object payload becomes an empty bag, exactly as a Rust handler sees no named arguments. */
export function femStoryActionArgs(args: unknown): Record<string, unknown> {
  return typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
}

/** @emoji 🪪️ Port of `crate::app_surface::next_id` (`⚙️engine/🖥️app-surface/🦀️.rs`): the smallest `"{prefix}{n}"` not already taken, starting at the current count. */
export function femNextId(existing: Iterable<string>, prefix: string): string {
  const ids = new Set(existing);
  let index = ids.size;
  while (ids.has(`${prefix}${index}`)) index += 1;
  return `${prefix}${index}`;
}
//#endregion 🔖️Fixtures

//#region 🔖️Omissions
/** @emoji 🕳️ One half of a window's Rust render this story cannot reproduce in the browser, with the reason — surfaced in every debug readout so a blank region is never mistaken for a bug. */
export type FemStoryOmission = { readonly id: string; readonly reason: string };

export const FEM2D_MESH_PREVIEW_OMISSION: FemStoryOmission = {
  id: "mesh-edge-*",
  reason: "fem2d_region_triangles → fem2d_mesh_preview is Rust-only (plugin wasm); the region mesh overlay is omitted rather than approximated",
};

export const FEM2D_SOLVER_OMISSION: FemStoryOmission = {
  id: "deformed-* / reaction-* / moment-* / contour-*",
  reason: "fem2d_solve_all is Rust-only (plugin wasm); every displacement/reaction/moment/von-Mises layer is omitted",
};

export const FEM3D_SOLID_MESH_OMISSION: FemStoryOmission = {
  id: "solid-*",
  reason: "fem3d_solid_mesh_entries → fem3d_mesh_preview is Rust-only (plugin wasm); solid boundary meshes are omitted",
};

export const FEM3D_SOLVER_OMISSION: FemStoryOmission = {
  id: "displacement offsets / von-Mises vertex colors",
  reason: "fem3d_solve_all is Rust-only (plugin wasm); the results scene renders the undeformed, uncolored structure",
};

/** @emoji 🕳️ The omission list as flat JSON for a `data-testid`'d debug panel. */
export function femStoryOmissions(omissions: readonly FemStoryOmission[]): readonly FemStoryOmission[] {
  return omissions;
}
//#endregion 🔖️Omissions

//#region 🔖️Fem2dScene
/** @emoji 📐️ `SCALE_2D` — model metres to screen pixels (`🪟️windows/🧱️model/🦀️.rs`). */
const FEM2D_SCALE = 20;
/** @emoji 📐️ `ORIGIN_2D` — screen-space offset so a structure anchored at (0,0) is not drawn at the canvas corner. */
const FEM2D_ORIGIN = 40;

/** @emoji 📐️ Port of `screen_2d`: model coordinates to the canvas' own screen space (y flipped). */
export function femScreen2d(x: number, y: number): FemVec2 {
  return [x * FEM2D_SCALE + FEM2D_ORIGIN, -y * FEM2D_SCALE + FEM2D_ORIGIN];
}

type FemLayer = Record<string, unknown>;

/** @emoji ➡️ Port of `vector_layer`: a two-point polyline from `origin` along `vector`, y negated. */
function femVectorLayer(id: string, origin: FemVec2, vector: FemVec2, color: string): FemLayer {
  return { kind: "polyline", id, points: [[origin[0], origin[1]], [origin[0] + vector[0], origin[1] - vector[1]]], color };
}

function femSign(value: number): number {
  return value < 0 ? -1 : 1;
}

/**
 * @emoji 🖼️ Port of `fem2d_structure_layers`: one circle per node, one line per member, one circle per
 * support and one red vector per load, in exactly the Rust order and with the Rust's own ids
 * (`node-<id>` / `el-<id>` / `support-<id>` / `load-<id>`). The three colors are the caller's, matching the
 * two Rust call sites: bright (`#38bdf8`/`#94a3b8`/`#f97316`) for the model window, a single muted
 * `#334155` for the results window's undeformed backdrop.
 */
export function fem2dStructureLayers(snapshot: Fem2dSnapshot, nodeColor: string, lineColor: string, supportColor: string): readonly FemLayer[] {
  const layers: FemLayer[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `node-${node.id}`, x: sx - 4, y: sy - 4, width: 8, height: 8, color: nodeColor });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const [x0, y0] = femScreen2d(start.x, start.y);
    const [x1, y1] = femScreen2d(end.x, end.y);
    layers.push({ kind: "line", id: `el-${element.id}`, x0, y0, x1, y1, color: lineColor });
  }
  for (const support of snapshot.supports) {
    const node = nodeById.get(support.nodeId);
    if (!node) continue;
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `support-${support.id}`, x: sx - 5, y: sy - 5, width: 10, height: 10, color: supportColor });
  }
  for (const loadCase of snapshot.loadCases) {
    for (const load of loadCase.loads) {
      if (load.kind === "nodal") {
        const node = nodeById.get(load.nodeId ?? "");
        if (!node) continue;
        const value = load.value ?? 0;
        const vector: FemVec2 = load.dof === "Tx" ? [femSign(value) * 18, 0] : load.dof === "Ty" ? [0, femSign(value) * 18] : [0, -12];
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(node.x, node.y), vector, "#ef4444"));
        continue;
      }
      if (load.kind === "memberUdl") {
        const element = snapshot.elements.find((entry) => entry.id === load.elementId);
        const start = element ? nodeById.get(element.start) : undefined;
        const end = element ? nodeById.get(element.end) : undefined;
        if (!start || !end) continue;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d((start.x + end.x) * 0.5, (start.y + end.y) * 0.5), [femSign(load.wx ?? 0) * 18, femSign(load.wy ?? 0) * 18], "#ef4444"));
        continue;
      }
      if (load.kind === "area") {
        const region = snapshot.regions.find((entry) => entry.id === load.regionId);
        if (!region || region.outline.length === 0) continue;
        const sum = region.outline.reduce((accumulator, point) => [accumulator[0] + point[0], accumulator[1] + point[1]] as const, [0, 0] as FemVec2);
        const count = region.outline.length;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(sum[0] / count, sum[1] / count), [0, -femSign(load.pressure ?? 0) * 18], "#ef4444"));
      }
    }
  }
  return layers;
}

/** 🎥️ Initial navigation for a newly opened FEM 2D window. */
export const FEM2D_DEFAULT_CAMERA: Viewport2d = { x: 0, y: 0, zoom: 1 };

/** @emoji 📐️ The `canvas-2d` scene node a fem2d window renders, projected the way `crate::app_surface::canvas_2d_surface` encodes it — `surfaceId` is the window's own `BODY_KEY`. */
export function buildFem2dSceneNode(layers: readonly FemLayer[], camera: Viewport2d, bodyKey: string, controllerId: string) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "canvas-2d",
    canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: JSON.stringify(layers) },
  } as const;
}
//#endregion 🔖️Fem2dScene

//#region 🔖️Fem3dScene
/** @emoji 🧊️ `NODE_SIZE_3D` / `MEMBER_THICKNESS_3D` (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs`). */
const FEM3D_NODE_SIZE = 0.05;
const FEM3D_MEMBER_THICKNESS = 0.05;

type FemQuat = readonly [number, number, number, number];
type FemVec3 = readonly [number, number, number];

/** @emoji 🧭️ Port of `quat_mul`: Hamilton product `a * b`, both `[x,y,z,w]`. */
function quatMul(a: FemQuat, b: FemQuat): FemQuat {
  const [ax, ay, az, aw] = a;
  const [bx, by, bz, bw] = b;
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz];
}

/** @emoji 🧭️ Port of `quat_roll_z`: `roll` radians about the local +Z axis. */
function quatRollZ(roll: number): FemQuat {
  const half = roll / 2;
  return [0, 0, Math.sin(half), Math.cos(half)];
}

/** @emoji 🧭️ Port of `quat_z_to`: shortest-arc rotation taking local +Z (the `"box"` mesh's long axis) onto unit `dir`, with the antiparallel case flipped 180° about X. */
function quatZTo(dir: FemVec3): FemQuat {
  const dot = Math.min(1, Math.max(-1, dir[2]));
  if (dot > 0.999999) return [0, 0, 0, 1];
  if (dot < -0.999999) return [1, 0, 0, 0];
  const axisLength = Math.sqrt(dir[1] * dir[1] + dir[0] * dir[0]);
  const half = Math.acos(dot) / 2;
  const sin = Math.sin(half);
  return [(-dir[1] / axisLength) * sin, (dir[0] / axisLength) * sin, 0, Math.cos(half)];
}

/** @emoji 📦️ Port of `mesh_box(1,1,1)` + `compute_normals` (`🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`): six quads as twelve non-indexed triangles, so accumulated normals are flat per face. This is the `"box"` mesh `world3d_meshes_json_from_kinds(&["box"])` puts in every fem3d scene. */
function femUnitBoxMeshData(): { readonly positions: number[]; readonly normals: number[]; readonly indices: number[] } {
  const h = 0.5;
  const faces: readonly (readonly FemVec3[])[] = [
    [[-h, -h, h], [h, -h, h], [h, h, h], [-h, h, h]],
    [[h, -h, -h], [-h, -h, -h], [-h, h, -h], [h, h, -h]],
    [[-h, h, h], [h, h, h], [h, h, -h], [-h, h, -h]],
    [[-h, -h, -h], [h, -h, -h], [h, -h, h], [-h, -h, h]],
    [[h, -h, h], [h, -h, -h], [h, h, -h], [h, h, h]],
    [[-h, -h, -h], [-h, -h, h], [-h, h, h], [-h, h, -h]],
  ];
  const positions: number[] = [];
  const normals: number[] = [];
  const indices: number[] = [];
  const pushTriangle = (a: FemVec3, b: FemVec3, c: FemVec3): void => {
    const base = positions.length / 3;
    positions.push(a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]);
    const e0: FemVec3 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const e1: FemVec3 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const raw: FemVec3 = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    const length = Math.max(Math.hypot(raw[0], raw[1], raw[2]), 1e-8);
    for (let corner = 0; corner < 3; corner += 1) normals.push(raw[0] / length, raw[1] / length, raw[2] / length);
    indices.push(base, base + 1, base + 2);
  };
  for (const [a, b, c, d] of faces) {
    pushTriangle(a!, b!, c!);
    pushTriangle(a!, c!, d!);
  }
  return { positions, normals, indices };
}

export const FEM3D_BOX_MESHES: readonly Record<string, unknown>[] = [{ id: "box", data: femUnitBoxMeshData() }];

/**
 * @emoji 🧊️ Port of `fem3d_structural_instances`: one small box per node, then one oriented box prism per
 * `Bar`/`Frame` member — positioned at the midpoint, scaled `[t,t,length]` so the mesh's long local Z axis
 * stretches along the member, rotated by `quat_z_to(dir) * quat_roll_z(roll)`. Ids are the Rust's own
 * (`node-<id>` / `el-<id>`). `displacements` are always absent here (no solver in the browser), so this is
 * always the undeformed structure.
 */
export function fem3dStructuralInstances(snapshot: Fem3dSnapshot): readonly Record<string, unknown>[] {
  const instances: Record<string, unknown>[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    instances.push({ id: `node-${node.id}`, meshId: "box", position: [node.x, node.y, node.z], rotation: [0, 0, 0, 1], scale: [FEM3D_NODE_SIZE, FEM3D_NODE_SIZE, FEM3D_NODE_SIZE], label: node.id });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const delta: FemVec3 = [end.x - start.x, end.y - start.y, end.z - start.z];
    const length = Math.max(Math.hypot(delta[0], delta[1], delta[2]), 1e-9);
    const direction: FemVec3 = [delta[0] / length, delta[1] / length, delta[2] / length];
    const rotation = quatMul(quatZTo(direction), quatRollZ(element.kind === "frame" ? element.roll : 0));
    instances.push({
      id: `el-${element.id}`,
      meshId: "box",
      position: [(start.x + end.x) / 2, (start.y + end.y) / 2, (start.z + end.z) / 2],
      rotation,
      scale: [FEM3D_MEMBER_THICKNESS, FEM3D_MEMBER_THICKNESS, length],
      label: element.id,
    });
  }
  return instances;
}

/** 🎥️ Initial navigation for a newly opened FEM 3D window. */
export const FEM3D_INITIAL_VIEWPORT: Viewport3dOrbit = { position: [4, -4, 3], target: [0, 0, 0], zoom: 1 };

/** @emoji ✅️ `world3d_selection_json("rectangle", &[], None)`. */
const FEM3D_SELECTION_JSON = JSON.stringify({ method: "rectangle", mode: "replace", ids: [], hoveredId: null });

/** @emoji 🌞️ `world3d_environment_json(&WorldSunConfig::default())`. */
const FEM3D_ENVIRONMENT_JSON = JSON.stringify({ sun: { enabled: false, azimuth: 45, elevation: 35, intensity: 0.85, color: "#ffffff" } });

/** @emoji 🌐️ The `world-3d` scene node a fem3d window renders, projected the way `crate::app_surface::world_3d_surface` encodes it — `surfaceId` is the window's own body key. */
export function buildFem3dSceneNode(instances: readonly Record<string, unknown>[], camera: Viewport3dOrbit, bodyKey: string, controllerId: string) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(camera),
      meshesJson: JSON.stringify(FEM3D_BOX_MESHES),
      instancesJson: JSON.stringify(instances),
      selectionJson: FEM3D_SELECTION_JSON,
      environmentJson: FEM3D_ENVIRONMENT_JSON,
      interactionJson: JSON.stringify({ activeUtility: "select" }),
    },
  } as const;
}
//#endregion 🔖️Fem3dScene

//#region 🔖️ResultDisplay
/** 👁️ Result presentation owned by the current window and translated at the render boundary. */
export type FemStoryResultDisplay = { readonly sourceId: string | null; readonly mode: string; readonly modeIndex: number };

export const FEM_DEFAULT_RESULT_DISPLAY: FemStoryResultDisplay = { sourceId: null, mode: "static", modeIndex: 0 };

/** @emoji 📊️ The case id `render_static` resolves: the selected `sourceId` when the document knows it, else the first load case, else none. The Rust filters against `fem2d_solve_all`'s result keys; with no solver here the document's own case/combination ids stand in, which is the same set for a solvable document. */
export function femResolveResultCase(display: FemStoryResultDisplay, caseIds: readonly string[]): string | null {
  if (display.sourceId !== null && caseIds.includes(display.sourceId)) return display.sourceId;
  return caseIds[0] ?? null;
}
//#endregion 🔖️ResultDisplay

//#region 🔖️Fem2dReducer
export type Fem2dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem2dSnapshot;
  readonly camera: Viewport2d;
  readonly resultDisplay: FemStoryResultDisplay;
};

/** @emoji 🎬️ The state a fem2d session starts in for one example id — `"demo"` loads the bundled fixture, every other id an empty document, exactly as `setActiveExample`'s handler branches. */
export function fem2dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem2dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM2D_STORY_EXAMPLE_ID ? FEM2D_DEMO_SNAPSHOT : EMPTY_FEM2D_SNAPSHOT,
    camera: FEM2D_DEFAULT_CAMERA,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem2dStoryAction(state: Fem2dStoryState, action: string, rawArgs: unknown): Fem2dStoryState {
  const args = femStoryActionArgs(rawArgs);
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem2dStoryStateFor(exampleId, state.locale).snapshot };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y }] } };
    }
    case "setCamera": {
      return { ...state, camera: parseViewport2d({ x: args.x, y: args.y, zoom: args.zoom }) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}
//#endregion 🔖️Fem2dReducer

//#region 🔖️Fem3dReducer
export type Fem3dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem3dSnapshot;
  readonly camera: Viewport3dOrbit;
  readonly resultDisplay: FemStoryResultDisplay;
};

/** 🎬️ Opens a story window with the registered example and explicit OS locale. */
export function fem3dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem3dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM3D_STORY_EXAMPLE_ID ? FEM3D_DEMO_SNAPSHOT : EMPTY_FEM3D_SNAPSHOT,
    camera: FEM3D_INITIAL_VIEWPORT,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem3dStoryAction(state: Fem3dStoryState, action: string, rawArgs: unknown): Fem3dStoryState {
  const args = femStoryActionArgs(rawArgs);
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem3dStoryStateFor(exampleId, state.locale).snapshot };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const z = typeof args.z === "number" ? args.z : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y, z }] } };
    }
    case "setCamera": {
      const camera = args.camera;
      return { ...state, camera: parseViewport3dOrbit(camera) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}
//#endregion 🔖️Fem3dReducer

//#region 🔖️RenderSummaries
/** @emoji 📋️ The bilingual document readout every fem2d story shows beside its canvas — the counts the window's own scene is built from, so the panel stays assertable even when a layer kind is empty. */
export function fem2dSummaryLines(snapshot: Fem2dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("regions", locale)}: ${snapshot.regions.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** @emoji 📋️ The fem3d counterpart, with `solids` in place of `regions`. */
export function fem3dSummaryLines(snapshot: Fem3dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("solids", locale)}: ${snapshot.solids.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** @emoji 📊️ The results-window caption line: `render_static`'s resolved case, or its `"No load case defined"` placeholder branch. */
export function femResultCaptionLine(caseId: string | null, display: FemStoryResultDisplay, locale: FemStoryLocale): string {
  if (caseId === null) return femStoryLabel("noLoadCase", locale);
  const suffix = display.mode === "static" ? "" : ` #${display.modeIndex}`;
  return `${femStoryLabel("activeCase", locale)}: ${caseId} — ${femStoryLabel("displayMode", locale)} ${display.mode}${suffix}`;
}
//#endregion 🔖️RenderSummaries

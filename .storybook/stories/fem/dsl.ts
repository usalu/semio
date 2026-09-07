// #region 🧲️Header
// 💻️ .storybook/stories/fem/dsl.ts
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
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

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
export type Fem3dSolid = { readonly id: string; readonly name: string; readonly outline: readonly FemVec2[]; readonly holes: readonly FemVec2[]; readonly baseZ: number; readonly height: number; readonly layers: number; readonly meshSize: number; readonly materialId: string };

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

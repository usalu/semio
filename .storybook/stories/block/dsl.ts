// #region 🧲️Header
// 💻️ .storybook/stories/block/dsl.ts
// Specs: Story-local parser for the three `🧱️block` DSL dialects (`block.block2d.dsl`, `block.block3d.dsl`,
// `block.block5d.dsl` — the `🗣️.dsl.semio` example assets under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/**/📚️examples/`).
// Summary: Shared by every `stories/block/**` story file so the fixture data in the stories is the REAL
// shipped example document, not hand-authored story data — mirroring `stories/puzzle/2d/Fixtures.stories.tsx`'s
// "parse the real `.dsl.semio` asset" discipline. Puzzle can reuse its plugin's own `parse_dsl` through a
// `wasm-bindgen` export (`puzzle2dParseDslJson`); block ships no such export (its `📦️packages/🦀️rust` crate is a
// WASM *component*, not a `wasm-bindgen` module, so there is no free function to call from the browser), hence
// this small TypeScript reader of the same text grammar. It is deliberately a READER only: it never re-emits
// DSL, so it can never drift into a second authority for the format.
// Grammar (as emitted by `🧬️schema/📸️snapshot/📝️text/🦀️.rs`): a `semio <dialect> v<n>` banner, top-level
// `key=value` lines, `name { key=value … }` blocks, and `name [col:TYPE …] { row … }` tables whose rows are
// whitespace-separated tokens — quoted strings, `[ … ]` groups, `_` (absent), `@x,y,z` coordinates,
// `^x,y,z` directions and `<n>rad`/`<n>deg` angles.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

//#region 🔖️Model
/** @emoji 🗂️ One parsed DSL document: the banner dialect/version, top-level scalars, `name { … }` blocks and `name [cols] { rows }` tables. */
export type BlockDslDocument = {
  readonly dialect: string;
  readonly version: string;
  readonly scalars: Readonly<Record<string, string>>;
  readonly blocks: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly tables: Readonly<Record<string, readonly Readonly<Record<string, unknown>>[]>>;
};
//#endregion 🔖️Model

//#region 🔖️Tokenizer
/** @emoji ✂️ Splits one DSL row into its column tokens: a quoted string, a bracketed `[ … ]` group, or a bare word. Never splits inside quotes or brackets. */
function tokenizeRow(line: string): string[] {
  const tokens: string[] = [];
  let index = 0;
  while (index < line.length) {
    const char = line[index]!;
    if (char === " " || char === "\t") {
      index += 1;
      continue;
    }
    if (char === '"') {
      let end = index + 1;
      let value = "";
      while (end < line.length && line[end] !== '"') {
        if (line[end] === "\\" && end + 1 < line.length) {
          value += line[end + 1];
          end += 2;
          continue;
        }
        value += line[end];
        end += 1;
      }
      tokens.push(JSON.stringify(value));
      index = end + 1;
      continue;
    }
    if (char === "[") {
      let depth = 0;
      let end = index;
      while (end < line.length) {
        if (line[end] === "[") depth += 1;
        else if (line[end] === "]") {
          depth -= 1;
          if (depth === 0) break;
        }
        end += 1;
      }
      tokens.push(line.slice(index, end + 1));
      index = end + 1;
      continue;
    }
    let end = index;
    while (end < line.length && line[end] !== " " && line[end] !== "\t") end += 1;
    tokens.push(line.slice(index, end));
    index = end;
  }
  return tokens;
}
//#endregion 🔖️Tokenizer

//#region 🔖️Values
const ANGLE_PATTERN = /^(-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?)(rad|deg)$/;
const NUMBER_PATTERN = /^-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?$/;

/** @emoji 🔢️ Coerces one column token to its typed value. `_` is the DSL's "absent" marker and becomes `undefined`; unknown column types fall back to the token's natural shape. */
export function coerceDslValue(token: string, columnType?: string): unknown {
  if (token === "_") return undefined;
  if (token.startsWith('"')) {
    const text = JSON.parse(token) as string;
    return columnType === "NUM" && NUMBER_PATTERN.test(text) ? Number(text) : text;
  }
  if (token.startsWith("[")) {
    const inner = token.slice(1, -1).trim();
    return inner === "" ? [] : tokenizeRow(inner).map((entry) => coerceDslValue(entry));
  }
  if (token.startsWith("@") || token.startsWith("^")) return token.slice(1).split(",").map(Number);
  const angle = ANGLE_PATTERN.exec(token);
  if (angle) return angle[2] === "deg" ? (Number(angle[1]) * Math.PI) / 180 : Number(angle[1]);
  if (token === "true") return true;
  if (token === "false") return false;
  if (NUMBER_PATTERN.test(token)) return Number(token);
  if (/^-?\d+(?:\.\d+)?(?:,-?\d+(?:\.\d+)?)+$/.test(token)) return token.split(",").map(Number);
  return token;
}

/** @emoji 🔤️ Column header `name:TYPE` → the camelCased record key the stories read plus the declared type. */
function parseColumnHeader(header: string): { readonly key: string; readonly type: string } {
  const colon = header.lastIndexOf(":");
  const rawName = colon > 0 ? header.slice(0, colon) : header;
  const type = colon > 0 ? header.slice(colon + 1) : "TEXT";
  return { key: rawName.replace(/-([a-z0-9])/g, (_, letter: string) => letter.toUpperCase()), type };
}

/** @emoji 🧩️ Splits a block body's `key=value key=value …` text into typed entries; quoted values keep their spaces. */
function parseBlockBody(text: string): Record<string, unknown> {
  const record: Record<string, unknown> = {};
  for (const token of tokenizeAssignments(text)) {
    const equals = token.indexOf("=");
    if (equals <= 0) continue;
    const key = token.slice(0, equals).replace(/-([a-z0-9])/g, (_, letter: string) => letter.toUpperCase());
    record[key] = coerceDslValue(token.slice(equals + 1));
  }
  return record;
}

/** @emoji ✂️ Splits `key=value` assignments, keeping quoted values (which may contain spaces) intact. */
function tokenizeAssignments(text: string): string[] {
  const tokens: string[] = [];
  let current = "";
  let quoted = false;
  for (let index = 0; index < text.length; index += 1) {
    const char = text[index]!;
    if (char === '"') {
      quoted = !quoted;
      current += char;
      continue;
    }
    if (!quoted && (char === " " || char === "\t" || char === "\n" || char === "\r")) {
      if (current !== "") tokens.push(current);
      current = "";
      continue;
    }
    current += char;
  }
  if (current !== "") tokens.push(current);
  return tokens;
}
//#endregion 🔖️Values

//#region 🔖️Parser
/** @emoji 📖️ Parses one `🗣️.dsl.semio` block document. Throws on a missing/foreign banner rather than returning a half-read document — a story rendering nothing is far harder to diagnose than a thrown fixture error. */
export function parseBlockDsl(text: string): BlockDslDocument {
  const lines = text.split(/\r?\n/);
  const banner = /^semio\s+(\S+)\s+v(\S+)\s*$/.exec(lines[0] ?? "");
  if (!banner) throw new Error(`[block-dsl] missing "semio <dialect> v<n>" banner: ${JSON.stringify(lines[0] ?? "")}`);
  const scalars: Record<string, string> = {};
  const blocks: Record<string, Record<string, unknown>> = {};
  const tables: Record<string, Record<string, unknown>[]> = {};

  let index = 1;
  while (index < lines.length) {
    const line = (lines[index] ?? "").trim();
    index += 1;
    if (line === "" || line.startsWith("#")) continue;

    const table = /^([a-z0-9-]+)\s*\[([^\]]*)\]\s*\{$/.exec(line);
    if (table) {
      const columns = table[2]!.trim().split(/\s+/).filter(Boolean).map(parseColumnHeader);
      const rows: Record<string, unknown>[] = [];
      while (index < lines.length && (lines[index] ?? "").trim() !== "}") {
        const rowText = (lines[index] ?? "").trim();
        index += 1;
        if (rowText === "") continue;
        const tokens = tokenizeRow(rowText);
        const row: Record<string, unknown> = {};
        columns.forEach((column, position) => {
          const token = tokens[position];
          row[column.key] = token === undefined ? undefined : coerceDslValue(token, column.type);
        });
        rows.push(row);
      }
      index += 1;
      tables[table[1]!] = rows;
      continue;
    }

    const block = /^([a-z0-9-]+)\s*\{$/.exec(line);
    if (block) {
      const body: string[] = [];
      while (index < lines.length && (lines[index] ?? "").trim() !== "}") {
        body.push(lines[index] ?? "");
        index += 1;
      }
      index += 1;
      blocks[block[1]!] = parseBlockBody(body.join("\n"));
      continue;
    }

    const equals = line.indexOf("=");
    if (equals > 0) scalars[line.slice(0, equals)] = line.slice(equals + 1);
  }

  return { dialect: banner[1]!, version: banner[2]!, scalars, blocks, tables };
}
//#endregion 🔖️Parser

//#region 🔖️Projections
export type BlockKindIdentity = { readonly id: string; readonly name: string; readonly label: string; readonly description: string };
export type BlockCamera2d = { readonly x: number; readonly y: number; readonly zoom: number };
export type BlockCamera3d = { readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly zoom: number };
export type BlockRepresentation = { readonly id: string; readonly name: string; readonly meshUrl?: string; readonly tags: readonly string[]; readonly lod?: string };

export type Block2dHandleKind = { readonly id: string; readonly name: string; readonly label: string; readonly color: string; readonly defaultWireKind: string };
export type Block2dHandleTemplate = { readonly id: string; readonly handleKind: string; readonly angle: number; readonly radius: number };
/** @emoji ◻️ The subset of `Block2dSnapshot` (`🗿️artifacts/◻️2d/…/🧬️schema/📸️snapshot/🦀️.rs`) the 2D stories render. */
export type Block2dSnapshot = {
  readonly nodeKind: BlockKindIdentity;
  readonly camera2d: BlockCamera2d;
  readonly handleKinds: readonly Block2dHandleKind[];
  readonly handles: readonly Block2dHandleTemplate[];
};

export type Block3dVortexKind = { readonly id: string; readonly label: string; readonly color: string; readonly defaultCableKind: string };
export type Block3dVortex = { readonly id: string; readonly vortexKind: string; readonly position: readonly [number, number, number]; readonly direction: readonly [number, number, number]; readonly radius: number; readonly label?: string };
/** @emoji 🧊️ The subset of `Block3dSnapshot` the 3D stories render (`vortexKinds` is `vortex_kinds_of`'s result — the `vortex-kind-extra` overflow half, which is the whole catalogue for a standalone example). */
export type Block3dSnapshot = {
  readonly objectKind: BlockKindIdentity;
  readonly camera3d: BlockCamera3d;
  readonly representations: readonly BlockRepresentation[];
  readonly vortexKinds: readonly Block3dVortexKind[];
  readonly vortices: readonly Block3dVortex[];
};

export type Block5dGripKind = { readonly id: string; readonly name: string; readonly label: string; readonly color: string; readonly defaultRopeKind: string };
export type Block5dGrip = {
  readonly id: string;
  readonly gripKind: string;
  readonly angle: number;
  readonly radius2d: number;
  readonly position: readonly [number, number, number];
  readonly direction: readonly [number, number, number];
  readonly radius3d: number;
};
/** @emoji 🖐️ The subset of `Block5dSnapshot` the 5D stories render — one part kind projected into both a 2D board and a 3D world. */
export type Block5dSnapshot = {
  readonly partKind: BlockKindIdentity;
  readonly part2d: { readonly shape: string; readonly radius: number };
  readonly camera2d: BlockCamera2d;
  readonly camera3d: BlockCamera3d;
  readonly representations: readonly BlockRepresentation[];
  readonly gripKinds: readonly Block5dGripKind[];
  readonly grips: readonly Block5dGrip[];
};

function text(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function num(value: unknown, fallback = 0): number {
  return typeof value === "number" ? value : fallback;
}

function vec3(value: unknown, fallback: readonly [number, number, number] = [0, 0, 0]): readonly [number, number, number] {
  return Array.isArray(value) && value.length === 3 && value.every((entry) => typeof entry === "number") ? ([value[0], value[1], value[2]] as const) : fallback;
}

function identity(block: Readonly<Record<string, unknown>> | undefined): BlockKindIdentity {
  return { id: text(block?.id), name: text(block?.name), label: text(block?.label), description: text(block?.description) };
}

function camera2d(block: Readonly<Record<string, unknown>> | undefined): BlockCamera2d {
  return { x: num(block?.x), y: num(block?.y), zoom: num(block?.zoom, 1) };
}

function camera3d(block: Readonly<Record<string, unknown>> | undefined): BlockCamera3d {
  return { position: vec3(block?.position, [4, -4, 3]), target: vec3(block?.target), zoom: num(block?.zoom, 1) };
}

function representations(rows: readonly Readonly<Record<string, unknown>>[] | undefined): readonly BlockRepresentation[] {
  return (rows ?? []).map((row) => ({
    id: text(row.id),
    name: text(row.name),
    meshUrl: typeof row.meshUrl === "string" ? row.meshUrl : undefined,
    tags: Array.isArray(row.tags) ? row.tags.map((tag) => String(tag)) : [],
    lod: typeof row.lod === "string" ? row.lod : undefined,
  }));
}

/** @emoji ◻️ `block.block2d.dsl` → the `Block2dSnapshot` slice the 2D stories render. */
export function parseBlock2dDsl(dslText: string): Block2dSnapshot {
  const doc = parseBlockDsl(dslText);
  return {
    nodeKind: identity(doc.blocks["node-kind"]),
    camera2d: camera2d(doc.blocks.camera2d),
    handleKinds: (doc.tables["handle-kinds"] ?? []).map((row) => ({ id: text(row.id), name: text(row.name), label: text(row.label), color: text(row.color), defaultWireKind: text(row.defaultWireKind) })),
    handles: (doc.tables.handles ?? []).map((row) => ({ id: text(row.id), handleKind: text(row.handleKind), angle: num(row.angle), radius: num(row.radius) })),
  };
}

/** @emoji 🧊️ `block.block3d.dsl` → the `Block3dSnapshot` slice the 3D stories render. */
export function parseBlock3dDsl(dslText: string): Block3dSnapshot {
  const doc = parseBlockDsl(dslText);
  return {
    objectKind: identity(doc.blocks["object-kind"]),
    camera3d: camera3d(doc.blocks.camera3d),
    representations: representations(doc.tables.representations),
    vortexKinds: (doc.tables["vortex-kind-extra"] ?? []).map((row) => ({ id: text(row.id), label: text(row.label), color: text(row.color), defaultCableKind: text(row.defaultCableKind) })),
    vortices: (doc.tables.vortices ?? []).map((row) => ({
      id: text(row.id),
      vortexKind: text(row.vortexKind),
      position: vec3(row.position),
      direction: vec3(row.direction, [0, 0, 1]),
      radius: num(row.radius, 0.1),
      label: typeof row.label === "string" ? row.label : undefined,
    })),
  };
}

/** @emoji 🖐️ `block.block5d.dsl` → the `Block5dSnapshot` slice the 5D stories render. */
export function parseBlock5dDsl(dslText: string): Block5dSnapshot {
  const doc = parseBlockDsl(dslText);
  const part2d = doc.blocks["part-2d"];
  return {
    partKind: identity(doc.blocks["part-kind"]),
    part2d: { shape: text(part2d?.shape, "circle"), radius: num(part2d?.radius, 20) },
    camera2d: camera2d(doc.blocks.camera2d),
    camera3d: camera3d(doc.blocks.camera3d),
    representations: representations(doc.tables.representations),
    gripKinds: (doc.tables["grip-kinds"] ?? []).map((row) => ({ id: text(row.id), name: text(row.name), label: text(row.label), color: text(row.color), defaultRopeKind: text(row.defaultRopeKind) })),
    grips: (doc.tables.grips ?? []).map((row) => ({
      id: text(row.id),
      gripKind: text(row.gripKind),
      angle: num(row.angle),
      radius2d: num(row.radius2d, 1),
      position: vec3(row.position),
      direction: vec3(row.direction, [0, 0, 1]),
      radius3d: num(row.radius3d, 0.1),
    })),
  };
}
//#endregion 🔖️Projections

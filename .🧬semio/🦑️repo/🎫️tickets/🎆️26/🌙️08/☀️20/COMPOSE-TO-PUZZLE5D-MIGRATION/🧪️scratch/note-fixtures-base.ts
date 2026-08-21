/** 🧪️ Temporary authoring aid for the handcrafted `🗒️note` mutation fixtures (ticket
 * `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Holds the ONE hand-designed base snapshot plus the
 * float-preserving JSON writer; the per-mutation `after` states, mutation payloads and Rust test
 * bodies live in `note-fixtures-cases.ts` — each written out longhand, never derived. */

/** 🔢️ Marks a Rust `f64`, so the emitted JSON keeps `0.0` (a serde_json `Float`) and never `0`. */
export const F = (n: number): string => `@@${Number.isInteger(n) ? n.toFixed(1) : String(n)}@@`;

/** 📝️ 2-space JSON with the `f64` markers unwrapped back to bare float literals. */
export const json = (value: unknown): string => `${JSON.stringify(value, null, 2).replace(/"@@(-?[0-9][0-9.eE+-]*)@@"/g, "$1")}\n`;

export const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value)) as T;

/** 🕸️ `note_text_child_handle("blk-text", [])` — verified against the real `DefaultHasher` under
 * the repo's pinned `nightly-2026-07-07` toolchain. */
export const TEXT_HANDLE_EMPTY = "note-text-eea42a3b80b1052b";
/** 🕸️ `note_text_child_handle("blk-text", [{"runs":[{"text":"Hello, note."}]}])`. */
export const TEXT_HANDLE_HELLO = "note-text-938222b3522927c6";

const textTarget = () => ({ artifactId: "blk-text-text", dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "text" } });

/** 📸️ The single hand-designed base document every case starts from: one block of each kind at
 * document root (text, ink, table, math, image) plus a group holding one nested image, and one
 * id-keyed image asset. Small, but it carries every entity any of the 33 mutations addresses. */
export const base = () => ({
  schema: "note.document",
  id: "note-fixture",
  title: "Field Notes",
  blocks: [
    {
      kind: "text",
      id: "blk-text",
      name: "Intro",
      x: F(0),
      y: F(0),
      width: F(280),
      height: F(120),
      rotation: F(0),
      visible: true,
      locked: false,
      content: { childId: TEXT_HANDLE_EMPTY, target: textTarget() },
      fontSize: F(16),
      fontWeight: "normal",
      align: "left",
    },
    {
      kind: "stroke",
      id: "blk-ink",
      name: "Sketch",
      x: F(20),
      y: F(160),
      width: F(80),
      height: F(40),
      rotation: F(0),
      visible: true,
      locked: false,
      points: [[F(0), F(0)], [F(10), F(10)]],
      strokeWidth: F(2),
      color: [F(0), F(0), F(0), F(1)],
    },
    {
      kind: "table",
      id: "blk-table",
      name: "Samples",
      x: F(0),
      y: F(220),
      width: F(320),
      height: F(160),
      rotation: F(0),
      visible: true,
      locked: false,
      columns: ["A", "B"],
      rows: [[{ content: "Alpha" }, { content: "" }], [{ content: "" }, { content: "" }]],
    },
    {
      kind: "math",
      id: "blk-math",
      name: "Formula",
      x: F(0),
      y: F(400),
      width: F(200),
      height: F(80),
      rotation: F(0),
      visible: true,
      locked: false,
      tex: "E = mc^2",
      displayMode: true,
    },
    {
      kind: "image",
      id: "blk-image",
      name: "Logo",
      x: F(340),
      y: F(0),
      width: F(240),
      height: F(160),
      rotation: F(0),
      visible: true,
      locked: false,
      imageKey: "asset-logo",
    },
    {
      kind: "group",
      id: "blk-group",
      name: "Callout",
      x: F(340),
      y: F(200),
      width: F(280),
      height: F(120),
      rotation: F(0),
      visible: true,
      locked: false,
      children: [
        {
          kind: "image",
          id: "blk-nested",
          name: "Badge",
          x: F(350),
          y: F(210),
          width: F(60),
          height: F(60),
          rotation: F(0),
          visible: true,
          locked: false,
          imageKey: "asset-logo",
        },
      ],
    },
  ],
  gridVisible: true,
  gridSpacing: F(32),
  gridSubdivisions: F(4),
  gridOpacity: F(0.35),
  snapEnabled: false,
  snapGridSpacing: F(8),
  pencilWidth: F(3),
  eraserRadius: F(12),
  assets: { "asset-logo": { mime: "image/png", data: "bG9nbw==", width: F(64), height: F(64) } },
});

type Snapshot = ReturnType<typeof base>;
type Block = Snapshot["blocks"][number] & Record<string, unknown>;

/** 🔎️ Root-level block lookup by id. */
export const at = (snapshot: Snapshot, id: string): Block => {
  const found = snapshot.blocks.find((block) => block.id === id) as Block | undefined;
  if (!found) throw new Error(`no root block ${id}`);
  return found;
};

/** 🔎️ The one nested block inside `blk-group`. */
export const nested = (snapshot: Snapshot): Block => (at(snapshot, "blk-group").children as Block[])[0];

export const indexOf = (snapshot: Snapshot, id: string): number => snapshot.blocks.findIndex((block) => block.id === id);

//#region 🔺️Diff
/** 🗜️ Exactly what `serde_json::to_string(&value)` emits: compact, declaration-ordered, floats kept
 * as floats. Used for the `blockJson` payload a `NoteBlockPatch` carries. */
export const compact = (value: unknown): string => JSON.stringify(value).replace(/"@@(-?[0-9][0-9.eE+-]*)@@"/g, "$1");

/** 🔺️ A fully-null `NoteDiff`. Its container is `#[serde(rename_all = "camelCase", default)]` and NO
 * field carries `skip_serializing_if`, so serde emits ALL 23 fields — `null` for every untouched
 * one. Field order is the struct's own declaration order. */
export const emptyDiff = () => ({
  artifact: null as unknown,
  schema: null as unknown,
  id: null as unknown,
  title: null as unknown,
  blocks: null as unknown,
  gridVisible: null as unknown,
  gridSpacing: null as unknown,
  gridSubdivisions: null as unknown,
  gridOpacity: null as unknown,
  snapEnabled: null as unknown,
  snapGridSpacing: null as unknown,
  pencilWidth: null as unknown,
  eraserRadius: null as unknown,
  assets: null as unknown,
  linkedArtifact: null as unknown,
  selectedBlockIds: null as unknown,
  activeUtilityId: null as unknown,
  engagementInput: null as unknown,
  cameraX: null as unknown,
  cameraY: null as unknown,
  cameraZoom: null as unknown,
  locale: null as unknown,
  hoveredBlockId: null as unknown,
});

/** 🧩 `NoteBlocksDelta` — also `default`-ed with no skips, so all four fields are always emitted. */
export const blocksDelta = (parts: { added?: unknown[]; removed?: string[]; patched?: unknown[]; reordered?: string[] | null }) => ({
  added: parts.added ?? [],
  removed: parts.removed ?? [],
  patched: parts.patched ?? [],
  reordered: parts.reordered ?? null,
});

/** ➕ One `NoteAddedBlockEntry`. */
export const addedEntry = (parentId: string | null, index: number | null, block: unknown) => ({ parentId, index, block });

/** 🩹 One `NoteBlockPatchEntry` — the whole updated block, serialized into `blockJson`. */
export const patchEntry = (id: string, block: unknown) => ({ id, patch: { blockJson: compact(block) } });

/** 🗂️ One `NoteAssetsDelta` — `Some(asset)` upserts, `null` removes. */
export const assetsDelta = (entries: Record<string, unknown>) => ({ entries });
//#endregion 🔺️Diff

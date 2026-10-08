/** 🔺️ Presentation durable delta: a field patch of the working source, a positional tile row delta (the presentation child handle is derived by apply). */
import { parseFigureTileDraft, parseFigureTileFrame, type FigureTileDraft, type FigureTileFrame } from "../🟦️.ts";

export interface PresentationOptionalAspect { value: number | null }
export interface PresentationOptionalPage { value: number | null }
export interface PresentationSourcePatch {
  src: string | null;
  kind: string | null;
  frame: FigureTileFrame | null;
  sourceAspect: PresentationOptionalAspect | null;
  pdfPage: PresentationOptionalPage | null;
}
export interface PresentationTilePatch { name: string | null; crop: FigureTileFrame | null }
export interface PresentationTilesModification { id: string; patch: PresentationTilePatch }
export interface PresentationTilesRemoval { id: string; index: number }
export interface PresentationTilesInsertion { index: number; row: FigureTileDraft }
export interface PresentationTilesRelocation { id: string; from: number; to: number }
export interface PresentationTilesDelta { removed: PresentationTilesRemoval[]; inserted: PresentationTilesInsertion[]; moved: PresentationTilesRelocation[]; modified: PresentationTilesModification[] }
export interface PresentationDiff {
  /** @state artifact */ schema: string | null;
  /** @state artifact */ source: PresentationSourcePatch | null;
  /** @state artifact */ tiles: PresentationTilesDelta | null;
}

const record = (value: unknown, at: string): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value must be an object`);
  return value as Record<string, unknown>;
};
const text = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: value must be a string`);
  return value;
};
const optional = <T>(value: unknown, parse: (value: unknown) => T): T | null => (value == null ? null : parse(value));
const list = (value: unknown, at: string): unknown[] => {
  if (!Array.isArray(value)) throw new Error(`${at}: value must be an array`);
  return value;
};
const slot = (value: unknown, at: string, kind: "number" | "integer"): { value: number | null } => {
  const row = record(value, at);
  const inner = row.value ?? null;
  if (inner !== null && (typeof inner !== "number" || !Number.isFinite(inner) || (kind === "integer" && !Number.isInteger(inner)))) throw new Error(`${at}.value: invalid number`);
  return { value: inner };
};

export function parsePresentationTilePatch(value: unknown, at = "$"): PresentationTilePatch {
  const row = record(value, at);
  return { name: optional(row.name, (name) => text(name, `${at}.name`)), crop: optional(row.crop, (crop) => parseFigureTileFrame(crop, `${at}.crop`)) };
}

export function parsePresentationSourcePatch(value: unknown, at = "$"): PresentationSourcePatch {
  const row = record(value, at);
  return {
    src: optional(row.src, (src) => text(src, `${at}.src`)),
    kind: optional(row.kind, (kind) => text(kind, `${at}.kind`)),
    frame: optional(row.frame, (frame) => parseFigureTileFrame(frame, `${at}.frame`)),
    sourceAspect: optional(row.sourceAspect, (aspect) => slot(aspect, `${at}.sourceAspect`, "number")),
    pdfPage: optional(row.pdfPage, (page) => slot(page, `${at}.pdfPage`, "integer")),
  };
}

const count = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0) throw new Error(`${at}: value must be a non-negative integer`);
  return value;
};

export function parsePresentationTilesDelta(value: unknown, at = "$"): PresentationTilesDelta {
  const row = record(value, at);
  return {
    removed: list(row.removed, `${at}.removed`).map((entry, index) => {
      const removal = record(entry, `${at}.removed[${index}]`);
      return { id: text(removal.id, `${at}.removed[${index}].id`), index: count(removal.index, `${at}.removed[${index}].index`) };
    }),
    inserted: list(row.inserted, `${at}.inserted`).map((entry, index) => {
      const insertion = record(entry, `${at}.inserted[${index}]`);
      return { index: count(insertion.index, `${at}.inserted[${index}].index`), row: parseFigureTileDraft(insertion.row, `${at}.inserted[${index}].row`) };
    }),
    moved: list(row.moved, `${at}.moved`).map((entry, index) => {
      const relocation = record(entry, `${at}.moved[${index}]`);
      return { id: text(relocation.id, `${at}.moved[${index}].id`), from: count(relocation.from, `${at}.moved[${index}].from`), to: count(relocation.to, `${at}.moved[${index}].to`) };
    }),
    modified: list(row.modified, `${at}.modified`).map((entry, index) => {
      const modification = record(entry, `${at}.modified[${index}]`);
      return { id: text(modification.id, `${at}.modified[${index}].id`), patch: parsePresentationTilePatch(modification.patch, `${at}.modified[${index}].patch`) };
    }),
  };
}

/** 🧮️ Resolves omitted delta fields to the native unchanged null value. */
export function parsePresentationDiff(value: unknown, at = "$"): PresentationDiff {
  const row = record(value, at);
  if (Object.keys(row).some((key) => !["schema", "source", "tiles"].includes(key))) throw new Error(`${at}: fields do not match PresentationDiff`);
  return {
    schema: optional(row.schema, (schema) => text(schema, `${at}.schema`)),
    source: optional(row.source, (source) => parsePresentationSourcePatch(source, `${at}.source`)),
    tiles: optional(row.tiles, (tiles) => parsePresentationTilesDelta(tiles, `${at}.tiles`)),
  };
}

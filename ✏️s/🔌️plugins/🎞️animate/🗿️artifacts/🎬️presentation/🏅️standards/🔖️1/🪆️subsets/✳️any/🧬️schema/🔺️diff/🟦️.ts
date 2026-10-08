/** 🔺️ Presentation durable delta: a field patch of the working source, an id-keyed tile row delta and the exact presentation child-slot replacement. */
import { parseArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseFigureTileDraft, parseFigureTileFrame, type ArtifactChild, type FigureTileDraft, type FigureTileFrame } from "../🟦️.ts";

export interface PresentationOptionalAspect { value: number | null }
export interface PresentationOptionalPage { value: number | null }
export interface PresentationSourcePatch {
  src: string | null;
  kind: string | null;
  frame: FigureTileFrame | null;
  sourceAspect: PresentationOptionalAspect | null;
  pdfPage: PresentationOptionalPage | null;
}
export interface PresentationTilePatch { id: string; name: string | null; crop: FigureTileFrame | null }
export interface PresentationTilesDelta { added: FigureTileDraft[]; removed: string[]; patched: PresentationTilePatch[]; reordered: string[] | null }
export interface PresentationDiff {
  /** @state artifact */ schema: string | null;
  /** @state artifact */ source: PresentationSourcePatch | null;
  /** @state artifact */ tiles: PresentationTilesDelta | null;
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild | null;
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
  return { id: text(row.id, `${at}.id`), name: optional(row.name, (name) => text(name, `${at}.name`)), crop: optional(row.crop, (crop) => parseFigureTileFrame(crop, `${at}.crop`)) };
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

export function parsePresentationTilesDelta(value: unknown, at = "$"): PresentationTilesDelta {
  const row = record(value, at);
  return {
    added: list(row.added, `${at}.added`).map((tile, index) => parseFigureTileDraft(tile, `${at}.added[${index}]`)),
    removed: list(row.removed, `${at}.removed`).map((id, index) => text(id, `${at}.removed[${index}]`)),
    patched: list(row.patched, `${at}.patched`).map((patch, index) => parsePresentationTilePatch(patch, `${at}.patched[${index}]`)),
    reordered: optional(row.reordered, (order) => list(order, `${at}.reordered`).map((id, index) => text(id, `${at}.reordered[${index}]`))),
  };
}

/** 🧮️ Resolves omitted delta fields to the native unchanged null value. */
export function parsePresentationDiff(value: unknown, at = "$"): PresentationDiff {
  const row = record(value, at);
  if (Object.keys(row).some((key) => !["schema", "source", "tiles", "presentation"].includes(key))) throw new Error(`${at}: fields do not match PresentationDiff`);
  return {
    schema: optional(row.schema, (schema) => text(schema, `${at}.schema`)),
    source: optional(row.source, (source) => parsePresentationSourcePatch(source, `${at}.source`)),
    tiles: optional(row.tiles, (tiles) => parsePresentationTilesDelta(tiles, `${at}.tiles`)),
    presentation: optional(row.presentation, (child) => parseArtifactChild(child)),
  };
}

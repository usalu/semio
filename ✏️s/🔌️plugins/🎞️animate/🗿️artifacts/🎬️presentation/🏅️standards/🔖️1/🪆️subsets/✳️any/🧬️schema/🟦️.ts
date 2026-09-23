/** 🧬️ Presentation document: one shared figure, its tile crops, and independently owned presentation and animation children. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

/** 📐️ Normalized `x,y,width,height` rect. */
export interface FigureTileFrame {
  x: number;
  y: number;
  width: number;
  height: number;
}

/** 🖼️ The one shared figure every tile crops. */
export interface FigureTileSource {
  src: string;
  kind: string;
  frame: FigureTileFrame;
  sourceAspect?: number;
  pdfPage?: number;
}

/** 🧱️ One named crop of the shared figure. */
export interface FigureTileDraft {
  id: string;
  name: string;
  crop: FigureTileFrame;
}

export interface PresentationArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ source: FigureTileSource;
  /** @state artifact */ tiles: FigureTileDraft[];
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ animation: ArtifactChild;
}

function parseRecord(value: unknown, at: string, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value must be an object`);
  const row = value as Record<string, unknown>;
  if (required.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !required.includes(key) && !optional.includes(key))) throw new Error(`${at}: fields do not match`);
  return row;
}

function parseNumber(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: value must be a finite number`);
  return value;
}

function parseString(value: unknown, at: string): string {
  if (typeof value !== "string") throw new Error(`${at}: value must be a string`);
  return value;
}

/** 📐️ Parses one exact normalized rect. */
export function parseFigureTileFrame(value: unknown, at = "$"): FigureTileFrame {
  const row = parseRecord(value, at, ["x", "y", "width", "height"]);
  return { x: parseNumber(row.x, `${at}.x`), y: parseNumber(row.y, `${at}.y`), width: parseNumber(row.width, `${at}.width`), height: parseNumber(row.height, `${at}.height`) };
}

/** 🖼️ Parses the shared figure; absent optional fields stay absent, exactly as the native encoder omits them. */
export function parseFigureTileSource(value: unknown, at = "$"): FigureTileSource {
  const row = parseRecord(value, at, ["src", "kind", "frame"], ["sourceAspect", "pdfPage"]);
  const source: FigureTileSource = { src: parseString(row.src, `${at}.src`), kind: parseString(row.kind, `${at}.kind`), frame: parseFigureTileFrame(row.frame, `${at}.frame`) };
  if (Object.hasOwn(row, "sourceAspect")) source.sourceAspect = parseNumber(row.sourceAspect, `${at}.sourceAspect`);
  if (Object.hasOwn(row, "pdfPage")) {
    if (!Number.isInteger(row.pdfPage) || (row.pdfPage as number) < 0) throw new Error(`${at}.pdfPage: value must be a non-negative integer`);
    source.pdfPage = row.pdfPage as number;
  }
  return source;
}

/** 🧱️ Parses one exact tile draft. */
export function parseFigureTileDraft(value: unknown, at = "$"): FigureTileDraft {
  const row = parseRecord(value, at, ["id", "name", "crop"]);
  return { id: parseString(row.id, `${at}.id`), name: parseString(row.name, `${at}.name`), crop: parseFigureTileFrame(row.crop, `${at}.crop`) };
}

/** 🧱️ Parses an exact tile list. */
export function parseFigureTileDrafts(value: unknown, at = "$"): FigureTileDraft[] {
  if (!Array.isArray(value)) throw new Error(`${at}: value must be an array`);
  return value.map((tile, index) => parseFigureTileDraft(tile, `${at}[${index}]`));
}

/** 🪪️ Parses the exact document boundary shared by all Presentation views. */
export function parsePresentationArtifact(value: unknown, at = "$"): PresentationArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>, keys = ["schema", "source", "tiles", "presentation", "animation"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match PresentationArtifact`);
  if (typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string`);
  return {
    schema: row.schema,
    source: parseFigureTileSource(row.source, `${at}.source`),
    tiles: parseFigureTileDrafts(row.tiles, `${at}.tiles`),
    presentation: parseArtifactChild(row.presentation),
    animation: parseArtifactChild(row.animation),
  };
}

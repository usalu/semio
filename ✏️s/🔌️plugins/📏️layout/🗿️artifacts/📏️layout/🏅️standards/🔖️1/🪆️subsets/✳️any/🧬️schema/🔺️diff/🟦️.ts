/** 🔺️ Canonical Layout diff with every native `ToValue` field present and nullable. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {
  parseCharacterStyle, parseFrame, parseGridSettings, parseImageLink, parseLayoutArtifact, parseLayoutDrawingChild,
  parsePage, parseParagraphStyle, parseParentPage, parseSpread, parseTextStory,
  type ArtifactLink, type CharacterStyle, type Frame, type GridSettings, type ImageLink,
  type LayoutArtifact, type LayoutDrawingChild, type Page, type ParagraphStyle, type ParentPage,
  type Spread, type TextStory,
} from "../🟦️.ts";
import { parseArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export * from "../🟦️.ts";

export interface ParagraphStylePatch { name: string | null }
export interface CharacterStylePatch { name: string | null }
export interface ParentPagePatch { name: string | null }
export interface SpreadPatch { name: string | null }
export interface TextStoryPatch { content: string | null }
export interface ImageLinkPatch { path: string | null }
export interface FramePatch { x: number | null; y: number | null; width: number | null; height: number | null; fill: [number, number, number, number] | null; stroke: [number, number, number, number] | null; wrap_mode: string | null; columns: number | null }
export interface PageFrameAdded { frame: Frame; index: number | null; layer_id: string | null }
export interface PageFramePatched { frame_id: string; patch: FramePatch }
export interface PagePatch { name: string | null; width: number | null; height: number | null; margin_top: number | null; margin_right: number | null; margin_bottom: number | null; margin_left: number | null; columns_count: number | null; columns_gutter: number | null; frame_added: PageFrameAdded | null; frame_removed: string | null; frame_patched: PageFramePatched | null }
export interface LayoutPagePatchEntry { id: string; patch: PagePatch }
export interface LayoutStoryPatchEntry { id: string; patch: TextStoryPatch }
export interface LayoutLinkPatchEntry { id: string; patch: ImageLinkPatch }
export interface LayoutParagraphStylePatchEntry { id: string; patch: ParagraphStylePatch }
export interface LayoutCharacterStylePatchEntry { id: string; patch: CharacterStylePatch }
export interface LayoutParentPagePatchEntry { id: string; patch: ParentPagePatch }
export interface LayoutSpreadPatchEntry { id: string; patch: SpreadPatch }
export interface LayoutPagesDelta { added: Page[]; removed: string[]; patched: LayoutPagePatchEntry[]; reordered: string[] | null }
export interface LayoutStoriesDelta { added: TextStory[]; removed: string[]; patched: LayoutStoryPatchEntry[]; reordered: string[] | null }
export interface LayoutLinksDelta { added: ImageLink[]; removed: string[]; patched: LayoutLinkPatchEntry[]; reordered: string[] | null }
export interface LayoutParagraphStylesDelta { added: ParagraphStyle[]; removed: string[]; patched: LayoutParagraphStylePatchEntry[]; reordered: string[] | null }
export interface LayoutCharacterStylesDelta { added: CharacterStyle[]; removed: string[]; patched: LayoutCharacterStylePatchEntry[]; reordered: string[] | null }
export interface LayoutParentPagesDelta { added: ParentPage[]; removed: string[]; patched: LayoutParentPagePatchEntry[]; reordered: string[] | null }
export interface LayoutSpreadsDelta { added: Spread[]; removed: string[]; patched: LayoutSpreadPatchEntry[]; reordered: string[] | null }
export interface LayoutDiff {
  /** @state artifact */ artifact: LayoutArtifact | null;
  /** @state artifact */ schema: string | null;
  /** @state artifact */ name: string | null;
  /** @state artifact */ grid: GridSettings | null;
  /** @state artifact */ paragraphStyles: LayoutParagraphStylesDelta | null;
  /** @state artifact */ characterStyles: LayoutCharacterStylesDelta | null;
  /** @state artifact */ stories: LayoutStoriesDelta | null;
  /** @state artifact */ links: LayoutLinksDelta | null;
  /** @state artifact */ parentPages: LayoutParentPagesDelta | null;
  /** @state artifact */ spreads: LayoutSpreadsDelta | null;
  /** @state artifact */ pages: LayoutPagesDelta | null;
  /** @state artifact */ printTarget: string | null;
  /** @state artifact */ dataFieldsJson: string | null;
  /** @state artifact @child kind=s.stdio.semio */ backgroundDrawing: LayoutDrawingChild | null;
  /** @state artifact @link_slot roles=model */ referencedModel: ArtifactLink | null;
}

const required = (row: Record<string, unknown>, keys: readonly string[], at: string): Record<string, unknown> => {
  const exact = parseSchemaRecord(row, keys, at), missing = keys.find((key) => !Object.hasOwn(exact, key));
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
  return exact;
};
const record = (value: unknown, keys: readonly string[], at: string): Record<string, unknown> => required(parseSchemaRecord(value, keys, at), keys, at);
const string = (value: unknown, at: string): string => { if (typeof value !== "string") throw new Error(`${at}: string required`); return value; };
const number = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: finite number required`); return value; };
const integer = (value: unknown, at: string): number => { const result = number(value, at); if (!Number.isSafeInteger(result) || result < 0) throw new Error(`${at}: unsigned integer required`); return result; };
const array = (value: unknown, at: string): unknown[] => { if (!Array.isArray(value)) throw new Error(`${at}: array required`); return value; };
const strings = (value: unknown, at: string): string[] => array(value, at).map((item, index) => string(item, `${at}[${index}]`));
const nullable = <T>(value: unknown, parse: (value: unknown, at: string) => T, at: string): T | null => value === null ? null : parse(value, at);
const rgba = (value: unknown, at: string): [number, number, number, number] => { const items = array(value, at).map((item, index) => number(item, `${at}[${index}]`)); if (items.length !== 4) throw new Error(`${at}: four numbers required`); return items as [number, number, number, number]; };

const simplePatch = <K extends "name" | "content" | "path">(value: unknown, key: K, at: string): Record<K, string | null> => {
  const row = record(value, [key], at);
  return { [key]: nullable(row[key], string, `${at}.${key}`) } as Record<K, string | null>;
};
export const parseParagraphStylePatch = (value: unknown, at = "$" ): ParagraphStylePatch => simplePatch(value, "name", at);
export const parseCharacterStylePatch = (value: unknown, at = "$" ): CharacterStylePatch => simplePatch(value, "name", at);
export const parseParentPagePatch = (value: unknown, at = "$" ): ParentPagePatch => simplePatch(value, "name", at);
export const parseSpreadPatch = (value: unknown, at = "$" ): SpreadPatch => simplePatch(value, "name", at);
export const parseTextStoryPatch = (value: unknown, at = "$" ): TextStoryPatch => simplePatch(value, "content", at);
export const parseImageLinkPatch = (value: unknown, at = "$" ): ImageLinkPatch => simplePatch(value, "path", at);
export function parseFramePatch(value: unknown, at = "$" ): FramePatch {
  const keys = ["x", "y", "width", "height", "fill", "stroke", "wrap_mode", "columns"], row = record(value, keys, at);
  return { x: nullable(row.x, number, `${at}.x`), y: nullable(row.y, number, `${at}.y`), width: nullable(row.width, number, `${at}.width`), height: nullable(row.height, number, `${at}.height`), fill: nullable(row.fill, rgba, `${at}.fill`), stroke: nullable(row.stroke, rgba, `${at}.stroke`), wrap_mode: nullable(row.wrap_mode, string, `${at}.wrap_mode`), columns: nullable(row.columns, integer, `${at}.columns`) };
}
export function parsePageFrameAdded(value: unknown, at = "$" ): PageFrameAdded {
  const keys = ["frame", "index", "layer_id"], row = record(value, keys, at);
  return { frame: parseFrame(row.frame, `${at}.frame`), index: nullable(row.index, integer, `${at}.index`), layer_id: nullable(row.layer_id, string, `${at}.layer_id`) };
}
export function parsePagePatch(value: unknown, at = "$" ): PagePatch {
  const keys = ["name", "width", "height", "margin_top", "margin_right", "margin_bottom", "margin_left", "columns_count", "columns_gutter", "frame_added", "frame_removed", "frame_patched"], row = record(value, keys, at);
  let framePatched: PageFramePatched | null = null;
  if (row.frame_patched !== null) { const nested = record(row.frame_patched, ["frame_id", "patch"], `${at}.frame_patched`); framePatched = { frame_id: string(nested.frame_id, `${at}.frame_patched.frame_id`), patch: parseFramePatch(nested.patch, `${at}.frame_patched.patch`) }; }
  return { name: nullable(row.name, string, `${at}.name`), width: nullable(row.width, number, `${at}.width`), height: nullable(row.height, number, `${at}.height`), margin_top: nullable(row.margin_top, number, `${at}.margin_top`), margin_right: nullable(row.margin_right, number, `${at}.margin_right`), margin_bottom: nullable(row.margin_bottom, number, `${at}.margin_bottom`), margin_left: nullable(row.margin_left, number, `${at}.margin_left`), columns_count: nullable(row.columns_count, integer, `${at}.columns_count`), columns_gutter: nullable(row.columns_gutter, number, `${at}.columns_gutter`), frame_added: nullable(row.frame_added, parsePageFrameAdded, `${at}.frame_added`), frame_removed: nullable(row.frame_removed, string, `${at}.frame_removed`), frame_patched: framePatched };
}

type Delta<T, P> = { added: T[]; removed: string[]; patched: P[]; reordered: string[] | null };
const parseDelta = <T, P>(value: unknown, parseItem: (value: unknown, at: string) => T, parsePatch: (value: unknown, at: string) => P, at: string): Delta<T, { id: string; patch: P }> => {
  const keys = ["added", "removed", "patched", "reordered"], row = record(value, keys, at);
  return { added: array(row.added, `${at}.added`).map((item, index) => parseItem(item, `${at}.added[${index}]`)), removed: strings(row.removed, `${at}.removed`), patched: array(row.patched, `${at}.patched`).map((item, index) => { const path = `${at}.patched[${index}]`, entry = record(item, ["id", "patch"], path); return { id: string(entry.id, `${path}.id`), patch: parsePatch(entry.patch, `${path}.patch`) }; }), reordered: nullable(row.reordered, strings, `${at}.reordered`) };
};
export const parseLayoutPagesDelta = (value: unknown, at = "$" ): LayoutPagesDelta => parseDelta(value, parsePage, parsePagePatch, at);
export const parseLayoutStoriesDelta = (value: unknown, at = "$" ): LayoutStoriesDelta => parseDelta(value, parseTextStory, parseTextStoryPatch, at);
export const parseLayoutLinksDelta = (value: unknown, at = "$" ): LayoutLinksDelta => parseDelta(value, parseImageLink, parseImageLinkPatch, at);
export const parseLayoutParagraphStylesDelta = (value: unknown, at = "$" ): LayoutParagraphStylesDelta => parseDelta(value, parseParagraphStyle, parseParagraphStylePatch, at);
export const parseLayoutCharacterStylesDelta = (value: unknown, at = "$" ): LayoutCharacterStylesDelta => parseDelta(value, parseCharacterStyle, parseCharacterStylePatch, at);
export const parseLayoutParentPagesDelta = (value: unknown, at = "$" ): LayoutParentPagesDelta => parseDelta(value, parseParentPage, parseParentPagePatch, at);
export const parseLayoutSpreadsDelta = (value: unknown, at = "$" ): LayoutSpreadsDelta => parseDelta(value, parseSpread, parseSpreadPatch, at);

/** 🔺️ Parses the exact, full native diff record and rejects stale wrapper or window fields. */
export function parseLayoutDiff(value: unknown, at = "$" ): LayoutDiff {
  const keys = ["artifact", "schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFieldsJson", "backgroundDrawing", "referencedModel"], row = record(value, keys, at);
  return {
    artifact: nullable(row.artifact, parseLayoutArtifact, `${at}.artifact`), schema: nullable(row.schema, string, `${at}.schema`), name: nullable(row.name, string, `${at}.name`), grid: nullable(row.grid, parseGridSettings, `${at}.grid`),
    paragraphStyles: nullable(row.paragraphStyles, parseLayoutParagraphStylesDelta, `${at}.paragraphStyles`), characterStyles: nullable(row.characterStyles, parseLayoutCharacterStylesDelta, `${at}.characterStyles`), stories: nullable(row.stories, parseLayoutStoriesDelta, `${at}.stories`), links: nullable(row.links, parseLayoutLinksDelta, `${at}.links`), parentPages: nullable(row.parentPages, parseLayoutParentPagesDelta, `${at}.parentPages`), spreads: nullable(row.spreads, parseLayoutSpreadsDelta, `${at}.spreads`), pages: nullable(row.pages, parseLayoutPagesDelta, `${at}.pages`),
    printTarget: nullable(row.printTarget, string, `${at}.printTarget`), dataFieldsJson: nullable(row.dataFieldsJson, string, `${at}.dataFieldsJson`), backgroundDrawing: nullable(row.backgroundDrawing, parseLayoutDrawingChild, `${at}.backgroundDrawing`), referencedModel: nullable(row.referencedModel, parseArtifactLink, `${at}.referencedModel`),
  };
}

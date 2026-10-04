/** 🔺️ Canonical Layout diff with every native `ToValue` field present and nullable. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {
  parseFormDictionary,type FormDictionary, parseCharacterStyle, parseFrame, parseGridSettings, parseImageLink, parseLayoutArtifact, parseLayoutDrawingChild,
  parseLayer, parseLayoutRect, parsePage, parsePageOverride, parseParagraphStyle, parseParentPage, parseSpread, parseTextStory, parseTextStyleRun,
  type ArtifactLink, type CharacterStyle, type Frame, type GridSettings, type ImageLink, type Layer,
  type LayoutArtifact, type LayoutDrawingChild, type LayoutRect, type Page, type PageOverride, type ParagraphStyle, type ParentPage,
  type Spread, type TextStory, type TextStyleRun,
} from "../🟦️.ts";
import { parseArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export * from "../🟦️.ts";

export interface ParagraphStylePatch { name?: string | null; fontFamily?: string | null; fontSize?: number | null; fontWeight?: number | null; leading?: number | null; tracking?: number | null; alignment?: string | null }
export interface CharacterStylePatch { name?: string | null; fontFamily?: string | null; fontSize?: number | null; fontWeight?: number | null; italic?: boolean | null; color?: [number, number, number, number] | null; tracking?: number | null }
export interface ParentPagePatch { name?: string | null; width?: number | null; height?: number | null }
export interface SpreadPatch { name?: string | null }
export interface TextStoryPatch { content: string | null; style_runs?: TextStyleRun[] | null }
export interface ImageLinkPatch { path: string | null; width?: number | null; height?: number | null; dpi?: number | null; color_profile?: string | null }
export interface FramePatch { x: number | null; y: number | null; width: number | null; height: number | null; rotation?: number | null; fill: [number, number, number, number] | null; stroke: [number, number, number, number] | null; wrap_mode: string | null; columns: number | null; locked?: boolean | null; visible?: boolean | null; story_id?: string | null; thread_next?: string | null; inset_x?: number | null; inset_y?: number | null; inset_width?: number | null; inset_height?: number | null }
export interface PageFrameAdded { frame: Frame; index: number | null; layer_id: string | null }
export interface PageFramePatched { frame_id: string; patch: FramePatch }
export interface PageLayerPatched { layer_id: string; name: string | null; visible: boolean | null; locked: boolean | null }
export interface PageFrameLayer { frame_id: string; layer_id: string }
/** 📄️ One page's sparse patch: `frames_patched` lists every field-patched frame of the page in page order. */
export interface PagePatch { name: string | null; width: number | null; height: number | null; margin_top: number | null; margin_right: number | null; margin_bottom: number | null; margin_left: number | null; columns_count: number | null; columns_gutter: number | null; frame_added: PageFrameAdded | null; frame_removed: string | null; frames_patched: PageFramePatched[]; layer_patched?: PageLayerPatched | null; parent_page_id?: string | null; guides?: LayoutRect[] | null; overrides?: PageOverride[] | null; layer_added?: Layer | null; layer_removed?: string | null; frame_layer?: PageFrameLayer | null; frame_order?: string[] | null }
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
  /** @state artifact */ dataFields: {dictionary:FormDictionary|null} | null;
  /** @state artifact @child kind=s.stdio.semio */ backgroundDrawing: LayoutDrawingChild | null;
  /** @state artifact @link_slot roles=model */ referencedModel: ArtifactLink | null;
}

const required = (row: Record<string, unknown>, keys: readonly string[], at: string): Record<string, unknown> => {
  const exact = parseSchemaRecord(row, keys, at), missing = keys.find((key) => !Object.hasOwn(exact, key));
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
  return exact;
};
const record = (value: unknown, keys: readonly string[], at: string): Record<string, unknown> => required(parseSchemaRecord(value, keys, at), keys, at);
const partial = (value: unknown, keys: readonly string[], optional: readonly string[], at: string): Record<string, unknown> => {
  const row = parseSchemaRecord(value, [...keys, ...optional], at), missing = keys.find((key) => !Object.hasOwn(row, key));
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
  return row;
};
const optional = <K extends string, T>(row: Record<string, unknown>, key: K, parse: (value: unknown, at: string) => T, at: string): Partial<Record<K, T | null>> => Object.hasOwn(row, key) ? ({ [key]: nullable(row[key], parse, `${at}.${key}`) } as Partial<Record<K, T | null>>) : {};
const string = (value: unknown, at: string): string => { if (typeof value !== "string") throw new Error(`${at}: string required`); return value; };
const number = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: finite number required`); return value; };
const boolean = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new Error(`${at}: boolean required`); return value; };
const integer = (value: unknown, at: string): number => { const result = number(value, at); if (!Number.isSafeInteger(result) || result < 0) throw new Error(`${at}: unsigned integer required`); return result; };
const array = (value: unknown, at: string): unknown[] => { if (!Array.isArray(value)) throw new Error(`${at}: array required`); return value; };
const strings = (value: unknown, at: string): string[] => array(value, at).map((item, index) => string(item, `${at}[${index}]`));
const nullable = <T>(value: unknown, parse: (value: unknown, at: string) => T, at: string): T | null => value === null ? null : parse(value, at);
const rgba = (value: unknown, at: string): [number, number, number, number] => { const items = array(value, at).map((item, index) => number(item, `${at}[${index}]`)); if (items.length !== 4) throw new Error(`${at}: four numbers required`); return items as [number, number, number, number]; };

export function parseParagraphStylePatch(value: unknown, at = "$" ): ParagraphStylePatch {
  const row = partial(value, [], ["name", "fontFamily", "fontSize", "fontWeight", "leading", "tracking", "alignment"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "fontFamily", string, at), ...optional(row, "fontSize", number, at), ...optional(row, "fontWeight", integer, at), ...optional(row, "leading", number, at), ...optional(row, "tracking", number, at), ...optional(row, "alignment", string, at) };
}
export function parseCharacterStylePatch(value: unknown, at = "$" ): CharacterStylePatch {
  const row = partial(value, [], ["name", "fontFamily", "fontSize", "fontWeight", "italic", "color", "tracking"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "fontFamily", string, at), ...optional(row, "fontSize", number, at), ...optional(row, "fontWeight", integer, at), ...optional(row, "italic", boolean, at), ...optional(row, "color", rgba, at), ...optional(row, "tracking", number, at) };
}
export function parseParentPagePatch(value: unknown, at = "$" ): ParentPagePatch {
  const row = partial(value, [], ["name", "width", "height"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "width", number, at), ...optional(row, "height", number, at) };
}
export function parseSpreadPatch(value: unknown, at = "$" ): SpreadPatch {
  return optional(partial(value, [], ["name"], at), "name", string, at);
}
export function parseTextStoryPatch(value: unknown, at = "$" ): TextStoryPatch {
  const row = partial(value, ["content"], ["style_runs"], at);
  return { content: nullable(row.content, string, `${at}.content`), ...optional(row, "style_runs", (runs, path) => array(runs, path).map((run, index) => parseTextStyleRun(run, `${path}[${index}]`)), at) };
}
export function parseImageLinkPatch(value: unknown, at = "$" ): ImageLinkPatch {
  const row = partial(value, ["path"], ["width", "height", "dpi", "color_profile"], at);
  return { path: nullable(row.path, string, `${at}.path`), ...optional(row, "width", integer, at), ...optional(row, "height", integer, at), ...optional(row, "dpi", integer, at), ...optional(row, "color_profile", string, at) };
}
export function parseFramePatch(value: unknown, at = "$" ): FramePatch {
  const keys = ["x", "y", "width", "height", "fill", "stroke", "wrap_mode", "columns"], row = partial(value, keys, ["rotation", "locked", "visible", "story_id", "thread_next", "inset_x", "inset_y", "inset_width", "inset_height"], at);
  return {
    x: nullable(row.x, number, `${at}.x`), y: nullable(row.y, number, `${at}.y`), width: nullable(row.width, number, `${at}.width`), height: nullable(row.height, number, `${at}.height`), ...optional(row, "rotation", number, at), fill: nullable(row.fill, rgba, `${at}.fill`), stroke: nullable(row.stroke, rgba, `${at}.stroke`), wrap_mode: nullable(row.wrap_mode, string, `${at}.wrap_mode`), columns: nullable(row.columns, integer, `${at}.columns`),
    ...optional(row, "locked", boolean, at), ...optional(row, "visible", boolean, at), ...optional(row, "story_id", string, at), ...optional(row, "thread_next", string, at), ...optional(row, "inset_x", number, at), ...optional(row, "inset_y", number, at), ...optional(row, "inset_width", number, at), ...optional(row, "inset_height", number, at),
  };
}
export function parsePageFramePatched(value: unknown, at = "$" ): PageFramePatched {
  const row = record(value, ["frame_id", "patch"], at);
  return { frame_id: string(row.frame_id, `${at}.frame_id`), patch: parseFramePatch(row.patch, `${at}.patch`) };
}
export function parsePageLayerPatched(value: unknown, at = "$" ): PageLayerPatched {
  const row = record(value, ["layer_id", "name", "visible", "locked"], at);
  return { layer_id: string(row.layer_id, `${at}.layer_id`), name: nullable(row.name, string, `${at}.name`), visible: nullable(row.visible, boolean, `${at}.visible`), locked: nullable(row.locked, boolean, `${at}.locked`) };
}
export function parsePageFrameLayer(value: unknown, at = "$" ): PageFrameLayer {
  const row = record(value, ["frame_id", "layer_id"], at);
  return { frame_id: string(row.frame_id, `${at}.frame_id`), layer_id: string(row.layer_id, `${at}.layer_id`) };
}
export function parsePageFrameAdded(value: unknown, at = "$" ): PageFrameAdded {
  const keys = ["frame", "index", "layer_id"], row = record(value, keys, at);
  return { frame: parseFrame(row.frame, `${at}.frame`), index: nullable(row.index, integer, `${at}.index`), layer_id: nullable(row.layer_id, string, `${at}.layer_id`) };
}
export function parsePagePatch(value: unknown, at = "$" ): PagePatch {
  const keys = ["name", "width", "height", "margin_top", "margin_right", "margin_bottom", "margin_left", "columns_count", "columns_gutter", "frame_added", "frame_removed", "frames_patched"], row = partial(value, keys, ["layer_patched", "parent_page_id", "guides", "overrides", "layer_added", "layer_removed", "frame_layer", "frame_order"], at);
  const list = <T>(parse: (value: unknown, at: string) => T) => (value: unknown, path: string): T[] => array(value, path).map((item, index) => parse(item, `${path}[${index}]`));
  return {
    name: nullable(row.name, string, `${at}.name`), width: nullable(row.width, number, `${at}.width`), height: nullable(row.height, number, `${at}.height`), margin_top: nullable(row.margin_top, number, `${at}.margin_top`), margin_right: nullable(row.margin_right, number, `${at}.margin_right`), margin_bottom: nullable(row.margin_bottom, number, `${at}.margin_bottom`), margin_left: nullable(row.margin_left, number, `${at}.margin_left`), columns_count: nullable(row.columns_count, integer, `${at}.columns_count`), columns_gutter: nullable(row.columns_gutter, number, `${at}.columns_gutter`), frame_added: nullable(row.frame_added, parsePageFrameAdded, `${at}.frame_added`), frame_removed: nullable(row.frame_removed, string, `${at}.frame_removed`),
    frames_patched: list(parsePageFramePatched)(row.frames_patched, `${at}.frames_patched`),
    ...optional(row, "layer_patched", parsePageLayerPatched, at), ...optional(row, "parent_page_id", string, at), ...optional(row, "guides", list(parseLayoutRect), at), ...optional(row, "overrides", list(parsePageOverride), at), ...optional(row, "layer_added", parseLayer, at), ...optional(row, "layer_removed", string, at), ...optional(row, "frame_layer", parsePageFrameLayer, at), ...optional(row, "frame_order", strings, at),
  };
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
  const keys = ["artifact", "schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFields", "backgroundDrawing", "referencedModel"], row = record(value, keys, at);
  return {
    artifact: nullable(row.artifact, parseLayoutArtifact, `${at}.artifact`), schema: nullable(row.schema, string, `${at}.schema`), name: nullable(row.name, string, `${at}.name`), grid: nullable(row.grid, parseGridSettings, `${at}.grid`),
    paragraphStyles: nullable(row.paragraphStyles, parseLayoutParagraphStylesDelta, `${at}.paragraphStyles`), characterStyles: nullable(row.characterStyles, parseLayoutCharacterStylesDelta, `${at}.characterStyles`), stories: nullable(row.stories, parseLayoutStoriesDelta, `${at}.stories`), links: nullable(row.links, parseLayoutLinksDelta, `${at}.links`), parentPages: nullable(row.parentPages, parseLayoutParentPagesDelta, `${at}.parentPages`), spreads: nullable(row.spreads, parseLayoutSpreadsDelta, `${at}.spreads`), pages: nullable(row.pages, parseLayoutPagesDelta, `${at}.pages`),
    printTarget: nullable(row.printTarget, string, `${at}.printTarget`), dataFields: nullable(row.dataFields, (value,at)=>{const change=required(record(value,["dictionary"],at),["dictionary"],at);return{dictionary:nullable(change.dictionary,parseFormDictionary,`${at}.dictionary`)};}, `${at}.dataFields`), backgroundDrawing: nullable(row.backgroundDrawing, parseLayoutDrawingChild, `${at}.backgroundDrawing`), referencedModel: nullable(row.referencedModel, parseArtifactLink, `${at}.referencedModel`),
  };
}

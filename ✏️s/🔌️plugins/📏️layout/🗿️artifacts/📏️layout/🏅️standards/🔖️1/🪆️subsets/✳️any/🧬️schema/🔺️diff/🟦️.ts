/** 🔺️ Canonical Layout diff with every native `ToValue` field present and nullable. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseFormDictionary,type FormDictionary,parseCharacterStyle,parseFrame,parseImageLink,parseLayoutDrawingChild,parseLayer,parseLayoutRect,parsePage,parsePageOverride,parseParagraphStyle,parseParentPage,parseSpread,parseTextStory,parseTextStyleRun,type ArtifactLink,type CharacterStyle,type Frame,type ImageLink,type Layer,type LayoutDrawingChild,type LayoutRect,type Page,type PageOverride,type ParagraphStyle,type ParentPage,type Spread,type TextStory,type TextStyleRun} from "../🟦️.ts";
import { parseArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export * from "../🟦️.ts";
import {parseBinary64,parseBinary32,type Binary64,type Binary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

type FormDictionaryEntry = FormDictionary["entries"][number];
function parseFormDictionaryEntry(candidate: unknown, at: string): FormDictionaryEntry { return parseFormDictionary({ entries: [candidate] }, at).entries[0]; }

export interface ParagraphStylePatch { name?: string | null; fontFamily?: string | null; fontSize?: Binary64 | null; fontWeight?: number | null; leading?: Binary64 | null; tracking?: Binary64 | null; alignment?: string | null }
export interface CharacterStylePatch { name?: string | null; fontFamily?: string | null; fontSize?: Binary64 | null; fontWeight?: number | null; italic?: boolean | null; color?: [Binary32, Binary32, Binary32, Binary32] | null; tracking?: Binary64 | null }
export interface ParentPagePatch { name?: string | null; width?: Binary64 | null; height?: Binary64 | null }
export interface SpreadPatch { name?: string | null }
export interface LayoutInsertion<T> { index: number; row: T }
export interface LayoutRemoval { id: string; index: number }
export interface LayoutRelocation { id: string; from: number; to: number }
export interface TextStyleRunRow { index: number; run: TextStyleRun }
export interface TextStyleRunsDelta { len?: number | null; rows: TextStyleRunRow[] }
export interface TextStoryPatch { content: string | null; style_runs?: TextStyleRunsDelta | null }
export interface PageGuideRow { index: number; guide: LayoutRect }
export interface PageGuidesDelta { len?: number | null; rows: PageGuideRow[] }
export interface PageOverridesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<PageOverride>[]; moved: LayoutRelocation[] }
export interface GridPatch { baselineGrid?: Binary64 | null; baselineOffset?: Binary64 | null; snapToBaseline?: boolean | null }
export interface PrintTargetChange { target: string | null }
export type DataFieldsPresence = "created" | "deleted" | "replaced";
export interface LayoutDataEntriesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<FormDictionaryEntry>[]; moved: LayoutRelocation[] }
export interface LayoutDataFieldsDelta { presence?: DataFieldsPresence | null; entries?: LayoutDataEntriesDelta | null }
export interface LayoutDrawingTextRow { index: number; text: string }
export interface ImageLinkPatch { path: string | null; width?: number | null; height?: number | null; dpi?: number | null; color_profile?: string | null }
export interface FramePatch { x: Binary64 | null; y: Binary64 | null; width: Binary64 | null; height: Binary64 | null; rotation?: Binary64 | null; fill: [Binary32, Binary32, Binary32, Binary32] | null; stroke: [Binary32, Binary32, Binary32, Binary32] | null; wrap_mode: string | null; columns: number | null; locked?: boolean | null; visible?: boolean | null; story_id?: string | null; thread_next?: string | null; inset_x?: Binary64 | null; inset_y?: Binary64 | null; inset_width?: Binary64 | null; inset_height?: Binary64 | null; layer_id?: string | null }
export interface LayerPatch { name?: string | null; visible?: boolean | null; locked?: boolean | null }
export interface PageFramesModification { id: string; patch: FramePatch }
export interface PageFramesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<Frame>[]; moved: LayoutRelocation[]; modified: PageFramesModification[] }
export interface PageLayersModification { id: string; patch: LayerPatch }
export interface PageLayersDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<Layer>[]; moved: LayoutRelocation[]; modified: PageLayersModification[] }
/** 📄️ One page's sparse patch: its fields, the positional guides and the keyed override, frame and layer rows. */
export interface PagePatch { name: string | null; width: Binary64 | null; height: Binary64 | null; margin_top: Binary64 | null; margin_right: Binary64 | null; margin_bottom: Binary64 | null; margin_left: Binary64 | null; columns_count: number | null; columns_gutter: Binary64 | null; parent_page_id?: string | null; guides?: PageGuidesDelta | null; overrides: PageOverridesDelta; frames: PageFramesDelta; layers: PageLayersDelta }
export interface LayoutPagesModification { id: string; patch: PagePatch }
export interface LayoutStoriesModification { id: string; patch: TextStoryPatch }
export interface LayoutLinksModification { id: string; patch: ImageLinkPatch }
export interface LayoutParagraphStylesModification { id: string; patch: ParagraphStylePatch }
export interface LayoutCharacterStylesModification { id: string; patch: CharacterStylePatch }
export interface LayoutParentPagesModification { id: string; patch: ParentPagePatch }
export interface LayoutSpreadsModification { id: string; patch: SpreadPatch }
export interface LayoutPagesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<Page>[]; moved: LayoutRelocation[]; modified: LayoutPagesModification[] }
export interface LayoutStoriesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<TextStory>[]; moved: LayoutRelocation[]; modified: LayoutStoriesModification[] }
export interface LayoutLinksDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<ImageLink>[]; moved: LayoutRelocation[]; modified: LayoutLinksModification[] }
export interface LayoutParagraphStylesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<ParagraphStyle>[]; moved: LayoutRelocation[]; modified: LayoutParagraphStylesModification[] }
export interface LayoutCharacterStylesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<CharacterStyle>[]; moved: LayoutRelocation[]; modified: LayoutCharacterStylesModification[] }
export interface LayoutParentPagesDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<ParentPage>[]; moved: LayoutRelocation[]; modified: LayoutParentPagesModification[] }
export interface LayoutSpreadsDelta { removed: LayoutRemoval[]; inserted: LayoutInsertion<Spread>[]; moved: LayoutRelocation[]; modified: LayoutSpreadsModification[] }
export interface LayoutDiff {
  /** @state artifact */ schema: string | null;
  /** @state artifact */ name: string | null;
  /** @state artifact */ grid: GridPatch | null;
  /** @state artifact */ paragraphStyles: LayoutParagraphStylesDelta | null;
  /** @state artifact */ characterStyles: LayoutCharacterStylesDelta | null;
  /** @state artifact */ stories: LayoutStoriesDelta | null;
  /** @state artifact */ links: LayoutLinksDelta | null;
  /** @state artifact */ parentPages: LayoutParentPagesDelta | null;
  /** @state artifact */ spreads: LayoutSpreadsDelta | null;
  /** @state artifact */ pages: LayoutPagesDelta | null;
  /** @state artifact */ printTarget: PrintTargetChange | null;
  /** @state artifact */ dataFields: LayoutDataFieldsDelta | null;
  /** @state artifact @child kind=s.stdio.semio */ backgroundDrawing: LayoutDrawingChild | null;
  /** @state artifact */ drawingTexts: LayoutDrawingTextRow[];
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
const nullable = <T>(value: unknown, parse: (value: unknown, at: string) => T, at: string): T | null => value === null ? null : parse(value, at);
const word = (value: unknown, _at: string): Binary64 => parseBinary64(value);
const rgba = (value: unknown, at: string): [Binary32, Binary32, Binary32, Binary32] => { const items = array(value, at).map(parseBinary32); if (items.length !== 4) throw new Error(`${at}: four numbers required`); return items as [Binary32, Binary32, Binary32, Binary32]; };

export function parseParagraphStylePatch(value: unknown, at = "$" ): ParagraphStylePatch {
  const row = partial(value, [], ["name", "fontFamily", "fontSize", "fontWeight", "leading", "tracking", "alignment"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "fontFamily", string, at), ...optional(row, "fontSize", word, at), ...optional(row, "fontWeight", integer, at), ...optional(row, "leading", word, at), ...optional(row, "tracking", word, at), ...optional(row, "alignment", string, at) };
}
export function parseCharacterStylePatch(value: unknown, at = "$" ): CharacterStylePatch {
  const row = partial(value, [], ["name", "fontFamily", "fontSize", "fontWeight", "italic", "color", "tracking"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "fontFamily", string, at), ...optional(row, "fontSize", word, at), ...optional(row, "fontWeight", integer, at), ...optional(row, "italic", boolean, at), ...optional(row, "color", rgba, at), ...optional(row, "tracking", word, at) };
}
export function parseParentPagePatch(value: unknown, at = "$" ): ParentPagePatch {
  const row = partial(value, [], ["name", "width", "height"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "width", word, at), ...optional(row, "height", word, at) };
}
export function parseSpreadPatch(value: unknown, at = "$" ): SpreadPatch {
  return optional(partial(value, [], ["name"], at), "name", string, at);
}
export function parseTextStyleRunsDelta(value: unknown, at = "$" ): TextStyleRunsDelta {
  const row = partial(value, ["rows"], ["len"], at);
  return { ...optional(row, "len", integer, at), rows: array(row.rows, `${at}.rows`).map((item, index) => { const path = `${at}.rows[${index}]`, entry = record(item, ["index", "run"], path); return { index: integer(entry.index, `${path}.index`), run: parseTextStyleRun(entry.run, `${path}.run`) }; }) };
}
export function parsePageGuidesDelta(value: unknown, at = "$" ): PageGuidesDelta {
  const row = partial(value, ["rows"], ["len"], at);
  return { ...optional(row, "len", integer, at), rows: array(row.rows, `${at}.rows`).map((item, index) => { const path = `${at}.rows[${index}]`, entry = record(item, ["index", "guide"], path); return { index: integer(entry.index, `${path}.index`), guide: parseLayoutRect(entry.guide, `${path}.guide`) }; }) };
}
const parseInserted = <T>(value: unknown, parseRow: (value: unknown, at: string) => T, at: string): LayoutInsertion<T>[] => array(value, at).map((item, index) => { const path = `${at}[${index}]`, entry = record(item, ["index", "row"], path); return { index: integer(entry.index, `${path}.index`), row: parseRow(entry.row, `${path}.row`) }; });
const parseRemovals = (value: unknown, at: string): LayoutRemoval[] => array(value, at).map((item, index) => { const path = `${at}[${index}]`, entry = record(item, ["id", "index"], path); return { id: string(entry.id, `${path}.id`), index: integer(entry.index, `${path}.index`) }; });
const parseRelocations = (value: unknown, at: string): LayoutRelocation[] => array(value, at).map((item, index) => { const path = `${at}[${index}]`, entry = record(item, ["id", "from", "to"], path); return { id: string(entry.id, `${path}.id`), from: integer(entry.from, `${path}.from`), to: integer(entry.to, `${path}.to`) }; });
export function parsePageOverridesDelta(value: unknown, at = "$" ): PageOverridesDelta {
  const row = record(value, ["removed", "inserted", "moved"], at);
  return { removed: parseRemovals(row.removed, `${at}.removed`), inserted: parseInserted(row.inserted, parsePageOverride, `${at}.inserted`), moved: parseRelocations(row.moved, `${at}.moved`) };
}
export function parseGridPatch(value: unknown, at = "$" ): GridPatch {
  const row = partial(value, [], ["baselineGrid", "baselineOffset", "snapToBaseline"], at);
  return { ...optional(row, "baselineGrid", word, at), ...optional(row, "baselineOffset", word, at), ...optional(row, "snapToBaseline", boolean, at) };
}
export function parseLayoutDataFieldsDelta(value: unknown, at = "$" ): LayoutDataFieldsDelta {
  const row = partial(value, [], ["presence", "entries"], at);
  const presence = (item: unknown, path: string): DataFieldsPresence => { if (item !== "created" && item !== "deleted" && item !== "replaced") throw new Error(`${path}: created, deleted or replaced required`); return item; };
  const entries = (item: unknown, path: string): LayoutDataEntriesDelta => {
    const entry = record(item, ["removed", "inserted", "moved"], path);
    return { removed: parseRemovals(entry.removed, `${path}.removed`), inserted: parseInserted(entry.inserted, parseFormDictionaryEntry, `${path}.inserted`), moved: parseRelocations(entry.moved, `${path}.moved`) };
  };
  return { ...optional(row, "presence", presence, at), ...optional(row, "entries", entries, at) };
}
export function parseTextStoryPatch(value: unknown, at = "$" ): TextStoryPatch {
  const row = partial(value, ["content"], ["style_runs"], at);
  return { content: nullable(row.content, string, `${at}.content`), ...optional(row, "style_runs", parseTextStyleRunsDelta, at) };
}
export function parseImageLinkPatch(value: unknown, at = "$" ): ImageLinkPatch {
  const row = partial(value, ["path"], ["width", "height", "dpi", "color_profile"], at);
  return { path: nullable(row.path, string, `${at}.path`), ...optional(row, "width", integer, at), ...optional(row, "height", integer, at), ...optional(row, "dpi", integer, at), ...optional(row, "color_profile", string, at) };
}
export function parseFramePatch(value: unknown, at = "$" ): FramePatch {
  const keys = ["x", "y", "width", "height", "fill", "stroke", "wrap_mode", "columns"], row = partial(value, keys, ["rotation", "locked", "visible", "story_id", "thread_next", "inset_x", "inset_y", "inset_width", "inset_height", "layer_id"], at);
  return {
    x: nullable(row.x, word, `${at}.x`), y: nullable(row.y, word, `${at}.y`), width: nullable(row.width, word, `${at}.width`), height: nullable(row.height, word, `${at}.height`), ...optional(row, "rotation", word, at), fill: nullable(row.fill, rgba, `${at}.fill`), stroke: nullable(row.stroke, rgba, `${at}.stroke`), wrap_mode: nullable(row.wrap_mode, string, `${at}.wrap_mode`), columns: nullable(row.columns, integer, `${at}.columns`),
    ...optional(row, "locked", boolean, at), ...optional(row, "visible", boolean, at), ...optional(row, "story_id", string, at), ...optional(row, "thread_next", string, at), ...optional(row, "inset_x", word, at), ...optional(row, "inset_y", word, at), ...optional(row, "inset_width", word, at), ...optional(row, "inset_height", word, at), ...optional(row, "layer_id", string, at),
  };
}
const parseDeltaRows = <T, P>(value: unknown, parseItem: (value: unknown, at: string) => T, parsePatch: ((value: unknown, at: string) => P) | null, at: string) => {
  const row = record(value, parsePatch ? ["removed", "inserted", "moved", "modified"] : ["removed", "inserted", "moved"], at);
  const base = { removed: parseRemovals(row.removed, `${at}.removed`), inserted: parseInserted(row.inserted, parseItem, `${at}.inserted`), moved: parseRelocations(row.moved, `${at}.moved`) };
  return parsePatch ? { ...base, modified: array(row.modified, `${at}.modified`).map((item, index) => { const path = `${at}.modified[${index}]`, entry = record(item, ["id", "patch"], path); return { id: string(entry.id, `${path}.id`), patch: parsePatch(entry.patch, `${path}.patch`) }; }) } : base;
};
export function parseLayerPatch(value: unknown, at = "$" ): LayerPatch {
  const row = partial(value, [], ["name", "visible", "locked"], at);
  return { ...optional(row, "name", string, at), ...optional(row, "visible", boolean, at), ...optional(row, "locked", boolean, at) };
}
export const parsePageFramesDelta = (value: unknown, at = "$" ): PageFramesDelta => parseDeltaRows(value, parseFrame, parseFramePatch, at) as PageFramesDelta;
export const parsePageLayersDelta = (value: unknown, at = "$" ): PageLayersDelta => parseDeltaRows(value, parseLayer, parseLayerPatch, at) as PageLayersDelta;
export function parsePagePatch(value: unknown, at = "$" ): PagePatch {
  const keys = ["name", "width", "height", "margin_top", "margin_right", "margin_bottom", "margin_left", "columns_count", "columns_gutter", "overrides", "frames", "layers"], row = partial(value, keys, ["parent_page_id", "guides"], at);
  return {
    name: nullable(row.name, string, `${at}.name`), width: nullable(row.width, word, `${at}.width`), height: nullable(row.height, word, `${at}.height`), margin_top: nullable(row.margin_top, word, `${at}.margin_top`), margin_right: nullable(row.margin_right, word, `${at}.margin_right`), margin_bottom: nullable(row.margin_bottom, word, `${at}.margin_bottom`), margin_left: nullable(row.margin_left, word, `${at}.margin_left`), columns_count: nullable(row.columns_count, integer, `${at}.columns_count`), columns_gutter: nullable(row.columns_gutter, word, `${at}.columns_gutter`),
    ...optional(row, "parent_page_id", string, at), ...optional(row, "guides", parsePageGuidesDelta, at),
    overrides: parsePageOverridesDelta(row.overrides, `${at}.overrides`), frames: parsePageFramesDelta(row.frames, `${at}.frames`), layers: parsePageLayersDelta(row.layers, `${at}.layers`),
  };
}

const parseDelta = <T, P>(value: unknown, parseItem: (value: unknown, at: string) => T, parsePatch: (value: unknown, at: string) => P, at: string) => parseDeltaRows(value, parseItem, parsePatch, at);
export const parseLayoutPagesDelta = (value: unknown, at = "$" ): LayoutPagesDelta => parseDelta(value, parsePage, parsePagePatch, at) as unknown as LayoutPagesDelta;
export const parseLayoutStoriesDelta = (value: unknown, at = "$" ): LayoutStoriesDelta => parseDelta(value, parseTextStory, parseTextStoryPatch, at) as unknown as LayoutStoriesDelta;
export const parseLayoutLinksDelta = (value: unknown, at = "$" ): LayoutLinksDelta => parseDelta(value, parseImageLink, parseImageLinkPatch, at) as unknown as LayoutLinksDelta;
export const parseLayoutParagraphStylesDelta = (value: unknown, at = "$" ): LayoutParagraphStylesDelta => parseDelta(value, parseParagraphStyle, parseParagraphStylePatch, at) as unknown as LayoutParagraphStylesDelta;
export const parseLayoutCharacterStylesDelta = (value: unknown, at = "$" ): LayoutCharacterStylesDelta => parseDelta(value, parseCharacterStyle, parseCharacterStylePatch, at) as unknown as LayoutCharacterStylesDelta;
export const parseLayoutParentPagesDelta = (value: unknown, at = "$" ): LayoutParentPagesDelta => parseDelta(value, parseParentPage, parseParentPagePatch, at) as unknown as LayoutParentPagesDelta;
export const parseLayoutSpreadsDelta = (value: unknown, at = "$" ): LayoutSpreadsDelta => parseDelta(value, parseSpread, parseSpreadPatch, at) as unknown as LayoutSpreadsDelta;

/** 🔺️ Parses the exact, full native diff record and rejects stale wrapper or window fields. */
export function parseLayoutDiff(value: unknown, at = "$" ): LayoutDiff {
  const keys = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFields", "backgroundDrawing", "drawingTexts", "referencedModel"], row = record(value, keys, at);
  return {
    schema: nullable(row.schema, string, `${at}.schema`), name: nullable(row.name, string, `${at}.name`), grid: nullable(row.grid, parseGridPatch, `${at}.grid`),
    paragraphStyles: nullable(row.paragraphStyles, parseLayoutParagraphStylesDelta, `${at}.paragraphStyles`), characterStyles: nullable(row.characterStyles, parseLayoutCharacterStylesDelta, `${at}.characterStyles`), stories: nullable(row.stories, parseLayoutStoriesDelta, `${at}.stories`), links: nullable(row.links, parseLayoutLinksDelta, `${at}.links`), parentPages: nullable(row.parentPages, parseLayoutParentPagesDelta, `${at}.parentPages`), spreads: nullable(row.spreads, parseLayoutSpreadsDelta, `${at}.spreads`), pages: nullable(row.pages, parseLayoutPagesDelta, `${at}.pages`),
    printTarget: nullable(row.printTarget, (value, at) => ({ target: nullable(record(value, ["target"], at).target, string, `${at}.target`) }), `${at}.printTarget`), dataFields: nullable(row.dataFields, parseLayoutDataFieldsDelta, `${at}.dataFields`), backgroundDrawing: nullable(row.backgroundDrawing, parseLayoutDrawingChild, `${at}.backgroundDrawing`),
    drawingTexts: array(row.drawingTexts, `${at}.drawingTexts`).map((item, index) => { const path = `${at}.drawingTexts[${index}]`, entry = record(item, ["index", "text"], path); return { index: integer(entry.index, `${path}.index`), text: string(entry.text, `${path}.text`) }; }), referencedModel: nullable(row.referencedModel, parseArtifactLink, `${at}.referencedModel`),
  };
}

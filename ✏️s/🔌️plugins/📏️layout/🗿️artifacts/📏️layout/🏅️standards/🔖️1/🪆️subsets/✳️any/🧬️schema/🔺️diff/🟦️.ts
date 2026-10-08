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
export interface TextStyleRunRow { index: number; run: TextStyleRun }
export interface TextStyleRunsDelta { len?: number | null; rows: TextStyleRunRow[] }
export interface TextStoryPatch { content: string | null; style_runs?: TextStyleRunsDelta | null }
export interface PageGuideRow { index: number; guide: LayoutRect }
export interface PageGuidesDelta { len?: number | null; rows: PageGuideRow[] }
export interface PageOverridePatchEntry { id: string; item: PageOverride }
export interface PageOverridesDelta { added: PageOverride[]; removed: string[]; patched: PageOverridePatchEntry[]; reordered: string[] | null }
export interface GridPatch { baselineGrid?: Binary64 | null; baselineOffset?: Binary64 | null; snapToBaseline?: boolean | null }
export interface PrintTargetChange { target: string | null }
export type DataFieldsPresence = "created" | "deleted" | "replaced";
export interface LayoutDataEntryPatchEntry { id: string; item: FormDictionaryEntry }
export interface LayoutDataEntriesDelta { added: FormDictionaryEntry[]; removed: string[]; patched: LayoutDataEntryPatchEntry[]; reordered: string[] | null }
export interface LayoutDataFieldsDelta { presence?: DataFieldsPresence | null; entries?: LayoutDataEntriesDelta | null }
export interface LayoutDrawingTextRow { index: number; text: string }
export interface ImageLinkPatch { path: string | null; width?: number | null; height?: number | null; dpi?: number | null; color_profile?: string | null }
export interface FramePatch { x: Binary64 | null; y: Binary64 | null; width: Binary64 | null; height: Binary64 | null; rotation?: Binary64 | null; fill: [Binary32, Binary32, Binary32, Binary32] | null; stroke: [Binary32, Binary32, Binary32, Binary32] | null; wrap_mode: string | null; columns: number | null; locked?: boolean | null; visible?: boolean | null; story_id?: string | null; thread_next?: string | null; inset_x?: Binary64 | null; inset_y?: Binary64 | null; inset_width?: Binary64 | null; inset_height?: Binary64 | null }
export interface PageLayerAdded { layer: Layer; index: number | null }
export interface PageFrameAdded { frame: Frame; index: number | null; layer_id: string | null }
export interface PageFramePatched { frame_id: string; patch: FramePatch }
export interface PageLayerPatched { layer_id: string; name: string | null; visible: boolean | null; locked: boolean | null }
export interface PageFrameLayer { frame_id: string; layer_id: string }
/** 📄️ One page's sparse patch: `frames_patched` lists every field-patched frame of the page in page order. */
export interface PagePatch { name: string | null; width: Binary64 | null; height: Binary64 | null; margin_top: Binary64 | null; margin_right: Binary64 | null; margin_bottom: Binary64 | null; margin_left: Binary64 | null; columns_count: number | null; columns_gutter: Binary64 | null; frame_added: PageFrameAdded | null; frame_removed: string | null; frames_patched: PageFramePatched[]; layer_patched?: PageLayerPatched | null; parent_page_id?: string | null; guides?: PageGuidesDelta | null; overrides?: PageOverridesDelta | null; layer_added?: PageLayerAdded | null; layer_removed?: string | null; frame_layer?: PageFrameLayer | null; frame_order?: string[] | null }
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
const strings = (value: unknown, at: string): string[] => array(value, at).map((item, index) => string(item, `${at}[${index}]`));
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
export function parsePageOverridesDelta(value: unknown, at = "$" ): PageOverridesDelta {
  const row = record(value, ["added", "removed", "patched", "reordered"], at);
  return {
    added: array(row.added, `${at}.added`).map((item, index) => parsePageOverride(item, `${at}.added[${index}]`)), removed: strings(row.removed, `${at}.removed`),
    patched: array(row.patched, `${at}.patched`).map((item, index) => { const path = `${at}.patched[${index}]`, entry = record(item, ["id", "item"], path); return { id: string(entry.id, `${path}.id`), item: parsePageOverride(entry.item, `${path}.item`) }; }),
    reordered: nullable(row.reordered, strings, `${at}.reordered`),
  };
}
export function parseGridPatch(value: unknown, at = "$" ): GridPatch {
  const row = partial(value, [], ["baselineGrid", "baselineOffset", "snapToBaseline"], at);
  return { ...optional(row, "baselineGrid", word, at), ...optional(row, "baselineOffset", word, at), ...optional(row, "snapToBaseline", boolean, at) };
}
export function parseLayoutDataFieldsDelta(value: unknown, at = "$" ): LayoutDataFieldsDelta {
  const row = partial(value, [], ["presence", "entries"], at);
  const presence = (item: unknown, path: string): DataFieldsPresence => { if (item !== "created" && item !== "deleted" && item !== "replaced") throw new Error(`${path}: created, deleted or replaced required`); return item; };
  const entries = (item: unknown, path: string): LayoutDataEntriesDelta => {
    const entry = record(item, ["added", "removed", "patched", "reordered"], path);
    return {
      added: array(entry.added, `${path}.added`).map((candidate, index) => parseFormDictionaryEntry(candidate, `${path}.added[${index}]`)), removed: strings(entry.removed, `${path}.removed`),
      patched: array(entry.patched, `${path}.patched`).map((candidate, index) => { const here = `${path}.patched[${index}]`, patch = record(candidate, ["id", "item"], here); return { id: string(patch.id, `${here}.id`), item: parseFormDictionaryEntry(patch.item, `${here}.item`) }; }),
      reordered: nullable(entry.reordered, strings, `${path}.reordered`),
    };
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
  const keys = ["x", "y", "width", "height", "fill", "stroke", "wrap_mode", "columns"], row = partial(value, keys, ["rotation", "locked", "visible", "story_id", "thread_next", "inset_x", "inset_y", "inset_width", "inset_height"], at);
  return {
    x: nullable(row.x, word, `${at}.x`), y: nullable(row.y, word, `${at}.y`), width: nullable(row.width, word, `${at}.width`), height: nullable(row.height, word, `${at}.height`), ...optional(row, "rotation", word, at), fill: nullable(row.fill, rgba, `${at}.fill`), stroke: nullable(row.stroke, rgba, `${at}.stroke`), wrap_mode: nullable(row.wrap_mode, string, `${at}.wrap_mode`), columns: nullable(row.columns, integer, `${at}.columns`),
    ...optional(row, "locked", boolean, at), ...optional(row, "visible", boolean, at), ...optional(row, "story_id", string, at), ...optional(row, "thread_next", string, at), ...optional(row, "inset_x", word, at), ...optional(row, "inset_y", word, at), ...optional(row, "inset_width", word, at), ...optional(row, "inset_height", word, at),
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
export function parsePageLayerAdded(value: unknown, at = "$" ): PageLayerAdded {
  const row = record(value, ["layer", "index"], at);
  return { layer: parseLayer(row.layer, `${at}.layer`), index: nullable(row.index, integer, `${at}.index`) };
}
export function parsePageFrameAdded(value: unknown, at = "$" ): PageFrameAdded {
  const keys = ["frame", "index", "layer_id"], row = record(value, keys, at);
  return { frame: parseFrame(row.frame, `${at}.frame`), index: nullable(row.index, integer, `${at}.index`), layer_id: nullable(row.layer_id, string, `${at}.layer_id`) };
}
export function parsePagePatch(value: unknown, at = "$" ): PagePatch {
  const keys = ["name", "width", "height", "margin_top", "margin_right", "margin_bottom", "margin_left", "columns_count", "columns_gutter", "frame_added", "frame_removed", "frames_patched"], row = partial(value, keys, ["layer_patched", "parent_page_id", "guides", "overrides", "layer_added", "layer_removed", "frame_layer", "frame_order"], at);
  const list = <T>(parse: (value: unknown, at: string) => T) => (value: unknown, path: string): T[] => array(value, path).map((item, index) => parse(item, `${path}[${index}]`));
  return {
    name: nullable(row.name, string, `${at}.name`), width: nullable(row.width, word, `${at}.width`), height: nullable(row.height, word, `${at}.height`), margin_top: nullable(row.margin_top, word, `${at}.margin_top`), margin_right: nullable(row.margin_right, word, `${at}.margin_right`), margin_bottom: nullable(row.margin_bottom, word, `${at}.margin_bottom`), margin_left: nullable(row.margin_left, word, `${at}.margin_left`), columns_count: nullable(row.columns_count, integer, `${at}.columns_count`), columns_gutter: nullable(row.columns_gutter, word, `${at}.columns_gutter`), frame_added: nullable(row.frame_added, parsePageFrameAdded, `${at}.frame_added`), frame_removed: nullable(row.frame_removed, string, `${at}.frame_removed`),
    frames_patched: list(parsePageFramePatched)(row.frames_patched, `${at}.frames_patched`),
    ...optional(row, "layer_patched", parsePageLayerPatched, at), ...optional(row, "parent_page_id", string, at), ...optional(row, "guides", parsePageGuidesDelta, at), ...optional(row, "overrides", parsePageOverridesDelta, at), ...optional(row, "layer_added", parsePageLayerAdded, at), ...optional(row, "layer_removed", string, at), ...optional(row, "frame_layer", parsePageFrameLayer, at), ...optional(row, "frame_order", strings, at),
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
  const keys = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFields", "backgroundDrawing", "drawingTexts", "referencedModel"], row = record(value, keys, at);
  return {
    schema: nullable(row.schema, string, `${at}.schema`), name: nullable(row.name, string, `${at}.name`), grid: nullable(row.grid, parseGridPatch, `${at}.grid`),
    paragraphStyles: nullable(row.paragraphStyles, parseLayoutParagraphStylesDelta, `${at}.paragraphStyles`), characterStyles: nullable(row.characterStyles, parseLayoutCharacterStylesDelta, `${at}.characterStyles`), stories: nullable(row.stories, parseLayoutStoriesDelta, `${at}.stories`), links: nullable(row.links, parseLayoutLinksDelta, `${at}.links`), parentPages: nullable(row.parentPages, parseLayoutParentPagesDelta, `${at}.parentPages`), spreads: nullable(row.spreads, parseLayoutSpreadsDelta, `${at}.spreads`), pages: nullable(row.pages, parseLayoutPagesDelta, `${at}.pages`),
    printTarget: nullable(row.printTarget, (value, at) => ({ target: nullable(record(value, ["target"], at).target, string, `${at}.target`) }), `${at}.printTarget`), dataFields: nullable(row.dataFields, parseLayoutDataFieldsDelta, `${at}.dataFields`), backgroundDrawing: nullable(row.backgroundDrawing, parseLayoutDrawingChild, `${at}.backgroundDrawing`),
    drawingTexts: array(row.drawingTexts, `${at}.drawingTexts`).map((item, index) => { const path = `${at}.drawingTexts[${index}]`, entry = record(item, ["index", "text"], path); return { index: integer(entry.index, `${path}.index`), text: string(entry.text, `${path}.text`) }; }), referencedModel: nullable(row.referencedModel, parseArtifactLink, `${at}.referencedModel`),
  };
}

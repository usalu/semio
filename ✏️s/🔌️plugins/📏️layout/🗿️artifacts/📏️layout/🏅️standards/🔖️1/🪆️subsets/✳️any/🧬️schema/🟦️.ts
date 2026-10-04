/** 🧬️ Canonical Layout document schema. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export type { ArtifactChild, ArtifactLink };
import {type Binary64,type Binary32,parseBinary64Transport,parseBinary32Transport} from '../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts';
import {parseSemioDrawingSnapshot,type SemioDrawingSnapshot} from '../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🟦️.ts';
import{parseFormDictionary,type FormDictionary}from"../../../../../../../../📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🟦️.ts";
export{parseFormDictionary,type FormDictionary};

export interface GridSettings { baselineGrid: Binary64; baselineOffset: Binary64; snapToBaseline: boolean }
export interface ParagraphStyle { id: string; name: string; fontFamily: string; fontSize: Binary64; fontWeight: number; leading: Binary64; tracking: Binary64; alignment: string }
export interface CharacterStyle { id: string; name: string | null; fontFamily: string | null; fontSize: Binary64 | null; fontWeight: number | null; italic: boolean | null; color: [Binary32, Binary32, Binary32, Binary32] | null; tracking: Binary64 | null }
export interface TextStyleRun { start: bigint; end: bigint; paragraphStyleId: string | null; characterStyleId: string | null }
export interface TextStory { id: string; content: string; styleRuns: TextStyleRun[] }
export interface ImageLink { id: string; path: string; hash: string; width: number; height: number; dpi: number; colorProfile: string | null; state: string | null; proxyDataUrl: string | null; artifactKind: string; artifactRef: string }
export interface LayoutBounds { x: Binary64; y: Binary64; w: Binary64; h: Binary64; rotation: Binary64 }
export interface LayoutRect { x: Binary64; y: Binary64; w: Binary64; h: Binary64 }
export interface PageMargins { top: Binary64; right: Binary64; bottom: Binary64; left: Binary64 }
export interface PageColumns { count: number; gutter: Binary64 }
export interface Layer { id: string; name: string; visible: boolean; locked: boolean; objectIds: string[] }
export interface FrameRect { kind: "rect"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; fill: [Binary32, Binary32, Binary32, Binary32] | null; stroke: [Binary32, Binary32, Binary32, Binary32] | null }
export interface FrameText { kind: "text"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; storyId: string; threadNext: string | null; columns: number; inset: LayoutRect; wrapMode: string }
export interface FrameImage { kind: "image"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; linkId: string }
export type Frame = FrameRect | FrameText | FrameImage;
export interface PageOverride { objectId: string; bounds: LayoutBounds | null; visible: boolean | null; locked: boolean | null }
export interface ParentPage { id: string; name: string; width: Binary64; height: Binary64; layerIds: string[]; layers: Layer[]; frames: Frame[] }
export interface Page { id: string; name: string; spreadId: string; parentPageId: string | null; width: Binary64; height: Binary64; margins: PageMargins; columns: PageColumns; guides: LayoutRect[]; layerIds: string[]; layers: Layer[]; frames: Frame[]; overrides: PageOverride[] }
export interface Spread { id: string; name: string; pageIds: string[] }
export interface LayoutDrawingChild { handle: ArtifactChild; content: SemioDrawingSnapshot }
export interface LayoutArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ name: string;
  /** @state artifact */ grid: GridSettings;
  /** @state artifact */ paragraphStyles: ParagraphStyle[];
  /** @state artifact */ characterStyles: CharacterStyle[];
  /** @state artifact */ stories: TextStory[];
  /** @state artifact */ links: ImageLink[];
  /** @state artifact */ parentPages: ParentPage[];
  /** @state artifact */ spreads: Spread[];
  /** @state artifact */ pages: Page[];
  /** @state artifact */ printTarget: string | null;
  /** @state artifact */ dataFields?: FormDictionary;
  /** @state artifact @child kind=s.stdio.semio */ backgroundDrawing?: LayoutDrawingChild;
  /** @state artifact @link_slot roles=model */ referencedModel?: ArtifactLink;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number }
export interface LayoutStringList { values: string[] }
export type LayoutParagraphStylesDelta = Readonly<Record<string, unknown>>;
export type LayoutCharacterStylesDelta = Readonly<Record<string, unknown>>;
export type LayoutStoriesDelta = Readonly<Record<string, unknown>>;
export type LayoutLinksDelta = Readonly<Record<string, unknown>>;
export type LayoutParentPagesDelta = Readonly<Record<string, unknown>>;
export type LayoutSpreadsDelta = Readonly<Record<string, unknown>>;
export type LayoutPagesDelta = Readonly<Record<string, unknown>>;

const required = (row: Record<string, unknown>, keys: readonly string[], at: string): void => {
  const missing = keys.find((key) => !Object.hasOwn(row, key));
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
};
const record = (value: unknown, keys: readonly string[], requiredKeys: readonly string[], at: string): Record<string, unknown> => {
  const row = parseSchemaRecord(value, keys, at);
  required(row, requiredKeys, at);
  return row;
};
const string = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: string required`);
  return value;
};
const number = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: finite number required`);
  return value;
};
const integer = (value: unknown, at: string): number => {
  const result = number(value, at);
  if (!Number.isSafeInteger(result) || result < 0 || result > 0xffffffff) throw new Error(`${at}: unsigned integer required`);
  return result;
};
const boolean = (value: unknown, at: string): boolean => {
  if (typeof value !== "boolean") throw new Error(`${at}: boolean required`);
  return value;
};
const array = (value: unknown, at: string): unknown[] => {
  if (!Array.isArray(value)) throw new Error(`${at}: array required`);
  return value;
};
const strings = (value: unknown, at: string): string[] => array(value, at).map((item, index) => string(item, `${at}[${index}]`));
const nullable = <T>(value: unknown, parse: (value: unknown, at: string) => T, at: string): T | null => value === null ? null : parse(value, at);
const word = (value: unknown, _at: string): Binary64 => parseBinary64Transport(value);
const unsigned64 = (value: unknown, at: string): bigint => { if(typeof value !== "bigint" || value < 0n || value > 0xffffffffffffffffn) throw new Error(`${at}: unsigned64 bigint required`); return value; };
const rgba = (value: unknown, at: string): [Binary32, Binary32, Binary32, Binary32] => {
  const items = array(value, at).map((item, index) => parseBinary32Transport(item));
  if (items.length !== 4) throw new Error(`${at}: four numbers required`);
  return items as [Binary32, Binary32, Binary32, Binary32];
};

export function parseGridSettings(value: unknown, at = "$" ): GridSettings {
  const keys = ["baselineGrid", "baselineOffset", "snapToBaseline"], row = record(value, keys, keys, at);
  return { baselineGrid: word(row.baselineGrid, `${at}.baselineGrid`), baselineOffset: word(row.baselineOffset, `${at}.baselineOffset`), snapToBaseline: boolean(row.snapToBaseline, `${at}.snapToBaseline`) };
}
export function parseParagraphStyle(value: unknown, at = "$" ): ParagraphStyle {
  const keys = ["id", "name", "fontFamily", "fontSize", "fontWeight", "leading", "tracking", "alignment"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), fontFamily: string(row.fontFamily, `${at}.fontFamily`), fontSize: word(row.fontSize, `${at}.fontSize`), fontWeight: integer(row.fontWeight, `${at}.fontWeight`), leading: word(row.leading, `${at}.leading`), tracking: word(row.tracking, `${at}.tracking`), alignment: string(row.alignment, `${at}.alignment`) };
}
export function parseCharacterStyle(value: unknown, at = "$" ): CharacterStyle {
  const keys = ["id", "name", "fontFamily", "fontSize", "fontWeight", "italic", "color", "tracking"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: nullable(row.name, string, `${at}.name`), fontFamily: nullable(row.fontFamily, string, `${at}.fontFamily`), fontSize: nullable(row.fontSize, word, `${at}.fontSize`), fontWeight: nullable(row.fontWeight, integer, `${at}.fontWeight`), italic: nullable(row.italic, boolean, `${at}.italic`), color: nullable(row.color, rgba, `${at}.color`), tracking: nullable(row.tracking, word, `${at}.tracking`) };
}
export function parseTextStyleRun(value: unknown, at = "$" ): TextStyleRun {
  const keys = ["start", "end", "paragraphStyleId", "characterStyleId"], row = record(value, keys, keys, at);
  return { start: unsigned64(row.start, `${at}.start`), end: unsigned64(row.end, `${at}.end`), paragraphStyleId: nullable(row.paragraphStyleId, string, `${at}.paragraphStyleId`), characterStyleId: nullable(row.characterStyleId, string, `${at}.characterStyleId`) };
}
export function parseTextStory(value: unknown, at = "$" ): TextStory {
  const keys = ["id", "content", "styleRuns"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), content: string(row.content, `${at}.content`), styleRuns: array(row.styleRuns, `${at}.styleRuns`).map((item, index) => parseTextStyleRun(item, `${at}.styleRuns[${index}]`)) };
}
export function parseImageLink(value: unknown, at = "$" ): ImageLink {
  const keys = ["id", "path", "hash", "width", "height", "dpi", "colorProfile", "state", "proxyDataUrl", "artifactKind", "artifactRef"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), path: string(row.path, `${at}.path`), hash: string(row.hash, `${at}.hash`), width: integer(row.width, `${at}.width`), height: integer(row.height, `${at}.height`), dpi: integer(row.dpi, `${at}.dpi`), colorProfile: nullable(row.colorProfile, string, `${at}.colorProfile`), state: nullable(row.state, string, `${at}.state`), proxyDataUrl: nullable(row.proxyDataUrl, string, `${at}.proxyDataUrl`), artifactKind: string(row.artifactKind, `${at}.artifactKind`), artifactRef: string(row.artifactRef, `${at}.artifactRef`) };
}
export function parseLayoutBounds(value: unknown, at = "$" ): LayoutBounds {
  const keys = ["x", "y", "w", "h", "rotation"], row = record(value, keys, keys, at);
  return { x: word(row.x, `${at}.x`), y: word(row.y, `${at}.y`), w: word(row.w, `${at}.w`), h: word(row.h, `${at}.h`), rotation: word(row.rotation, `${at}.rotation`) };
}
export function parseLayoutRect(value: unknown, at = "$" ): LayoutRect {
  const keys = ["x", "y", "w", "h"], row = record(value, keys, keys, at);
  return { x: word(row.x, `${at}.x`), y: word(row.y, `${at}.y`), w: word(row.w, `${at}.w`), h: word(row.h, `${at}.h`) };
}
export function parsePageMargins(value: unknown, at = "$" ): PageMargins {
  const keys = ["top", "right", "bottom", "left"], row = record(value, keys, keys, at);
  return { top: word(row.top, `${at}.top`), right: word(row.right, `${at}.right`), bottom: word(row.bottom, `${at}.bottom`), left: word(row.left, `${at}.left`) };
}
export function parsePageColumns(value: unknown, at = "$" ): PageColumns {
  const keys = ["count", "gutter"], row = record(value, keys, keys, at);
  return { count: integer(row.count, `${at}.count`), gutter: word(row.gutter, `${at}.gutter`) };
}
export function parseLayer(value: unknown, at = "$" ): Layer {
  const keys = ["id", "name", "visible", "locked", "objectIds"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), visible: boolean(row.visible, `${at}.visible`), locked: boolean(row.locked, `${at}.locked`), objectIds: strings(row.objectIds, `${at}.objectIds`) };
}
export function parseFrame(value: unknown, at = "$" ): Frame {
  const base = ["kind", "id", "layerId", "bounds", "locked", "visible"];
  const discriminator = parseSchemaRecord(value, [...base, "fill", "stroke", "storyId", "threadNext", "columns", "inset", "wrapMode", "linkId"], at).kind;
  if (discriminator === "rect") {
    const keys = [...base, "fill", "stroke"], row = record(value, keys, keys, at);
    return { kind: "rect", id: string(row.id, `${at}.id`), layerId: string(row.layerId, `${at}.layerId`), bounds: parseLayoutBounds(row.bounds, `${at}.bounds`), locked: nullable(row.locked, boolean, `${at}.locked`), visible: nullable(row.visible, boolean, `${at}.visible`), fill: nullable(row.fill, rgba, `${at}.fill`), stroke: nullable(row.stroke, rgba, `${at}.stroke`) };
  }
  if (discriminator === "text") {
    const keys = [...base, "storyId", "threadNext", "columns", "inset", "wrapMode"], row = record(value, keys, keys, at);
    return { kind: "text", id: string(row.id, `${at}.id`), layerId: string(row.layerId, `${at}.layerId`), bounds: parseLayoutBounds(row.bounds, `${at}.bounds`), locked: nullable(row.locked, boolean, `${at}.locked`), visible: nullable(row.visible, boolean, `${at}.visible`), storyId: string(row.storyId, `${at}.storyId`), threadNext: nullable(row.threadNext, string, `${at}.threadNext`), columns: integer(row.columns, `${at}.columns`), inset: parseLayoutRect(row.inset, `${at}.inset`), wrapMode: string(row.wrapMode, `${at}.wrapMode`) };
  }
  if (discriminator === "image") {
    const keys = [...base, "linkId"], row = record(value, keys, keys, at);
    return { kind: "image", id: string(row.id, `${at}.id`), layerId: string(row.layerId, `${at}.layerId`), bounds: parseLayoutBounds(row.bounds, `${at}.bounds`), locked: nullable(row.locked, boolean, `${at}.locked`), visible: nullable(row.visible, boolean, `${at}.visible`), linkId: string(row.linkId, `${at}.linkId`) };
  }
  throw new Error(`${at}.kind: frame kind required`);
}
export function parsePageOverride(value: unknown, at = "$" ): PageOverride {
  const keys = ["objectId", "bounds", "visible", "locked"], row = record(value, keys, keys, at);
  return { objectId: string(row.objectId, `${at}.objectId`), bounds: nullable(row.bounds, parseLayoutBounds, `${at}.bounds`), visible: nullable(row.visible, boolean, `${at}.visible`), locked: nullable(row.locked, boolean, `${at}.locked`) };
}
export function parseParentPage(value: unknown, at = "$" ): ParentPage {
  const keys = ["id", "name", "width", "height", "layerIds", "layers", "frames"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), width: word(row.width, `${at}.width`), height: word(row.height, `${at}.height`), layerIds: strings(row.layerIds, `${at}.layerIds`), layers: array(row.layers, `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)), frames: array(row.frames, `${at}.frames`).map((item, index) => parseFrame(item, `${at}.frames[${index}]`)) };
}
export function parsePage(value: unknown, at = "$" ): Page {
  const keys = ["id", "name", "spreadId", "parentPageId", "width", "height", "margins", "columns", "guides", "layerIds", "layers", "frames", "overrides"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), spreadId: string(row.spreadId, `${at}.spreadId`), parentPageId: nullable(row.parentPageId, string, `${at}.parentPageId`), width: word(row.width, `${at}.width`), height: word(row.height, `${at}.height`), margins: parsePageMargins(row.margins, `${at}.margins`), columns: parsePageColumns(row.columns, `${at}.columns`), guides: array(row.guides, `${at}.guides`).map((item, index) => parseLayoutRect(item, `${at}.guides[${index}]`)), layerIds: strings(row.layerIds, `${at}.layerIds`), layers: array(row.layers, `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)), frames: array(row.frames, `${at}.frames`).map((item, index) => parseFrame(item, `${at}.frames[${index}]`)), overrides: array(row.overrides, `${at}.overrides`).map((item, index) => parsePageOverride(item, `${at}.overrides[${index}]`)) };
}
export function parseSpread(value: unknown, at = "$" ): Spread {
  const keys = ["id", "name", "pageIds"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), pageIds: strings(row.pageIds, `${at}.pageIds`) };
}
export function parseLayoutDrawingChild(value: unknown, at = "$" ): LayoutDrawingChild {
  const keys = ["handle", "content"], row = record(value, keys, keys, at), handle = parseArtifactChild(row.handle);
  return { handle, content: parseSemioDrawingSnapshot(row.content, `${at}.content`) };
}
export function parseLayoutArtifact(value: unknown, at = "$" ): LayoutArtifact {
  const keys = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFields", "backgroundDrawing", "referencedModel"];
  const row = record(value, keys, keys.slice(0, 11), at);
  const result: LayoutArtifact = {
    schema: string(row.schema, `${at}.schema`), name: string(row.name, `${at}.name`), grid: parseGridSettings(row.grid, `${at}.grid`),
    paragraphStyles: array(row.paragraphStyles, `${at}.paragraphStyles`).map((item, index) => parseParagraphStyle(item, `${at}.paragraphStyles[${index}]`)),
    characterStyles: array(row.characterStyles, `${at}.characterStyles`).map((item, index) => parseCharacterStyle(item, `${at}.characterStyles[${index}]`)),
    stories: array(row.stories, `${at}.stories`).map((item, index) => parseTextStory(item, `${at}.stories[${index}]`)),
    links: array(row.links, `${at}.links`).map((item, index) => parseImageLink(item, `${at}.links[${index}]`)),
    parentPages: array(row.parentPages, `${at}.parentPages`).map((item, index) => parseParentPage(item, `${at}.parentPages[${index}]`)),
    spreads: array(row.spreads, `${at}.spreads`).map((item, index) => parseSpread(item, `${at}.spreads[${index}]`)),
    pages: array(row.pages, `${at}.pages`).map((item, index) => parsePage(item, `${at}.pages[${index}]`)),
    printTarget: nullable(row.printTarget, string, `${at}.printTarget`),
  };
  if (Object.hasOwn(row, "dataFields")) result.dataFields = parseFormDictionary(row.dataFields, `${at}.dataFields`);
  if (Object.hasOwn(row, "backgroundDrawing")) result.backgroundDrawing = parseLayoutDrawingChild(row.backgroundDrawing, `${at}.backgroundDrawing`);
  if (Object.hasOwn(row, "referencedModel")) result.referencedModel = parseArtifactLink(row.referencedModel);
  return result;
}
export function parseLayoutDropPreviewState(value: unknown, at = "$" ): LayoutDropPreviewState {
  const keys = ["kind", "x", "y"], row = record(value, keys, keys, at);
  return { kind: string(row.kind, `${at}.kind`), x: number(row.x, `${at}.x`), y: number(row.y, `${at}.y`) };
}
export function parseLayoutStringList(value: unknown, at = "$" ): LayoutStringList {
  const keys = ["values"], row = record(value, keys, keys, at);
  return { values: strings(row.values, `${at}.values`) };
}

export {LAYOUT_SQLITE_SCHEMA,layoutSnapshotToSqliteDatabase,layoutSnapshotFromSqliteDatabase} from "./📸️snapshot/🪶️sqlite/🟦️.ts";
export {layoutArtifactFromNativeJson,layoutArtifactNativeJson,layoutDiffFromNativeJson,layoutDiffNativeJson} from "./🪪️native-json/🟦️.ts";

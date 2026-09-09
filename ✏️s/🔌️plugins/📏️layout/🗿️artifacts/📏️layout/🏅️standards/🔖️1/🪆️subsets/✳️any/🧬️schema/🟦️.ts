/** 🧬️ Canonical Layout document schema. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export type { ArtifactChild, ArtifactLink };

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean }
export interface ParagraphStyle { id: string; name: string; fontFamily: string; fontSize: number; fontWeight: number; leading: number; tracking: number; alignment: string }
export interface CharacterStyle { id: string; name: string | null; fontFamily: string | null; fontSize: number | null; fontWeight: number | null; italic: boolean | null; color: [number, number, number, number] | null; tracking: number | null }
export interface TextStyleRun { start: number; end: number; paragraphStyleId: string | null; characterStyleId: string | null }
export interface TextStory { id: string; content: string; styleRuns: TextStyleRun[] }
export interface ImageLink { id: string; path: string; hash: string; width: number; height: number; dpi: number; colorProfile: string | null; state: string | null; proxyDataUrl: string | null }
export interface LayoutBounds { x: number; y: number; w: number; h: number; rotation: number }
export interface LayoutRect { x: number; y: number; w: number; h: number }
export interface PageMargins { top: number; right: number; bottom: number; left: number }
export interface PageColumns { count: number; gutter: number }
export interface Layer { id: string; name: string; visible: boolean; locked: boolean; objectIds: string[] }
export interface FrameRect { kind: "rect"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; fill: [number, number, number, number] | null; stroke: [number, number, number, number] | null }
export interface FrameText { kind: "text"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; storyId: string; threadNext: string | null; columns: number; inset: LayoutRect; wrapMode: string }
export interface FrameImage { kind: "image"; id: string; layerId: string; bounds: LayoutBounds; locked: boolean | null; visible: boolean | null; linkId: string }
export type Frame = FrameRect | FrameText | FrameImage;
export interface PageOverride { objectId: string; bounds: LayoutBounds | null; visible: boolean | null; locked: boolean | null }
export interface ParentPage { id: string; name: string; width: number; height: number; layerIds: string[]; layers: Layer[]; frames: Frame[] }
export interface Page { id: string; name: string; spreadId: string; parentPageId: string | null; width: number; height: number; margins: PageMargins; columns: PageColumns; guides: LayoutRect[]; layerIds: string[]; layers: Layer[]; frames: Frame[]; overrides: PageOverride[] }
export interface Spread { id: string; name: string; pageIds: string[] }
export interface LayoutSemioDrawingSnapshot { readonly schema: string; readonly canvas: Readonly<Record<string, unknown>>; readonly styles: readonly unknown[]; readonly layers: readonly unknown[] }
export interface LayoutDrawingChild { handle: ArtifactChild; content: LayoutSemioDrawingSnapshot }
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
  /** @state artifact */ dataFieldsJson?: string;
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
  if (!Number.isSafeInteger(result) || result < 0) throw new Error(`${at}: unsigned integer required`);
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
const rgba = (value: unknown, at: string): [number, number, number, number] => {
  const items = array(value, at).map((item, index) => number(item, `${at}[${index}]`));
  if (items.length !== 4) throw new Error(`${at}: four numbers required`);
  return items as [number, number, number, number];
};

export function parseGridSettings(value: unknown, at = "$" ): GridSettings {
  const keys = ["baselineGrid", "baselineOffset", "snapToBaseline"], row = record(value, keys, keys, at);
  return { baselineGrid: number(row.baselineGrid, `${at}.baselineGrid`), baselineOffset: number(row.baselineOffset, `${at}.baselineOffset`), snapToBaseline: boolean(row.snapToBaseline, `${at}.snapToBaseline`) };
}
export function parseParagraphStyle(value: unknown, at = "$" ): ParagraphStyle {
  const keys = ["id", "name", "fontFamily", "fontSize", "fontWeight", "leading", "tracking", "alignment"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), fontFamily: string(row.fontFamily, `${at}.fontFamily`), fontSize: number(row.fontSize, `${at}.fontSize`), fontWeight: integer(row.fontWeight, `${at}.fontWeight`), leading: number(row.leading, `${at}.leading`), tracking: number(row.tracking, `${at}.tracking`), alignment: string(row.alignment, `${at}.alignment`) };
}
export function parseCharacterStyle(value: unknown, at = "$" ): CharacterStyle {
  const keys = ["id", "name", "fontFamily", "fontSize", "fontWeight", "italic", "color", "tracking"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: nullable(row.name, string, `${at}.name`), fontFamily: nullable(row.fontFamily, string, `${at}.fontFamily`), fontSize: nullable(row.fontSize, number, `${at}.fontSize`), fontWeight: nullable(row.fontWeight, integer, `${at}.fontWeight`), italic: nullable(row.italic, boolean, `${at}.italic`), color: nullable(row.color, rgba, `${at}.color`), tracking: nullable(row.tracking, number, `${at}.tracking`) };
}
export function parseTextStyleRun(value: unknown, at = "$" ): TextStyleRun {
  const keys = ["start", "end", "paragraphStyleId", "characterStyleId"], row = record(value, keys, keys, at);
  return { start: integer(row.start, `${at}.start`), end: integer(row.end, `${at}.end`), paragraphStyleId: nullable(row.paragraphStyleId, string, `${at}.paragraphStyleId`), characterStyleId: nullable(row.characterStyleId, string, `${at}.characterStyleId`) };
}
export function parseTextStory(value: unknown, at = "$" ): TextStory {
  const keys = ["id", "content", "styleRuns"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), content: string(row.content, `${at}.content`), styleRuns: array(row.styleRuns, `${at}.styleRuns`).map((item, index) => parseTextStyleRun(item, `${at}.styleRuns[${index}]`)) };
}
export function parseImageLink(value: unknown, at = "$" ): ImageLink {
  const keys = ["id", "path", "hash", "width", "height", "dpi", "colorProfile", "state", "proxyDataUrl"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), path: string(row.path, `${at}.path`), hash: string(row.hash, `${at}.hash`), width: integer(row.width, `${at}.width`), height: integer(row.height, `${at}.height`), dpi: integer(row.dpi, `${at}.dpi`), colorProfile: nullable(row.colorProfile, string, `${at}.colorProfile`), state: nullable(row.state, string, `${at}.state`), proxyDataUrl: nullable(row.proxyDataUrl, string, `${at}.proxyDataUrl`) };
}
export function parseLayoutBounds(value: unknown, at = "$" ): LayoutBounds {
  const keys = ["x", "y", "w", "h", "rotation"], row = record(value, keys, keys, at);
  return { x: number(row.x, `${at}.x`), y: number(row.y, `${at}.y`), w: number(row.w, `${at}.w`), h: number(row.h, `${at}.h`), rotation: number(row.rotation, `${at}.rotation`) };
}
export function parseLayoutRect(value: unknown, at = "$" ): LayoutRect {
  const keys = ["x", "y", "w", "h"], row = record(value, keys, keys, at);
  return { x: number(row.x, `${at}.x`), y: number(row.y, `${at}.y`), w: number(row.w, `${at}.w`), h: number(row.h, `${at}.h`) };
}
export function parsePageMargins(value: unknown, at = "$" ): PageMargins {
  const keys = ["top", "right", "bottom", "left"], row = record(value, keys, keys, at);
  return { top: number(row.top, `${at}.top`), right: number(row.right, `${at}.right`), bottom: number(row.bottom, `${at}.bottom`), left: number(row.left, `${at}.left`) };
}
export function parsePageColumns(value: unknown, at = "$" ): PageColumns {
  const keys = ["count", "gutter"], row = record(value, keys, keys, at);
  return { count: integer(row.count, `${at}.count`), gutter: number(row.gutter, `${at}.gutter`) };
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
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), width: number(row.width, `${at}.width`), height: number(row.height, `${at}.height`), layerIds: strings(row.layerIds, `${at}.layerIds`), layers: array(row.layers, `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)), frames: array(row.frames, `${at}.frames`).map((item, index) => parseFrame(item, `${at}.frames[${index}]`)) };
}
export function parsePage(value: unknown, at = "$" ): Page {
  const keys = ["id", "name", "spreadId", "parentPageId", "width", "height", "margins", "columns", "guides", "layerIds", "layers", "frames", "overrides"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), spreadId: string(row.spreadId, `${at}.spreadId`), parentPageId: nullable(row.parentPageId, string, `${at}.parentPageId`), width: number(row.width, `${at}.width`), height: number(row.height, `${at}.height`), margins: parsePageMargins(row.margins, `${at}.margins`), columns: parsePageColumns(row.columns, `${at}.columns`), guides: array(row.guides, `${at}.guides`).map((item, index) => parseLayoutRect(item, `${at}.guides[${index}]`)), layerIds: strings(row.layerIds, `${at}.layerIds`), layers: array(row.layers, `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)), frames: array(row.frames, `${at}.frames`).map((item, index) => parseFrame(item, `${at}.frames[${index}]`)), overrides: array(row.overrides, `${at}.overrides`).map((item, index) => parsePageOverride(item, `${at}.overrides[${index}]`)) };
}
export function parseSpread(value: unknown, at = "$" ): Spread {
  const keys = ["id", "name", "pageIds"], row = record(value, keys, keys, at);
  return { id: string(row.id, `${at}.id`), name: string(row.name, `${at}.name`), pageIds: strings(row.pageIds, `${at}.pageIds`) };
}
function parseDrawing(value: unknown, at: string): LayoutSemioDrawingSnapshot {
  const keys = ["schema", "canvas", "styles", "layers"], row = record(value, keys, keys, at);
  const canvas = parseSchemaRecord(row.canvas, ["width", "height", "background"], `${at}.canvas`);
  required(canvas, ["width", "height"], `${at}.canvas`);
  number(canvas.width, `${at}.canvas.width`); number(canvas.height, `${at}.canvas.height`);
  array(row.styles, `${at}.styles`); array(row.layers, `${at}.layers`);
  return row as unknown as LayoutSemioDrawingSnapshot;
}
export function parseLayoutDrawingChild(value: unknown, at = "$" ): LayoutDrawingChild {
  const keys = ["handle", "content"], row = record(value, keys, keys, at), handle = parseArtifactChild(row.handle);
  if (handle.childId !== handle.target.artifactId) throw new Error(`${at}: childId must equal target.artifactId`);
  const dialect = handle.target.dialect;
  if (dialect.artifactKind !== "s.stdio.semio" || dialect.standard !== "v1" || dialect.subset !== "drawing") throw new Error(`${at}: expected s.stdio.semio@v1/drawing`);
  return { handle, content: parseDrawing(row.content, `${at}.content`) };
}
export function parseLayoutArtifact(value: unknown, at = "$" ): LayoutArtifact {
  const keys = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFieldsJson", "backgroundDrawing", "referencedModel"];
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
  if (Object.hasOwn(row, "dataFieldsJson")) result.dataFieldsJson = string(row.dataFieldsJson, `${at}.dataFieldsJson`);
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

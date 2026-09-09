/** 🧬️ Layout LayoutArtifact schema. */
import type {
  CharacterStyle,
  ImageLink,
  Page,
  ParagraphStyle,
  ParentPage,
  Spread,
  TextStory,
} from "./📸️snapshot/🟦️.ts";

export interface LayoutArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  name: string;
  /** @state artifact */
  grid: GridSettings;
  /** @state artifact */
  paragraphStyles: ParagraphStyle[];
  /** @state artifact */
  characterStyles: CharacterStyle[];
  /** @state artifact */
  stories: TextStory[];
  /** @state artifact */
  links: ImageLink[];
  /** @state artifact */
  parentPages: ParentPage[];
  /** @state artifact */
  spreads: Spread[];
  /** @state artifact */
  pages: Page[];
  /** @state artifact */
  printTarget?: string;
  /** @state artifact */
  dataFieldsJson?: string;
}

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutStringList { values: string[]; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutArtifactGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutArtifactGuardRefusal(at, why);
};

type layoutLayoutArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutArtifactGuardReject(at, "value is not an object");
export const layoutLayoutArtifactGuardArray = (value: unknown, at: string, bounds: layoutLayoutArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutArtifactGuardString = (value: unknown, at: string, bounds: layoutLayoutArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutArtifactGuardReject(at, "value is not a boolean"));
export const layoutLayoutArtifactGuardNumber = (value: unknown, at: string, bounds: layoutLayoutArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutArtifactGuardInteger = (value: unknown, at: string, bounds: layoutLayoutArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutArtifactGuardNumber(value, at, bounds) : layoutLayoutArtifactGuardReject(at, "value is not an integer");
export const layoutLayoutArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutArtifact(value: unknown, at = "$"): LayoutArtifact {
  const row = layoutLayoutArtifactGuardObject(value, at);
  return {
    schema: layoutLayoutArtifactGuardString(row["schema"], `${at}.schema`),
    name: layoutLayoutArtifactGuardString(row["name"], `${at}.name`),
    grid: parseGridSettings(row["grid"], `${at}.grid`),
    paragraphStyles: layoutLayoutArtifactGuardArray(row["paragraphStyles"], `${at}.paragraphStyles`).map((item, index) => parseParagraphStyle(item, `${at}.paragraphStyles[${index}]`)),
    characterStyles: layoutLayoutArtifactGuardArray(row["characterStyles"], `${at}.characterStyles`).map((item, index) => parseCharacterStyle(item, `${at}.characterStyles[${index}]`)),
    stories: layoutLayoutArtifactGuardArray(row["stories"], `${at}.stories`).map((item, index) => parseTextStory(item, `${at}.stories[${index}]`)),
    links: layoutLayoutArtifactGuardArray(row["links"], `${at}.links`).map((item, index) => parseImageLink(item, `${at}.links[${index}]`)),
    parentPages: layoutLayoutArtifactGuardArray(row["parentPages"], `${at}.parentPages`).map((item, index) => parseParentPage(item, `${at}.parentPages[${index}]`)),
    spreads: layoutLayoutArtifactGuardArray(row["spreads"], `${at}.spreads`).map((item, index) => parseSpread(item, `${at}.spreads[${index}]`)),
    pages: layoutLayoutArtifactGuardArray(row["pages"], `${at}.pages`).map((item, index) => parsePage(item, `${at}.pages[${index}]`)),
    printTarget: row["printTarget"] === undefined ? undefined : layoutLayoutArtifactGuardString(row["printTarget"], `${at}.printTarget`),
    dataFieldsJson: row["dataFieldsJson"] === undefined ? undefined : layoutLayoutArtifactGuardString(row["dataFieldsJson"], `${at}.dataFieldsJson`),
  };
}

export function parseGridSettings(value: unknown, at = "$"): GridSettings {
  const row = layoutLayoutArtifactGuardObject(value, at);
  return {
    baselineGrid: layoutLayoutArtifactGuardNumber(row["baselineGrid"], `${at}.baselineGrid`),
    baselineOffset: layoutLayoutArtifactGuardNumber(row["baselineOffset"], `${at}.baselineOffset`),
    snapToBaseline: layoutLayoutArtifactGuardBoolean(row["snapToBaseline"], `${at}.snapToBaseline`),
  };
}

export function parseLayoutDropPreviewState(value: unknown, at = "$"): LayoutDropPreviewState {
  const row = layoutLayoutArtifactGuardObject(value, at);
  return {
    kind: layoutLayoutArtifactGuardString(row["kind"], `${at}.kind`),
    x: layoutLayoutArtifactGuardNumber(row["x"], `${at}.x`),
    y: layoutLayoutArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseLayoutStringList(value: unknown, at = "$"): LayoutStringList {
  const row = layoutLayoutArtifactGuardObject(value, at);
  return {
    values: layoutLayoutArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => layoutLayoutArtifactGuardString(item, `${at}.values[${index}]`)),
  };
}

export type ParagraphStyle = Readonly<Record<string, unknown>>;

export function parseParagraphStyle(value: unknown, at = "$"): ParagraphStyle {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type CharacterStyle = Readonly<Record<string, unknown>>;

export function parseCharacterStyle(value: unknown, at = "$"): CharacterStyle {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type TextStory = Readonly<Record<string, unknown>>;

export function parseTextStory(value: unknown, at = "$"): TextStory {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type ImageLink = Readonly<Record<string, unknown>>;

export function parseImageLink(value: unknown, at = "$"): ImageLink {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type ParentPage = Readonly<Record<string, unknown>>;

export function parseParentPage(value: unknown, at = "$"): ParentPage {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type Spread = Readonly<Record<string, unknown>>;

export function parseSpread(value: unknown, at = "$"): Spread {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type Page = Readonly<Record<string, unknown>>;

export function parsePage(value: unknown, at = "$"): Page {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutParagraphStylesDelta = Readonly<Record<string, unknown>>;

export function parseLayoutParagraphStylesDelta(value: unknown, at = "$"): LayoutParagraphStylesDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutCharacterStylesDelta = Readonly<Record<string, unknown>>;

export function parseLayoutCharacterStylesDelta(value: unknown, at = "$"): LayoutCharacterStylesDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutStoriesDelta = Readonly<Record<string, unknown>>;

export function parseLayoutStoriesDelta(value: unknown, at = "$"): LayoutStoriesDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutLinksDelta = Readonly<Record<string, unknown>>;

export function parseLayoutLinksDelta(value: unknown, at = "$"): LayoutLinksDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutParentPagesDelta = Readonly<Record<string, unknown>>;

export function parseLayoutParentPagesDelta(value: unknown, at = "$"): LayoutParentPagesDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutSpreadsDelta = Readonly<Record<string, unknown>>;

export function parseLayoutSpreadsDelta(value: unknown, at = "$"): LayoutSpreadsDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

export type LayoutPagesDelta = Readonly<Record<string, unknown>>;

export function parseLayoutPagesDelta(value: unknown, at = "$"): LayoutPagesDelta {
  return layoutLayoutArtifactGuardObject(value, `${at}`);
}

/** 🧬️ Presentation artifact schema — every field with its state class. */
export interface PresentationArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ source: FigureTileSource;
  /** @state artifact */ tiles: FigureTileDraft[];
  /** @state presence */ selectedIds: string[];
  /** @state config */ engagementInput: string;
}
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class animatePresentationArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const animatePresentationArtifactGuardReject = (at: string, why: string): never => {
  throw new animatePresentationArtifactGuardRefusal(at, why);
};

type animatePresentationArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type animatePresentationArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type animatePresentationArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const animatePresentationArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : animatePresentationArtifactGuardReject(at, "value is not an object");
export const animatePresentationArtifactGuardArray = (value: unknown, at: string, bounds: animatePresentationArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return animatePresentationArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) animatePresentationArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) animatePresentationArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const animatePresentationArtifactGuardString = (value: unknown, at: string, bounds: animatePresentationArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return animatePresentationArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) animatePresentationArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) animatePresentationArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) animatePresentationArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const animatePresentationArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : animatePresentationArtifactGuardReject(at, "value is not a boolean"));
export const animatePresentationArtifactGuardNumber = (value: unknown, at: string, bounds: animatePresentationArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return animatePresentationArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) animatePresentationArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) animatePresentationArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const animatePresentationArtifactGuardInteger = (value: unknown, at: string, bounds: animatePresentationArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? animatePresentationArtifactGuardNumber(value, at, bounds) : animatePresentationArtifactGuardReject(at, "value is not an integer");
export const animatePresentationArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : animatePresentationArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const animatePresentationArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : animatePresentationArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePresentationArtifact(value: unknown, at = "$"): PresentationArtifact {
  const row = animatePresentationArtifactGuardObject(value, at);
  return {
    schema: animatePresentationArtifactGuardString(row["schema"], `${at}.schema`),
    source: parseFigureTileSource(row["source"], `${at}.source`),
    tiles: animatePresentationArtifactGuardArray(row["tiles"], `${at}.tiles`).map((item, index) => parseFigureTileDraft(item, `${at}.tiles[${index}]`)),
    selectedIds: animatePresentationArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => animatePresentationArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    engagementInput: animatePresentationArtifactGuardString(row["engagementInput"], `${at}.engagementInput`),
  };
}

export function parseFigureTileFrame(value: unknown, at = "$"): FigureTileFrame {
  const row = animatePresentationArtifactGuardObject(value, at);
  return {
    x: animatePresentationArtifactGuardNumber(row["x"], `${at}.x`),
    y: animatePresentationArtifactGuardNumber(row["y"], `${at}.y`),
    width: animatePresentationArtifactGuardNumber(row["width"], `${at}.width`),
    height: animatePresentationArtifactGuardNumber(row["height"], `${at}.height`),
  };
}

export function parseFigureTileSource(value: unknown, at = "$"): FigureTileSource {
  const row = animatePresentationArtifactGuardObject(value, at);
  return {
    src: animatePresentationArtifactGuardString(row["src"], `${at}.src`),
    kind: animatePresentationArtifactGuardString(row["kind"], `${at}.kind`),
    frame: parseFigureTileFrame(row["frame"], `${at}.frame`),
    sourceAspect: row["sourceAspect"] === undefined ? undefined : animatePresentationArtifactGuardNumber(row["sourceAspect"], `${at}.sourceAspect`),
    pdfPage: row["pdfPage"] === undefined ? undefined : animatePresentationArtifactGuardInteger(row["pdfPage"], `${at}.pdfPage`),
  };
}

export function parseFigureTileDraft(value: unknown, at = "$"): FigureTileDraft {
  const row = animatePresentationArtifactGuardObject(value, at);
  return {
    id: animatePresentationArtifactGuardString(row["id"], `${at}.id`),
    name: animatePresentationArtifactGuardString(row["name"], `${at}.name`),
    crop: parseFigureTileFrame(row["crop"], `${at}.crop`),
  };
}

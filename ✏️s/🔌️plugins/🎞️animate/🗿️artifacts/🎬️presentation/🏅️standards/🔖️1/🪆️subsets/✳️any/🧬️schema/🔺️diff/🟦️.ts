/** 🔺️ Presentation diff schema — sparse field delta. */
export interface PresentationDiff {
  /** @state artifact */ artifact?: PresentationArtifact | null;
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ source?: FigureTileSource | null;
  /** @state artifact */ tiles?: PresentationTilesDelta | null;
}
export interface PresentationStringList { values: string[]; }
export interface PresentationTilesDelta { added: FigureTileDraft[]; removed: string[]; patched: PresentationTilePatchEntry[]; reordered?: string[] | null; }
export interface PresentationTilePatchEntry { id: string; patch: FigureTileDraftPatch; }
export interface FigureTileDraftPatch { name?: string | null; crop?: FigureTileFrame | null; }
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
export interface PresentationArtifact {
  schema: string;
  source: FigureTileSource;
  tiles: FigureTileDraft[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class animatePresentationDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const animatePresentationDiffGuardReject = (at: string, why: string): never => {
  throw new animatePresentationDiffGuardRefusal(at, why);
};

type animatePresentationDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type animatePresentationDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type animatePresentationDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const animatePresentationDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : animatePresentationDiffGuardReject(at, "value is not an object");
export const animatePresentationDiffGuardArray = (value: unknown, at: string, bounds: animatePresentationDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return animatePresentationDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) animatePresentationDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) animatePresentationDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const animatePresentationDiffGuardString = (value: unknown, at: string, bounds: animatePresentationDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return animatePresentationDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) animatePresentationDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) animatePresentationDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) animatePresentationDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const animatePresentationDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : animatePresentationDiffGuardReject(at, "value is not a boolean"));
export const animatePresentationDiffGuardNumber = (value: unknown, at: string, bounds: animatePresentationDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return animatePresentationDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) animatePresentationDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) animatePresentationDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const animatePresentationDiffGuardInteger = (value: unknown, at: string, bounds: animatePresentationDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? animatePresentationDiffGuardNumber(value, at, bounds) : animatePresentationDiffGuardReject(at, "value is not an integer");
export const animatePresentationDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : animatePresentationDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const animatePresentationDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : animatePresentationDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePresentationStringList(value: unknown, at = "$"): PresentationStringList {
  const row = animatePresentationDiffGuardObject(value, at);
  return {
    values: animatePresentationDiffGuardArray(row["values"], `${at}.values`).map((item, index) => animatePresentationDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parsePresentationTilePatchEntry(value: unknown, at = "$"): PresentationTilePatchEntry {
  const row = animatePresentationDiffGuardObject(value, at);
  return {
    id: animatePresentationDiffGuardString(row["id"], `${at}.id`),
    patch: parseFigureTileDraftPatch(row["patch"], `${at}.patch`),
  };
}

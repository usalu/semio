/** 🧬️ Layout snapshot schema — artifact-lane fields only, mirrors `📸️snapshot/🦀️.rs`'s
 *  `LayoutSnapshot` field-for-field. */

export interface LayoutSnapshot {
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
  /** @state artifact @child kind=s.stdio.semio.drawing */
  backgroundDrawing?: LayoutDrawingChild;
  /** @state artifact @link_slot roles=model */
  referencedModel?: ArtifactLink;
}

export interface GridSettings {
  baselineGrid: number;
  baselineOffset: number;
  snapToBaseline: boolean;
}

export interface ParagraphStyle {
  id: string;
  name: string;
  fontFamily: string;
  fontSize: number;
  fontWeight: number;
  leading: number;
  tracking: number;
  alignment: string;
}

export interface CharacterStyle {
  id: string;
  name?: string;
  fontFamily?: string;
  fontSize?: number;
  fontWeight?: number;
  italic?: boolean;
  color?: [number, number, number, number];
  tracking?: number;
}

export interface TextStyleRun {
  start: number;
  end: number;
  paragraphStyleId?: string;
  characterStyleId?: string;
}

export interface TextStory {
  id: string;
  content: string;
  styleRuns: TextStyleRun[];
}

export interface ImageLink {
  id: string;
  path: string;
  hash: string;
  width: number;
  height: number;
  dpi: number;
  colorProfile?: string;
  state?: string;
  proxyDataUrl?: string;
}

export interface LayoutBounds {
  x: number;
  y: number;
  w: number;
  h: number;
  rotation: number;
}

export interface LayoutRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface PageMargins {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface PageColumns {
  count: number;
  gutter: number;
}

export interface Layer {
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  objectIds: string[];
}

export type Frame = FrameRect | FrameText | FrameImage;

export interface FrameRect {
  kind: "rect";
  id: string;
  layerId: string;
  bounds: LayoutBounds;
  locked?: boolean;
  visible?: boolean;
  fill?: [number, number, number, number];
  stroke?: [number, number, number, number];
}

export interface FrameText {
  kind: "text";
  id: string;
  layerId: string;
  bounds: LayoutBounds;
  locked?: boolean;
  visible?: boolean;
  storyId: string;
  threadNext?: string;
  columns: number;
  inset: LayoutRect;
  wrapMode: string;
}

export interface FrameImage {
  kind: "image";
  id: string;
  layerId: string;
  bounds: LayoutBounds;
  locked?: boolean;
  visible?: boolean;
  linkId: string;
}

export interface PageOverride {
  objectId: string;
  bounds?: LayoutBounds;
  visible?: boolean;
  locked?: boolean;
}

export interface ParentPage {
  id: string;
  name: string;
  width: number;
  height: number;
  layerIds: string[];
  layers: Layer[];
  frames: Frame[];
}

export interface Page {
  id: string;
  name: string;
  spreadId: string;
  parentPageId?: string;
  width: number;
  height: number;
  margins: PageMargins;
  columns: PageColumns;
  guides: LayoutRect[];
  layerIds: string[];
  layers: Layer[];
  frames: Frame[];
  overrides: PageOverride[];
}

export interface Spread {
  id: string;
  name: string;
  pageIds: string[];
}

/** 🌉️ Opaque mirror of `store::os_io::ArtifactRef` — a cross-cutting framework identity type, out of
 *  this facet's own domain. */
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

/** 🌉️ Mirrors `store::ArtifactChild<S>` (`#[serde(rename_all = "camelCase")]`, `child_id`/`target`
 *  fields only — the `local_owner`/`PhantomData<S>` fields are `#[serde(skip)]`). */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

/** 🌉️ Opaque mirror of `store::LinkPin` — a tagged enum (`Head` / `Checkpoint{id}` /
 *  `Snapshot{blob}`), out of this facet's own domain. */
export interface LinkPin { [key: string]: unknown; }

/** 🌉️ Mirrors `store::ArtifactLink` (`target`/`pin`/`role`). */
export interface ArtifactLink {
  target: ArtifactRef;
  pin: LinkPin;
  role: string;
}

/** 🌉️ Opaque mirror of stdio's `SemioDrawingSnapshot` — a composed child subset from a different
 *  plugin, out of this facet's own domain. */
export interface LayoutSemioDrawingSnapshot { [key: string]: unknown; }

/** 🌉️ Mirrors `crate::artifacts::layout::LayoutDrawingChild` (`handle`/`content`). */
export interface LayoutDrawingChild {
  handle: ArtifactChildHandle;
  content: LayoutSemioDrawingSnapshot;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutSnapshotGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutSnapshotGuardRefusal(at, why);
};

type layoutLayoutSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutSnapshotGuardReject(at, "value is not an object");
export const layoutLayoutSnapshotGuardArray = (value: unknown, at: string, bounds: layoutLayoutSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutSnapshotGuardString = (value: unknown, at: string, bounds: layoutLayoutSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutSnapshotGuardReject(at, "value is not a boolean"));
export const layoutLayoutSnapshotGuardNumber = (value: unknown, at: string, bounds: layoutLayoutSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutSnapshotGuardInteger = (value: unknown, at: string, bounds: layoutLayoutSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutSnapshotGuardNumber(value, at, bounds) : layoutLayoutSnapshotGuardReject(at, "value is not an integer");
export const layoutLayoutSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutSnapshot(value: unknown, at = "$"): LayoutSnapshot {
  const row = layoutLayoutSnapshotGuardObject(value, at);
  return {
    schema: layoutLayoutSnapshotGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : layoutLayoutSnapshotGuardString(row["value"], `${at}.value`),
  };
}

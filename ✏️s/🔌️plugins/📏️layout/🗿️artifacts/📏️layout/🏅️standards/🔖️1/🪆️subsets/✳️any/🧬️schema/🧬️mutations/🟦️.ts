/** 🧬️ LayoutMutation — closed semantic mutation vocabulary for the layout document, mirrors
 *  `🧬️mutations/🦀️.rs`'s `LayoutMutation` enum and its per-verb leaf structs field-for-field
 *  (`.../🧬️mutations/<verb-folder>/🦀️.rs`). The enum carries no `#[value(tag)]`, so it wires
 *  EXTERNALLY TAGGED: `{ "<PascalCaseVariantName>": { ...leaf fields } }`. Every leaf struct carries
 *  `#[value(rename_all = "camelCase")]`, so its fields wire camelCase (`{"ChangePageWidth":
 *  {"id":"page-1","newWidth":240.0}}`); an `Option` field wires `null` when absent. */

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

//#region 🔖️Leaves
export interface RenameLayout {
  newName: string;
}

export interface ChangePrintTarget {
  newPrintTarget: string | null;
}

export interface ChangeDataFields {
  newFields: import("../🟦️.ts").FormDictionary | null;
}

export interface CreatePage {
  page: Page;
  index: number | null;
}

export interface DeletePage {
  id: string;
}

export interface RenamePage {
  id: string;
  newName: string;
}

export interface ChangePageWidth {
  id: string;
  newWidth: number;
}

export interface ChangePageHeight {
  id: string;
  newHeight: number;
}

export interface UpdatePageMargins {
  id: string;
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface UpdatePageColumns {
  id: string;
  count: number;
  gutter: number;
}

export interface ReorderPages {
  id: string;
  toIndex: number;
}

export interface CreateStory {
  story: TextStory;
  index: number | null;
}

export interface DeleteStory {
  id: string;
}

export interface EditStory {
  id: string;
  newContent: string;
}

export interface CreateLink {
  link: ImageLink;
  index: number | null;
}

export interface DeleteLink {
  id: string;
}

export interface ChangeLinkPath {
  id: string;
  newPath: string;
}

export interface CreateFrame {
  pageId: string;
  frame: Frame;
  index: number | null;
  layerId: string | null;
}

export interface DeleteFrame {
  pageId: string;
  frameId: string;
}

export interface MoveFrame {
  pageId: string;
  frameId: string;
  newX: number;
  newY: number;
}

export interface ResizeFrame {
  pageId: string;
  frameId: string;
  newWidth: number;
  newHeight: number;
}

export interface RotateFrame {
  pageId: string;
  frameId: string;
  newRotation: number;
}

export interface ChangeFrameFill {
  pageId: string;
  frameId: string;
  newFill: [number, number, number, number] | null;
}

export interface ChangeFrameStroke {
  pageId: string;
  frameId: string;
  newStroke: [number, number, number, number] | null;
}

export interface ChangeFrameWrapMode {
  pageId: string;
  frameId: string;
  newWrapMode: string;
}

export interface ChangeFrameColumns {
  pageId: string;
  frameId: string;
  newColumns: number;
}

export interface UpdateGrid {
  baselineGrid: number;
  baselineOffset: number;
  snapToBaseline: boolean;
}

export interface SetFrameFlags {
  pageId: string;
  frameId: string;
  locked: boolean | null;
  visible: boolean | null;
}

export interface UpdateParagraphStyle {
  id: string;
  name: string;
  fontFamily: string;
  fontSize: number;
  fontWeight: number;
  leading: number;
  tracking: number;
  alignment: string;
}

export interface UpdateTextFrame {
  pageId: string;
  frameId: string;
  storyId: string;
  threadNext: string | null;
  insetX: number;
  insetY: number;
  insetWidth: number;
  insetHeight: number;
}

export interface UpdateLayer {
  pageId: string;
  layerId: string;
  name: string;
  visible: boolean;
  locked: boolean;
}

export interface CreateCharacterStyle {
  id: string;
  name: string | null;
  index: number | null;
}

export interface DeleteCharacterStyle {
  id: string;
}

export interface UpdateCharacterStyle {
  id: string;
  name: string | null;
  fontFamily: string | null;
  fontSize: number | null;
  fontWeight: number | null;
  italic: boolean | null;
  color: [number, number, number, number] | null;
  tracking: number | null;
}

export interface UpdateParentPage {
  id: string;
  name: string;
  width: number;
  height: number;
}

export interface UpdateSpread {
  id: string;
  name: string;
}

export interface SetPageParent {
  id: string;
  parentPageId: string | null;
}

export interface SetPageGuides {
  id: string;
  guides: LayoutRect[];
}

export interface SetStoryRuns {
  id: string;
  runs: TextStyleRun[];
}

export interface UpdateLink {
  id: string;
  width: number;
  height: number;
  dpi: number;
  colorProfile: string | null;
}

export interface SetPageOverrides {
  id: string;
  overrides: PageOverride[];
}

export interface CreateLayer {
  pageId: string;
  id: string;
  name: string;
  index: number | null;
  remove?: boolean;
}

export interface SetFrameLayer {
  pageId: string;
  frameId: string;
  layerId: string;
}

export interface SetDrawingText {
  index: number;
  text: string;
}

export interface ReorderFrame {
  pageId: string;
  frameId: string;
  forward: boolean;
}

/** ✋️ `drag-frames`: frames of one page dragged by one common offset (relative; replays on any base). */
export interface DragFrames {
  pageId: string;
  targets: string[];
  dx: number;
  dy: number;
}

/** 🔃️ `rotate-frames`: frames of one page turned by `angle` radians about the recorded pivot. */
export interface RotateFrames {
  pageId: string;
  targets: string[];
  pivotX: number;
  pivotY: number;
  angle: number;
}

/** 🗜️ `scale-frames`: frames of one page scaled by positive per-axis factors about the recorded pivot. */
export interface ScaleFrames {
  pageId: string;
  targets: string[];
  pivotX: number;
  pivotY: number;
  sx: number;
  sy: number;
}
//#endregion 🔖️Leaves

//#region 🔖️Mutations
export type LayoutMutation =
  | { RenameLayout: RenameLayout }
  | { ChangePrintTarget: ChangePrintTarget }
  | { ChangeDataFields: ChangeDataFields }
  | { CreatePage: CreatePage }
  | { DeletePage: DeletePage }
  | { RenamePage: RenamePage }
  | { ChangePageWidth: ChangePageWidth }
  | { ChangePageHeight: ChangePageHeight }
  | { UpdatePageMargins: UpdatePageMargins }
  | { UpdatePageColumns: UpdatePageColumns }
  | { ReorderPages: ReorderPages }
  | { CreateStory: CreateStory }
  | { DeleteStory: DeleteStory }
  | { EditStory: EditStory }
  | { CreateLink: CreateLink }
  | { DeleteLink: DeleteLink }
  | { ChangeLinkPath: ChangeLinkPath }
  | { CreateFrame: CreateFrame }
  | { DeleteFrame: DeleteFrame }
  | { MoveFrame: MoveFrame }
  | { ResizeFrame: ResizeFrame }
  | { RotateFrame: RotateFrame }
  | { ChangeFrameFill: ChangeFrameFill }
  | { ChangeFrameStroke: ChangeFrameStroke }
  | { ChangeFrameWrapMode: ChangeFrameWrapMode }
  | { ChangeFrameColumns: ChangeFrameColumns }
  | { UpdateGrid: UpdateGrid }
  | { SetFrameFlags: SetFrameFlags }
  | { UpdateParagraphStyle: UpdateParagraphStyle }
  | { UpdateTextFrame: UpdateTextFrame }
  | { UpdateLayer: UpdateLayer }
  | { CreateCharacterStyle: CreateCharacterStyle }
  | { DeleteCharacterStyle: DeleteCharacterStyle }
  | { UpdateCharacterStyle: UpdateCharacterStyle }
  | { UpdateParentPage: UpdateParentPage }
  | { UpdateSpread: UpdateSpread }
  | { SetPageParent: SetPageParent }
  | { SetPageGuides: SetPageGuides }
  | { SetStoryRuns: SetStoryRuns }
  | { UpdateLink: UpdateLink }
  | { SetPageOverrides: SetPageOverrides }
  | { CreateLayer: CreateLayer }
  | { SetFrameLayer: SetFrameLayer }
  | { SetDrawingText: SetDrawingText }
  | { ReorderFrame: ReorderFrame }
  | { DragFrames: DragFrames }
  | { RotateFrames: RotateFrames }
  | { ScaleFrames: ScaleFrames };
//#endregion 🔖️Mutations

//#region 🔖️FrameSelection
const frameSelectionRecord = (value: unknown, keys: readonly string[], at: string): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: object required`);
  const row = value as Record<string, unknown>;
  const unknown = Object.keys(row).find((key) => !keys.includes(key)), missing = keys.find((key) => !Object.hasOwn(row, key));
  if (unknown !== undefined) throw new Error(`${at}.${unknown}: unknown field`);
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
  return row;
};
const frameSelectionNumber = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: finite number required`);
  return value;
};
const frameSelectionFactor = (value: unknown, at: string): number => {
  const factor = frameSelectionNumber(value, at);
  if (factor <= 0) throw new Error(`${at}: factor must be positive`);
  return factor;
};
const frameSelectionTargets = (value: unknown, at: string): string[] => {
  if (!Array.isArray(value) || value.length === 0) throw new Error(`${at}: at least one frame required`);
  const targets = value.map((item, index) => {
    if (typeof item !== "string" || item.length === 0) throw new Error(`${at}[${index}]: frame id required`);
    return item;
  });
  if (new Set(targets).size !== targets.length) throw new Error(`${at}: a frame is named twice`);
  return targets;
};
const frameSelectionPage = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: page id required`);
  return value;
};

/** ✋️ Parses one `drag-frames` payload exactly as its leaf schema admits it. */
export function parseDragFrames(value: unknown, at = "$"): DragFrames {
  const row = frameSelectionRecord(value, ["pageId", "targets", "dx", "dy"], at);
  return { pageId: frameSelectionPage(row.pageId, `${at}.pageId`), targets: frameSelectionTargets(row.targets, `${at}.targets`), dx: frameSelectionNumber(row.dx, `${at}.dx`), dy: frameSelectionNumber(row.dy, `${at}.dy`) };
}

/** 🔃️ Parses one `rotate-frames` payload exactly as its leaf schema admits it. */
export function parseRotateFrames(value: unknown, at = "$"): RotateFrames {
  const row = frameSelectionRecord(value, ["pageId", "targets", "pivotX", "pivotY", "angle"], at);
  return { pageId: frameSelectionPage(row.pageId, `${at}.pageId`), targets: frameSelectionTargets(row.targets, `${at}.targets`), pivotX: frameSelectionNumber(row.pivotX, `${at}.pivotX`), pivotY: frameSelectionNumber(row.pivotY, `${at}.pivotY`), angle: frameSelectionNumber(row.angle, `${at}.angle`) };
}

/** 🗜️ Parses one `scale-frames` payload exactly as its leaf schema admits it. */
export function parseScaleFrames(value: unknown, at = "$"): ScaleFrames {
  const row = frameSelectionRecord(value, ["pageId", "targets", "pivotX", "pivotY", "sx", "sy"], at);
  return { pageId: frameSelectionPage(row.pageId, `${at}.pageId`), targets: frameSelectionTargets(row.targets, `${at}.targets`), pivotX: frameSelectionNumber(row.pivotX, `${at}.pivotX`), pivotY: frameSelectionNumber(row.pivotY, `${at}.pivotY`), sx: frameSelectionFactor(row.sx, `${at}.sx`), sy: frameSelectionFactor(row.sy, `${at}.sy`) };
}
//#endregion 🔖️FrameSelection

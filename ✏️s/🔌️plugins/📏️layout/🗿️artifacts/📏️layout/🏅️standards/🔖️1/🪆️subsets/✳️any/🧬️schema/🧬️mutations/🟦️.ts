/** 🧬️ Canonical externally tagged semantic commands over the owned Layout model. */
import type { Binary64, Binary32 } from "@semio-tech/framework/value/ieee754";
import type { LayoutBounds, LayoutRect, PageMargins, PageColumns, Layer, Frame, FrameRect, FrameText, FrameImage, PageOverride, Page, TextStyleRun, TextStory, ImageLink } from "../🟦️.ts";
export type { LayoutBounds, LayoutRect, PageMargins, PageColumns, Layer, Frame, FrameRect, FrameText, FrameImage, PageOverride, Page, TextStyleRun, TextStory, ImageLink };

//#region 🔖️Leaves
export interface RenameLayout {
  newName: string;
}

export interface ChangePrintTarget {
  newPrintTarget: string | null;
}

export interface ChangeDataFields {
  newFields: import("../🟦️.ts").QuestionDictionary | null;
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
  newWidth: Binary64;
}

export interface ChangePageHeight {
  id: string;
  newHeight: Binary64;
}

export interface UpdatePageMargins {
  id: string;
  top: Binary64;
  right: Binary64;
  bottom: Binary64;
  left: Binary64;
}

export interface UpdatePageColumns {
  id: string;
  count: number;
  gutter: Binary64;
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
  offset: number;
  delete: number;
  insert: string;
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
  newX: Binary64;
  newY: Binary64;
}

export interface ResizeFrame {
  pageId: string;
  frameId: string;
  newWidth: Binary64;
  newHeight: Binary64;
}

export interface RotateFrame {
  pageId: string;
  frameId: string;
  newRotation: Binary64;
}

export interface ChangeFrameFill {
  pageId: string;
  frameId: string;
  newFill: [Binary32, Binary32, Binary32, Binary32] | null;
}

export interface ChangeFrameStroke {
  pageId: string;
  frameId: string;
  newStroke: [Binary32, Binary32, Binary32, Binary32] | null;
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
  baselineGrid: Binary64;
  baselineOffset: Binary64;
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
  fontSize: Binary64;
  fontWeight: number;
  leading: Binary64;
  tracking: Binary64;
  alignment: string;
}

export interface UpdateTextFrame {
  pageId: string;
  frameId: string;
  storyId: string;
  threadNext: string | null;
  insetX: Binary64;
  insetY: Binary64;
  insetWidth: Binary64;
  insetHeight: Binary64;
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
  fontSize: Binary64 | null;
  fontWeight: number | null;
  italic: boolean | null;
  color: [Binary32, Binary32, Binary32, Binary32] | null;
  tracking: Binary64 | null;
}

export interface UpdateParentPage {
  id: string;
  name: string;
  width: Binary64;
  height: Binary64;
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
  remove: boolean;
}

export interface SetFrameLayer {
  pageId: string;
  frameId: string;
  layerId: string;
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
  dx: Binary64;
  dy: Binary64;
}

/** 🔃️ `rotate-frames`: frames of one page turned by `angle` radians about the recorded pivot. */
export interface RotateFrames {
  pageId: string;
  targets: string[];
  pivotX: Binary64;
  pivotY: Binary64;
  angle: Binary64;
}

/** 🗜️ `scale-frames`: frames of one page scaled by positive per-axis factors about the recorded pivot. */
export interface ScaleFrames {
  pageId: string;
  targets: string[];
  pivotX: Binary64;
  pivotY: Binary64;
  sx: Binary64;
  sy: Binary64;
}

export interface CreateParagraphStyle { style: import("../🟦️.ts").ParagraphStyle; index: number | null }
export interface DeleteParagraphStyle { id: string }
export interface CreateParentPage { page: import("../🟦️.ts").ParentPage; index: number | null }
export interface DeleteParentPage { id: string }
export interface CreateSpread { spread: import("../🟦️.ts").Spread; index: number | null }
export interface DeleteSpread { id: string }
export interface SetPageSpread { id: string; spreadId: string; index: number | null }
export interface ReorderLayer { pageId: string; layerId: string; toIndex: number }
export interface SetParentPageContent { id: string; layers: Layer[]; frames: Frame[] }
export interface ReplaceDocument { document: import("../📸️snapshot/🟦️.ts").LayoutSnapshot }
//#endregion 🔖️Leaves

//#region 🔖️Mutations
export interface SetPageBackgroundDrawing {pageId:string;drawing:import("../🟦️.ts").LayoutDrawingMember|null}
export type LayoutMutation =
  | { SetPageBackgroundDrawing: SetPageBackgroundDrawing }
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
  | { ReorderFrame: ReorderFrame }
  | { DragFrames: DragFrames }
  | { RotateFrames: RotateFrames }
  | { ScaleFrames: ScaleFrames }
  | { CreateParagraphStyle: CreateParagraphStyle }
  | { DeleteParagraphStyle: DeleteParagraphStyle }
  | { CreateParentPage: CreateParentPage }
  | { DeleteParentPage: DeleteParentPage }
  | { CreateSpread: CreateSpread }
  | { DeleteSpread: DeleteSpread }
  | { SetPageSpread: SetPageSpread }
  | { ReorderLayer: ReorderLayer }
  | { SetParentPageContent: SetParentPageContent }
  | { ReplaceDocument: ReplaceDocument };
//#endregion 🔖️Mutations


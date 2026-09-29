/** 🧬️ LayoutMutation — closed semantic mutation vocabulary for the layout document, mirrors
 *  `🧬️mutations/🦀️.rs`'s `LayoutMutation` enum and its 28 per-verb leaf structs
 *  field-for-field (`.../🧬️mutations/<verb-folder>/🦀️.rs`, one flat leaf file per verb, no nested
 *  `🦠️mutation` subfolder). `LayoutMutation` carries NO `#[serde(tag = ...)]` — confirmed absent on
 *  the enum itself and on every one of its 28 leaf structs — so it serializes with serde's default
 *  EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, proven by
 *  every committed `🧪️tests/*​/🦠️mutation/🔣️.json` fixture (e.g. `{"ChangePageWidth":
 *  {"id":"page-1","new_width":240.0}}`). This is NOT raster/jack's internally-tagged `{ mutation:
 *  'camelCase', ...fields }` shape — those enums carry an explicit `#[serde(tag = "mutation",
 *  rename_all = "camelCase")]` that layout's `LayoutMutation` lacks. None of the 28 leaf structs
 *  carry `#[serde(rename_all = ...)]` either, so every leaf's own field names are the literal Rust
 *  snake_case names verbatim (also confirmed field-by-field against the committed fixtures). */

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
  new_name: string;
}

export interface ChangePrintTarget {
  new_print_target?: string;
}

export interface ChangeDataFields {
  new_json?: string;
}

export interface CreatePage {
  page: Page;
  index?: number;
}

export interface DeletePage {
  id: string;
}

export interface RenamePage {
  id: string;
  new_name: string;
}

export interface ChangePageWidth {
  id: string;
  new_width: number;
}

export interface ChangePageHeight {
  id: string;
  new_height: number;
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
  to_index: number;
}

export interface CreateStory {
  story: TextStory;
  index?: number;
}

export interface DeleteStory {
  id: string;
}

export interface EditStory {
  id: string;
  new_content: string;
}

export interface CreateLink {
  link: ImageLink;
  index?: number;
}

export interface DeleteLink {
  id: string;
}

export interface ChangeLinkPath {
  id: string;
  new_path: string;
}

export interface CreateFrame {
  page_id: string;
  frame: Frame;
  index?: number;
  layer_id?: string;
}

export interface DeleteFrame {
  page_id: string;
  frame_id: string;
}

export interface MoveFrame {
  page_id: string;
  frame_id: string;
  new_x: number;
  new_y: number;
}

export interface ResizeFrame {
  page_id: string;
  frame_id: string;
  new_width: number;
  new_height: number;
}

export interface RotateFrame {
  page_id: string;
  frame_id: string;
  new_rotation: number;
}

export interface ChangeFrameFill {
  page_id: string;
  frame_id: string;
  new_fill?: [number, number, number, number];
}

export interface ChangeFrameStroke {
  page_id: string;
  frame_id: string;
  new_stroke?: [number, number, number, number];
}

export interface ChangeFrameWrapMode {
  page_id: string;
  frame_id: string;
  new_wrap_mode: string;
}

export interface ChangeFrameColumns {
  page_id: string;
  frame_id: string;
  new_columns: number;
}

export interface UpdateGrid {
  baseline_grid: number;
  baseline_offset: number;
  snap_to_baseline: boolean;
}

export interface SetFrameFlags {
  page_id: string;
  frame_id: string;
  locked?: boolean;
  visible?: boolean;
}

export interface UpdateParagraphStyle {
  id: string;
  name: string;
  font_family: string;
  font_size: number;
  font_weight: number;
  leading: number;
  tracking: number;
  alignment: string;
}

export interface UpdateTextFrame {
  page_id: string;
  frame_id: string;
  story_id: string;
  thread_next?: string;
  inset_x: number;
  inset_y: number;
  inset_width: number;
  inset_height: number;
}

export interface UpdateLayer {
  page_id: string;
  layer_id: string;
  name: string;
  visible: boolean;
  locked: boolean;
}

export interface CreateCharacterStyle { id: string }
export interface DeleteCharacterStyle { id: string }
export interface UpdateCharacterStyle { id: string }
export interface UpdateParentPage { id: string }
export interface UpdateSpread { id: string }
export interface SetPageParent { id: string }
export interface SetPageGuides { id: string }
export interface SetStoryRuns { id: string }
export interface UpdateLink { id: string }
export interface SetPageOverrides { id: string }
export interface CreateLayer { id: string }
export interface SetFrameLayer { id: string }
export interface SetDrawingText { id: string }
export interface ReorderFrame { id: string }
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
  | { ReorderFrame: ReorderFrame };
//#endregion 🔖️Mutations

/** 🧬️ Layout diff schema — sparse field delta over the artifact, mirrors `🔺️diff/🦀️.rs`'s
 *  `LayoutDiff` field-for-field. */

export interface LayoutDiff {
  /** @state artifact */
  artifact?: LayoutArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  name?: string;
  /** @state artifact */
  grid?: GridSettings;
  /** @state artifact */
  paragraphStyles?: LayoutParagraphStylesDelta;
  /** @state artifact */
  characterStyles?: LayoutCharacterStylesDelta;
  /** @state artifact */
  stories?: LayoutStoriesDelta;
  /** @state artifact */
  links?: LayoutLinksDelta;
  /** @state artifact */
  parentPages?: LayoutParentPagesDelta;
  /** @state artifact */
  spreads?: LayoutSpreadsDelta;
  /** @state artifact */
  pages?: LayoutPagesDelta;
  /** @state artifact */
  printTarget?: string | null;
  /** @state artifact */
  dataFieldsJson?: string | null;
  /** @state artifact @child kind=s.stdio.semio.drawing */
  backgroundDrawing?: LayoutDrawingChild | null;
  /** @state artifact @link_slot roles=model */
  referencedModel?: ArtifactLink | null;
  /** @state presence */
  selectedIds?: LayoutStringList;
  /** @state config */
  activePageId?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  previewCameraX?: number;
  /** @state config */
  previewCameraY?: number;
  /** @state config */
  previewCameraZoom?: number;
  /** @state config */
  dropPreview?: LayoutDropPreviewState;
  /** @state config */
  locale?: string;
  /** @state artifact */
  hoveredId?: string | null;
}

/** 🌉️ Opaque mirror of the full `LayoutArtifact` aggregate (artifact + presence + config lanes) —
 *  out of this facet's own scope (the diff's `artifact` field carries a whole-artifact replace,
 *  distinct from every other sparse per-field entry below it). */
export interface LayoutArtifact { [key: string]: unknown; }

export interface LayoutDropPreviewState {
  kind: string;
  x: number;
  y: number;
}

export interface LayoutStringList {
  values: string[];
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

export interface ParentPage {
  id: string;
  name: string;
  width: number;
  height: number;
  layerIds: string[];
  layers: Layer[];
  frames: Frame[];
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

export interface LayoutPagesDelta {
  added: Page[];
  removed: string[];
  patched: LayoutPagePatchEntry[];
  reordered?: string[];
}

export interface LayoutPagePatchEntry {
  id: string;
  patch: PagePatch;
}

export interface LayoutStoriesDelta {
  added: TextStory[];
  removed: string[];
  patched: LayoutStoryPatchEntry[];
  reordered?: string[];
}

export interface LayoutStoryPatchEntry {
  id: string;
  patch: TextStoryPatch;
}

export interface LayoutLinksDelta {
  added: ImageLink[];
  removed: string[];
  patched: LayoutLinkPatchEntry[];
  reordered?: string[];
}

export interface LayoutLinkPatchEntry {
  id: string;
  patch: ImageLinkPatch;
}

export interface LayoutParagraphStylesDelta {
  added: ParagraphStyle[];
  removed: string[];
  patched: LayoutParagraphStylePatchEntry[];
  reordered?: string[];
}

export interface LayoutParagraphStylePatchEntry {
  id: string;
  patch: ParagraphStylePatch;
}

export interface LayoutCharacterStylesDelta {
  added: CharacterStyle[];
  removed: string[];
  patched: LayoutCharacterStylePatchEntry[];
  reordered?: string[];
}

export interface LayoutCharacterStylePatchEntry {
  id: string;
  patch: CharacterStylePatch;
}

export interface LayoutParentPagesDelta {
  added: ParentPage[];
  removed: string[];
  patched: LayoutParentPagePatchEntry[];
  reordered?: string[];
}

export interface LayoutParentPagePatchEntry {
  id: string;
  patch: ParentPagePatch;
}

export interface LayoutSpreadsDelta {
  added: Spread[];
  removed: string[];
  patched: LayoutSpreadPatchEntry[];
  reordered?: string[];
}

export interface LayoutSpreadPatchEntry {
  id: string;
  patch: SpreadPatch;
}

export interface ParagraphStylePatch {
  name?: string;
}

export interface CharacterStylePatch {
  name?: string;
}

export interface ParentPagePatch {
  name?: string;
}

export interface SpreadPatch {
  name?: string;
}

export interface TextStoryPatch {
  content?: string;
}

export interface ImageLinkPatch {
  path?: string;
}

/** 🩹️ `PagePatch`'s Rust struct has no `#[serde(rename_all = "camelCase")]`, so its wire field names
 *  stay snake_case — mirrored verbatim, not camelCased. */
export interface PagePatch {
  name?: string;
  width?: number;
  height?: number;
  margin_top?: number;
  margin_right?: number;
  margin_bottom?: number;
  margin_left?: number;
  columns_count?: number;
  columns_gutter?: number;
  frame_added?: PageFrameAdded;
  frame_removed?: string;
  frame_patched?: PageFramePatched;
}

/** 🌱️ Same no-`rename_all` snake_case wire shape as `PagePatch` itself. */
export interface PageFrameAdded {
  frame: Frame;
  index?: number;
  layer_id?: string;
}

/** 🩹️ Same no-`rename_all` snake_case wire shape as `PagePatch` itself. */
export interface PageFramePatched {
  frame_id: string;
  patch: FramePatch;
}

/** 🖼️ Same no-`rename_all` snake_case wire shape as `PagePatch` itself. */
export interface FramePatch {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  fill?: [number, number, number, number] | null;
  stroke?: [number, number, number, number] | null;
  wrap_mode?: string;
  columns?: number;
}

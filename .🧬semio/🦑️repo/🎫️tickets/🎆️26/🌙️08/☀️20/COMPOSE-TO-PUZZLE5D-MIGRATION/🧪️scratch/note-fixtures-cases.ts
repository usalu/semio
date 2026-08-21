/** 🧪️ The 33 handcrafted `🗒️note` mutation fixture cases. Every `after` is the base with exactly
 * what that mutation's own `🔺️diff/🦀️component.rs` does applied by hand; every `extraBody` is a
 * per-mutation assertion block that would FAIL for any other mutation. Nothing here is derived
 * from a mutation's name or docstring. */
import { addedEntry, assetsDelta, blocksDelta, clone, compact, emptyDiff, F, at, base, indexOf, nested, patchEntry, TEXT_HANDLE_HELLO } from "./note-fixtures-base.ts";

export type Case = {
  readonly dir: string;
  readonly slug: string;
  readonly caseName: string;
  readonly modName: string;
  readonly mutation: Record<string, unknown>;
  readonly after: () => unknown;
  /** 🔺️ The serialized `NoteDiff` this mutation's own `🔺️diff/🦀️component.rs` constructs — every
   * field it sets and nothing else. Receives this case's own committed `after` so a whole-block
   * `patched` entry transcribes the very block value the after-snapshot carries. */
  readonly diff: (after: ReturnType<typeof base>) => Record<string, unknown>;
  readonly diffDoc: string;
  readonly applyDiffDoc: string;
  readonly applyDoc: string;
  readonly inverseDoc: string;
  readonly outcomeDoc: string;
  readonly extraUse?: string;
  readonly extraDoc: string;
  readonly extraName: string;
  readonly extraBody: string;
};

/** 📥️ Exact per-case schema-helper import line — no case pulls in a helper it does not assert with. */
const useSchema = (...names: readonly string[]): string => `use crate::artifacts::note::schema::{${names.join(", ")}};\n`;
const NODE_USE = "use crate::artifacts::note::NoteBlockNode;\n";

const photoBlock = () => ({
  kind: "image",
  id: "blk-photo",
  name: "Photo",
  x: F(620),
  y: F(0),
  width: F(200),
  height: F(140),
  rotation: F(0),
  visible: true,
  locked: false,
  imageKey: "asset-logo",
});

const mathCopy = () => ({
  kind: "math",
  id: "blk-math-copy",
  name: "Formula copy",
  x: F(0),
  y: F(400),
  width: F(200),
  height: F(80),
  rotation: F(0),
  visible: true,
  locked: false,
  tex: "E = mc^2",
  displayMode: true,
});

const inkCopy = () => ({
  kind: "stroke",
  id: "blk-ink-copy",
  name: "Sketch copy",
  x: F(20),
  y: F(160),
  width: F(80),
  height: F(40),
  rotation: F(0),
  visible: true,
  locked: false,
  points: [[F(0), F(0)], [F(10), F(10)]],
  strokeWidth: F(2),
  color: [F(0), F(0), F(0), F(1)],
});

const tableCopy = () => ({
  kind: "table",
  id: "blk-table-copy",
  name: "Samples copy",
  x: F(0),
  y: F(220),
  width: F(320),
  height: F(160),
  rotation: F(0),
  visible: true,
  locked: false,
  columns: ["A", "B"],
  rows: [[{ content: "Alpha" }, { content: "" }], [{ content: "" }, { content: "" }]],
});

export const cases: readonly Case[] = [
  {
    dir: "🏷️rename-note",
    slug: "rename-note",
    caseName: "retitles-the-document",
    modName: "rename_note",
    mutation: { mutation: "renameNote", newTitle: "Field Notes v2" },
    after: () => {
      const s = clone(base());
      s.title = "Field Notes v2";
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), title: "Field Notes v2" }),
    diffDoc: "Only the scalar `title` slot is set (as `Some(Some(..))`); every other `NoteDiff` field stays `None`, so no collection is even mentioned.",
    applyDiffDoc: "Applying the committed title-only delta to `before` reaches `after` — proof the rename never rewrites the block tree wholesale.",
    applyDoc: "`rename-note` writes `title` and nothing else — the diff sets only `NoteDiff.title`.",
    inverseDoc: "The inverse re-issues `rename-note` with the base's own prior title.",
    outcomeDoc: "The title genuinely differs from the base's, so the `mutation.no-op` warn guard does not fire.",
    extraDoc: "🏷️ Only the document title moves: blocks, assets and every grid/snap/tool setting stay byte-identical.",
    extraName: "only_the_document_title_changes",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("rename-note applies");
    assert_eq!(base.title.as_deref(), Some("Field Notes"), "rename-note/retitles-the-document: the base must start from the old title");
    assert_eq!(applied.title.as_deref(), Some("Field Notes v2"), "rename-note/retitles-the-document: the new title must be written");
    assert_eq!(applied.blocks, base.blocks, "rename-note must never touch the block tree");
    assert_eq!(applied.assets, base.assets, "rename-note must never touch the asset map");
    assert_eq!((applied.grid_spacing, applied.snap_enabled, applied.pencil_width), (base.grid_spacing, base.snap_enabled, base.pencil_width), "rename-note must never touch grid/snap/tool settings");`,
  },
  {
    dir: "👁️change-grid-visible",
    slug: "change-grid-visible",
    caseName: "hides-the-grid",
    modName: "change_grid_visible",
    mutation: { mutation: "changeGridVisible", newVisible: false },
    after: () => {
      const s = clone(base());
      s.gridVisible = false;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), gridVisible: false }),
    diffDoc: "Only the scalar `gridVisible` slot is set; `gridSpacing`/`gridSubdivisions`/`gridOpacity` stay `None` beside it.",
    applyDiffDoc: "The committed `gridVisible`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-grid-visible` writes `NoteDiff.grid_visible` only.",
    inverseDoc: "The inverse restores the base's own `grid_visible`, here `Some(true)`.",
    outcomeDoc: "`Some(false)` differs from the base's `Some(true)`, so the equality no-op guard does not fire.",
    extraDoc: "👁️ Grid visibility flips to false while the grid's own geometry settings are left alone.",
    extraName: "grid_visibility_flips_without_touching_grid_geometry",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-grid-visible applies");
    assert_eq!(base.grid_visible, Some(true), "change-grid-visible/hides-the-grid: the base must start with a visible grid");
    assert_eq!(applied.grid_visible, Some(false), "change-grid-visible/hides-the-grid: the grid must end up hidden");
    assert_eq!(applied.grid_spacing, Some(32.0), "hiding the grid must not resize it");
    assert_eq!(applied.grid_subdivisions, Some(4.0), "hiding the grid must not resubdivide it");
    assert_eq!(applied.grid_opacity, Some(0.35), "hiding the grid must not fade it");
    assert_eq!(applied.blocks, base.blocks, "hiding the grid must not touch the block tree");`,
  },
  {
    dir: "📏️change-grid-spacing",
    slug: "change-grid-spacing",
    caseName: "widens-grid-spacing",
    modName: "change_grid_spacing",
    mutation: { mutation: "changeGridSpacing", newSpacing: F(48) },
    after: () => {
      const s = clone(base());
      s.gridSpacing = F(48);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), gridSpacing: F(48) }),
    diffDoc: "Only the scalar `gridSpacing` slot is set; `snapGridSpacing` is a different field and stays `None`.",
    applyDiffDoc: "The committed `gridSpacing`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-grid-spacing` writes `NoteDiff.grid_spacing` only.",
    inverseDoc: "The inverse restores the base's own `grid_spacing`, here `Some(32.0)`.",
    outcomeDoc: "48.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.",
    extraDoc: "📏 The DRAWN grid spacing widens 32→48 while the independent SNAP grid spacing holds at 8.",
    extraName: "drawn_grid_spacing_widens_without_moving_the_snap_grid",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-grid-spacing applies");
    assert_eq!(base.grid_spacing, Some(32.0), "change-grid-spacing/widens-grid-spacing: the base must start at 32.0");
    assert_eq!(applied.grid_spacing, Some(48.0), "change-grid-spacing/widens-grid-spacing: spacing must widen to 48.0");
    assert_eq!(applied.snap_grid_spacing, Some(8.0), "the snap grid is a separate setting and must not follow the drawn grid");
    assert_eq!(applied.grid_subdivisions, Some(4.0), "widening the grid must not change its subdivision count");
    assert_eq!(applied.grid_visible, Some(true), "widening the grid must not change its visibility");`,
  },
  {
    dir: "🔢️change-grid-subdivisions",
    slug: "change-grid-subdivisions",
    caseName: "doubles-grid-subdivisions",
    modName: "change_grid_subdivisions",
    mutation: { mutation: "changeGridSubdivisions", newSubdivisions: F(8) },
    after: () => {
      const s = clone(base());
      s.gridSubdivisions = F(8);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), gridSubdivisions: F(8) }),
    diffDoc: "Only the scalar `gridSubdivisions` slot is set; the spacing it subdivides stays `None`.",
    applyDiffDoc: "The committed `gridSubdivisions`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-grid-subdivisions` writes `NoteDiff.grid_subdivisions` only.",
    inverseDoc: "The inverse restores the base's own `grid_subdivisions`, here `Some(4.0)`.",
    outcomeDoc: "8.0 is finite and >= 1.0, so the `mutation.invariant` fatal floor guard does not fire.",
    extraDoc: "🔢 Subdivisions double 4→8; the guard this leaf enforces is a floor of 1, not positivity.",
    extraName: "subdivisions_double_while_spacing_is_untouched",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-grid-subdivisions applies");
    assert_eq!(base.grid_subdivisions, Some(4.0), "change-grid-subdivisions/doubles-grid-subdivisions: the base must start at 4.0");
    assert_eq!(applied.grid_subdivisions, Some(8.0), "change-grid-subdivisions/doubles-grid-subdivisions: subdivisions must double to 8.0");
    assert!(applied.grid_subdivisions.expect("subdivisions are set") >= 1.0, "the applied value must satisfy this leaf's own >= 1 floor");
    assert_eq!(applied.grid_spacing, Some(32.0), "subdividing must not resize the grid itself");
    assert_eq!(applied.grid_opacity, Some(0.35), "subdividing must not fade the grid");`,
  },
  {
    dir: "🌫️change-grid-opacity",
    slug: "change-grid-opacity",
    caseName: "raises-grid-opacity",
    modName: "change_grid_opacity",
    mutation: { mutation: "changeGridOpacity", newOpacity: F(0.75) },
    after: () => {
      const s = clone(base());
      s.gridOpacity = F(0.75);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), gridOpacity: F(0.75) }),
    diffDoc: "Only the scalar `gridOpacity` slot is set; `gridVisible` stays `None`, so fading and hiding remain independent.",
    applyDiffDoc: "The committed `gridOpacity`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-grid-opacity` writes `NoteDiff.grid_opacity` only.",
    inverseDoc: "The inverse restores the base's own `grid_opacity`, here `Some(0.35)`.",
    outcomeDoc: "0.75 lies inside this leaf's closed `0.0..=1.0` band, so the `mutation.invariant` fatal guard does not fire.",
    extraDoc: "🌫️ Opacity rises 0.35→0.75 and stays inside the closed 0..=1 band this leaf alone enforces.",
    extraName: "opacity_rises_and_stays_inside_the_unit_band",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-grid-opacity applies");
    assert_eq!(base.grid_opacity, Some(0.35), "change-grid-opacity/raises-grid-opacity: the base must start at 0.35");
    assert_eq!(applied.grid_opacity, Some(0.75), "change-grid-opacity/raises-grid-opacity: opacity must rise to 0.75");
    let opacity = applied.grid_opacity.expect("opacity is set");
    assert!((0.0..=1.0).contains(&opacity), "the applied opacity must satisfy this leaf's own 0..=1 band");
    assert_eq!(applied.grid_visible, Some(true), "fading the grid is not the same as hiding it");
    assert_eq!(applied.grid_spacing, Some(32.0), "fading the grid must not resize it");`,
  },
  {
    dir: "🧲️change-snap-enabled",
    slug: "change-snap-enabled",
    caseName: "enables-snap",
    modName: "change_snap_enabled",
    mutation: { mutation: "changeSnapEnabled", newEnabled: true },
    after: () => {
      const s = clone(base());
      s.snapEnabled = true;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), snapEnabled: true }),
    diffDoc: "Only the scalar `snapEnabled` slot is set; crucially `blocks` stays `None`, so no block is retro-snapped.",
    applyDiffDoc: "The committed `snapEnabled`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-snap-enabled` writes `NoteDiff.snap_enabled` only.",
    inverseDoc: "The inverse restores the base's own `snap_enabled`, here `Some(false)`.",
    outcomeDoc: "`Some(true)` differs from the base's `Some(false)`, so the equality no-op guard does not fire.",
    extraDoc: "🧲 Snapping switches on without altering the snap step, and without moving any block that would now snap.",
    extraName: "snapping_switches_on_without_moving_blocks",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-snap-enabled applies");
    assert_eq!(base.snap_enabled, Some(false), "change-snap-enabled/enables-snap: the base must start with snapping off");
    assert_eq!(applied.snap_enabled, Some(true), "change-snap-enabled/enables-snap: snapping must switch on");
    assert_eq!(applied.snap_grid_spacing, Some(8.0), "enabling snapping must not change the snap step");
    assert_eq!(applied.blocks, base.blocks, "enabling snapping must never retro-snap existing blocks");`,
  },
  {
    dir: "📐️change-snap-grid-spacing",
    slug: "change-snap-grid-spacing",
    caseName: "halves-snap-grid-spacing",
    modName: "change_snap_grid_spacing",
    mutation: { mutation: "changeSnapGridSpacing", newSpacing: F(4) },
    after: () => {
      const s = clone(base());
      s.snapGridSpacing = F(4);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), snapGridSpacing: F(4) }),
    diffDoc: "Only the scalar `snapGridSpacing` slot is set; the drawn `gridSpacing` stays `None`.",
    applyDiffDoc: "The committed `snapGridSpacing`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-snap-grid-spacing` writes `NoteDiff.snap_grid_spacing` only.",
    inverseDoc: "The inverse restores the base's own `snap_grid_spacing`, here `Some(8.0)`.",
    outcomeDoc: "4.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.",
    extraDoc: "📐 The SNAP step halves 8→4 while the DRAWN grid spacing stays at 32, and snapping stays off.",
    extraName: "snap_step_halves_without_touching_the_drawn_grid",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-snap-grid-spacing applies");
    assert_eq!(base.snap_grid_spacing, Some(8.0), "change-snap-grid-spacing/halves-snap-grid-spacing: the base must start at 8.0");
    assert_eq!(applied.snap_grid_spacing, Some(4.0), "change-snap-grid-spacing/halves-snap-grid-spacing: the snap step must halve to 4.0");
    assert_eq!(applied.grid_spacing, Some(32.0), "the drawn grid is a separate setting and must not follow the snap step");
    assert_eq!(applied.snap_enabled, Some(false), "changing the snap step must not enable snapping");`,
  },
  {
    dir: "✏️change-pencil-width",
    slug: "change-pencil-width",
    caseName: "thickens-pencil",
    modName: "change_pencil_width",
    mutation: { mutation: "changePencilWidth", newWidth: F(5) },
    after: () => {
      const s = clone(base());
      s.pencilWidth = F(5);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), pencilWidth: F(5) }),
    diffDoc: "Only the scalar `pencilWidth` slot is set; `blocks` stays `None`, which is what proves the tool setting cannot reach an already-drawn stroke.",
    applyDiffDoc: "The committed `pencilWidth`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-pencil-width` writes `NoteDiff.pencil_width` only.",
    inverseDoc: "The inverse restores the base's own `pencil_width`, here `Some(3.0)`.",
    outcomeDoc: "5.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "✏️ The pencil TOOL width changes; the already-drawn `blk-ink` stroke keeps its own 2.0 width.",
    extraName: "tool_width_changes_but_existing_ink_keeps_its_own_width",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-pencil-width applies");
    assert_eq!(base.pencil_width, Some(3.0), "change-pencil-width/thickens-pencil: the base pencil must start at 3.0");
    assert_eq!(applied.pencil_width, Some(5.0), "change-pencil-width/thickens-pencil: the pencil must thicken to 5.0");
    let NoteBlockNode::Ink { stroke_width, .. } = find_block(&applied.blocks, "blk-ink").expect("the document still carries its ink block") else {
        panic!("change-pencil-width/thickens-pencil: blk-ink must still be an ink block");
    };
    assert_eq!(*stroke_width, 2.0, "the pencil tool width is a document setting — it must never retro-edit an already-drawn stroke");
    assert_eq!(applied.eraser_radius, Some(12.0), "the eraser is a separate tool setting");`,
  },
  {
    dir: "🧽️change-eraser-radius",
    slug: "change-eraser-radius",
    caseName: "enlarges-eraser",
    modName: "change_eraser_radius",
    mutation: { mutation: "changeEraserRadius", newRadius: F(24) },
    after: () => {
      const s = clone(base());
      s.eraserRadius = F(24);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), eraserRadius: F(24) }),
    diffDoc: "Only the scalar `eraserRadius` slot is set; `blocks` stays `None`, so enlarging the eraser erases nothing.",
    applyDiffDoc: "The committed `eraserRadius`-only delta carries `before` to `after` on its own.",
    applyDoc: "`change-eraser-radius` writes `NoteDiff.eraser_radius` only.",
    inverseDoc: "The inverse restores the base's own `eraser_radius`, here `Some(12.0)`.",
    outcomeDoc: "24.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.",
    extraDoc: "🧽 The eraser radius doubles while the pencil tool and every block stay exactly as they were.",
    extraName: "eraser_radius_doubles_and_erases_nothing",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-eraser-radius applies");
    assert_eq!(base.eraser_radius, Some(12.0), "change-eraser-radius/enlarges-eraser: the base eraser must start at 12.0");
    assert_eq!(applied.eraser_radius, Some(24.0), "change-eraser-radius/enlarges-eraser: the eraser must grow to 24.0");
    assert_eq!(applied.pencil_width, Some(3.0), "the pencil is a separate tool setting");
    assert_eq!(applied.blocks, base.blocks, "enlarging the eraser must not itself erase anything");`,
  },
  {
    dir: "🆕️create-asset",
    slug: "create-asset",
    caseName: "adds-a-second-image-asset",
    modName: "create_asset",
    mutation: { mutation: "createAsset", key: "asset-sketch", asset: { mime: "image/jpeg", data: "c2tldGNo" } },
    after: () => {
      const s = clone(base());
      (s.assets as Record<string, unknown>)["asset-sketch"] = { mime: "image/jpeg", data: "c2tldGNo" };
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), assets: assetsDelta({ "asset-sketch": { mime: "image/jpeg", data: "c2tldGNo" } }) }),
    diffDoc: "One `assets.entries` UPSERT keyed by the new id — the sibling `asset-logo` key is not in the delta at all, so it cannot be rewritten.",
    applyDiffDoc: "The committed single-key asset upsert carries `before` to `after` on its own.",
    applyDoc: "`create-asset` emits a single-key asset UPSERT entry, leaving every other key alone.",
    inverseDoc: "The inverse is `delete-asset` on the freshly created key.",
    outcomeDoc: "`asset-sketch` is absent from the base, so the `mutation.duplicate-id` fatal guard does not fire.",
    extraDoc: "🆕 A second asset key appears; the pre-existing `asset-logo` entry survives untouched, dimensions included.",
    extraName: "new_asset_key_appears_beside_the_untouched_existing_one",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("create-asset applies");
    assert_eq!(base.assets.len(), 1, "create-asset/adds-a-second-image-asset: the base must carry exactly one asset");
    assert_eq!(applied.assets.len(), 2, "create-asset/adds-a-second-image-asset: the asset map must grow by exactly one key");
    let created = applied.assets.get("asset-sketch").expect("the created key must exist");
    assert_eq!(created.mime, "image/jpeg", "the created asset must carry the payload the mutation named");
    assert_eq!((created.width, created.height), (None, None), "the created asset declares no intrinsic dimensions");
    assert_eq!(applied.assets.get("asset-logo"), base.assets.get("asset-logo"), "creating an asset must never rewrite a sibling key");
    assert_eq!(applied.blocks, base.blocks, "creating an asset must not touch the block tree");`,
  },
  {
    dir: "🔁️replace-asset-payload",
    slug: "replace-asset-payload",
    caseName: "swaps-logo-payload-for-svg",
    modName: "replace_asset_payload",
    mutation: { mutation: "replaceAssetPayload", key: "asset-logo", newAsset: { mime: "image/svg+xml", data: "PHN2Zy8+" } },
    after: () => {
      const s = clone(base());
      (s.assets as Record<string, unknown>)["asset-logo"] = { mime: "image/svg+xml", data: "PHN2Zy8+" };
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), assets: assetsDelta({ "asset-logo": { mime: "image/svg+xml", data: "PHN2Zy8+" } }) }),
    diffDoc: "One `assets.entries` UPSERT carrying the WHOLE new asset value — no per-field asset patch shape exists, which is exactly why the old dimensions vanish.",
    applyDiffDoc: "The committed whole-value asset upsert carries `before` to `after` on its own.",
    applyDoc: "`replace-asset-payload` is a WHOLE-VALUE swap: the stored asset is replaced, not merged.",
    inverseDoc: "The inverse re-issues `replace-asset-payload` carrying the base's own prior asset value.",
    outcomeDoc: "`asset-logo` exists and its payload genuinely differs, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.",
    extraDoc: "🔁 The whole asset value is swapped — the old PNG's 64x64 dimensions are dropped, not merged forward.",
    extraName: "whole_asset_value_is_swapped_dropping_the_old_dimensions",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("replace-asset-payload applies");
    assert_eq!(applied.assets.len(), base.assets.len(), "replace-asset-payload/swaps-logo-payload-for-svg: replacing must not change the key count");
    let prior = base.assets.get("asset-logo").expect("the base asset exists");
    assert_eq!((prior.mime.as_str(), prior.width, prior.height), ("image/png", Some(64.0), Some(64.0)), "the base asset must start as a sized PNG");
    let next = applied.assets.get("asset-logo").expect("the replaced asset exists");
    assert_eq!(next.mime, "image/svg+xml", "the replaced asset must carry the new mime");
    assert_eq!(next.data, "PHN2Zy8+", "the replaced asset must carry the new payload");
    assert_eq!((next.width, next.height), (None, None), "a WHOLE-VALUE swap drops the prior dimensions instead of merging them forward");`,
  },
  {
    dir: "🗑️delete-asset",
    slug: "delete-asset",
    caseName: "removes-the-logo-asset",
    modName: "delete_asset",
    mutation: { mutation: "deleteAsset", key: "asset-logo" },
    after: () => {
      const s = clone(base()) as Record<string, unknown>;
      delete s.assets;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), assets: assetsDelta({ "asset-logo": null }) }),
    diffDoc: "One `assets.entries` REMOVAL (a `null` value under the key); `blocks` stays `None`, which is the machine-readable proof that this leaf has no block cascade.",
    applyDiffDoc: "The committed single-key asset removal carries `before` to `after` on its own.",
    applyDoc: "`delete-asset` emits a single-key asset REMOVAL entry (`None` value) for the addressed key.",
    inverseDoc: "The inverse is `create-asset` re-carrying the base's own prior asset value verbatim.",
    outcomeDoc: "`asset-logo` exists in the base, so the `mutation.target-missing` error guard does not fire.",
    extraDoc: "🗑️ The asset map empties (and therefore disappears from the JSON); the image blocks that referenced the key are deliberately left dangling — this leaf has no block cascade.",
    extraName: "asset_map_empties_and_referencing_blocks_are_left_dangling",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("delete-asset applies");
    assert!(base.assets.contains_key("asset-logo"), "delete-asset/removes-the-logo-asset: the base must carry the key being deleted");
    assert!(applied.assets.is_empty(), "delete-asset/removes-the-logo-asset: the asset map must end up empty");
    assert_eq!(applied.blocks, base.blocks, "delete-asset has NO block cascade — the image blocks keep their now-dangling imageKey");
    assert!(!AFTER.contains("assets"), "an empty asset map is skipped by serde, so the committed after-snapshot must carry no \\"assets\\" key");`,
  },
  {
    dir: "➕️create-block",
    slug: "create-block",
    caseName: "inserts-a-photo-block-at-root-index-2",
    modName: "create_block",
    mutation: { mutation: "createBlock", block: photoBlock(), parentId: null, index: 2 },
    after: () => {
      const s = clone(base());
      s.blocks.splice(2, 0, photoBlock() as never);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ added: [addedEntry(null, 2, photoBlock())] }) }),
    diffDoc: "One `blocks.added` entry carrying `(parentId: null, index: 2)` and the whole new node; `removed`/`patched` stay empty and `reordered` stays `null` — never a whole-`blocks` swap.",
    applyDiffDoc: "The committed single-`added` delta carries `before` to `after` on its own, inserting at the addressed index.",
    applyDoc: "`create-block` emits ONE `added` entry carrying the payload's own `(parent_id, index)` — never a whole-`blocks` swap.",
    inverseDoc: "The inverse is `delete-block` on the created id.",
    outcomeDoc: "`blk-photo` is absent and `parent_id` is `None`, so neither the `mutation.duplicate-id` nor the container `mutation.invariant` fatal guard fires.",
    extraUse: useSchema("find_block", "find_block_location"),
    extraDoc: "➕ The new block lands at the ADDRESSED root index 2, pushing the table/math/image/group right — not appended at the end.",
    extraName: "new_block_lands_at_the_addressed_index_not_appended",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("create-block applies");
    assert!(find_block(&base.blocks, "blk-photo").is_none(), "create-block/inserts-a-photo-block-at-root-index-2: the base must not already carry the new id");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 1, "create-block must grow the root list by exactly one");
    assert_eq!(find_block_location(&applied.blocks, "blk-photo"), Some((None, 2)), "the block must land at the addressed root index 2, not be appended");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 3)), "the block formerly at index 2 must have been pushed right");
    assert_eq!(find_block_location(&applied.blocks, "blk-text"), Some((None, 0)), "blocks before the insertion point must not move");`,
  },
  {
    dir: "❌️delete-block",
    slug: "delete-block",
    caseName: "removes-the-math-block",
    modName: "delete_block",
    mutation: { mutation: "deleteBlock", id: "blk-math" },
    after: () => {
      const s = clone(base());
      s.blocks.splice(indexOf(base(), "blk-math"), 1);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ removed: ["blk-math"] }) }),
    diffDoc: "One id in `blocks.removed`; nothing is added, patched or reordered, so the surviving siblings close up by the apply layer's own rules rather than by an authored list.",
    applyDiffDoc: "The committed single-`removed` delta carries `before` to `after` on its own.",
    applyDoc: "`delete-block` emits ONE `removed` id; the block's siblings close up around it.",
    inverseDoc: "The inverse is `create-block` re-carrying the block AND its exact `(parent_id, index)` from `find_block_location`.",
    outcomeDoc: "`blk-math` exists in the base, so the `mutation.target-missing` error guard does not fire.",
    extraUse: useSchema("find_block", "find_block_location"),
    extraDoc: "❌ The math block at a NON-LAST root index disappears and its right-hand siblings shift left by one.",
    extraName: "non_last_block_is_removed_and_siblings_shift_left",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("delete-block applies");
    assert_eq!(find_block_location(&base.blocks, "blk-math"), Some((None, 3)), "delete-block/removes-the-math-block: the base must hold blk-math at a non-last root index");
    assert!(find_block(&applied.blocks, "blk-math").is_none(), "delete-block/removes-the-math-block: the block must be gone");
    assert_eq!(applied.blocks.len(), base.blocks.len() - 1, "delete-block must shrink the root list by exactly one");
    assert_eq!(find_block_location(&applied.blocks, "blk-image"), Some((None, 3)), "the sibling after the deleted block must shift left into index 3");
    assert!(find_block(&applied.blocks, "blk-nested").is_some(), "deleting a root block must not disturb the group's nested subtree");`,
  },
  {
    dir: "🧺️delete-blocks",
    slug: "delete-blocks",
    caseName: "removes-the-ink-and-image-blocks",
    modName: "delete_blocks",
    mutation: { mutation: "deleteBlocks", ids: ["blk-ink", "blk-image"] },
    after: () => {
      const s = clone(base());
      s.blocks = s.blocks.filter((block) => block.id !== "blk-ink" && block.id !== "blk-image");
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ removed: ["blk-ink", "blk-image"] }) }),
    diffDoc: "Both ids land in ONE `blocks.removed` list, in the payload's own order — the batch verb collapses to a single delta, not two.",
    applyDiffDoc: "The committed two-id `removed` delta carries `before` to `after` in one apply.",
    applyDoc: "`delete-blocks` emits ONE `removed` list holding every addressed id that actually exists.",
    inverseDoc: "The inverse is a `create-block` PER removed id, ordered so the caller's reversal replays them lowest-index-first and each original absolute index stays valid.",
    outcomeDoc: "Both ids exist, so neither the `mutation.target-missing` error nor the `mutation.partial` warn fires.",
    extraUse: useSchema("find_block", "find_block_location"),
    extraDoc: "🧺 Two NON-ADJACENT root blocks go in one operation — the batch verb, not two `delete-block`s — and the survivors keep their relative order.",
    extraName: "two_non_adjacent_blocks_go_in_one_batch",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("delete-blocks applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "delete-blocks/removes-the-ink-and-image-blocks: blk-ink must start at root index 1");
    assert_eq!(find_block_location(&base.blocks, "blk-image"), Some((None, 4)), "delete-blocks/removes-the-ink-and-image-blocks: blk-image must start at root index 4, non-adjacent to blk-ink");
    assert!(find_block(&applied.blocks, "blk-ink").is_none(), "blk-ink must be gone");
    assert!(find_block(&applied.blocks, "blk-image").is_none(), "blk-image must be gone");
    assert_eq!(applied.blocks.len(), base.blocks.len() - 2, "delete-blocks must shrink the root list by exactly two in ONE operation");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 1)), "survivors must close up in their original relative order");
    assert_eq!(inverse_note_mutation(&base, &mutation()).len(), 2, "the inverse must be one create-block per removed id");`,
  },
  {
    dir: "🎯️duplicate-block",
    slug: "duplicate-block",
    caseName: "copies-the-math-block-right-after-its-source",
    modName: "duplicate_block",
    mutation: { mutation: "duplicateBlock", sourceId: "blk-math", block: mathCopy() },
    after: () => {
      const s = clone(base());
      s.blocks.splice(indexOf(base(), "blk-math") + 1, 0, mathCopy() as never);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ added: [addedEntry(null, 4, mathCopy())] }) }),
    diffDoc: "One `blocks.added` entry whose index is the SOURCE's base index + 1; `removed` stays empty, so the source is provably untouched.",
    applyDiffDoc: "The committed single-`added` delta carries `before` to `after` on its own.",
    applyDoc: "`duplicate-block` places the copy at the SOURCE's own `(parent_id, index + 1)`, taken from the base — the payload carries no position at all.",
    inverseDoc: "The inverse is `delete-block` on the copy's id.",
    outcomeDoc: "The source exists and the copy's id is fresh, so neither the `mutation.target-missing` error nor the `mutation.duplicate-id` fatal fires.",
    extraUse: useSchema("block_name", "find_block", "find_block_location"),
    extraDoc: "🎯 The copy lands immediately AFTER its source (source index + 1), keeping the source in place, and carries the payload's own new identity.",
    extraName: "copy_lands_immediately_after_its_source",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("duplicate-block applies");
    assert_eq!(find_block_location(&base.blocks, "blk-math"), Some((None, 3)), "duplicate-block/copies-the-math-block-right-after-its-source: the source must start at root index 3");
    assert_eq!(find_block_location(&applied.blocks, "blk-math"), Some((None, 3)), "the source must stay exactly where it was");
    assert_eq!(find_block_location(&applied.blocks, "blk-math-copy"), Some((None, 4)), "the copy must land at source index + 1");
    assert_eq!(block_name(find_block(&applied.blocks, "blk-math-copy").expect("the copy exists")), "Formula copy", "the copy carries the identity the payload named");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 1, "duplicate-block adds exactly one block");`,
  },
  {
    dir: "👥️duplicate-blocks",
    slug: "duplicate-blocks",
    caseName: "copies-ink-and-table-with-shifting-indices",
    modName: "duplicate_blocks",
    mutation: { mutation: "duplicateBlocks", sourceIds: ["blk-ink", "blk-table"], blocks: [inkCopy(), tableCopy()] },
    after: () => {
      const s = clone(base());
      s.blocks.splice(2, 0, inkCopy() as never);
      s.blocks.splice(3, 0, tableCopy() as never);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ added: [addedEntry(null, 2, inkCopy()), addedEntry(null, 3, tableCopy())] }) }),
    diffDoc: "Two `blocks.added` entries, each pinned to its own source's BASE index + 1 (2 and 3) — the committed indices are what make the sequential-insert skew reproducible.",
    applyDiffDoc: "Applying both committed `added` entries in order carries `before` to `after`, skew included.",
    applyDoc: "`duplicate-blocks` emits ONE `added` entry per (source, copy) pair, each pinned to that source's own base index + 1 — the entries are then inserted in order, so later indices are read against an already-grown list.",
    inverseDoc: "The inverse is a single `delete-blocks` naming every copy's id.",
    outcomeDoc: "Both sources exist and both copy ids are fresh, so none of the `mutation.duplicate-id` fatal, `mutation.target-missing` error or `mutation.partial` warn fires.",
    extraUse: useSchema("find_block_location"),
    extraDoc: "👥 Indices are computed against the BASE but applied sequentially, so the second copy lands ahead of its own source — the observable batch-insert skew this leaf has and `duplicate-block` cannot.",
    extraName: "second_copy_lands_ahead_of_its_own_source_from_index_skew",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("duplicate-blocks applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: blk-ink must start at root index 1");
    assert_eq!(find_block_location(&base.blocks, "blk-table"), Some((None, 2)), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: blk-table must start at root index 2");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 2, "duplicate-blocks adds exactly one copy per source in ONE operation");
    assert_eq!(find_block_location(&applied.blocks, "blk-ink-copy"), Some((None, 2)), "the first copy lands at its source index + 1");
    assert_eq!(find_block_location(&applied.blocks, "blk-table-copy"), Some((None, 3)), "the second copy uses its BASE index + 1 against an already-grown list");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 4)), "so the second copy ends up ahead of its own source — the batch-insert skew this leaf owns");`,
  },
  {
    dir: "🚚️move-block-to-container",
    slug: "move-block-to-container",
    caseName: "reparents-ink-into-the-callout-group",
    modName: "move_block_to_container",
    mutation: { mutation: "moveBlockToContainer", id: "blk-ink", newParentId: "blk-group", index: 0 },
    after: () => {
      const s = clone(base());
      const ink = at(s, "blk-ink");
      s.blocks = s.blocks.filter((block) => block.id !== "blk-ink");
      (at(s, "blk-group").children as unknown[]).unshift(ink);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ removed: ["blk-ink"], added: [addedEntry("blk-group", 0, at(base(), "blk-ink"))] }) }),
    diffDoc: "A `removed` id AND an `added` entry in the SAME delta — the reparent is one atomic sparse change, and the added entry carries the block value verbatim from the base.",
    applyDiffDoc: "The committed remove+add delta carries `before` to `after` in one apply (removals run before additions).",
    applyDoc: "`move-block-to-container` emits a `removed` id AND an `added` entry in ONE diff; apply runs removals before additions, so the block is lifted and re-placed atomically.",
    inverseDoc: "The inverse re-issues `move-block-to-container` with the base's own `(parent_id, index)` from `find_block_location`.",
    outcomeDoc: "The block exists, the container is a real group, and it is not the block itself, so neither the `mutation.target-missing` error nor the `mutation.invariant` fatal fires.",
    extraUse: useSchema("block_bounds", "find_block", "find_block_location"),
    extraDoc: "🚚 The block changes PARENT, not coordinates: it leaves the root, enters the group at index 0 ahead of the existing child, and keeps its own x/y.",
    extraName: "block_changes_parent_at_an_index_without_moving_in_space",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("move-block-to-container applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "move-block-to-container/reparents-ink-into-the-callout-group: blk-ink must start at the document root");
    assert_eq!(find_block_location(&applied.blocks, "blk-ink"), Some((Some("blk-group".to_string()), 0)), "blk-ink must end up as the group's first child");
    assert_eq!(find_block_location(&applied.blocks, "blk-nested"), Some((Some("blk-group".to_string()), 1)), "the group's existing child must be pushed right by the index-0 insertion");
    assert_eq!(applied.blocks.len(), base.blocks.len() - 1, "the root list must lose exactly the reparented block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-ink").expect("the moved block exists")), block_bounds(find_block(&base.blocks, "blk-ink").expect("the base block exists")), "reparenting must not move the block in space");`,
  },
  {
    dir: "🤏️drag-blocks",
    slug: "drag-blocks",
    caseName: "nudges-ink-and-the-whole-group-subtree",
    modName: "drag_blocks",
    mutation: { mutation: "dragBlocks", ids: ["blk-ink", "blk-group"], dx: F(12), dy: F(-8) },
    after: () => {
      const s = clone(base());
      const ink = at(s, "blk-ink");
      ink.x = F(32);
      ink.y = F(152);
      const group = at(s, "blk-group");
      group.x = F(352);
      group.y = F(192);
      const badge = nested(s);
      badge.x = F(362);
      badge.y = F(202);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-ink", at(after, "blk-ink")), patchEntry("blk-group", at(after, "blk-group"))] }) }),
    diffDoc: "Two `blocks.patched` entries, one per addressed id; the group's `blockJson` already contains its offset child, so the subtree recursion is visible in the delta itself.",
    applyDiffDoc: "The committed two-entry `patched` delta carries `before` to `after` on its own.",
    applyDoc: "`drag-blocks` emits ONE `patched` entry per addressed block, each holding the offset whole-block value produced by `offset_block_tree`.",
    inverseDoc: "The inverse is the same `drag-blocks` with `(-dx, -dy)` — no snapshot lookup at all.",
    outcomeDoc: "Both ids exist, so neither the `mutation.target-missing` error nor the `mutation.partial` warn fires.",
    extraUse: useSchema("block_bounds", "find_block"),
    extraDoc: "🤏 A RELATIVE offset, applied to several blocks at once, that recurses into a dragged group's children — the nested badge moves with its parent.",
    extraName: "relative_offset_recurses_into_the_dragged_group_subtree",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("drag-blocks applies");
    assert_eq!(block_bounds(find_block(&base.blocks, "blk-ink").expect("the base ink block exists")), (20.0, 160.0, 80.0, 40.0), "drag-blocks/nudges-ink-and-the-whole-group-subtree: the base geometry must be the one this case was derived from");
    let (ink_x, ink_y, ink_w, ink_h) = block_bounds(find_block(&applied.blocks, "blk-ink").expect("the dragged ink block exists"));
    assert_eq!((ink_x, ink_y), (32.0, 152.0), "the offset must be RELATIVE: (20, 160) + (12, -8)");
    assert_eq!((ink_w, ink_h), (80.0, 40.0), "dragging must never resize a block");
    let (group_x, group_y, ..) = block_bounds(find_block(&applied.blocks, "blk-group").expect("the dragged group exists"));
    assert_eq!((group_x, group_y), (352.0, 192.0), "the group itself takes the same offset");
    let (badge_x, badge_y, ..) = block_bounds(find_block(&applied.blocks, "blk-nested").expect("the nested badge exists"));
    assert_eq!((badge_x, badge_y), (362.0, 202.0), "a dragged group carries its children: the offset recurses into the subtree");`,
  },
  {
    dir: "🔖️rename-block",
    slug: "rename-block",
    caseName: "renames-the-table-block",
    modName: "rename_block",
    mutation: { mutation: "renameBlock", id: "blk-table", newName: "Measurements" },
    after: () => {
      const s = clone(base());
      at(s, "blk-table").name = "Measurements";
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-table", at(after, "blk-table"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` is the whole table node with only `name` changed — the delta shape is whole-block, so the assertion is that nothing ELSE inside it moved.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`rename-block` emits ONE whole-block `patched` entry whose only changed field is `name`.",
    inverseDoc: "The inverse re-issues `rename-block` with the base block's own prior name.",
    outcomeDoc: "The block exists and its name genuinely differs, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.",
    extraUse: `${useSchema("block_name", "find_block")}${NODE_USE}`,
    extraDoc: "🔖 The display NAME changes while the block's identity, kind and table payload are untouched.",
    extraName: "display_name_changes_but_identity_and_payload_do_not",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("rename-block applies");
    assert_eq!(block_name(find_block(&base.blocks, "blk-table").expect("the base table exists")), "Samples", "rename-block/renames-the-table-block: the base must start from the old name");
    let renamed = find_block(&applied.blocks, "blk-table").expect("the renamed block is still addressable by its id");
    assert_eq!(block_name(renamed), "Measurements", "rename-block/renames-the-table-block: the new name must be written");
    assert!(matches!(renamed, NoteBlockNode::Table { columns, rows, .. } if columns.len() == 2 && rows.len() == 2), "renaming must leave the table payload alone");
    assert_eq!(applied.blocks.len(), base.blocks.len(), "renaming must not add or remove blocks");`,
  },
  {
    dir: "👀️change-block-visible",
    slug: "change-block-visible",
    caseName: "hides-the-image-block",
    modName: "change_block_visible",
    mutation: { mutation: "changeBlockVisible", id: "blk-image", newVisible: false },
    after: () => {
      const s = clone(base());
      at(s, "blk-image").visible = false;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-image", at(after, "blk-image"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` differs from the base node only in `visible`; `removed` stays empty, which is the delta-level proof that hiding is not deleting.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`change-block-visible` emits ONE whole-block `patched` entry whose only changed field is `visible`.",
    inverseDoc: "The inverse re-issues `change-block-visible` with the base block's own prior visibility.",
    outcomeDoc: "The block exists and is currently visible, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.",
    extraUse: useSchema("block_locked", "block_visible", "find_block", "find_block_location"),
    extraDoc: "👀 The block is HIDDEN, not deleted and not locked — it stays in the tree at its own index.",
    extraName: "block_is_hidden_not_deleted_and_not_locked",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-visible applies");
    assert!(block_visible(find_block(&base.blocks, "blk-image").expect("the base image exists")), "change-block-visible/hides-the-image-block: the base block must start visible");
    let hidden = find_block(&applied.blocks, "blk-image").expect("a hidden block is still in the tree");
    assert!(!block_visible(hidden), "change-block-visible/hides-the-image-block: the block must end up hidden");
    assert!(!block_locked(hidden), "hiding must not also lock the block");
    assert_eq!(find_block_location(&applied.blocks, "blk-image"), Some((None, 4)), "hiding must not reorder the tree");
    assert_eq!(applied.blocks.len(), base.blocks.len(), "hiding is not deleting");`,
  },
  {
    dir: "🔒️change-block-locked",
    slug: "change-block-locked",
    caseName: "locks-the-callout-group",
    modName: "change_block_locked",
    mutation: { mutation: "changeBlockLocked", id: "blk-group", newLocked: true },
    after: () => {
      const s = clone(base());
      at(s, "blk-group").locked = true;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-group", at(after, "blk-group"))] }) }),
    diffDoc: "One `blocks.patched` entry for the group; its `blockJson` embeds the nested child STILL UNLOCKED, so the absence of a cascade is committed, not merely observed.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`change-block-locked` emits ONE whole-block `patched` entry whose only changed field is `locked`.",
    inverseDoc: "The inverse re-issues `change-block-locked` with the base block's own prior locked flag.",
    outcomeDoc: "The block exists and is currently unlocked, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.",
    extraUse: useSchema("block_locked", "block_visible", "find_block"),
    extraDoc: "🔒 Locking a GROUP is not recursive: the group's own flag flips, its nested child stays unlocked.",
    extraName: "locking_a_group_does_not_cascade_to_its_children",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-locked applies");
    assert!(!block_locked(find_block(&base.blocks, "blk-group").expect("the base group exists")), "change-block-locked/locks-the-callout-group: the base group must start unlocked");
    let locked = find_block(&applied.blocks, "blk-group").expect("the locked group is still in the tree");
    assert!(block_locked(locked), "change-block-locked/locks-the-callout-group: the group must end up locked");
    assert!(block_visible(locked), "locking must not also hide the block");
    assert!(!block_locked(find_block(&applied.blocks, "blk-nested").expect("the nested badge exists")), "this leaf patches ONE block — locking a group must not cascade into its children");`,
  },
  {
    dir: "📍️move-block",
    slug: "move-block",
    caseName: "repositions-the-math-block",
    modName: "move_block",
    mutation: { mutation: "moveBlock", id: "blk-math", newX: F(40), newY: F(320) },
    after: () => {
      const s = clone(base());
      const math = at(s, "blk-math");
      math.x = F(40);
      math.y = F(320);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-math", at(after, "blk-math"))] }) }),
    diffDoc: "One `blocks.patched` entry addressing exactly one id — no sibling appears in the delta, so no sibling can move.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`move-block` emits ONE whole-block `patched` entry whose only changed fields are `x`/`y`.",
    inverseDoc: "The inverse re-issues `move-block` with the base block's own prior `(x, y)` from `block_bounds`.",
    outcomeDoc: "The block exists, the target is finite, and it genuinely differs from the current position, so none of the `mutation.target-missing` error, `mutation.invariant` fatal or `mutation.no-op` warn fires.",
    extraUse: useSchema("block_bounds", "find_block"),
    extraDoc: "📍 An ABSOLUTE reposition of a single block — the new coordinates replace the old ones outright and the size is untouched.",
    extraName: "single_block_takes_absolute_coordinates_without_resizing",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("move-block applies");
    assert_eq!(block_bounds(find_block(&base.blocks, "blk-math").expect("the base math block exists")), (0.0, 400.0, 200.0, 80.0), "move-block/repositions-the-math-block: the base geometry must be the one this case was derived from");
    let (x, y, width, height) = block_bounds(find_block(&applied.blocks, "blk-math").expect("the moved block exists"));
    assert_eq!((x, y), (40.0, 320.0), "the coordinates are ABSOLUTE — they replace (0, 400) rather than offsetting it");
    assert_eq!((width, height), (200.0, 80.0), "moving must never resize the block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-text").expect("the text block exists")), (0.0, 0.0, 280.0, 120.0), "move-block addresses exactly ONE block — no sibling may move");`,
  },
  {
    dir: "↔️resize-block",
    slug: "resize-block",
    caseName: "enlarges-the-image-block",
    modName: "resize_block",
    mutation: { mutation: "resizeBlock", id: "blk-image", newWidth: F(320), newHeight: F(200) },
    after: () => {
      const s = clone(base());
      const image = at(s, "blk-image");
      image.width = F(320);
      image.height = F(200);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-image", at(after, "blk-image"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` keeps the original `x`/`y` — the top-left anchor is pinned by the committed delta, not just by the end state.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`resize-block` emits ONE whole-block `patched` entry whose only changed fields are `width`/`height`.",
    inverseDoc: "The inverse re-issues `resize-block` with the base block's own prior `(width, height)` from `block_bounds`.",
    outcomeDoc: "The block exists, the size is finite AND strictly positive, and it genuinely differs, so none of the `mutation.target-missing` error, `mutation.invariant` fatal or `mutation.no-op` warn fires.",
    extraUse: useSchema("block_bounds", "find_block"),
    extraDoc: "↔️ The extent grows from its top-left anchor: width/height change, x/y do not — and this leaf's invariant demands strictly positive extents.",
    extraName: "extent_grows_from_the_unchanged_top_left_anchor",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("resize-block applies");
    assert_eq!(block_bounds(find_block(&base.blocks, "blk-image").expect("the base image exists")), (340.0, 0.0, 240.0, 160.0), "resize-block/enlarges-the-image-block: the base geometry must be the one this case was derived from");
    let (x, y, width, height) = block_bounds(find_block(&applied.blocks, "blk-image").expect("the resized block exists"));
    assert_eq!((width, height), (320.0, 200.0), "the extent must take the addressed size");
    assert_eq!((x, y), (340.0, 0.0), "resizing anchors at the top-left — the position must not drift");
    assert!(width > 0.0 && height > 0.0, "this leaf's own invariant is strict positivity, not mere finiteness");`,
  },
  {
    dir: "🔤️change-block-font-size",
    slug: "change-block-font-size",
    caseName: "enlarges-the-intro-font",
    modName: "change_block_font_size",
    mutation: { mutation: "changeBlockFontSize", id: "blk-text", newFontSize: F(24) },
    after: () => {
      const s = clone(base());
      at(s, "blk-text").fontSize = F(24);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-text", at(after, "blk-text"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` still carries the UNCHANGED `content` child handle — the delta itself proves a font change never reminis composed content.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`change-block-font-size` emits ONE whole-block `patched` entry whose only changed field is `font_size`.",
    inverseDoc: "The inverse re-issues `change-block-font-size` with the base text block's own prior size.",
    outcomeDoc: "The block exists AND is a text block, and 24.0 differs from 16.0, so neither the `mutation.target-missing` error (absent or non-text) nor the `mutation.no-op` warn fires.",
    extraUse: `${useSchema("block_bounds", "find_block")}${NODE_USE}`,
    extraDoc: "🔤 A TEXT-ONLY field: the font size changes while the block's composed content handle, weight, alignment and box are untouched.",
    extraName: "text_only_font_size_changes_leaving_the_content_handle_alone",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-font-size applies");
    let NoteBlockNode::Text { font_size: before_size, content: before_content, .. } = find_block(&base.blocks, "blk-text").expect("the base text block exists") else {
        panic!("change-block-font-size/enlarges-the-intro-font: the base block must be a text block");
    };
    assert_eq!(*before_size, 16.0, "change-block-font-size/enlarges-the-intro-font: the base must start at 16.0");
    let NoteBlockNode::Text { font_size, content, font_weight, align, .. } = find_block(&applied.blocks, "blk-text").expect("the text block survives") else {
        panic!("change-block-font-size must not change the block's kind");
    };
    assert_eq!(*font_size, 24.0, "change-block-font-size/enlarges-the-intro-font: the font must grow to 24.0");
    assert_eq!(content, before_content, "resizing the font must not remint the composed text child handle");
    assert_eq!((font_weight.as_str(), align.as_str()), ("normal", "left"), "resizing the font must not restyle the block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-text").expect("the text block exists")), (0.0, 0.0, 280.0, 120.0), "resizing the font must not reflow the block's box");`,
  },
  {
    dir: "📝️edit-block-text",
    slug: "edit-block-text",
    caseName: "replaces-the-intro-paragraphs",
    modName: "edit_block_text",
    mutation: { mutation: "editBlockText", id: "blk-text", newParagraphs: [{ runs: [{ text: "Hello, note." }] }] },
    after: () => {
      const s = clone(base());
      (at(s, "blk-text").content as Record<string, unknown>).childId = TEXT_HANDLE_HELLO;
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-text", at(after, "blk-text"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` carries the REMINTED `content.childId` and the same `target` — the paragraphs never appear in the delta, only the handle they address.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own, without any working-scene cache being consulted.",
    applyDoc: "`edit-block-text` remints the block's COMPOSED child handle from `(block_id, new_paragraphs)` — the paragraphs themselves never land in the snapshot.",
    inverseDoc: "The inverse reads the base handle's paragraphs back out of the working-scene cache; the base handle here is the one minted for an EMPTY paragraph list, which is exactly what an uncached read returns.",
    outcomeDoc: "The block exists AND is a text block, so the `mutation.target-missing` error guard (absent or non-text) does not fire; this leaf has no no-op guard at all.",
    extraUse: `${useSchema("block_bounds", "find_block")}${NODE_USE}`,
    extraDoc: "📝 Content addressing in action: the persisted `content.child_id` changes because the paragraphs changed, while the child's `target` slot (keyed by block id) stays put.",
    extraName: "content_child_id_is_reminted_while_the_target_slot_is_stable",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-text applies");
    let NoteBlockNode::Text { content: before_content, .. } = find_block(&base.blocks, "blk-text").expect("the base text block exists") else {
        panic!("edit-block-text/replaces-the-intro-paragraphs: the base block must be a text block");
    };
    let NoteBlockNode::Text { content, font_size, .. } = find_block(&applied.blocks, "blk-text").expect("the text block survives") else {
        panic!("edit-block-text must not change the block's kind");
    };
    assert_ne!(content.child_id, before_content.child_id, "edit-block-text/replaces-the-intro-paragraphs: new paragraphs must mint a new content-addressed child id");
    assert_eq!(content.target, before_content.target, "the child SLOT is keyed by block id, so it must not move when the content changes");
    assert_eq!(*font_size, 16.0, "editing the text must not restyle the block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-text").expect("the text block exists")), (0.0, 0.0, 280.0, 120.0), "editing the text must not reflow the block's box");`,
  },
  {
    dir: "🧮️edit-block-math",
    slug: "edit-block-math",
    caseName: "replaces-the-tex-with-pythagoras",
    modName: "edit_block_math",
    mutation: { mutation: "editBlockMath", id: "blk-math", newTex: "a^2 + b^2 = c^2" },
    after: () => {
      const s = clone(base());
      at(s, "blk-math").tex = "a^2 + b^2 = c^2";
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-math", at(after, "blk-math"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` carries the new `tex` verbatim and the same `displayMode`.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`edit-block-math` emits ONE whole-block `patched` entry whose only changed field is `tex`.",
    inverseDoc: "The inverse re-issues `edit-block-math` with the base math block's own prior TeX source.",
    outcomeDoc: "The block exists AND is a math block, and the TeX genuinely differs, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.",
    extraUse: `${useSchema("block_bounds", "find_block")}${NODE_USE}`,
    extraDoc: "🧮 The authored TeX SOURCE is replaced verbatim — nothing is parsed or normalized, and `display_mode` is a separate concern this leaf never touches.",
    extraName: "tex_source_is_replaced_verbatim_and_display_mode_is_untouched",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-math applies");
    let NoteBlockNode::Math { tex: before_tex, .. } = find_block(&base.blocks, "blk-math").expect("the base math block exists") else {
        panic!("edit-block-math/replaces-the-tex-with-pythagoras: the base block must be a math block");
    };
    assert_eq!(before_tex, "E = mc^2", "edit-block-math/replaces-the-tex-with-pythagoras: the base must start from the old TeX");
    let NoteBlockNode::Math { tex, display_mode, .. } = find_block(&applied.blocks, "blk-math").expect("the math block survives") else {
        panic!("edit-block-math must not change the block's kind");
    };
    assert_eq!(tex, "a^2 + b^2 = c^2", "the TeX source must be stored verbatim, unparsed and unnormalized");
    assert!(*display_mode, "display_mode is a separate concern — editing the source must not flip it");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-math").expect("the math block exists")), (0.0, 400.0, 200.0, 80.0), "editing the TeX must not re-lay-out the block");`,
  },
  {
    dir: "🖊️change-block-ink-width",
    slug: "change-block-ink-width",
    caseName: "thickens-the-sketch-stroke",
    modName: "change_block_ink_width",
    mutation: { mutation: "changeBlockInkWidth", id: "blk-ink", newStrokeWidth: F(6) },
    after: () => {
      const s = clone(base());
      at(s, "blk-ink").strokeWidth = F(6);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-ink", at(after, "blk-ink"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` keeps the original `points` and `color`; `pencilWidth` stays `None`, so the block edit cannot leak into the tool setting.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`change-block-ink-width` emits ONE whole-block `patched` entry whose only changed field is `stroke_width`.",
    inverseDoc: "The inverse re-issues `change-block-ink-width` with the base ink block's own prior width.",
    outcomeDoc: "The block exists AND is an ink block, and 6.0 differs from 2.0, so neither the `mutation.target-missing` error (absent or non-ink) nor the `mutation.no-op` warn fires.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "🖊️ The DRAWN stroke's own width changes; its point list, colour and the document's pencil tool setting are all left alone.",
    extraName: "drawn_stroke_width_changes_without_touching_points_or_the_tool",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-ink-width applies");
    let NoteBlockNode::Ink { stroke_width: before_width, points: before_points, .. } = find_block(&base.blocks, "blk-ink").expect("the base ink block exists") else {
        panic!("change-block-ink-width/thickens-the-sketch-stroke: the base block must be an ink block");
    };
    assert_eq!(*before_width, 2.0, "change-block-ink-width/thickens-the-sketch-stroke: the base stroke must start at 2.0");
    let NoteBlockNode::Ink { stroke_width, points, color, .. } = find_block(&applied.blocks, "blk-ink").expect("the ink block survives") else {
        panic!("change-block-ink-width must not change the block's kind");
    };
    assert_eq!(*stroke_width, 6.0, "change-block-ink-width/thickens-the-sketch-stroke: the stroke must thicken to 6.0");
    assert_eq!(points, before_points, "thickening a stroke must not redraw its geometry");
    assert_eq!(*color, [0.0, 0.0, 0.0, 1.0], "thickening a stroke must not recolour it");
    assert_eq!(applied.pencil_width, Some(3.0), "the document's pencil TOOL width is a separate setting");`,
  },
  {
    dir: "🎨️edit-block-ink-stroke",
    slug: "edit-block-ink-stroke",
    caseName: "redraws-the-sketch-polyline",
    modName: "edit_block_ink_stroke",
    mutation: {
      mutation: "editBlockInkStroke",
      id: "blk-ink",
      newPoints: [[F(0), F(0)], [F(10), F(4)], [F(20), F(0)]],
      newX: F(25),
      newY: F(150),
      newWidth: F(20),
      newHeight: F(4),
    },
    after: () => {
      const s = clone(base());
      const ink = at(s, "blk-ink");
      ink.points = [[F(0), F(0)], [F(10), F(4)], [F(20), F(0)]];
      ink.x = F(25);
      ink.y = F(150);
      ink.width = F(20);
      ink.height = F(4);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-ink", at(after, "blk-ink"))] }) }),
    diffDoc: "ONE `blocks.patched` entry carrying the new `points` AND the new box together — the atomicity is structural: there is no second entry to tear against.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`edit-block-ink-stroke` emits ONE whole-block `patched` entry that rewrites `points` AND the whole bounding box in a single atomic step.",
    inverseDoc: "The inverse re-issues `edit-block-ink-stroke` carrying the base ink block's own prior points and box.",
    outcomeDoc: "The block exists AND is an ink block, and points/box genuinely differ, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn (which compares all five fields at once) fires.",
    extraUse: `${useSchema("block_bounds", "find_block")}${NODE_USE}`,
    extraDoc: "🎨 Geometry and bounding box move ATOMICALLY in one operation — a 2-point stroke becomes a 3-point polyline and the box retightens around it — while the stroke width is a different leaf's concern.",
    extraName: "points_and_bounding_box_are_rewritten_atomically",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-ink-stroke applies");
    let NoteBlockNode::Ink { points: before_points, .. } = find_block(&base.blocks, "blk-ink").expect("the base ink block exists") else {
        panic!("edit-block-ink-stroke/redraws-the-sketch-polyline: the base block must be an ink block");
    };
    assert_eq!(before_points.len(), 2, "edit-block-ink-stroke/redraws-the-sketch-polyline: the base stroke must start as a 2-point segment");
    let NoteBlockNode::Ink { points, stroke_width, .. } = find_block(&applied.blocks, "blk-ink").expect("the ink block survives") else {
        panic!("edit-block-ink-stroke must not change the block's kind");
    };
    assert_eq!(points, &vec![[0.0, 0.0], [10.0, 4.0], [20.0, 0.0]], "the whole point list must be replaced by the addressed polyline");
    assert_eq!(*stroke_width, 2.0, "the stroke WIDTH belongs to change-block-ink-width, not to this leaf");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-ink").expect("the ink block exists")), (25.0, 150.0, 20.0, 4.0), "the bounding box is rewritten in the SAME atomic step as the geometry");`,
  },
  {
    dir: "⬇️insert-table-row",
    slug: "insert-table-row",
    caseName: "appends-a-blank-third-row",
    modName: "insert_table_row",
    mutation: { mutation: "insertTableRow", id: "blk-table" },
    after: () => {
      const s = clone(base());
      (at(s, "blk-table").rows as unknown[]).push([{ content: "" }, { content: "" }]);
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-table", at(after, "blk-table"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` holds three rows and still two columns — the appended row's width is baked into the committed delta.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`insert-table-row` emits ONE whole-block `patched` entry appending a row whose width is read from the CURRENT column count.",
    inverseDoc: "The inverse is `remove-table-row`, which pops the row just appended.",
    outcomeDoc: "The block exists AND is a table, so the `mutation.target-missing` error guard fires for neither reason; this leaf has no no-op guard at all.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "⬇️ A blank row is APPENDED at the bottom, sized to the current column count, leaving existing cell content alone.",
    extraName: "blank_row_is_appended_sized_to_the_current_column_count",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("insert-table-row applies");
    let NoteBlockNode::Table { rows: before_rows, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("insert-table-row/appends-a-blank-third-row: the base block must be a table");
    };
    assert_eq!(before_rows.len(), 2, "insert-table-row/appends-a-blank-third-row: the base table must start with two rows");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("insert-table-row must not change the block's kind");
    };
    assert_eq!(rows.len(), 3, "insert-table-row/appends-a-blank-third-row: exactly one row must be appended");
    assert_eq!(columns.len(), 2, "adding a row must never add a column");
    assert_eq!(rows[2].len(), columns.len(), "the appended row is sized from the CURRENT column count");
    assert!(rows[2].iter().all(|cell| cell.content.is_empty()), "the appended row must be blank");
    assert_eq!(rows[0][0].content, "Alpha", "appending must not disturb existing cell content");`,
  },
  {
    dir: "⬆️remove-table-row",
    slug: "remove-table-row",
    caseName: "drops-the-trailing-blank-row",
    modName: "remove_table_row",
    mutation: { mutation: "removeTableRow", id: "blk-table" },
    after: () => {
      const s = clone(base());
      (at(s, "blk-table").rows as unknown[]).pop();
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-table", at(after, "blk-table"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` holds the single surviving row — the popped row is the LAST one, which the committed delta makes checkable.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`remove-table-row` emits ONE whole-block `patched` entry that pops the LAST row — the row index is never a payload field.",
    inverseDoc: "The inverse is `insert-table-row`, which re-appends a blank row of the current width; the fixture's trailing row is blank precisely so that round-trips.",
    outcomeDoc: "The block exists, is a table, and holds more than one row, so neither the `mutation.target-missing` error nor the one-row-floor `mutation.no-op` warn fires.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "⬆️ The LAST row is popped and the column count is untouched; the surviving row keeps its content, and the table stays above this leaf's 1-row floor.",
    extraName: "last_row_is_popped_and_the_one_row_floor_still_holds",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("remove-table-row applies");
    let NoteBlockNode::Table { rows: before_rows, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("remove-table-row/drops-the-trailing-blank-row: the base block must be a table");
    };
    assert_eq!(before_rows.len(), 2, "remove-table-row/drops-the-trailing-blank-row: the base table must start above the 1-row floor");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("remove-table-row must not change the block's kind");
    };
    assert_eq!(rows.len(), 1, "remove-table-row/drops-the-trailing-blank-row: exactly one row must be popped");
    assert!(!rows.is_empty(), "this leaf refuses to go below its own 1-row floor");
    assert_eq!(columns.len(), 2, "removing a row must never remove a column");
    assert_eq!(rows[0][0].content, "Alpha", "the LAST row is the one popped — the first row's content survives");`,
  },
  {
    dir: "➡️insert-table-column",
    slug: "insert-table-column",
    caseName: "appends-the-lettered-column-c",
    modName: "insert_table_column",
    mutation: { mutation: "insertTableColumn", id: "blk-table" },
    after: () => {
      const s = clone(base());
      const table = at(s, "blk-table");
      (table.columns as string[]).push("C");
      for (const row of table.rows as unknown[][]) row.push({ content: "" });
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-table", at(after, "blk-table"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` holds headers `A`/`B`/`C` and a blank cell added to every row — the rectangularity invariant is committed, not inferred.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`insert-table-column` emits ONE whole-block `patched` entry appending a header lettered `A + (column_count % 26)` AND a blank cell to every row.",
    inverseDoc: "The inverse is `remove-table-column`, which pops the column just appended.",
    outcomeDoc: "The block exists AND is a table, so the `mutation.target-missing` error guard fires for neither reason; this leaf has no no-op guard at all.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "➡️ The new header is LETTERED from the current column count (two columns yield `C`), and every existing row grows one blank cell so the table stays rectangular.",
    extraName: "lettered_header_is_appended_and_every_row_stays_rectangular",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("insert-table-column applies");
    let NoteBlockNode::Table { columns: before_columns, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("insert-table-column/appends-the-lettered-column-c: the base block must be a table");
    };
    assert_eq!(before_columns, &vec!["A".to_string(), "B".to_string()], "insert-table-column/appends-the-lettered-column-c: the base table must start with headers A and B");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("insert-table-column must not change the block's kind");
    };
    assert_eq!(columns, &vec!["A".to_string(), "B".to_string(), "C".to_string()], "the header is lettered from the CURRENT column count, so two columns yield \\"C\\"");
    assert_eq!(rows.len(), 2, "adding a column must never add a row");
    assert!(rows.iter().all(|row| row.len() == columns.len()), "every row must grow a cell so the table stays rectangular");
    assert!(rows.iter().all(|row| row[2].content.is_empty()), "the appended cells must be blank");`,
  },
  {
    dir: "⬅️remove-table-column",
    slug: "remove-table-column",
    caseName: "drops-the-trailing-column-b",
    modName: "remove_table_column",
    mutation: { mutation: "removeTableColumn", id: "blk-table" },
    after: () => {
      const s = clone(base());
      const table = at(s, "blk-table");
      (table.columns as string[]).pop();
      for (const row of table.rows as unknown[][]) row.pop();
      return s;
    },
    diff: (after) => ({ ...emptyDiff(), blocks: blocksDelta({ patched: [patchEntry("blk-table", at(after, "blk-table"))] }) }),
    diffDoc: "One `blocks.patched` entry whose `blockJson` holds one header and one cell per row — header and cells go together in the same committed value.",
    applyDiffDoc: "The committed single-`patched` delta carries `before` to `after` on its own.",
    applyDoc: "`remove-table-column` emits ONE whole-block `patched` entry that pops the LAST header AND the last cell of every row — the column index is never a payload field.",
    inverseDoc: "The inverse is `insert-table-column`, which re-appends header `B` and a blank cell per row; the fixture's trailing column is blank precisely so that round-trips.",
    outcomeDoc: "The block exists, is a table, and holds more than one column, so neither the `mutation.target-missing` error nor the one-column-floor `mutation.no-op` warn fires.",
    extraUse: `${useSchema("find_block")}${NODE_USE}`,
    extraDoc: "⬅️ The LAST header goes together with one cell per row, so the table stays rectangular and above this leaf's 1-column floor; the row count is untouched.",
    extraName: "last_column_and_its_cells_go_together_keeping_the_table_rectangular",
    extraBody: `    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("remove-table-column applies");
    let NoteBlockNode::Table { columns: before_columns, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("remove-table-column/drops-the-trailing-column-b: the base block must be a table");
    };
    assert_eq!(before_columns.len(), 2, "remove-table-column/drops-the-trailing-column-b: the base table must start above the 1-column floor");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("remove-table-column must not change the block's kind");
    };
    assert_eq!(columns, &vec!["A".to_string()], "the LAST header must be the one popped");
    assert_eq!(rows.len(), 2, "removing a column must never remove a row");
    assert!(rows.iter().all(|row| row.len() == 1), "one cell per row goes with the header, keeping the table rectangular");
    assert_eq!(rows[0][0].content, "Alpha", "the surviving column keeps its content");`,
  },
];

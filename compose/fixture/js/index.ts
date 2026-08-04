// #region 🧲️Header
/** @emoji 🧪️ `@semio-tech/compose-fixture` — test and story JSON only; not for application or play bundles. */
// #endregion 🧲️Header

export { default as DragDesign } from "../drag/design.compose.json";
export { default as DragDiffDesignFree } from "../drag/diff.design.free.compose.json";
export { default as DragDiffDesign } from "../drag/diff.design.compose.json";
export { default as DragOffset } from "../drag/offset.compose.json";
export { default as DragPieces } from "../drag/pieces.compose.json";
export { default as MoveVector } from "../move/vector.compose.json";
export { default as MoveDiffDesign } from "../move/diff.design.compose.json";
export { default as MoveStoryDesign } from "../move/story.design.compose.json";
export { default as InvalidKit } from "../invalid.kit.compose.json";
export { default as MetabolismKitDiffInverted } from "../metabolism.kit.diff.inverted.compose.json";
export { default as MetabolismKitDiff } from "../metabolism.kit.diff.compose.json";
export { default as MetabolismKitDiffed } from "../metabolism.kit.diffed.compose.json";
export { default as MetabolismMetaKit } from "../metabolism.meta.kit.compose.json";
export { default as MetabolismShallowKit } from "../metabolism.shallow.kit.compose.json";
export { default as RepresentationSelectionCases } from "../representation.selection.compose.json";
export { default as NakaginCapsuleTowerCopySelection } from "../nakagin-capsule-tower.copy.design.selection.compose.json";
export { default as NakaginCapsuleTowerCopyDesign } from "../nakagin-capsule-tower.copy.design.compose.json";
export { default as NakaginCapsuleTowerDeletedDesignDiff } from "../nakagin-capsule-tower.deleted.design.diff.compose.json";
export { default as NakaginCapsuleTowerDeletedSelection } from "../nakagin-capsule-tower.deleted.selection.compose.json";
export { default as MetabolismKitFilteredNakaginCapsuleTower, default as NakaginCapsuleTowerFilteredKit } from "../nakagin-capsule-tower.filtered.kit.compose.json";
export { default as NakaginCapsuleTowerMetaDesign } from "../nakagin-capsule-tower.meta.design.compose.json";
export { default as NakaginCapsuleTowerPasteDesignDiff } from "../nakagin-capsule-tower.paste.design.diff.compose.json";
export { default as NakaginCapsuleTowerPasteDesign } from "../nakagin-capsule-tower.paste.design.compose.json";
export { default as NakaginCapsuleTowerPasteWithCoordinateDesignDiff } from "../nakagin-capsule-tower.paste.with-coordinate.design.diff.compose.json";
export { default as NakaginCapsuleTowerShallowDesign } from "../nakagin-capsule-tower.shallow.design.compose.json";
export { default as NakaginCapsuleTowerWithDiffDesign } from "../nakagin-capsule-tower.with-diff.design.compose.json";
export { default as NakaginCapsuleTowerDiffDesign } from "../nakagin-capsule-tower.diff.design.compose.json";
export { default as TambourMetaType } from "../tambour.meta.type.compose.json";
export { default as TambourShallowType } from "../tambour.shallow.type.compose.json";
export { default as InvalidKitValidation } from "../validation.compose.json";
export { default as ValidateKitDiffCases } from "../validate-kit-diff.cases.compose.json";
export { default as FlattenMerkleCases } from "../flatten-merkle.cases.compose.json";
export { default as HashCases } from "../hash.cases.compose.json";
export { default as QualitySumCases } from "../quality-sum.cases.compose.json";
export { default as DesignWithDiffCases } from "../design-with-diff.cases.compose.json";
export { default as FilterKitCases } from "../filter-kit.cases.compose.json";
export { default as FindReplaceableTypesCases } from "../find-replaceable-types.cases.compose.json";
export { default as FlattenCases } from "../flatten.cases.compose.json";
export { default as SyntheticFindReplaceableKit } from "../synthetic-find-replaceable.kit.compose.json";
export { default as ExportDesignRepresentationCases } from "../export-design-representation.cases.compose.json";
export { default as DeleteCases } from "../delete.cases.compose.json";
export { default as CopyPasteCases } from "../copy-paste.cases.compose.json";

//#region 🌱️MetabolismKitDerived
// 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: relocated from `@semio-tech/assets` — these derived
// exports (and the 7.3MB JSON fixture behind them) had zero runtime consumers outside
// `.storybook/stories/compose/**`, but `@semio-tech/assets` is statically imported by `ui-react`'s
// barrel, so every document paid the parse + flatten cost for nothing. `compose-fixture` is
// storybook/test-only by convention (see this file's header), so this is where the fixture-derived
// data belongs; `MetabolismShallowKit` above is the same JSON these are computed from.
import metabolismShallowKitForDerivation from "../metabolism.shallow.kit.compose.json";

const MetabolismKitDataInner = { wip: { initialKit: metabolismShallowKitForDerivation } };
export { MetabolismKitDataInner as MetabolismKit };

/** @emoji 📎️ Reads a materialized `{ hash, items }` collection without importing Node-only repository tooling into browser bundles. */
function __fixtureItemsOf<T = Record<string, unknown>>(node: unknown): readonly T[] {
  return node && typeof node === "object" && Array.isArray((node as { items?: unknown }).items) ? (node as { items: T[] }).items : [];
}

/** @emoji 🧾️ Resolves the materialized kit payload from `wip.initialKit`. */
function __metabolismKitInner(): Record<string, unknown> {
  const root = MetabolismKitDataInner as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  return (inner && typeof inner === "object" ? inner : {}) ?? {};
}

/** @emoji 🏛️ Flattens kinds from root `types` or nested `typologies[].types`. */
function __kitTypesFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootTypes = __fixtureItemsOf(inner["types"]);
  if (rootTypes.length > 0) return rootTypes;
  return __fixtureItemsOf(inner["typologies"]).flatMap((topo) => __fixtureItemsOf((topo as { types?: unknown }).types));
}

/** @emoji 🏛️ Flattens designs from root `designs` or nested `typologies[].designs`. */
function __kitDesignsFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootDesigns = __fixtureItemsOf(inner["designs"]);
  if (rootDesigns.length > 0) return rootDesigns;
  return __fixtureItemsOf(inner["typologies"]).flatMap((topo) => __fixtureItemsOf((topo as { designs?: unknown }).designs));
}

/** Metabolism kit types array */
export const MetabolismKitTypes = __kitTypesFromInner(__metabolismKitInner());
/** Metabolism kit designs array */
export const MetabolismKitDesigns = __kitDesignsFromInner(__metabolismKitInner());
/** Metabolism kit typologies array */
export const MetabolismKitTypologies = __fixtureItemsOf(__metabolismKitInner()["typologies"]);
/** Metabolism kit families array */
export const MetabolismKitFamilies = __fixtureItemsOf(__metabolismKitInner()["families"]);
/** Metabolism kit qualities array */
export const MetabolismKitQualities = __fixtureItemsOf(__metabolismKitInner()["qualities"]);
/** Metabolism kit files array */
export const MetabolismKitFiles = __fixtureItemsOf(__metabolismKitInner()["files"]);
/** Metabolism kit folders array */
export const MetabolismKitFolders = __fixtureItemsOf(__metabolismKitInner()["folders"]);
/** Metabolism kit authors array */
export const MetabolismKitAuthors = __fixtureItemsOf(__metabolismKitInner()["authors"]);
/** Metabolism kit tags array */
export const MetabolismKitTags = __fixtureItemsOf(__metabolismKitInner()["tags"]);
/** Metabolism kit concepts array */
export const MetabolismKitConcepts = __fixtureItemsOf(__metabolismKitInner()["concepts"]);
/** Metabolism kit attributes array */
export const MetabolismKitAttributes = __fixtureItemsOf(__metabolismKitInner()["attributes"]);
/** Metabolism kit Nakagin Capsule Tower designs subset */
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => String((design as { name?: string }).name ?? "") === "Nakagin Capsule Tower") ?? [];

/** Builds id and name lookup maps from an item array. */
const buildLookup = (items: readonly any[] = []) => {
  const byId: Record<string, any> = {};
  const byName: Record<string, any> = {};
  items.forEach((item) => {
    if (!item) return;
    if (item.id) byId[item.id] = item;
    if (item.name) byName[item.name] = item;
  });
  return { byId, byName };
};

const typeLookup = buildLookup(MetabolismKitTypes);
const designLookup = buildLookup(MetabolismKitDesigns);
const typologyLookup = buildLookup(MetabolismKitTypologies);
const familyLookup = buildLookup(MetabolismKitFamilies);

/** Metabolism kit types indexed by id */
export const MetabolismKitTypesById = typeLookup.byId;
/** Metabolism kit types indexed by name */
export const MetabolismKitTypesByName = typeLookup.byName;
/** Metabolism kit designs indexed by id */
export const MetabolismKitDesignsById = designLookup.byId;
/** Metabolism kit designs indexed by name */
export const MetabolismKitDesignsByName = designLookup.byName;
/** Metabolism kit typologies indexed by id */
export const MetabolismKitTypologiesById = typologyLookup.byId;
/** Metabolism kit typologies indexed by name */
export const MetabolismKitTypologiesByName = typologyLookup.byName;
/** Metabolism kit families indexed by id */
export const MetabolismKitFamiliesById = familyLookup.byId;
/** Metabolism kit families indexed by name */
export const MetabolismKitFamiliesByName = familyLookup.byName;

const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => String((d as { name?: string }).name ?? "") === "Nakagin Capsule Tower");
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find(
  (d) => String((d as { name?: string }).name ?? "") === "Flat" && String((d as { parent?: { id?: string } }).parent?.id ?? "") === String((nakaginCapsuleTowerDesign as { id?: string } | undefined)?.id ?? ""),
);
/** Nakagin Capsule Tower Flat variant piece data with plane and center */
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  ((nakaginCapsuleTowerFlatDesign as { pieces?: { name?: string; plane?: unknown; center?: unknown }[] } | undefined)?.pieces ?? []).map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🌱️MetabolismKitDerived

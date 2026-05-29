// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲Header

//#region 🗃️Exports
// Re-exports and data constants MUST come from the Metabolism kit assets.

import MetabolismKitData from "../fixtures/stores/metabolism/wip/initialKit/kit.semio.json";
export * from "./icons";

//#region 🔖KitBootstrapHelpers
/** @emoji 🧾 Normalizes list-or-{items} shapes found on `wip.initialKit` DTOs. */
function __itemsOf<T>(node: unknown): readonly T[] {
  if (Array.isArray(node)) return node as readonly T[];
  if (node && typeof node === "object" && "items" in node && Array.isArray((node as { items: unknown }).items)) return (node as { items: T[] }).items;
  return [];
}

/** @emoji 🧾 Resolves the materialized kit payload (legacy root vs `wip.initialKit`). */
function __metabolismKitInner(): Record<string, unknown> {
  const root = MetabolismKitData as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  return (inner && typeof inner === "object" ? inner : (MetabolismKitData as unknown as Record<string, unknown>)) ?? {};
}
//#endregion 🔖KitBootstrapHelpers
export { default as DragDesign } from "../fixtures/drag/design.semio.json";
export { default as DragDiffDesignFree } from "../fixtures/drag/diff.design.free.semio.json";
export { default as DragDiffDesign } from "../fixtures/drag/diff.design.semio.json";
export { default as DragOffset } from "../fixtures/drag/offset.semio.json";
export { default as DragPieces } from "../fixtures/drag/pieces.semio.json";
export { default as MoveVector } from "../fixtures/move/vector.semio.json";
export { default as MoveDiffDesign } from "../fixtures/move/diff.design.semio.json";
export { default as MoveStoryDesign } from "../fixtures/move/story.design.semio.json";
export { default as InvalidKit } from "../fixtures/invalid.kit.semio.json";
export { default as MetabolismKitDiffInverted } from "../fixtures/metabolism.kit.diff.inverted.semio.json";
export { default as MetabolismKitDiff } from "../fixtures/metabolism.kit.diff.semio.json";
export { default as MetabolismKitDiffed } from "../fixtures/metabolism.kit.diffed.semio.json";
export { default as MetabolismMetaKit } from "../fixtures/metabolism.meta.kit.semio.json";
export { default as MetabolismShallowKit } from "../fixtures/metabolism.shallow.kit.semio.json";
export { default as RepresentationSelectionCases } from "../fixtures/representation.selection.semio.json";
export { default as NakaginCapsuleTowerCopySelection } from "../fixtures/nakagin-capsule-tower.copy.design.selection.semio.json";
export { default as NakaginCapsuleTowerCopyDesign } from "../fixtures/nakagin-capsule-tower.copy.design.semio.json";
export { default as NakaginCapsuleTowerDeletedDesignDiff } from "../fixtures/nakagin-capsule-tower.deleted.design.diff.semio.json";
export { default as NakaginCapsuleTowerDeletedSelection } from "../fixtures/nakagin-capsule-tower.deleted.selection.semio.json";
export { default as MetabolismKitFilteredNakaginCapsuleTower, default as NakaginCapsuleTowerFilteredKit } from "../fixtures/nakagin-capsule-tower.filtered.kit.semio.json";
export { default as NakaginCapsuleTowerMetaDesign } from "../fixtures/nakagin-capsule-tower.meta.design.semio.json";
export { default as NakaginCapsuleTowerPasteDesignDiff } from "../fixtures/nakagin-capsule-tower.paste.design.diff.semio.json";
export { default as NakaginCapsuleTowerPasteDesign } from "../fixtures/nakagin-capsule-tower.paste.design.semio.json";
export { default as NakaginCapsuleTowerPasteWithCoordinateDesignDiff } from "../fixtures/nakagin-capsule-tower.paste.with-coordinate.design.diff.semio.json";
export { default as NakaginCapsuleTowerShallowDesign } from "../fixtures/nakagin-capsule-tower.shallow.design.semio.json";
export { default as NakaginCapsuleTowerWithDiffDesign } from "../fixtures/nakagin-capsule-tower.with-diff.design.semio.json";
export { default as NakaginCapsuleTowerDiffDesign, default as NakginCapsuleTowerDiffDesign } from "../fixtures/nakgin-capsule-tower.diff.design.semio.json";
export { default as TambourMetaType } from "../fixtures/tambour.meta.type.semio.json";
export { default as TambourShallowType } from "../fixtures/tambour.shallow.type.semio.json";
export { default as InvalidKitValidation } from "../fixtures/validation.semio.json";
export { default as ValidateKitDiffCases } from "../fixtures/validate-kit-diff.cases.semio.json";
export { default as FlattenMerkleCases } from "../fixtures/flatten-merkle.cases.semio.json";
export { default as HashCases } from "../fixtures/hash.cases.semio.json";
export { default as QualitySumCases } from "../fixtures/quality-sum.cases.semio.json";
export { default as DesignWithDiffCases } from "../fixtures/design-with-diff.cases.semio.json";
export { default as FilterKitCases } from "../fixtures/filter-kit.cases.semio.json";
export { default as FindReplaceableTypesCases } from "../fixtures/find-replaceable-types.cases.semio.json";
export { default as FlattenCases } from "../fixtures/flatten.cases.semio.json";
export { default as SyntheticFindReplaceableKit } from "../fixtures/synthetic-find-replaceable.kit.semio.json";
export { default as ExportDesignRepresentationCases } from "../fixtures/export-design-representation.cases.semio.json";
export { default as DeleteCases } from "../fixtures/delete.cases.semio.json";
export { default as CopyPasteCases } from "../fixtures/copy-paste.cases.semio.json";
export { MetabolismKitData as MetabolismKit };

/**
 * Metabolism kit types array
 **/
export const MetabolismKitTypes = __itemsOf(__metabolismKitInner()["types"]);
/**
 * Metabolism kit designs array
 **/
export const MetabolismKitDesigns = __itemsOf(__metabolismKitInner()["designs"]);
/**
 * Metabolism kit families array
 **/
export const MetabolismKitFamilies = __itemsOf(__metabolismKitInner()["families"]);
/**
 * Metabolism kit qualities array
 **/
export const MetabolismKitQualities = __itemsOf(__metabolismKitInner()["qualities"]);
/**
 * Metabolism kit files array
 **/
export const MetabolismKitFiles = __itemsOf(__metabolismKitInner()["files"]);
/**
 * Metabolism kit folders array
 **/
export const MetabolismKitFolders = __itemsOf(__metabolismKitInner()["folders"]);
/**
 * Metabolism kit authors array
 **/
export const MetabolismKitAuthors = __itemsOf(__metabolismKitInner()["authors"]);
/**
 * Metabolism kit tags array
 **/
export const MetabolismKitTags = __itemsOf(__metabolismKitInner()["tags"]);
/**
 * Metabolism kit concepts array
 **/
export const MetabolismKitConcepts = __itemsOf(__metabolismKitInner()["concepts"]);
/**
 * Metabolism kit attributes array
 **/
export const MetabolismKitAttributes = __itemsOf(__metabolismKitInner()["attributes"]);
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 **/
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => String((design as { name?: string }).name ?? "") === "Nakagin Capsule Tower") ?? [];

/**
 * Builds id and name lookup maps from an item array
 *
 * Callers MUST provide an array of objects with optional id and name fields
 * buildLookup holds the data fields for a buildLookup record.
 **/
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

/**
 * typeLookup holds the data fields for a typeLookup record.
 **/
const typeLookup = buildLookup(MetabolismKitTypes);
/**
 * Design lookup maps by id and name
 **/
const designLookup = buildLookup(MetabolismKitDesigns);
/**
 * Family lookup maps by id and name
 **/
const familyLookup = buildLookup(MetabolismKitFamilies);

/**
 * Metabolism kit types indexed by id
 **/
export const MetabolismKitTypesById = typeLookup.byId;
/**
 * Metabolism kit types indexed by name
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by id
 **/
export const MetabolismKitDesignsById = designLookup.byId;
/**
 * Metabolism kit designs indexed by name
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit families indexed by id
 **/
export const MetabolismKitFamiliesById = familyLookup.byId;
/**
 * Metabolism kit families indexed by name
 **/
export const MetabolismKitFamiliesByName = familyLookup.byName;
/**
 * nakaginCapsuleTowerDesign holds the data fields for a nakaginCapsuleTowerDesign record.
 **/
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => String((d as { name?: string }).name ?? "") === "Nakagin Capsule Tower");
/**
 * nakaginCapsuleTowerFlatDesign holds the data fields for a nakaginCapsuleTowerFlatDesign record.
 **/
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find(
  (d) => String((d as { name?: string }).name ?? "") === "Flat" && String((d as { parent?: { id?: string } }).parent?.id ?? "") === String((nakaginCapsuleTowerDesign as { id?: string } | undefined)?.id ?? ""),
);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  ((nakaginCapsuleTowerFlatDesign as { pieces?: { name?: string; plane?: unknown; center?: unknown }[] } | undefined)?.pieces ?? []).map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🗃️Exports

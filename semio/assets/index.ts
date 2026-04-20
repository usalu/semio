// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲Header

//#region 🗃️Exports
// Re-exports and data constants MUST come from the Metabolism kit assets.

import MetabolismKitData from "./semio/metabolism.kit.semio.json";
export * from "./icons";
export { default as DragDesign } from "./semio/drag/design.semio.json";
export { default as DragDiffDesignFree } from "./semio/drag/diff.design.free.semio.json";
export { default as DragDiffDesign } from "./semio/drag/diff.design.semio.json";
export { default as DragOffset } from "./semio/drag/offset.semio.json";
export { default as DragPieces } from "./semio/drag/pieces.semio.json";
export { default as MoveVector } from "./semio/move/vector.semio.json";
export { default as MoveDiffDesign } from "./semio/move/diff.design.semio.json";
export { default as MoveStoryDesign } from "./semio/move/story.design.semio.json";
export { default as InvalidKit } from "./semio/invalid.kit.semio.json";
export { default as MetabolismKitDiffInverted } from "./semio/metabolism.kit.diff.inverted.semio.json";
export { default as MetabolismKitDiff } from "./semio/metabolism.kit.diff.semio.json";
export { default as MetabolismKitDiffed } from "./semio/metabolism.kit.diffed.semio.json";
export { default as MetabolismMetaKit } from "./semio/metabolism.meta.kit.semio.json";
export { default as MetabolismShallowKit } from "./semio/metabolism.shallow.kit.semio.json";
export { default as RepresentationSelectionCases } from "./semio/representation.selection.semio.json";
export { default as NakaginCapsuleTowerCopySelection } from "./semio/nakagin-capsule-tower.copy.design.selection.semio.json";
export { default as NakaginCapsuleTowerCopyDesign } from "./semio/nakagin-capsule-tower.copy.design.semio.json";
export { default as NakaginCapsuleTowerDeletedDesignDiff } from "./semio/nakagin-capsule-tower.deleted.design.diff.semio.json";
export { default as NakaginCapsuleTowerDeletedSelection } from "./semio/nakagin-capsule-tower.deleted.selection.semio.json";
export { default as MetabolismKitFilteredNakaginCapsuleTower, default as NakaginCapsuleTowerFilteredKit } from "./semio/nakagin-capsule-tower.filtered.kit.semio.json";
export { default as NakaginCapsuleTowerMetaDesign } from "./semio/nakagin-capsule-tower.meta.design.semio.json";
export { default as NakaginCapsuleTowerPasteDesignDiff } from "./semio/nakagin-capsule-tower.paste.design.diff.semio.json";
export { default as NakaginCapsuleTowerPasteDesign } from "./semio/nakagin-capsule-tower.paste.design.semio.json";
export { default as NakaginCapsuleTowerPasteWithCoordDesignDiff } from "./semio/nakagin-capsule-tower.paste.with-coord.design.diff.semio.json";
export { default as NakaginCapsuleTowerShallowDesign } from "./semio/nakagin-capsule-tower.shallow.design.semio.json";
export { default as NakaginCapsuleTowerWithDiffDesign } from "./semio/nakagin-capsule-tower.with-diff.design.semio.json";
export { default as NakaginCapsuleTowerDiffDesign, default as NakginCapsuleTowerDiffDesign } from "./semio/nakgin-capsule-tower.diff.design.semio.json";
export { default as TambourMetaType } from "./semio/tambour.meta.type.semio.json";
export { default as TambourShallowType } from "./semio/tambour.shallow.type.semio.json";
export { default as InvalidKitValidation } from "./semio/validation.semio.json";
export { default as ValidateKitDiffCases } from "./semio/validate-kit-diff.cases.semio.json";
export { default as FlattenMerkleCases } from "./semio/flatten-merkle.cases.semio.json";
export { default as HashCases } from "./semio/hash.cases.semio.json";
export { default as QualitySumCases } from "./semio/quality-sum.cases.semio.json";
export { default as DesignWithDiffCases } from "./semio/design-with-diff.cases.semio.json";
export { default as FilterKitCases } from "./semio/filter-kit.cases.semio.json";
export { default as FindReplaceableTypesCases } from "./semio/find-replaceable-types.cases.semio.json";
export { default as FlattenCases } from "./semio/flatten.cases.semio.json";
export { default as SyntheticFindReplaceableKit } from "./semio/synthetic-find-replaceable.kit.semio.json";
export { default as ExportDesignRepresentationCases } from "./semio/export-design-representation.cases.semio.json";
export { default as DeleteCases } from "./semio/delete.cases.semio.json";
export { default as CopyPasteCases } from "./semio/copy-paste.cases.semio.json";
export { MetabolismKitData as MetabolismKit };

/**
 * Metabolism kit types array
 **/
export const MetabolismKitTypes = MetabolismKitData.types ?? [];
/**
 * Metabolism kit designs array
 **/
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
/**
 * Metabolism kit families array
 **/
export const MetabolismKitFamilies = (MetabolismKitData as { families?: unknown[] }).families ?? [];
/**
 * Metabolism kit qualities array
 **/
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
/**
 * Metabolism kit files array
 **/
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
/**
 * Metabolism kit folders array
 **/
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
/**
 * Metabolism kit authors array
 **/
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
/**
 * Metabolism kit tags array
 **/
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
/**
 * Metabolism kit concepts array
 **/
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
/**
 * Metabolism kit attributes array
 **/
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 **/
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => design.name === "Nakagin Capsule Tower") ?? [];

/**
 * Builds guid and name lookup maps from an item array
 *
 * Callers MUST provide an array of objects with optional guid and name fields
 * buildLookup holds the data fields for a buildLookup record.
 **/
const buildLookup = (items: any[] = []) => {
  const byGuid: Record<string, any> = {};
  const byName: Record<string, any> = {};
  items.forEach((item) => {
    if (!item) return;
    if (item.guid) byGuid[item.guid] = item;
    if (item.name) byName[item.name] = item;
  });
  return { byGuid, byName };
};

/**
 * typeLookup holds the data fields for a typeLookup record.
 **/
const typeLookup = buildLookup(MetabolismKitTypes);
/**
 * Design lookup maps by guid and name
 **/
const designLookup = buildLookup(MetabolismKitDesigns);
/**
 * Family lookup maps by guid and name
 **/
const familyLookup = buildLookup(MetabolismKitFamilies);

/**
 * Metabolism kit types indexed by guid
 **/
export const MetabolismKitTypesByGuid = typeLookup.byGuid;
/**
 * Metabolism kit types indexed by name
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by guid
 **/
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
/**
 * Metabolism kit designs indexed by name
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit families indexed by guid
 **/
export const MetabolismKitFamiliesByGuid = familyLookup.byGuid;
/**
 * Metabolism kit families indexed by name
 **/
export const MetabolismKitFamiliesByName = familyLookup.byName;
/**
 * nakaginCapsuleTowerDesign holds the data fields for a nakaginCapsuleTowerDesign record.
 **/
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
/**
 * nakaginCapsuleTowerFlatDesign holds the data fields for a nakaginCapsuleTowerFlatDesign record.
 **/
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🗃️Exports

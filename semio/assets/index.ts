// #region 🔖Header
// [👤semio🏪assets💻index](semiorepo://p/u/semio/b/a/assets/f/index.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, models and images.

// #endregion 🔖Header

//#region 🔖Exports
// [👤semio🏪assets💻index🔖exports](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports)
// Re-exports and data constants MUST come from the Metabolism kit assets.

import MetabolismKitData from "./semio/kit_metabolism.json";
export * from "./icons";
export { default as MetabolismKitDiff } from "./semio/diff_kit_metabolism.json";
export { default as MetabolismKitDiffInverted } from "./semio/diff_kit_metabolism_inverted.json";
export { default as InvalidKit } from "./semio/kit_invalid.json";
export { default as MetabolismKitDiffed } from "./semio/kit_metabolism_diffed.json";
export { default as InvalidKitValidation } from "./semio/validation.json";
export { MetabolismKitData as MetabolismKit };
export { default as DragDesign } from "./semio/drag/design.json";
export { default as DragPieces } from "./semio/drag/pieces.json";
export { default as DragOffset } from "./semio/drag/offset.json";
export { default as DragDiffDesign } from "./semio/drag/diff_design.json";
export { default as DragDiffDesignFree } from "./semio/drag/diff_design_free.json";
export { default as ModelSelectionCases } from "./semio/model_selection.json";

/**
 * Metabolism kit types array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkittypes](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitTypes)
 **/
export const MetabolismKitTypes = MetabolismKitData.types ?? [];
/**
 * Metabolism kit designs array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitdesigns](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitDesigns)
 **/
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
/**
 * Metabolism kit ports array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitports](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitPorts)
 **/
export const MetabolismKitPorts = MetabolismKitData.ports ?? [];
/**
 * Metabolism kit qualities array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitqualities](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitQualities)
 **/
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
/**
 * Metabolism kit files array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitfiles](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitFiles)
 **/
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
/**
 * Metabolism kit folders array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitfolders](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitFolders)
 **/
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
/**
 * Metabolism kit authors array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitauthors](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitAuthors)
 **/
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
/**
 * Metabolism kit tags array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkittags](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitTags)
 **/
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
/**
 * Metabolism kit concepts array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitconcepts](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitConcepts)
 **/
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
/**
 * Metabolism kit attributes array
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitattributes](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitAttributes)
 **/
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitnakagincapsuletowerdesigns](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitNakaginCapsuleTowerDesigns)
 **/
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => design.name === "Nakagin Capsule Tower") ?? [];

/**
 * Builds guid and name lookup maps from an item array
 *
 * Callers MUST provide an array of objects with optional guid and name fields
 * [👤semio🏪assets💻index🔖exports🪨buildlookup](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/buildLookup)
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
 * [👤semio🏪assets💻index🔖exports🪨typelookup](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/typeLookup)
 **/
const typeLookup = buildLookup(MetabolismKitTypes);
/**
 * [👤semio🏪assets💻index🔖exports🪨designlookup](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/designLookup)
 * Design lookup maps by guid and name
 **/
const designLookup = buildLookup(MetabolismKitDesigns);
/**
 * [👤semio🏪assets💻index🔖exports🪨portlookup](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/portLookup)
 * Port lookup maps by guid and name
 **/
const portLookup = buildLookup(MetabolismKitPorts);

/**
 * Metabolism kit types indexed by guid
 * [👤semio🏪assets💻index🔖exports🪨metabolismkittypesbyguid](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitTypesByGuid)
 **/
export const MetabolismKitTypesByGuid = typeLookup.byGuid;
/**
 * Metabolism kit types indexed by name
 * [👤semio🏪assets💻index🔖exports🪨metabolismkittypesbyname](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitTypesByName)
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by guid
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitdesignsbyguid](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitDesignsByGuid)
 **/
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
/**
 * Metabolism kit designs indexed by name
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitdesignsbyname](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitDesignsByName)
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit ports indexed by guid
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitportsbyguid](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitPortsByGuid)
 **/
export const MetabolismKitPortsByGuid = portLookup.byGuid;
/**
 * Metabolism kit ports indexed by name
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitportsbyname](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitPortsByName)
 **/
export const MetabolismKitPortsByName = portLookup.byName;
/**
 * nakaginCapsuleTowerDesign holds the data fields for a nakaginCapsuleTowerDesign record.
 * [👤semio🏪assets💻index🔖exports🪨nakagincapsuletowerdesign](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/nakaginCapsuleTowerDesign)
 **/
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
/**
 * nakaginCapsuleTowerFlatDesign holds the data fields for a nakaginCapsuleTowerFlatDesign record.
 * [👤semio🏪assets💻index🔖exports🪨nakagincapsuletowerflatdesign](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/nakaginCapsuleTowerFlatDesign)
 **/
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 * [👤semio🏪assets💻index🔖exports🪨metabolismkitnakagincapsuletowerflatpieces](semiorepo://p/u/semio/b/a/assets/f/index.ts/s/Exports/d/i/MetabolismKitNakaginCapsuleTowerFlatPieces)
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🔖Exports

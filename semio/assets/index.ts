// #region 🔖Header

// [👤semio🏪assets💻indexts](semiorepo://file/SEMIO/ASSETS/INDEX.TS)

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

// [🔖semio/assets/index.ts#Exports](semiorepo://section/semio/assets/index.ts/EXPORTS)
// Re-exports and data constants MUST come from the Metabolism kit assets.

import MetabolismKitData from "./semio/kit_metabolism.json";
export * from "./icons";
export { default as MetabolismKitDiff } from "./semio/diff_kit_metabolism.json";
export { default as MetabolismKitDiffInverted } from "./semio/diff_kit_metabolism_inverted.json";
export { default as InvalidKit } from "./semio/kit_invalid.json";
export { default as MetabolismKitDiffed } from "./semio/kit_metabolism_diffed.json";
export { default as InvalidKitValidation } from "./semio/validation.json";
export { MetabolismKitData as MetabolismKit };

/**
 * Metabolism kit types array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitTypes](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITTYPES)
 **/
export const MetabolismKitTypes = MetabolismKitData.types ?? [];
/**
 * Metabolism kit designs array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitDesigns](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITDESIGNS)
 **/
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
/**
 * Metabolism kit ports array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitPorts](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITPORTS)
 **/
export const MetabolismKitPorts = MetabolismKitData.ports ?? [];
/**
 * Metabolism kit qualities array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitQualities](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITQUALITIES)
 **/
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
/**
 * Metabolism kit files array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitFiles](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITFILES)
 **/
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
/**
 * Metabolism kit folders array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitFolders](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITFOLDERS)
 **/
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
/**
 * Metabolism kit authors array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitAuthors](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITAUTHORS)
 **/
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
/**
 * Metabolism kit tags array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitTags](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITTAGS)
 **/
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
/**
 * Metabolism kit concepts array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitConcepts](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITCONCEPTS)
 **/
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
/**
 * Metabolism kit attributes array
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitAttributes](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITATTRIBUTES)
 **/
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitNakaginCapsuleTowerDesigns](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITNAKAGINCAPSULETOWERDESIGNS)
 **/
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => design.name === "Nakagin Capsule Tower") ?? [];

// Builds guid and name lookup maps from an item array
// Callers MUST provide an array of objects with optional guid and name fields
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

// Type lookup maps by guid and name
const typeLookup = buildLookup(MetabolismKitTypes);
// Design lookup maps by guid and name
const designLookup = buildLookup(MetabolismKitDesigns);
// Port lookup maps by guid and name
const portLookup = buildLookup(MetabolismKitPorts);

/**
 * Metabolism kit types indexed by guid
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitTypesByGuid](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITTYPESBYGUID)
 **/
export const MetabolismKitTypesByGuid = typeLookup.byGuid;
/**
 * Metabolism kit types indexed by name
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitTypesByName](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITTYPESBYNAME)
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by guid
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitDesignsByGuid](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITDESIGNSBYGUID)
 **/
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
/**
 * Metabolism kit designs indexed by name
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitDesignsByName](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITDESIGNSBYNAME)
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit ports indexed by guid
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitPortsByGuid](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITPORTSBYGUID)
 **/
export const MetabolismKitPortsByGuid = portLookup.byGuid;
/**
 * Metabolism kit ports indexed by name
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitPortsByName](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITPORTSBYNAME)
 **/
export const MetabolismKitPortsByName = portLookup.byName;

// Nakagin Capsule Tower root design reference
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
// Nakagin Capsule Tower Flat variant design reference
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 *
 *  * [🪨semio/assets/index.ts#Exports§MetabolismKitNakaginCapsuleTowerFlatPieces](semiorepo://definition/semio/assets/index.ts/EXPORTS/METABOLISMKITNAKAGINCAPSULETOWERFLATPIECES)
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🔖Exports

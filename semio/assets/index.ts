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

// [👤semio🏪assets💻indexts🔖exports](semiorepo://section/SEMIO/ASSETS/INDEX.TS/EXPORTS)
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

/**
 * Metabolism kit types array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkittypes](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-TYPES)
 **/
export const MetabolismKitTypes = MetabolismKitData.types ?? [];
/**
 * Metabolism kit designs array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitdesigns](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-DESIGNS)
 **/
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
/**
 * Metabolism kit ports array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitports](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-PORTS)
 **/
export const MetabolismKitPorts = MetabolismKitData.ports ?? [];
/**
 * Metabolism kit qualities array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitqualities](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-QUALITIES)
 **/
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
/**
 * Metabolism kit files array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitfiles](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-FILES)
 **/
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
/**
 * Metabolism kit folders array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitfolders](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-FOLDERS)
 **/
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
/**
 * Metabolism kit authors array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitauthors](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-AUTHORS)
 **/
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
/**
 * Metabolism kit tags array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkittags](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-TAGS)
 **/
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
/**
 * Metabolism kit concepts array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitconcepts](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-CONCEPTS)
 **/
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
/**
 * Metabolism kit attributes array
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitattributes](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-ATTRIBUTES)
 **/
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitnakagincapsuletowerdesigns](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-NAKAGIN-CAPSULE-TOWER-DESIGNS)
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
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkittypesbyguid](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-TYPES-BY-GUID)
 **/
export const MetabolismKitTypesByGuid = typeLookup.byGuid;
/**
 * Metabolism kit types indexed by name
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkittypesbyname](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-TYPES-BY-NAME)
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by guid
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitdesignsbyguid](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-DESIGNS-BY-GUID)
 **/
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
/**
 * Metabolism kit designs indexed by name
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitdesignsbyname](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-DESIGNS-BY-NAME)
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit ports indexed by guid
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitportsbyguid](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-PORTS-BY-GUID)
 **/
export const MetabolismKitPortsByGuid = portLookup.byGuid;
/**
 * Metabolism kit ports indexed by name
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitportsbyname](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-PORTS-BY-NAME)
 **/
export const MetabolismKitPortsByName = portLookup.byName;

// Nakagin Capsule Tower root design reference
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
// Nakagin Capsule Tower Flat variant design reference
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 *
 *  * [👤semio🏪assets💻indexts🔖exports🪨metabolismkitnakagincapsuletowerflatpieces](semiorepo://definition/SEMIO/ASSETS/INDEX.TS/EXPORTS/METABOLISM-KIT-NAKAGIN-CAPSULE-TOWER-FLAT-PIECES)
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🔖Exports

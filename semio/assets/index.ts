// #region 🔖Header

// 💻semio/assets/index.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

//#region 🔖Exports
// Re-exports and data constants MUST come from the Metabolism kit assets.

import MetabolismKitData from "./semio/kit_metabolism.json";
export * from "./icons";
export { default as MetabolismKitDiff } from "./semio/diff_kit_metabolism.json";
export { default as MetabolismKitDiffInverted } from "./semio/diff_kit_metabolism_inverted.json";
export { default as InvalidKit } from "./semio/kit_invalid.json";
export { default as MetabolismKitDiffed } from "./semio/kit_metabolism_diffed.json";
export { default as InvalidKitValidation } from "./semio/validation.json";
export { MetabolismKitData as MetabolismKit };

// Metabolism kit types array
export const MetabolismKitTypes = MetabolismKitData.types ?? [];
// Metabolism kit designs array
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
// Metabolism kit ports array
export const MetabolismKitPorts = MetabolismKitData.ports ?? [];
// Metabolism kit qualities array
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
// Metabolism kit files array
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
// Metabolism kit folders array
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
// Metabolism kit authors array
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
// Metabolism kit tags array
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
// Metabolism kit concepts array
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
// Metabolism kit attributes array
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
// Metabolism kit Nakagin Capsule Tower designs subset
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

// Metabolism kit types indexed by guid
export const MetabolismKitTypesByGuid = typeLookup.byGuid;
// Metabolism kit types indexed by name
export const MetabolismKitTypesByName = typeLookup.byName;
// Metabolism kit designs indexed by guid
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
// Metabolism kit designs indexed by name
export const MetabolismKitDesignsByName = designLookup.byName;
// Metabolism kit ports indexed by guid
export const MetabolismKitPortsByGuid = portLookup.byGuid;
// Metabolism kit ports indexed by name
export const MetabolismKitPortsByName = portLookup.byName;

// Nakagin Capsule Tower root design reference
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
// Nakagin Capsule Tower Flat variant design reference
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
// Nakagin Capsule Tower Flat variant piece data with plane and center
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🔖Exports

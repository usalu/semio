// #region Header

// assets/index.ts

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

// #endregion Header

import MetabolismKitData from "./semio/kit_metabolism.json";
export * from "./icons";
export { default as MetabolismKitDiff } from "./semio/diff_kit_metabolism.json";
export { default as MetabolismKitDiffInverted } from "./semio/diff_kit_metabolism_inverted.json";
export { default as InvalidKit } from "./semio/kit_invalid.json";
export { default as MetabolismKitDiffed } from "./semio/kit_metabolism_diffed.json";
export { default as InvalidKitValidation } from "./semio/validation.json";
export { MetabolismKitData as MetabolismKit };

export const MetabolismKitTypes = MetabolismKitData.types ?? [];
export const MetabolismKitDesigns = MetabolismKitData.designs ?? [];
export const MetabolismKitPorts = MetabolismKitData.ports ?? [];
export const MetabolismKitQualities = (MetabolismKitData as { qualities?: unknown[] }).qualities ?? [];
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
export const MetabolismKitAttributes = (MetabolismKitData as { attributes?: unknown[] }).attributes ?? [];
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => design.name === "Nakagin Capsule Tower") ?? [];

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

const typeLookup = buildLookup(MetabolismKitTypes);
const designLookup = buildLookup(MetabolismKitDesigns);
const portLookup = buildLookup(MetabolismKitPorts);

export const MetabolismKitTypesByGuid = typeLookup.byGuid;
export const MetabolismKitTypesByName = typeLookup.byName;
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
export const MetabolismKitDesignsByName = designLookup.byName;
export const MetabolismKitPortsByGuid = portLookup.byGuid;
export const MetabolismKitPortsByName = portLookup.byName;

const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find((d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];

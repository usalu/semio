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
export const MetabolismKitInterfaces = MetabolismKitData.interfaces ?? [];
export const MetabolismKitQualities = MetabolismKitData.qualities ?? [];
export const MetabolismKitFiles = MetabolismKitData.files ?? [];
export const MetabolismKitFolders = MetabolismKitData.folders ?? [];
export const MetabolismKitAuthors = MetabolismKitData.authors ?? [];
export const MetabolismKitTags = MetabolismKitData.tags ?? [];
export const MetabolismKitConcepts = MetabolismKitData.concepts ?? [];
export const MetabolismKitAttributes = MetabolismKitData.attributes ?? [];
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter(
    (design) => design.name === "Nakagin Capsule Tower",
) ?? [];

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
const interfaceLookup = buildLookup(MetabolismKitInterfaces);

export const MetabolismKitTypesByGuid = typeLookup.byGuid;
export const MetabolismKitTypesByName = typeLookup.byName;
export const MetabolismKitDesignsByGuid = designLookup.byGuid;
export const MetabolismKitDesignsByName = designLookup.byName;
export const MetabolismKitInterfacesByGuid = interfaceLookup.byGuid;
export const MetabolismKitInterfacesByName = interfaceLookup.byName;

const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => d.name === "Nakagin Capsule Tower");
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find(
    (d) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid,
);
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
    nakaginCapsuleTowerFlatDesign?.pieces?.map((p) => ({
        name: p.name,
        plane: p.plane,
        center: p.center,
    })) ?? [];

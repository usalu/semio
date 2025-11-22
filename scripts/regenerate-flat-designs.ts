import * as fs from "fs";
import * as path from "path";
import { applyDesignDiff, flattenDesign, Kit } from "../js/js/semio";

const assetsPath = path.join(process.cwd(), "assets/semio");

const kitJson = fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8");
const kit = JSON.parse(kitJson) as Kit;

const designNames = [
    "Nakagin Capsule Tower",
    "Slanted",
    "Twisted",
    "Dancing",
    "Capsule Dream"
];

const fileNames = [
    "design_nakagin-capsule-tower_flat.json",
    "design_nakagin-capsule-tower_slanted_flat.json",
    "design_nakagin-capsule-tower_twisted_flat.json",
    "design_nakagin-capsule-tower_dancing_flat.json",
    "design_capsule-dream_flat.json"
];

for (let i = 0; i < designNames.length; i++) {
    const designName = designNames[i];
    const fileName = fileNames[i];

    const design = kit.designs?.find((d) => d.name === designName);
    if (!design) {
        console.error(`Design "${designName}" not found`);
        continue;
    }

    console.log(`Flattening ${designName}...`);
    const flatDesignDiff = flattenDesign(kit, design.guid);
    const flatDesign = applyDesignDiff(design, flatDesignDiff);

    const outputPath = path.join(assetsPath, fileName);
    fs.writeFileSync(outputPath, JSON.stringify(flatDesign, null, 2));
    console.log(`✓ Wrote ${fileName}`);
}

console.log("\nDone!");

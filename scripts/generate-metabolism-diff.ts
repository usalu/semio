// #region Header

// generate-metabolism-diff.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { promises as fs } from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import {
    applyKitDiff,
    Attribute,
    Author,
    Design,
    Folder,
    getKitDiff,
    guid,
    Interface,
    inverseKitDiff,
    Kit,
    File as KitFile,
    Quality,
    Type
} from "../js/js/semio";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function main() {
    console.log("Loading Metabolism kit...");
    const kitPath = path.join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
    const kitJson = await fs.readFile(kitPath, "utf-8");
    const original: Kit = JSON.parse(kitJson);

    console.log("Creating comprehensive diff...");

    // First, create a modified version of the kit by applying incremental changes
    let modified: Kit = JSON.parse(JSON.stringify(original));

    // Modify top-level properties
    modified.name = "Metabolism Modified";
    modified.version = "r25.08-1";
    modified.description = "Modified version for comprehensive diff testing";
    modified.icon = "modified-icon.svg";
    modified.image = "modified-image.png";
    modified.homepage = "https://modified.example.com";
    modified.license = "MIT-Modified";
    modified.concepts = ["metabolism", "nakagin", "modified"];

    // 1. Types: Add, remove, update
    if (modified.types && modified.types.length > 0) {
        // Remove first type
        modified.types.splice(0, 1);

        // Update second type (now first)
        if (modified.types.length > 0) {
            modified.types[0].name = modified.types[0].name + " Modified";
            modified.types[0].description = "Updated description";
            modified.types[0].ports = modified.types[0].ports || [];
            modified.types[0].ports.push({
                guid: guid(),
                name: "new-port",
                point: { x: 1, y: 1, z: 1 },
                direction: { x: 0, y: 1, z: 0 },
                t: 0.5,
                mandatory: true,
            });
        }

        // Add new type
        const newType: Type = {
            guid: guid(),
            name: "New Test Type",
            virtual: true,
            unit: "mm",
            description: "A new type added for testing",
            createdAt: new Date() as any,
            updatedAt: new Date() as any,
            ports: [
                {
                    guid: guid(),
                    name: "test-port",
                    point: { x: 0, y: 0, z: 0 },
                    direction: { x: 0, y: 0, z: 1 },
                    t: 0,
                    mandatory: false,
                },
            ],
        };
        modified.types.push(newType);
    }

    // 2. Designs: Add, remove, update
    if (modified.designs && modified.designs.length > 0) {
        // Remove first design
        modified.designs.splice(0, 1);

        // Update second design (now first)
        if (modified.designs.length > 0) {
            modified.designs[0].name = modified.designs[0].name + " Modified";
            modified.designs[0].description = "Updated design description";
            modified.designs[0].pieces = modified.designs[0].pieces || [];
            modified.designs[0].pieces.push({
                guid: guid(),
                type: original.types?.[3]?.guid ? { guid: original.types[3].guid } : undefined,
                plane: {
                    origin: { x: 5, y: 5, z: 5 },
                    xAxis: { x: 1, y: 0, z: 0 },
                    yAxis: { x: 0, y: 1, z: 0 },
                },
                scale: 1.5,
            });
        }

        // Add new design
        const newDesign: Design = {
            guid: guid(),
            name: "New Test Design",
            unit: "mm",
            description: "A new design added for testing",
            createdAt: new Date() as any,
            updatedAt: new Date() as any,
            pieces: [
                {
                    guid: guid(),
                    type: original.types?.[2]?.guid ? { guid: original.types[2].guid } : undefined,
                    plane: {
                        origin: { x: 0, y: 0, z: 0 },
                        xAxis: { x: 1, y: 0, z: 0 },
                        yAxis: { x: 0, y: 1, z: 0 },
                    },
                    scale: 1.0,
                },
            ],
        };
        modified.designs.push(newDesign);
    }

    // 3. Qualities: Add, remove, update
    if (modified.qualities && modified.qualities.length > 0) {
        // Remove first quality
        modified.qualities.splice(0, 1);

        // Update second quality (now first)
        if (modified.qualities.length > 0) {
            modified.qualities[0].name = modified.qualities[0].name + " Modified";
            modified.qualities[0].description = "Updated quality description";
            modified.qualities[0].defaultValue = 99;
        }

        // Add new quality
        const newQuality: Quality = {
            guid: guid(),
            key: "test.quality",
            name: "Test Quality",
            kind: 1,
            defaultSiUnit: "m",
            defaultImperialUnit: "ft",
            defaultValue: 10,
            canScale: true,
            description: "A new quality for testing",
        };
        modified.qualities.push(newQuality);
    }

    // 4. Interfaces: Add
    modified.interfaces = modified.interfaces || [];
    const newInterface: Interface = {
        guid: guid(),
        name: "Test Interface",
        description: "A new interface for testing",
        icon: "test-icon.svg",
    };
    modified.interfaces.push(newInterface);

    // 5. Files: Add, remove
    if (modified.files && modified.files.length > 0) {
        // Remove first file
        modified.files.splice(0, 1);

        // Add new file
        const newFile: KitFile = {
            guid: guid(),
            name: "new-file.txt",
            createdAt: new Date() as any,
            updatedAt: new Date() as any,
        };
        modified.files.push(newFile);
    }

    // 6. Folders: Add
    modified.folders = modified.folders || [];
    const newFolder: Folder = {
        guid: guid(),
        name: "test-folder",
        description: "A new folder for testing",
        createdAt: new Date() as any,
        updatedAt: new Date() as any,
    };
    modified.folders.push(newFolder);

    // 7. Authors: Add, remove, update
    if (modified.authors && modified.authors.length > 0) {
        // Remove first author
        modified.authors.splice(0, 1);

        // Update second author (now first)
        if (modified.authors.length > 0) {
            modified.authors[0].name = modified.authors[0].name + " Modified";
            modified.authors[0].email = "modified@example.com";
        }

        // Add new author
        const newAuthor: Author = {
            guid: guid(),
            name: "Test Author",
            email: "test@example.com",
        };
        modified.authors.push(newAuthor);
    }

    // 8. Attributes: Add
    modified.attributes = modified.attributes || [];
    const newAttribute: Attribute = {
        guid: guid(),
        key: "test.attribute",
        value: "test value",
        definition: "A test attribute",
    };
    modified.attributes.push(newAttribute);

    console.log("Computing diff from original to modified...");
    const diff = getKitDiff(original, modified);

    console.log("Computing inverse diff...");
    const inverseDiff = inverseKitDiff(original, diff);

    console.log("Applying forward diff...");
    const diffed = applyKitDiff(original, diff);

    console.log("Writing diff files...");
    const outputDir = path.join(__dirname, "..", "assets", "semio");

    await fs.writeFile(
        path.join(outputDir, "diff_kit_metabolism.json"),
        JSON.stringify(diff, null, 2),
        "utf-8"
    );

    await fs.writeFile(
        path.join(outputDir, "diff_kit_metabolism_inverted.json"),
        JSON.stringify(inverseDiff, null, 2),
        "utf-8"
    );

    await fs.writeFile(
        path.join(outputDir, "kit_metabolism_diffed.json"),
        JSON.stringify(diffed, null, 2),
        "utf-8"
    );

    console.log("Generating flattened designs...");
    const { flattenDesign, applyDesignDiff } = await import("../js/js/semio");

    const nakagin = original.designs?.find((d) => d.name === "Nakagin Capsule Tower");
    const slanted = original.designs?.find((d) => d.name === "Slanted");
    const twisted = original.designs?.find((d) => d.name === "Twisted");
    const dancing = original.designs?.find((d) => d.name === "Dancing");
    const capsuleDream = original.designs?.find((d) => d.name === "Capsule Dream");

    if (nakagin) {
        const flatDesignDiff = flattenDesign(original, nakagin.guid);
        const flatDesign = applyDesignDiff(nakagin, flatDesignDiff);
        await fs.writeFile(
            path.join(outputDir, "design_nakagin_flat.json"),
            JSON.stringify(flatDesign, null, 2),
            "utf-8"
        );
    }

    if (slanted) {
        const flatDesignDiff = flattenDesign(original, slanted.guid);
        const flatDesign = applyDesignDiff(slanted, flatDesignDiff);
        await fs.writeFile(
            path.join(outputDir, "design_slanted_flat.json"),
            JSON.stringify(flatDesign, null, 2),
            "utf-8"
        );
    }

    if (twisted) {
        const flatDesignDiff = flattenDesign(original, twisted.guid);
        const flatDesign = applyDesignDiff(twisted, flatDesignDiff);
        await fs.writeFile(
            path.join(outputDir, "design_twisted_flat.json"),
            JSON.stringify(flatDesign, null, 2),
            "utf-8"
        );
    }

    if (dancing) {
        const flatDesignDiff = flattenDesign(original, dancing.guid);
        const flatDesign = applyDesignDiff(dancing, flatDesignDiff);
        await fs.writeFile(
            path.join(outputDir, "design_dancing_flat.json"),
            JSON.stringify(flatDesign, null, 2),
            "utf-8"
        );
    }

    if (capsuleDream) {
        const flatDesignDiff = flattenDesign(original, capsuleDream.guid);
        const flatDesign = applyDesignDiff(capsuleDream, flatDesignDiff);
        await fs.writeFile(
            path.join(outputDir, "design_capsule_dream_flat.json"),
            JSON.stringify(flatDesign, null, 2),
            "utf-8"
        );
    }

    console.log("Done!");
    console.log(`- diff_kit_metabolism.json: ${JSON.stringify(diff).length} bytes`);
    console.log(`- diff_kit_metabolism_inverted.json: ${JSON.stringify(inverseDiff).length} bytes`);
    console.log(`- kit_metabolism_diffed.json: ${JSON.stringify(diffed).length} bytes`);
}

main().catch(console.error);

// #region Header

// scripts/generate-metabolism-diff.ts

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

// #region Header

// #endregion

import { promises as fs } from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { applyKitDiff, Attribute, Author, Concept, Design, Folder, getKitDiff, guid, Interface, inverseKitDiff, Kit, File as KitFile, Quality, Tag, Type } from "../js/js/semio";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function main() {
  console.log("Loading Metabolism kit...");
  const kitPath = path.join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
  const kitJson = await fs.readFile(kitPath, "utf-8");
  let original: Kit = JSON.parse(kitJson);

  console.log("Removing flattened designs from original...");
  original.designs = original.designs?.filter((d) => !d.parent);

  console.log("Creating comprehensive diff...");

  let modified: Kit = JSON.parse(JSON.stringify(original));

  modified.name = "Metabolism Modified";
  modified.version = "r25.08-1";
  modified.description = "Modified version for comprehensive diff testing";
  modified.icon = "modified-icon.svg";
  modified.image = "modified-image.png";
  modified.homepage = "https://modified.example.com";
  modified.license = "MIT-Modified";

  if (modified.types && modified.types.length > 0) {
    modified.types.splice(0, 1);

    if (modified.types.length > 0) {
      modified.types[0].name = modified.types[0].name + " Modified";
      modified.types[0].description = "Updated description";
      modified.types[0].connectors = modified.types[0].connectors || [];
      modified.types[0].connectors.push({
        guid: guid(),
        name: "new-port",
        point: { x: 1, y: 1, z: 1 },
        direction: { x: 0, y: 1, z: 0 },
        t: 0.5,
        mandatory: true,
      });
    }

    const newType: Type = {
      guid: guid(),
      name: "New Test Type",
      virtual: true,
      unit: "mm",
      description: "A new type added for testing",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
      connectors: [
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

  if (modified.designs && modified.designs.length > 0) {
    modified.designs.splice(0, 1);

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

  if (modified.tags && modified.tags.length > 0) {
    const removedTag = modified.tags.splice(0, 1)[0];

    if (modified.tags.length > 0) {
      modified.tags[0].name = modified.tags[0].name + " Modified";
      modified.tags[0].description = "Updated tag description";
    }

    const newTag: Tag = {
      guid: guid(),
      name: "New Test Tag",
      description: "A new tag for testing",
      icon: "test-tag-icon.svg",
    };
    modified.tags.push(newTag);
  }

  if (modified.concepts && modified.concepts.length > 0) {
    modified.concepts.splice(0, 1);

    if (modified.concepts.length > 0) {
      modified.concepts[0].name = modified.concepts[0].name + " Modified";
      modified.concepts[0].description = "Updated concept description";
    }

    const newConcept: Concept = {
      guid: guid(),
      name: "New Test Concept",
      description: "A new concept for testing",
      icon: "test-concept-icon.svg",
    };
    modified.concepts.push(newConcept);
  }

  if (modified.interfaces && modified.interfaces.length > 0) {
    modified.interfaces.splice(0, 1);

    if (modified.interfaces.length > 0) {
      modified.interfaces[0].name = modified.interfaces[0].name + " Modified";
      modified.interfaces[0].description = "Updated interface description";
    }

    const newInterface: Interface = {
      guid: guid(),
      name: "New Test Interface",
      description: "A new interface for testing",
      icon: "test-interface-icon.svg",
    };
    modified.interfaces.push(newInterface);
  } else {
    modified.interfaces = [
      {
        guid: guid(),
        name: "Test Interface",
        description: "A new interface for testing",
        icon: "test-icon.svg",
      },
    ];
  }

  if (modified.qualities && modified.qualities.length > 0) {
    modified.qualities.splice(0, 1);

    if (modified.qualities.length > 0) {
      modified.qualities[0].name = modified.qualities[0].name + " Modified";
      modified.qualities[0].description = "Updated quality description";
      modified.qualities[0].defaultValue = 99;
    }

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

  if (modified.files && modified.files.length > 0) {
    modified.files.splice(0, 1);

    if (modified.files.length > 0) {
      modified.files[0].name = "updated-" + modified.files[0].name;
    }

    const newFile: KitFile = {
      guid: guid(),
      name: "new-file.txt",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
    };
    modified.files.push(newFile);
  }

  if (modified.folders && modified.folders.length > 0) {
    modified.folders.splice(0, 1);

    if (modified.folders.length > 0) {
      modified.folders[0].name = modified.folders[0].name + " Modified";
      modified.folders[0].description = "Updated folder description";
    }

    const newFolder: Folder = {
      guid: guid(),
      name: "test-folder",
      description: "A new folder for testing",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
    };
    modified.folders.push(newFolder);
  } else {
    modified.folders = [
      {
        guid: guid(),
        name: "test-folder",
        description: "A new folder for testing",
        createdAt: new Date() as any,
        updatedAt: new Date() as any,
      },
    ];
  }

  if (modified.authors && modified.authors.length > 1) {
    modified.authors[0].name = modified.authors[0].name + " Modified";
    modified.authors[0].email = "modified@example.com";

    modified.authors.splice(1, 1);

    const newAuthor: Author = {
      guid: guid(),
      name: "Test Author",
      email: "test@example.com",
    };
    modified.authors.push(newAuthor);
  }

  if (modified.attributes && modified.attributes.length > 0) {
    modified.attributes.splice(0, 1);

    if (modified.attributes.length > 0) {
      modified.attributes[0].value = "modified-value";
      modified.attributes[0].definition = "Updated attribute definition";
    }

    const newAttribute: Attribute = {
      guid: guid(),
      key: "test.attribute",
      value: "test value",
      definition: "A test attribute",
    };
    modified.attributes.push(newAttribute);
  } else {
    modified.attributes = [
      {
        guid: guid(),
        key: "test.attribute",
        value: "test value",
        definition: "A test attribute",
      },
    ];
  }

  console.log("Computing diff from original to modified...");
  const diff = getKitDiff(original, modified);

  console.log("Computing inverse diff...");
  const inverseDiff = inverseKitDiff(original, diff);

  console.log("Applying forward diff...");
  const diffed = applyKitDiff(original, diff);

  console.log("Writing diff files...");
  const outputDir = path.join(__dirname, "..", "assets", "semio");

  await fs.writeFile(path.join(outputDir, "diff_kit_metabolism.json"), JSON.stringify(diff, null, 2), "utf-8");

  await fs.writeFile(path.join(outputDir, "diff_kit_metabolism_inverted.json"), JSON.stringify(inverseDiff, null, 2), "utf-8");

  await fs.writeFile(path.join(outputDir, "kit_metabolism_diffed.json"), JSON.stringify(diffed, null, 2), "utf-8");

  console.log("Done!");
  console.log(`- diff_kit_metabolism.json: ${JSON.stringify(diff).length} bytes`);
  console.log(`- diff_kit_metabolism_inverted.json: ${JSON.stringify(inverseDiff).length} bytes`);
  console.log(`- kit_metabolism_diffed.json: ${JSON.stringify(diffed).length} bytes`);
}

main().catch(console.error);

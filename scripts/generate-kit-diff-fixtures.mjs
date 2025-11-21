#!/usr/bin/env node

import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.join(__dirname, "..");

const SEED = 42;

class SeededRandom {
  constructor(seed) {
    this.seed = seed;
  }

  next() {
    this.seed = (this.seed * 9301 + 49297) % 233280;
    return this.seed / 233280;
  }

  integer(min, max) {
    return Math.floor(this.next() * (max - min + 1)) + min;
  }

  boolean() {
    return this.next() > 0.5;
  }

  pick(array) {
    return array[this.integer(0, array.length - 1)];
  }

  string(length = 10) {
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    return Array.from({ length }, () => chars[this.integer(0, chars.length - 1)]).join("");
  }

  guid() {
    return `${this.string(8)}-${this.string(4)}-${this.string(4)}-${this.string(4)}-${this.string(12)}`;
  }
}

async function loadKit() {
  const kitPath = path.join(ROOT, "assets", "semio", "kit_metabolism.json");
  const content = await fs.readFile(kitPath, "utf-8");
  return JSON.parse(content);
}

function createKitDiff(kit, rng) {
  const diff = {};

  diff.name = `${kit.name} (Modified)`;
  diff.version = "2.0.0";
  diff.description = "Modified version for testing";
  diff.icon = "modified-icon.svg";
  diff.image = "modified-image.png";
  diff.homepage = "https://modified.example.com";
  diff.license = "MIT-Modified";

  diff.attributes = {
    added: [
      {
        guid: rng.guid(),
        key: "test.added",
        value: "new-attribute",
      },
    ],
  };

  if (kit.attributes && kit.attributes.length > 0) {
    diff.attributes.updated = [
      {
        id: kit.attributes[0].guid,
        diff: {
          value: "updated-value",
        },
      },
    ];

    if (kit.attributes.length > 1) {
      diff.attributes.removed = [kit.attributes[kit.attributes.length - 1].guid];
    }
  }

  if (kit.types && kit.types.length > 0) {
    diff.types = {};

    const newType = {
      guid: rng.guid(),
      name: "New Test Type",
      description: "Added for testing",
      canScale: true,
      canMirror: false,
      models: [
        {
          guid: rng.guid(),
          name: "Default Model",
          url: "models/test.obj",
          tags: ["default"],
        },
      ],
      ports: [
        {
          guid: rng.guid(),
          point: { x: 0, y: 0, z: 0 },
          direction: { x: 0, y: 1, z: 0 },
          t: 0,
          isMandatory: false,
        },
      ],
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    diff.types.added = [newType];

    const typeToUpdate = kit.types[0];
    diff.types.updated = [
      {
        id: typeToUpdate.guid,
        diff: {
          name: `${typeToUpdate.name} (Updated)`,
          description: "Updated description",
          ports: {
            added: [
              {
                guid: rng.guid(),
                point: { x: 1, y: 0, z: 0 },
                direction: { x: 1, y: 0, z: 0 },
                t: 0.25,
                isMandatory: false,
              },
            ],
          },
        },
      },
    ];

    if (kit.types.length > 2) {
      diff.types.removed = [kit.types[kit.types.length - 1].guid];
    }
  }

  if (kit.designs && kit.designs.length > 0) {
    diff.designs = {};

    const newDesign = {
      guid: rng.guid(),
      name: "New Test Design",
      description: "Added for testing",
      pieces: [],
      connections: [],
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    diff.designs.added = [newDesign];

    const designToUpdate = kit.designs[0];
    diff.designs.updated = [
      {
        id: designToUpdate.guid,
        diff: {
          name: `${designToUpdate.name} (Updated)`,
          description: "Updated design description",
        },
      },
    ];

    if (kit.designs.length > 2) {
      diff.designs.removed = [kit.designs[kit.designs.length - 1].guid];
    }
  }

  if (kit.qualities && kit.qualities.length > 0) {
    diff.qualities = {};

    const newQuality = {
      guid: rng.guid(),
      key: "test.quality",
      name: "Test Quality",
      description: "Added for testing",
      kind: 1,
      defaultSiUnit: "m",
      defaultImperialUnit: "ft",
      min: 0,
      max: 100,
      defaultValue: 50,
    };
    diff.qualities.added = [newQuality];

    const qualityToUpdate = kit.qualities[0];
    diff.qualities.updated = [
      {
        id: qualityToUpdate.guid,
        diff: {
          name: `${qualityToUpdate.name} (Updated)`,
          description: "Updated quality description",
          benchmarks: {
            added: [
              {
                guid: rng.guid(),
                name: "New Benchmark",
                min: 10,
                max: 20,
              },
            ],
          },
        },
      },
    ];

    if (kit.qualities.length > 2) {
      diff.qualities.removed = [kit.qualities[kit.qualities.length - 1].guid];
    }
  }

  if (kit.interfaces && kit.interfaces.length > 0) {
    diff.interfaces = {};

    const newInterface = {
      guid: rng.guid(),
      name: "New Test Interface",
      description: "Added for testing",
      compatibleInterfaces: [],
    };
    diff.interfaces.added = [newInterface];

    const interfaceToUpdate = kit.interfaces[0];
    diff.interfaces.updated = [
      {
        id: interfaceToUpdate.guid,
        diff: {
          name: `${interfaceToUpdate.name} (Updated)`,
          description: "Updated interface description",
        },
      },
    ];

    if (kit.interfaces.length > 2) {
      diff.interfaces.removed = [kit.interfaces[kit.interfaces.length - 1].guid];
    }
  }

  if (kit.files && kit.files.length > 0) {
    diff.files = {};

    const newFile = {
      guid: rng.guid(),
      name: "new-test-file.txt",
      size: 1024,
      hash: rng.string(32),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    diff.files.added = [newFile];

    const fileToUpdate = kit.files[0];
    diff.files.updated = [
      {
        id: fileToUpdate.guid,
        diff: {
          name: `${fileToUpdate.name}.updated`,
          size: (fileToUpdate.size || 0) + 100,
        },
      },
    ];

    if (kit.files.length > 2) {
      diff.files.removed = [kit.files[kit.files.length - 1].guid];
    }
  }

  if (kit.folders && kit.folders.length > 0) {
    diff.folders = {};

    const newFolder = {
      guid: rng.guid(),
      name: "new-test-folder",
      description: "Added for testing",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    diff.folders.added = [newFolder];

    const folderToUpdate = kit.folders[0];
    diff.folders.updated = [
      {
        id: folderToUpdate.guid,
        diff: {
          name: `${folderToUpdate.name} (Updated)`,
          description: "Updated folder description",
        },
      },
    ];

    if (kit.folders.length > 2) {
      diff.folders.removed = [kit.folders[kit.folders.length - 1].guid];
    }
  }

  if (kit.authors && kit.authors.length > 0) {
    diff.authors = {};

    const newAuthor = {
      guid: rng.guid(),
      name: "Test Author",
      email: "test@example.com",
    };
    diff.authors.added = [newAuthor];

    const authorToUpdate = kit.authors[0];
    diff.authors.updated = [
      {
        id: authorToUpdate.guid,
        diff: {
          name: `${authorToUpdate.name} (Updated)`,
          email: `updated-${authorToUpdate.email}`,
        },
      },
    ];

    if (kit.authors.length > 2) {
      diff.authors.removed = [kit.authors[kit.authors.length - 1].guid];
    }
  }

  return diff;
}

function applyCollectionDiff(base, diff) {
  if (!diff) return base;

  let result = [...(base || [])];

  if (diff.removed) {
    result = result.filter((item) => !diff.removed.includes(item.guid));
  }

  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((item) => item.guid === update.id);
      if (index !== -1) {
        result[index] = { ...result[index], ...update.diff };
      }
    }
  }

  if (diff.added) {
    result.push(...diff.added);
  }

  return result;
}

function applyKitDiff(kit, diff) {
  const result = { ...kit };

  if (diff.name !== undefined) result.name = diff.name;
  if (diff.version !== undefined) result.version = diff.version;
  if (diff.description !== undefined) result.description = diff.description;
  if (diff.icon !== undefined) result.icon = diff.icon;
  if (diff.image !== undefined) result.image = diff.image;
  if (diff.remote !== undefined) result.remote = diff.remote;
  if (diff.homepage !== undefined) result.homepage = diff.homepage;
  if (diff.license !== undefined) result.license = diff.license;
  if (diff.concepts !== undefined) result.concepts = diff.concepts;

  result.types = applyCollectionDiff(kit.types, diff.types);
  result.designs = applyCollectionDiff(kit.designs, diff.designs);
  result.qualities = applyCollectionDiff(kit.qualities, diff.qualities);
  result.interfaces = applyCollectionDiff(kit.interfaces, diff.interfaces);
  result.files = applyCollectionDiff(kit.files, diff.files);
  result.folders = applyCollectionDiff(kit.folders, diff.folders);
  result.authors = applyCollectionDiff(kit.authors, diff.authors);
  result.attributes = applyCollectionDiff(kit.attributes, diff.attributes);

  result.updatedAt = new Date().toISOString();

  return result;
}

function inverseCollectionDiff(original, diff) {
  if (!diff) return undefined;

  const inverse = {};

  if (diff.removed) {
    const removedItems = diff.removed
      .map((guid) => original.find((item) => item.guid === guid))
      .filter(Boolean);
    if (removedItems.length > 0) {
      inverse.added = removedItems;
    }
  }

  if (diff.updated) {
    inverse.updated = diff.updated.map((update) => {
      const originalItem = original.find((item) => item.guid === update.id);
      const inverseDiff = {};

      for (const key in update.diff) {
        if (originalItem && key in originalItem) {
          inverseDiff[key] = originalItem[key];
        }
      }

      return { id: update.id, diff: inverseDiff };
    });
  }

  if (diff.added) {
    inverse.removed = diff.added.map((item) => item.guid);
  }

  return Object.keys(inverse).length > 0 ? inverse : undefined;
}

function inverseKitDiff(original, diff) {
  const inverse = {};

  if (diff.name !== undefined) inverse.name = original.name;
  if (diff.version !== undefined) inverse.version = original.version;
  if (diff.description !== undefined) inverse.description = original.description;
  if (diff.icon !== undefined) inverse.icon = original.icon;
  if (diff.image !== undefined) inverse.image = original.image;
  if (diff.remote !== undefined) inverse.remote = original.remote;
  if (diff.homepage !== undefined) inverse.homepage = original.homepage;
  if (diff.license !== undefined) inverse.license = original.license;
  if (diff.concepts !== undefined) inverse.concepts = original.concepts;

  if (diff.types) inverse.types = inverseCollectionDiff(original.types || [], diff.types);
  if (diff.designs) inverse.designs = inverseCollectionDiff(original.designs || [], diff.designs);
  if (diff.qualities) inverse.qualities = inverseCollectionDiff(original.qualities || [], diff.qualities);
  if (diff.interfaces) inverse.interfaces = inverseCollectionDiff(original.interfaces || [], diff.interfaces);
  if (diff.files) inverse.files = inverseCollectionDiff(original.files || [], diff.files);
  if (diff.folders) inverse.folders = inverseCollectionDiff(original.folders || [], diff.folders);
  if (diff.authors) inverse.authors = inverseCollectionDiff(original.authors || [], diff.authors);
  if (diff.attributes) inverse.attributes = inverseCollectionDiff(original.attributes || [], diff.attributes);

  return inverse;
}

async function main() {
  console.log("Loading metabolism kit...");
  const kit = await loadKit();

  console.log("Generating kit diff with seed:", SEED);
  const rng = new SeededRandom(SEED);
  const diff = createKitDiff(kit, rng);

  console.log("Applying diff to generate modified kit...");
  const diffedKit = applyKitDiff(kit, diff);

  console.log("Calculating inverse diff...");
  const inverseDiff = inverseKitDiff(kit, diff);

  const outputDir = path.join(ROOT, "assets", "semio");

  console.log("Writing diff_kit_metabolism.json...");
  await fs.writeFile(path.join(outputDir, "diff_kit_metabolism.json"), JSON.stringify(diff, null, 2));

  console.log("Writing diff_kit_metabolism_inverted.json...");
  await fs.writeFile(path.join(outputDir, "diff_kit_metabolism_inverted.json"), JSON.stringify(inverseDiff, null, 2));

  console.log("Writing kit_metabolism_diffed.json...");
  await fs.writeFile(path.join(outputDir, "kit_metabolism_diffed.json"), JSON.stringify(diffedKit, null, 2));

  console.log("Done! Generated 3 fixture files.");
}

main().catch(console.error);

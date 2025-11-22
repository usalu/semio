#!/usr/bin/env node

import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { getKitDiff, applyKitDiff, inverseKitDiff } from "../js/js/semio.ts";

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

function createModifiedKit(kit, rng) {
  const modified = JSON.parse(JSON.stringify(kit));

  modified.name = `${kit.name} (Modified)`;
  modified.version = "2.0.0";
  modified.description = "Modified version for testing";
  modified.icon = "modified-icon.svg";
  modified.image = "modified-image.png";
  modified.homepage = "https://modified.example.com";
  modified.license = "MIT-Modified";

  if (!modified.attributes) modified.attributes = [];
  modified.attributes.push({
    guid: rng.guid(),
    key: "test.added",
    value: "new-attribute",
  });
  if (modified.attributes.length > 1) {
    modified.attributes[0].value = "updated-value";
  }
  if (modified.attributes.length > 2) {
    modified.attributes.pop();
  }

  if (modified.types && modified.types.length > 0) {
    modified.types.push({
      guid: rng.guid(),
      name: "New Test Type",
      description: "Added for testing",
      canScale: true,
      canMirror: false,
      models: [
        {
          guid: rng.guid(),
          name: "Default Model",
          file: { guid: rng.guid() },
          tags: ["default"],
        },
      ],
      ports: [
        {
          guid: rng.guid(),
          point: { x: 0, y: 0, z: 0 },
          direction: { x: 0, y: 1, z: 0 },
          t: 0,
          mandatory: false,
        },
      ],
      createdAt: new Date(),
      updatedAt: new Date(),
    });

    modified.types[0].name = `${modified.types[0].name} (Updated)`;
    modified.types[0].description = "Updated description";
    if (!modified.types[0].ports) modified.types[0].ports = [];
    modified.types[0].ports.push({
      guid: rng.guid(),
      point: { x: 1, y: 0, z: 0 },
      direction: { x: 1, y: 0, z: 0 },
      t: 0.25,
      mandatory: false,
    });

    if (modified.types.length > 2) {
      modified.types.pop();
    }
  }

  if (modified.designs && modified.designs.length > 0) {
    modified.designs.push({
      guid: rng.guid(),
      name: "New Test Design",
      description: "Added for testing",
      pieces: [],
      connections: [],
      createdAt: new Date(),
      updatedAt: new Date(),
    });

    modified.designs[0].name = `${modified.designs[0].name} (Updated)`;
    modified.designs[0].description = "Updated design description";

    if (modified.designs.length > 2) {
      modified.designs.pop();
    }
  }

  if (modified.qualities && modified.qualities.length > 0) {
    modified.qualities.push({
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
    });

    modified.qualities[0].name = `${modified.qualities[0].name} (Updated)`;
    modified.qualities[0].description = "Updated quality description";
    if (!modified.qualities[0].benchmarks) modified.qualities[0].benchmarks = [];
    modified.qualities[0].benchmarks.push({
      guid: rng.guid(),
      name: "New Benchmark",
      min: 80,
      max: 100,
    });

    if (modified.qualities.length > 2) {
      modified.qualities.pop();
    }
  }

  if (modified.files && modified.files.length > 0) {
    modified.files.push({
      guid: rng.guid(),
      name: "test-file.txt",
      createdAt: new Date(),
      updatedAt: new Date(),
    });

    modified.files[0].name = `${modified.files[0].name}.updated`;

    if (modified.files.length > 2) {
      modified.files.pop();
    }
  }

  if (modified.authors && modified.authors.length > 0) {
    modified.authors.push({
      guid: rng.guid(),
      name: "Test Author",
      email: "test@example.com",
    });

    modified.authors[0].email = "updated@example.com";

    if (modified.authors.length > 2) {
      modified.authors.pop();
    }
  }

  return modified;
}

async function main() {
  console.log("Loading metabolism kit...");
  const kit = await loadKit();

  console.log("Generating kit diff with seed:", SEED);
  const rng = new SeededRandom(SEED);
  
  const modifiedKit = createModifiedKit(kit, rng);

  console.log("Computing diff from original to modified...");
  const diff = getKitDiff(kit, modifiedKit);

  console.log("Applying diff to generate modified kit...");
  const diffedKit = applyKitDiff(kit, diff);

  console.log("Calculating inverse diff...");
  const inverseDiff = inverseKitDiff(kit, diff);

  const outputDir = path.join(ROOT, "assets", "semio");

  console.log("Writing diff_kit_metabolism.json...");
  await fs.writeFile(
    path.join(outputDir, "diff_kit_metabolism.json"),
    JSON.stringify(diff, null, 2)
  );

  console.log("Writing diff_kit_metabolism_inverted.json...");
  await fs.writeFile(
    path.join(outputDir, "diff_kit_metabolism_inverted.json"),
    JSON.stringify(inverseDiff, null, 2)
  );

  console.log("Writing kit_metabolism_diffed.json...");
  await fs.writeFile(
    path.join(outputDir, "kit_metabolism_diffed.json"),
    JSON.stringify(diffedKit, null, 2)
  );

  console.log("Done!");
}

main().catch(console.error);

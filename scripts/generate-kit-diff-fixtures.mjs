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

  if (modified.authors && modified.authors.length > 0) {
    modified.authors.push({
      guid: rng.guid(),
      name: "Test Author",
      email: "test@example.com",
    });

    modified.authors[0].email = "updated@example.com";
  }

  return modified;
}

async function main() {
  console.log("Loading metabolism kit...");
  const kit = await loadKit();

  console.log("Generating modified kit with seed:", SEED);
  const rng = new SeededRandom(SEED);

  const modifiedKit = createModifiedKit(kit, rng);

  const outputDir = path.join(ROOT, "assets", "semio");

  console.log("Writing kit_metabolism_diffed.json (the target state)...");
  await fs.writeFile(path.join(outputDir, "kit_metabolism_diffed.json"), JSON.stringify(modifiedKit, null, 2));

  console.log("\nNOTE: After writing kit_metabolism_diffed.json, you need to:");
  console.log("1. Run: cd js/js && npm test -- semio.test.ts");
  console.log("2. The test will fail but will compute the correct diff");
  console.log("3. Manually extract the computed diff from test output");
  console.log("4. Save it to diff_kit_metabolism.json");
  console.log("5. Compute inverse diff and save to diff_kit_metabolism_inverted.json");
  console.log("\nOR use TypeScript-enabled environment to compute diffs directly.");

  console.log("Done!");
}

main().catch(console.error);

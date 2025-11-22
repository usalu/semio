import { describe, it } from "vitest";
import { MetabolismKit } from "@semio/assets";
import { applyKitDiff, getKitDiff, inverseKitDiff, Kit } from "./semio";
import fs from "fs/promises";
import path from "path";

const SEED = 42;

class SeededRandom {
  seed: number;

  constructor(seed: number) {
    this.seed = seed;
  }

  next() {
    this.seed = (this.seed * 9301 + 49297) % 233280;
    return this.seed / 233280;
  }

  integer(min: number, max: number) {
    return Math.floor(this.next() * (max - min + 1)) + min;
  }

  string(length: number = 10) {
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    return Array.from({ length }, () => chars[this.integer(0, chars.length - 1)]).join("");
  }

  guid() {
    return `${this.string(8)}-${this.string(4)}-${this.string(4)}-${this.string(4)}-${this.string(12)}`;
  }
}

function createModifiedKit(kit: Kit, rng: SeededRandom): Kit {
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

describe("Generate Kit Diff Fixtures", () => {
  it("should generate kit diff fixtures", async () => {
    const kit = MetabolismKit as unknown as Kit;
    const rng = new SeededRandom(SEED);

    console.log("Generating modified kit...");
    const modifiedKit = createModifiedKit(kit, rng);

    console.log("Computing diff...");
    const diff = getKitDiff(kit, modifiedKit);

    console.log("Computing inverse diff...");
    const inverseDiff = inverseKitDiff(kit, diff);

    console.log("Applying diff to verify...");
    const diffedKit = applyKitDiff(kit, diff);

    const outputDir = path.join(process.cwd(), "..", "..", "assets", "semio");

    console.log("Writing fixtures...");
    await fs.writeFile(path.join(outputDir, "diff_kit_metabolism.json"), JSON.stringify(diff, null, 2));
    await fs.writeFile(path.join(outputDir, "diff_kit_metabolism_inverted.json"), JSON.stringify(inverseDiff, null, 2));
    await fs.writeFile(path.join(outputDir, "kit_metabolism_diffed.json"), JSON.stringify(diffedKit, null, 2));

    console.log("Done!");
  });
});

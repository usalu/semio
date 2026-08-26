import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { fixedFilenameContractIdsForPath, loadTaxonomy, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️TicketCargoFixedAuthority
const ticketRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = join(ticketRoot, "../../../../../../../");
const vectors = JSON.parse(readFileSync(join(ticketRoot, "🧪️ticket-cargo-fixed-authority", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  vectors: { path: string; expectedContractId: string | null }[];
};

describe("ticket Cargo fixed authority", () => {
  const taxonomy = loadTaxonomy();

  test("admits only governed ticket Cargo leaves and the repository lock", () => {
    expect(vectors.schemaVersion).toBe(1);
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    for (const vector of vectors.vectors) {
      const expected = vector.expectedContractId ? [vector.expectedContractId] : [];
      expect(fixedFilenameContractIdsForPath(vector.path, taxonomy)).toEqual(expected);
    }
  });

  test("matches Cargo metadata authority for an isolated ticket package", () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️ticket-cargo-package-"));
    try {
      writeFileSync(join(fixture, "Cargo.toml"), '[package]\nname = "taxonomy-cargo-authority-probe"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n');
      mkdirSync(join(fixture, "src"));
      writeFileSync(join(fixture, "src", "lib.rs"), "pub fn probe() {}\n");
      const result = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1", "--manifest-path", join(fixture, "Cargo.toml")], { cwd: fixture, encoding: "utf8" });
      expect(result.status).toBe(0);
      const metadata = JSON.parse(result.stdout) as { packages: { name: string; manifest_path: string }[] };
      expect(metadata.packages.map((entry) => entry.name)).toEqual(["taxonomy-cargo-authority-probe"]);
      const manifestPath = relative(repoRoot, metadata.packages[0]!.manifest_path).replaceAll("\\", "/");
      expect(fixedFilenameContractIdsForPath(manifestPath, taxonomy)).toEqual(["ticket-cargo-manifest"]);
      expect(fixedFilenameContractIdsForPath(relative(repoRoot, join(fixture, "Cargo.lock")).replaceAll("\\", "/"), taxonomy)).toEqual(["ticket-cargo-lock"]);
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️TicketCargoFixedAuthority

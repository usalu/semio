import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** 🎪️ Ensures the demonstrator wasm link only pulls pane artifact crates, not full multi-artifact plugins. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, expect, it } = vitest;
  const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../../..");
  const cargoPath = join(repoRoot, "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml");
  const cargo = readFileSync(cargoPath, "utf8");

  describe("demonstratorCompileClosure", () => {
    it("does not link whole puzzle, procedural, or gis plugin composition crates", () => {
      expect(cargo).not.toMatch(/semio-s-plugin-puzzle/);
      expect(cargo).not.toMatch(/semio-s-plugin-procedural/);
      expect(cargo).not.toMatch(/semio-s-plugin-gis/);
    });

    it("links only the artifact crates used by the eight demonstrator panes", () => {
      expect(cargo).toContain("semio-s-artifact-puzzle-3d");
      expect(cargo).toContain("semio-s-artifact-procedural-generation3d");
      expect(cargo).toContain("semio-s-artifact-gis-gismap");
      expect(cargo).not.toContain("semio-s-artifact-puzzle-2d");
      expect(cargo).not.toContain("semio-s-artifact-puzzle-5d");
      expect(cargo).not.toContain("semio-s-artifact-procedural-generation2d");
      expect(cargo).not.toContain("semio-s-artifact-procedural-assembly");
      expect(cargo).not.toContain("semio-s-artifact-gis-gisterrain");
    });
  });
}

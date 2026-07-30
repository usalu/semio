// #region 🧭Header
/** @emoji 🏗️ `@semio-tech/cad-js-module-aec-building` — building BIM STEP import profile. */
// #endregion 🧭Header

import { registerImportProfile, typologyFromStepLayer, type TypologyRef } from "@semio-tech/cad-js-core";

// #region 🏷️Ids
export const AEC_BUILDING_MODEL_DEFINITION_ID = "aec.building";
// #endregion 🏷️Ids

// #region 🪪ImportProfile
const BUILDING_LAYER_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  slab: "building.building.slab",
  slabs: "building.building.slab",
  beam: "building.building.beam",
  beams: "building.building.beam",
  column: "building.building.column",
  columns: "building.building.column",
  wall: "building.building.wall",
  walls: "building.building.wall",
  roof: "building.building.roof",
  roofs: "building.building.roof",
  foundation: "building.building.foundation",
  foundations: "building.building.foundation",
  stair: "building.building.stair",
  stairs: "building.building.stair",
  ceiling: "building.building.ceiling",
  ceilings: "building.building.ceiling",
  railing: "building.building.railing",
  railings: "building.building.railing",
  door: "building.building.door",
  doors: "building.building.door",
  window: "building.building.window",
  windows: "building.building.window",
};
// #endregion 🪪ImportProfile

// #region 📦Register
/** @emoji 📦 Registers building STEP import profile on the core engine. */
export function register(): void {
  registerImportProfile(AEC_BUILDING_MODEL_DEFINITION_ID, {
    layerTypology: BUILDING_LAYER_TYPOLOGY,
    fallbackTypology: "building.building.slab" as TypologyRef,
  });
}
// #endregion 📦Register

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("@semio-tech/cad-js-runtime");

  bootstrapCadModules();

  describe("@semio-tech/cad-js-module-aec-building", () => {
    it("maps STEP layer names to building typologies", () => {
      expect(typologyFromStepLayer("Beams", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.beam");
      expect(typologyFromStepLayer("Column", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.column");
    });
  });
}
// #endregion 🧪Tests

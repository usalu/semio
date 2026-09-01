// #region 🧭️Header
/** @emoji 🏗️ `@semio-tech/cad-js-module-aec-building` — building BIM STEP import profile. */
// #endregion 🧭️Header

import { core } from "@semio-tech/cad-js";
const { registerImportProfile, typologyFromStepLayer } = core;
type TypologyRef = core.TypologyRef;

// #region 🏷️Ids
export const AEC_BUILDING_MODEL_DEFINITION_ID = "aec.building";
// #endregion 🏷️Ids

// #region 🪪️ImportProfile
const BUILDING_LAYER_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  slab: "building.building.slab" as TypologyRef,
  slabs: "building.building.slab" as TypologyRef,
  beam: "building.building.beam" as TypologyRef,
  beams: "building.building.beam" as TypologyRef,
  column: "building.building.column" as TypologyRef,
  columns: "building.building.column" as TypologyRef,
  wall: "building.building.wall" as TypologyRef,
  walls: "building.building.wall" as TypologyRef,
  roof: "building.building.roof" as TypologyRef,
  roofs: "building.building.roof" as TypologyRef,
  foundation: "building.building.foundation" as TypologyRef,
  foundations: "building.building.foundation" as TypologyRef,
  stair: "building.building.stair" as TypologyRef,
  stairs: "building.building.stair" as TypologyRef,
  ceiling: "building.building.ceiling" as TypologyRef,
  ceilings: "building.building.ceiling" as TypologyRef,
  railing: "building.building.railing" as TypologyRef,
  railings: "building.building.railing" as TypologyRef,
  door: "building.building.door" as TypologyRef,
  doors: "building.building.door" as TypologyRef,
  window: "building.building.window" as TypologyRef,
  windows: "building.building.window" as TypologyRef,
};
// #endregion 🪪️ImportProfile

// #region 📦️Register
/** @emoji 📦️ Registers building STEP import profile on the core engine. */
export function register(): void {
  registerImportProfile(AEC_BUILDING_MODEL_DEFINITION_ID, {
    layerTypology: BUILDING_LAYER_TYPOLOGY,
    fallbackTypology: "building.building.slab" as TypologyRef,
  });
}
// #endregion 📦️Register

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { runtime } = await import("@semio-tech/cad-js");

  runtime.bootstrapCadModules();

  describe("@semio-tech/cad-js-module-aec-building", () => {
    it("maps STEP layer names to building typologies", () => {
      expect(typologyFromStepLayer("Beams", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.beam");
      expect(typologyFromStepLayer("Column", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.column");
    });
  });
}
// #endregion 🧪️Tests

import type { AecBuildingTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import * as vitestRuntime from "vitest";
import { AEC_BUILDING_MODEL_DEFINITION_ID } from "../../🟦️.ts";
import { core } from "@semio-tech/cad-js";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: AecBuildingTestDependencies, source: TestSource): Promise<void> {
  const { AEC_BUILDING_MODEL_DEFINITION_ID, typologyFromStepLayer } = dependencies;

  const { describe, expect, it } = vitest;
  const runtime = await import("@semio-tech/cad-cad-ts/standards/1/subsets/any/editor/engine/runtime");

  runtime.bootstrapCadModules();

  describe("@semio-tech/cad-js-module-aec-building", () => {
    it("maps STEP layer names to building typologies", () => {
      expect(typologyFromStepLayer("Beams", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.beam");
      expect(typologyFromStepLayer("Column", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.column");
    });
  });

}

const dependencies: AecBuildingTestDependencies = { AEC_BUILDING_MODEL_DEFINITION_ID, typologyFromStepLayer: core.typologyFromStepLayer };
await registerTests1(vitestRuntime, dependencies, { url: new URL("../../🟦️.ts", import.meta.url).href });

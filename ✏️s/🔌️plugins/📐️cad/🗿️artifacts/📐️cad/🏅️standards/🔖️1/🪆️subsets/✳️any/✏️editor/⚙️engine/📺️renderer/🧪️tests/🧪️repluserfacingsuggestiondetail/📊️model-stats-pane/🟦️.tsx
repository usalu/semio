/** 📊️ The model stats pane tests. */
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 📈 Registers the ModelStatsPane tests. */
export function registerModelStatsPaneTests(kit: ReplTestKit): void {
  const { defaultModelDefinitionId, describe, expect, it, listStatDefinitionsForModelDefinition, statDefinitionAppliesToScope } = kit;
  describe("ModelStatsPane", () => {
    it("resolves shape stat labels for the active model definition", () => {
      const definitions = listStatDefinitionsForModelDefinition(defaultModelDefinitionId());
      const geometry = definitions.find((row) => row.id === "spatial.shape.geometry");
      expect(geometry?.label).toBe("Geometry KPIs");
      expect(geometry?.outputs.map((row) => row.label)).toContain("Total volume");
      expect(statDefinitionAppliesToScope(geometry!, "model")).toBe(true);
    });
  });
}

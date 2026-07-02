/** @emoji 🔒 CAD play shape fixture ids and slug resolution for locked playground hosts. */
export const CAD_PLAY_SHAPE_ASSET_IDS = ["concrete-forest-left", "concrete-forest-right"] as const;

/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a CAD shape asset id. */
export function resolveCadPlayFixtureSlug(slug: string): string | undefined {
  const aliases: Record<string, string> = { concrete: "concrete-forest-left" };
  const normalized = aliases[slug] ?? slug;
  return (CAD_PLAY_SHAPE_ASSET_IDS as readonly string[]).includes(normalized) ? normalized : undefined;
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("resolveCadPlayFixtureSlug", () => {
    it("maps concrete shorthand to concrete-forest-left", () => {
      expect(resolveCadPlayFixtureSlug("concrete")).toBe("concrete-forest-left");
      expect(resolveCadPlayFixtureSlug("concrete-forest-right")).toBe("concrete-forest-right");
    });
  });
}

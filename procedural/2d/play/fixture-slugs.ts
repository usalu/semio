/** @emoji 🔒 Procedural 2D play fixture ids and slug resolution for locked playground hosts. */
export const PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID = "procedural2d-default";

export const PROCEDURAL_2D_PLAY_FILE_FIXTURE_IDS = ["default"] as const;

/** @emoji 🔒 Resolves a playground fixture slug to a procedural 2D fixture id. */
export function resolveProcedural2dPlayFixtureSlug(slug: string): string | undefined {
	if (slug === PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID || slug === "default") return "default";
	return (PROCEDURAL_2D_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("resolveProcedural2dPlayFixtureSlug", () => {
		it("maps default shorthand to default fixture file", () => {
			expect(resolveProcedural2dPlayFixtureSlug("default")).toBe("default");
			expect(resolveProcedural2dPlayFixtureSlug(PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID)).toBe("default");
		});
	});
}

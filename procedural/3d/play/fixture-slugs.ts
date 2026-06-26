/** @emoji 🔒 Procedural play fixture ids and slug resolution for locked playground hosts. */
export const PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID = "procedural-default";

export const PROCEDURAL_PLAY_FILE_FIXTURE_IDS = [
	"hexagonal-mushroom-column",
	"rectangle-extrude-volume",
	"sphere-cut-with-torus",
] as const;

export const PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID = "hexagonal-mushroom-column";

/** @emoji 🔒 Resolves a playground fixture slug (e.g. `hexagonal-column`) to a procedural fixture id. */
export function resolveProceduralPlayFixtureSlug(slug: string): string | undefined {
	const aliases: Record<string, string> = {
		"hexagonal-column": PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID,
		column: PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID,
	};
	const normalized = aliases[slug] ?? slug;
	if (normalized === PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID) return normalized;
	return (PROCEDURAL_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(normalized) ? normalized : undefined;
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("resolveProceduralPlayFixtureSlug", () => {
		it("maps hexagonal-column shorthand to hexagonal-mushroom-column", () => {
			expect(resolveProceduralPlayFixtureSlug("hexagonal-column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID);
			expect(resolveProceduralPlayFixtureSlug("column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID);
		});
	});
}

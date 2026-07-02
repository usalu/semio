export const DRAW_PLAY_FIXTURE_DEFAULT_ID = "semio";

export const DRAW_PLAY_FILE_FIXTURE_IDS = ["semio"] as const;

export function resolveDrawPlayFixtureSlug(slug: string): string | undefined {
	if (slug === DRAW_PLAY_FIXTURE_DEFAULT_ID) return "semio";
	return (DRAW_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

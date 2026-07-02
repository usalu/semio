export const DRAW_PLAY_EXAMPLE_DEFAULT_ID = "semio";

export const DRAW_PLAY_FILE_EXAMPLE_IDS = ["semio"] as const;

export function resolveDrawPlayExampleSlug(slug: string): string | undefined {
	if (slug === DRAW_PLAY_EXAMPLE_DEFAULT_ID) return "semio";
	return (DRAW_PLAY_FILE_EXAMPLE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

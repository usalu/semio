export const RASTER_PLAY_FIXTURE_DEFAULT_ID = "semio";

export const RASTER_PLAY_FILE_FIXTURE_IDS = ["semio"] as const;

export function resolveRasterPlayFixtureSlug(slug: string): string | undefined {
	if (slug === RASTER_PLAY_FIXTURE_DEFAULT_ID) return "semio";
	return (RASTER_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

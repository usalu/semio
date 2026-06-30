export const RASTER_PLAY_FIXTURE_DEFAULT_ID = "raster-default";

export const RASTER_PLAY_FILE_FIXTURE_IDS = ["default", "photo-edit", "paint"] as const;

export function resolveRasterPlayFixtureSlug(slug: string): string | undefined {
	if (slug === RASTER_PLAY_FIXTURE_DEFAULT_ID || slug === "default") return "default";
	return (RASTER_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

export const RASTER_PLAY_EXAMPLE_DEFAULT_ID = "semio";

export const RASTER_PLAY_FILE_EXAMPLE_IDS = ["semio"] as const;

export function resolveRasterPlayExampleSlug(slug: string): string | undefined {
	if (slug === RASTER_PLAY_EXAMPLE_DEFAULT_ID) return "semio";
	return (RASTER_PLAY_FILE_EXAMPLE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

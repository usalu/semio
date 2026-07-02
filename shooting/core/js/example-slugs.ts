export const SHOOTING_PLAY_EXAMPLE_DEFAULT_ID = "shooting-default";

export const SHOOTING_PLAY_FILE_EXAMPLE_IDS = ["base-icon"] as const;

export function resolveShootingPlayExampleSlug(slug: string): string | undefined {
	if (slug === SHOOTING_PLAY_EXAMPLE_DEFAULT_ID) return slug;
	return (SHOOTING_PLAY_FILE_EXAMPLE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

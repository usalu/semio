export const SHOOTING_PLAY_FIXTURE_DEFAULT_ID = "shooting-default";

export const SHOOTING_PLAY_FILE_FIXTURE_IDS = ["base-icon"] as const;

export function resolveShootingPlayFixtureSlug(slug: string): string | undefined {
	if (slug === SHOOTING_PLAY_FIXTURE_DEFAULT_ID) return slug;
	return (SHOOTING_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

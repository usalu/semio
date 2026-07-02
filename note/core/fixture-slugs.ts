export const NOTE_PLAY_FIXTURE_DEFAULT_ID = "semio";

export const NOTE_PLAY_FILE_FIXTURE_IDS = ["semio"] as const;

export function resolveNotePlayFixtureSlug(slug: string): string | undefined {
	if (slug === NOTE_PLAY_FIXTURE_DEFAULT_ID) return "semio";
	return (NOTE_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

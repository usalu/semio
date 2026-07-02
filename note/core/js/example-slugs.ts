export const NOTE_PLAY_EXAMPLE_DEFAULT_ID = "semio";

export const NOTE_PLAY_FILE_EXAMPLE_IDS = ["semio"] as const;

export function resolveNotePlayExampleSlug(slug: string): string | undefined {
	if (slug === NOTE_PLAY_EXAMPLE_DEFAULT_ID) return "semio";
	return (NOTE_PLAY_FILE_EXAMPLE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

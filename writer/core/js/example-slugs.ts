export const WRITER_PLAY_EXAMPLE_DEFAULT_ID = "jack";

const WRITER_PLAY_EXAMPLE_ALIASES: Record<string, string> = {
	jack: "jack",
};

export function resolveWriterPlayExampleSlug(raw: string | undefined): string {
	if (!raw) return WRITER_PLAY_EXAMPLE_DEFAULT_ID;
	return WRITER_PLAY_EXAMPLE_ALIASES[raw] ?? raw;
}

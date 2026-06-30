export const WRITER_PLAY_FIXTURE_DEFAULT_ID = "jack";

const WRITER_PLAY_FIXTURE_ALIASES: Record<string, string> = {
	jack: "jack",
};

export function resolveWriterPlayFixtureSlug(raw: string | undefined): string {
	if (!raw) return WRITER_PLAY_FIXTURE_DEFAULT_ID;
	return WRITER_PLAY_FIXTURE_ALIASES[raw] ?? raw;
}

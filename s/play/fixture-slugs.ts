/** @emoji 🧪 S play fixture slug resolution. */
export const S_PLAY_FIXTURE_DEFAULT_ID = "demo" as const;

const S_PLAY_FIXTURE_ALIASES: Readonly<Record<string, string>> = {
	demo: "demo",
	"demo-studio": "demo",
};

export function resolveSPlayFixtureSlug(raw: string | undefined): string {
	const trimmed = raw?.trim();
	if (!trimmed) return S_PLAY_FIXTURE_DEFAULT_ID;
	return S_PLAY_FIXTURE_ALIASES[trimmed] ?? trimmed;
}

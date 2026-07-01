/** @emoji 🧪 Semios play fixture slug resolution. */
export const SEMIOS_PLAY_FIXTURE_DEFAULT_ID = "demo" as const;

const SEMIOS_PLAY_FIXTURE_ALIASES: Readonly<Record<string, string>> = {
	demo: "demo",
	"demo-studio": "demo",
};

export function resolveSemiosPlayFixtureSlug(raw: string | undefined): string {
	const trimmed = raw?.trim();
	if (!trimmed) return SEMIOS_PLAY_FIXTURE_DEFAULT_ID;
	return SEMIOS_PLAY_FIXTURE_ALIASES[trimmed] ?? trimmed;
}

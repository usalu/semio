/** @emoji 🧪 S play fixture slug resolution. */
export const S_PLAY_EXAMPLE_DEFAULT_ID = "demo" as const;

const S_PLAY_EXAMPLE_ALIASES: Readonly<Record<string, string>> = {
	demo: "demo",
	"demo-studio": "demo",
};

export function resolveSPlayExampleSlug(raw: string | undefined): string {
	const trimmed = raw?.trim();
	if (!trimmed) return S_PLAY_EXAMPLE_DEFAULT_ID;
	return S_PLAY_EXAMPLE_ALIASES[trimmed] ?? trimmed;
}

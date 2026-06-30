export const FORMS_PLAY_FIXTURE_DEFAULT_ID = "forms-default";

export const FORMS_PLAY_FILE_FIXTURE_IDS = ["default", "onboarding"] as const;

export function resolveFormsPlayFixtureSlug(slug: string): string | undefined {
	if (slug === FORMS_PLAY_FIXTURE_DEFAULT_ID || slug === "default") return "default";
	return (FORMS_PLAY_FILE_FIXTURE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

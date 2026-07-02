export const FORMS_PLAY_EXAMPLE_DEFAULT_ID = "forms-building-component";

export const FORMS_PLAY_FILE_EXAMPLE_IDS = ["building-component"] as const;

export function resolveFormsPlayExampleSlug(slug: string): string | undefined {
	if (slug === FORMS_PLAY_EXAMPLE_DEFAULT_ID || slug === "building-component") return "building-component";
	return (FORMS_PLAY_FILE_EXAMPLE_IDS as readonly string[]).includes(slug) ? slug : undefined;
}

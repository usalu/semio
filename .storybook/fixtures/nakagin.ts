// #region 🧲Header
// 💻 .storybook/fixtures/nakagin.ts — lightweight Nakagin sample rows for elements UI stories.
// #endregion 🧲Header

//#region 🔖NakaginFixtures
/** @emoji 🏺 Nakagin capsule tower blurb for textarea/table demos. */
export const nakagin = {
	description:
		"The digital shadow of the former Nakagin Capsule Tower — a mixed-use residential and office tower designed by architect Kisho Kurokawa in Shimbashi, Tokyo. Completed in 1972, it exemplified Japanese Metabolism.",
} as const;

/** @emoji 👤 Sample architects for {@link Table} stories. */
export const architects = [
	{ id: "1", name: "Kisho Kurokawa", role: "Principal", email: "kurokawa@example.com" },
	{ id: "2", name: "Maki Fumihiko", role: "Metabolist", email: "maki@example.com" },
	{ id: "3", name: "Kiyonori Kikutake", role: "Metabolist", email: "kikutake@example.com" },
] as const;
//#endregion 🔖NakaginFixtures

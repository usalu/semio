// #region 🧲Header
/** @emoji 🏪 Barrel for vendored UI icons, fonts, and static assets. */
// #endregion 🧲Header

//#region 🗃️Exports
export { ICONS, ICON_NAMES, isIconName, type IconName } from "../icon/generated/icons.ts";
export { assertUniqueIconConceptAssignments, ICON_CONCEPT_ASSIGNMENTS, type IconConceptId } from "./icon_concepts.ts";
export { resolveCatalogIconNameFromTheme, resolveCatalogIconSvgFromTheme } from "./icon_resolver.ts";
export { SHORTCODE_CATALOG, SHORTCODE_EMOJI, shortcodeCatalogKey, shortcodeEmoji, type ShortcodeCatalogName, type ShortcodeEmojiName } from "../icon/generated/shortcodes.ts";
//#endregion 🗃️Exports

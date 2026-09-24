/** 🔎️ U4 census: every chrome text/surface pair of the DEFAULT theme, both appearances — what a correct
 * pairing law would warn about on an untouched palette. */
import theme from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🔣️.json" with { type: "json" };
import { contrastRatioRgba, resolveThemeAppearancePalettes, type UiTheme } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts";
for (const appearance of ["light", "dark"] as const) {
  const chrome = resolveThemeAppearancePalettes(theme as unknown as UiTheme, appearance).chrome;
  const keys = Object.keys(chrome);
  const rows = [];
  for (const text of keys.filter((key) => /foreground$/iu.test(key))) for (const surface of keys.filter((key) => !/foreground$/iu.test(key) && !/^border/u.test(key))) rows.push(`${text}/${surface}=${contrastRatioRgba(chrome[text]!, chrome[surface]!).toFixed(2)}`);
  for (const border of keys.filter((key) => /^border/u.test(key))) for (const surface of ["base", "panel"]) rows.push(`${border}/${surface}=${contrastRatioRgba(chrome[border]!, chrome[surface]!).toFixed(2)}`);
  console.log(appearance, rows.join("  "));
}

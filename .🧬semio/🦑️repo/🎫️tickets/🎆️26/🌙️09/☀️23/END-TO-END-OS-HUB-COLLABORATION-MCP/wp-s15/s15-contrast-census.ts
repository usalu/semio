/** 🔎️ S15 — WCAG census of the DEFAULT `semio` theme's chrome text/surface pairs (`THEME_CHROME_CONTRAST_PAIRS`), both appearances. */
import { THEME_CHROME_CONTRAST_PAIRS, contrastRatioRgba, resolveThemeAppearancePalettes, semioTheme } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts";
const hex = (c: readonly number[]) => `#${c.slice(0, 3).map((v) => v.toString(16).padStart(2, "0")).join("")}`;
for (const appearance of ["light", "dark"] as const) {
  const chrome = resolveThemeAppearancePalettes(semioTheme(), appearance).chrome as Record<string, [number, number, number, number]>;
  for (const pair of THEME_CHROME_CONTRAST_PAIRS) {
    const ratio = contrastRatioRgba(chrome[pair.text]!, chrome[pair.surface]!);
    console.log(`${appearance} ${pair.text}(${hex(chrome[pair.text]!)}) on ${pair.surface}(${hex(chrome[pair.surface]!)}) = ${ratio.toFixed(2)} ${ratio >= 4.5 ? "AA" : ratio >= 3 ? "AA-large-only" : "FAIL"}`);
  }
}

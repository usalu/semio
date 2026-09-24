/** 🔎️ S15 — candidate paints for the failing default-theme chrome pairs, resolved through the theme's OWN resolver
 * (`resolveThemeAppearancePalettes` on a patched copy) and graded by `THEME_CHROME_CONTRAST_PAIRS`. */
import { THEME_CHROME_CONTRAST_PAIRS, contrastRatioRgba, resolveThemeAppearancePalettes, semioTheme } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts";
const hex = (c: readonly number[]) => `#${c.slice(0, 3).map((v) => v.toString(16).padStart(2, "0")).join("")}`;
const base = process.env.S15_THEME ? (await import(process.env.S15_THEME, { with: { type: "json" } })).default : semioTheme();
const verdict = (appearance: "light" | "dark", patch: Record<string, unknown>) => {
  const theme = structuredClone(base) as any;
  Object.assign(theme.appearances[appearance].chrome, patch);
  const chrome = resolveThemeAppearancePalettes(theme, appearance).chrome as Record<string, number[]>;
  const rows = THEME_CHROME_CONTRAST_PAIRS.map((pair) => ({ pair: `${pair.text}/${pair.surface}`, ratio: contrastRatioRgba(chrome[pair.text] as any, chrome[pair.surface] as any) }));
  return { worst: rows.filter((row) => row.ratio < 4.5).map((row) => `${row.pair}=${row.ratio.toFixed(2)}`), paints: Object.fromEntries(Object.keys(patch).map((key) => [key, hex(chrome[key]!)])), hoverVsBase: contrastRatioRgba(chrome.activeHover as any, chrome.activeBase as any).toFixed(2), rows };
};
const [appearance, json] = [process.argv[2] as "light" | "dark", process.argv[3] ?? "{}"];
const result = verdict(appearance, JSON.parse(json));
console.log(appearance, json, "→", JSON.stringify(result.paints), "fails:", result.worst.join(" ") || "none", "hover/base", result.hoverVsBase);

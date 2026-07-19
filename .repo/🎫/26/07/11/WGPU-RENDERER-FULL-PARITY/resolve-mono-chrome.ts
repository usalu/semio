import { resolveThemeAppearancePalettes, parseUiTheme } from "/Users/ueli/Documents/semio/ui/styling/js/theme.ts";
import mono from "/Users/ueli/Documents/semio/ui/styling/theme/mono.theme.json" with { type: "json" };

const theme = parseUiTheme(mono);
for (const appearance of ["light", "dark"] as const) {
  const palettes = resolveThemeAppearancePalettes(theme, appearance);
  console.log(appearance, JSON.stringify(palettes.chrome));
}

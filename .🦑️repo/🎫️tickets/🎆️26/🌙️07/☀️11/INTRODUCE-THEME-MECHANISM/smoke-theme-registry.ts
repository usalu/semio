import { activeUiTheme, builtinUiThemes, serializeCanvasThemeJson, setActiveUiTheme, subscribeActiveUiTheme, semioTheme } from "../../../../../../ui/styling/js/index.ts";
import { STYLING_BOARD_PALETTES } from "../../../../../../ui/styling/js/tokens.generated.ts";

console.log("semioTheme().id =", semioTheme().id);
console.log("activeUiTheme().id (default) =", activeUiTheme().id);

const themes = builtinUiThemes();
console.log(
  "builtinUiThemes ids:",
  themes.map((t) => t.id),
);

const beforeLight = JSON.parse(serializeCanvasThemeJson("light"));
console.log("before switch, rasterClear matches baked:", JSON.stringify(beforeLight.rasterClear) === JSON.stringify(STYLING_BOARD_PALETTES.light.rasterClear));

const mono = themes.find((t) => t.id === "mono");
if (!mono) {
  console.log("NOTE: mono not found via import.meta.glob in this bun runtime (expected outside Vite) — skipping switch test");
} else {
  let notified = "";
  const unsub = subscribeActiveUiTheme((t) => (notified = t.id));
  setActiveUiTheme(mono);
  console.log("after switch, active =", activeUiTheme().id, "notified =", notified);
  const afterLight = JSON.parse(serializeCanvasThemeJson("light"));
  console.log("after switch, rasterClear differs from baked:", JSON.stringify(afterLight.rasterClear) !== JSON.stringify(STYLING_BOARD_PALETTES.light.rasterClear));
  console.log("mono light.board.rasterClear:", afterLight.rasterClear);
  unsub();
}

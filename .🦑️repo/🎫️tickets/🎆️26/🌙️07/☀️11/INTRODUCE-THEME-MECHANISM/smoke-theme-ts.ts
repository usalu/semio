import { parseUiTheme, resolveThemeAppearancePalettes, resolveThemeMetrics, serializeUiTheme } from "../../../../../../ui/styling/js/theme.ts";
import { STYLING_SEMIO_THEME } from "../../../../../../ui/styling/js/tokens.generated.ts";

const semio = parseUiTheme(STYLING_SEMIO_THEME);
console.log("parsed semio ok, id=", semio.id);

const light = resolveThemeAppearancePalettes(semio, "light");
console.log("light.board.edgeStroke =", light.board.edgeStroke);

const rt = parseUiTheme(JSON.parse(serializeUiTheme(semio)));
console.log("round-trip equal:", JSON.stringify(rt) === JSON.stringify(semio));

const metrics = resolveThemeMetrics(semio.metrics);
console.log("dag.componentWidth =", metrics.dag?.componentWidth);

try {
  parseUiTheme({ ...semio, colors: { ...semio.colors, primary: undefined } });
  console.log("ERROR: should have thrown");
} catch (e) {
  console.log("threw as expected on missing color:", (e as Error).message.slice(0, 60));
}

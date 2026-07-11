import { JSDOM } from "jsdom";

const dom = new JSDOM("<!doctype html><html><body></body></html>");
(globalThis as any).document = dom.window.document;
(globalThis as any).window = dom.window;

const { activeUiTheme, applyUiThemeToDocument, parseUiTheme, serializeCanvasThemeJson, serializeUiTheme, setActiveUiTheme, semioTheme } = await import(
  "../../../../../../ui/styling/js/index.ts"
);
const { STYLING_SEMIO_THEME } = await import("../../../../../../ui/styling/js/tokens.generated.ts");

// 1. Baseline: pristine semio applied.
setActiveUiTheme(semioTheme());
console.log("baseline --color-dark:", dom.window.document.documentElement.style.getPropertyValue("--color-dark"));
console.log("baseline data-ui-theme:", dom.window.document.documentElement.dataset.uiTheme);
console.log("baseline board rasterClear via serializeCanvasThemeJson:", JSON.parse(serializeCanvasThemeJson("light")).rasterClear);

// 2. Simulate a draft edit: clone semio, mutate colors.dark, keep id "semio" (mirrors the settings-tab draft flow).
const draft = structuredClone(semioTheme());
draft.colors.dark = "#00ff00";
setActiveUiTheme(draft);
console.log("after edit --color-dark:", dom.window.document.documentElement.style.getPropertyValue("--color-dark"));
console.log("after edit data-ui-theme:", dom.window.document.documentElement.dataset.uiTheme);

// 3. Save-as: clone with a new id/label (mirrors saveTheme()).
const saved = { ...draft, id: "custom.my-theme", label: "My Theme" };
const reparsed = parseUiTheme(JSON.parse(serializeUiTheme(saved)));
console.log("saved theme round-trips:", reparsed.id === "custom.my-theme" && reparsed.colors.dark === "#00ff00");

// 4. Reset: back to pristine semio.
setActiveUiTheme(semioTheme());
console.log("after reset --color-dark:", dom.window.document.documentElement.style.getPropertyValue("--color-dark"));
console.log("after reset matches generated STYLING_SEMIO_THEME.colors.dark:", (STYLING_SEMIO_THEME as any).colors.dark);

console.log("activeUiTheme().id after reset:", activeUiTheme().id);

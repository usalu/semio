import { PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, playgroundPlayBootHtmlPlugin } from "../../../../../../ui/styling/vite-elements-assets.ts";

console.log("theme script contains snapshot key:", PLAYGROUND_PLAY_BOOT_THEME_SCRIPT.includes("ui.chrome.theme.snapshot"));
console.log("theme script contains --color-:", PLAYGROUND_PLAY_BOOT_THEME_SCRIPT.includes("--color-"));
console.log("theme script contains dataset.uiTheme:", PLAYGROUND_PLAY_BOOT_THEME_SCRIPT.includes("dataset.uiTheme"));

const plugin = playgroundPlayBootHtmlPlugin();
const result = (plugin.transformIndexHtml as any).handler();
const tags = result.tags;
const kinds = tags.map((t: any) => (t.children === PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT ? "appearance" : t.children === PLAYGROUND_PLAY_BOOT_THEME_SCRIPT ? "theme" : t.attrs && "href" in t.attrs ? "stylesheet" : "other"));
console.log("tag order:", kinds);
console.log("appearance before theme:", kinds.indexOf("appearance") < kinds.indexOf("theme"));
console.log("theme before stylesheet:", kinds.indexOf("theme") < kinds.indexOf("stylesheet"));

// Simulate the actual browser execution against a minimal document/localStorage stub.
const store: Record<string, string> = {
  "ui.chrome.theme.snapshot": JSON.stringify({
    id: "mono",
    colors: { dark: "#111111", primary: "#8c8c8c" },
    spacing: { compact: "0.2rem" },
    appearances: { light: { chrome: { base: { token: "dark" }, foreground: { token: "primary" } } }, dark: { chrome: {} } },
  }),
};
const styleProps: Record<string, string> = {};
const bodyStyle: Record<string, string> = {};
(globalThis as any).localStorage = { getItem: (k: string) => store[k] ?? null };
(globalThis as any).document = {
  documentElement: {
    classList: { contains: () => false },
    style: {
      setProperty: (k: string, v: string) => {
        styleProps[k] = v;
      },
    },
    dataset: {} as Record<string, string>,
  },
  body: { style: bodyStyle },
};
// eslint-disable-next-line no-eval
eval(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT);
console.log("applied --color-dark:", styleProps["--color-dark"]);
console.log("applied --spacing-compact:", styleProps["--spacing-compact"]);
console.log("applied data-ui-theme:", (globalThis as any).document.documentElement.dataset.uiTheme);
console.log("applied body backgroundColor (resolved chrome.base -> dark -> #111111):", bodyStyle.backgroundColor);
console.log("applied body color (resolved chrome.foreground -> primary -> #8c8c8c):", bodyStyle.color);

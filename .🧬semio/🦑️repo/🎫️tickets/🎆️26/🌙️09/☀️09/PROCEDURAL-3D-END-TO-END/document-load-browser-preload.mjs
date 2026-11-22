import { JSDOM } from "jsdom";

const { window } = new JSDOM("", { url: "http://localhost/" });
for (const name of ["window", "document", "navigator", "Node", "Element", "HTMLElement", "MutationObserver"]) {
  Object.defineProperty(globalThis, name, { configurable: true, value: name === "window" ? window : window[name] });
}

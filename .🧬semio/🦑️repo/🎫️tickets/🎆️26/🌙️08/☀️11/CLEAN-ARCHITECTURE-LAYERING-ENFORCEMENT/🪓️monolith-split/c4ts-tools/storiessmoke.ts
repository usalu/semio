// usage: bun storiessmoke.ts <label> <stories-module-path> : mounts the stories gallery in jsdom for the chemistry-rules menagerie, lets React and the animation frames run, and writes G/c4ts-stories-<label>.html (normalised markup)
import { JSDOM } from "jsdom";
import { readFileSync, writeFileSync } from "node:fs";
import { plugin } from "bun";
const [label, modulePath] = process.argv.slice(2);
plugin({ name: "css-stub", setup(build) { build.onLoad({ filter: /\.css$/ }, () => ({ contents: "export default {};", loader: "js" })); } });
const dom = new JSDOM("<!doctype html><html><body><div id='root'></div></body></html>", { pretendToBeVisual: true, url: "http://localhost/" });
const g = globalThis as Record<string, unknown>;
for (const key of ["window", "document", "navigator", "HTMLElement", "Element", "Node", "SVGElement", "MutationObserver", "getComputedStyle", "requestAnimationFrame", "cancelAnimationFrame", "IntersectionObserver", "ResizeObserver", "matchMedia", "PointerEvent", "MouseEvent", "Event", "CustomEvent", "KeyboardEvent", "DOMRect", "HTMLCanvasElement", "performance"]) {
  const value = (dom.window as unknown as Record<string, unknown>)[key];
  if (value !== undefined && key !== "performance") g[key] = value;
}
g.window = dom.window; g.IS_REACT_ACT_ENVIRONMENT = true;
(dom.window as unknown as Record<string, unknown>).matchMedia = (query: string) => ({ matches: false, media: query, addEventListener() {}, removeEventListener() {}, addListener() {}, removeListener() {} });
g.matchMedia = (dom.window as unknown as Record<string, unknown>).matchMedia;
g.IntersectionObserver = class { observe() {} unobserve() {} disconnect() {} };
g.ResizeObserver = class { observe() {} unobserve() {} disconnect() {} };
(dom.window as unknown as Record<string, unknown>).IntersectionObserver = g.IntersectionObserver;
(dom.window as unknown as Record<string, unknown>).ResizeObserver = g.ResizeObserver;
const PP = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets";
const fixture = JSON.parse(readFileSync(`${PP}/🧪️tests/🧫️fixtures/⚗️chemistry-rules/🔣️.json`, "utf8"));
const stories = await import(modulePath);
const root = dom.window.document.getElementById("root") as HTMLElement;
stories.mountStories(root, { menagerie: fixture.menagerie }, "fixtures/chemistry", null);
for (let i = 0; i < 30; i += 1) await new Promise((resolve) => setTimeout(resolve, 20));
const html = root.innerHTML.replace(/\s+/g, " ").replace(/id="[^"]*"/g, "").replace(/blob:[^"']*/g, "");
writeFileSync(`${process.env.G}/c4ts-stories-${label}.html`, html);
console.log(`${label}: ${html.length} chars, ${root.querySelectorAll("*").length} elements, ${root.querySelectorAll("canvas").length} canvases, title=${dom.window.document.title}`);
process.exit(0);

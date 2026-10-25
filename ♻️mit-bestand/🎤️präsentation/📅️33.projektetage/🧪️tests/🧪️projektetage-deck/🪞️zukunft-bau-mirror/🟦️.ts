/** 🪞️ The archived Zukunft Bau page stays self-contained: no remote subresources and no dangling local references (browser oracle: the jsdom document and style sheet of the test environment). */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

export type MirrorReference = Readonly<{ tag: string; attribute: "href" | "src"; target: string; relation: string }>;

const subresourceTags = new Set(["audio", "iframe", "img", "script", "source", "video"]);
const subresourceRelations = new Set(["icon", "modulepreload", "preload", "prefetch", "stylesheet"]);
const urlPattern = /url\(\s*(?:"([^"]*)"|'([^']*)'|([^)\s]*))\s*\)/gu;

/** 🔎 Every `href` and `src` of the markup elements in document order (comments and script or style bodies skipped), tag and `rel` included. */
export function htmlReferences(html: string): MirrorReference[] {
  const markup = html.replace(/<!--[\s\S]*?-->/gu, "").replace(/(<(script|style)\b[^>]*>)[\s\S]*?(<\/\2>)/giu, "$1$3");
  return [...markup.matchAll(/<([a-z][a-z0-9-]*)\b([^>]*)>/giu)].flatMap(([, tag, attributes = ""]) => {
    const relation = /\brel="([^"]*)"/iu.exec(attributes)?.[1] ?? "";
    return (["href", "src"] as const).flatMap((attribute) => {
      const target = new RegExp(`\\b${attribute}="([^"]*)"`, "iu").exec(attributes)?.[1];
      return target === undefined ? [] : [{ tag: (tag ?? "").toLowerCase(), attribute, target: target.replaceAll("&amp;", "&"), relation: relation.toLowerCase() }];
    });
  });
}

/** 🔤 The `url(...)` targets of one `@font-face` or style text. */
export function cssUrls(text: string): string[] {
  return [...text.matchAll(urlPattern)].map((match) => match[1] ?? match[2] ?? match[3] ?? "");
}

/** 🔠 The `url(...)` targets of every `@font-face` rule of a style sheet. */
export function fontFaceUrls(css: string): string[] {
  return [...css.replace(/\/\*[\s\S]*?\*\//gu, "").matchAll(/@font-face\s*\{([^}]*)\}/gu)].flatMap(([, body = ""]) => cssUrls(body));
}

export const isRemote = (target: string): boolean => /^(?:[a-z][a-z0-9+.-]*:)?\/\//iu.test(target);
export const isLocalFileTarget = (target: string): boolean => target !== "" && !/^(?:#|[a-z][a-z0-9+.-]*:)/iu.test(target) && !target.startsWith("//");

/** 🌍 The remote subresources the page would fetch while loading: stylesheets, scripts, images, media, icons and preloads. */
export function remoteSubresources(references: readonly MirrorReference[]): MirrorReference[] {
  return references.filter((reference) => isRemote(reference.target) && (subresourceTags.has(reference.tag) || (reference.tag === "link" && reference.relation.split(/\s+/u).some((relation) => subresourceRelations.has(relation)))));
}

/** 🕳️ The local targets that do not resolve to a file, relative to the referencing file or, when root-absolute, to the served root. */
export function danglingTargets(targets: readonly string[], directory: string, servedRoot: string): string[] {
  return targets.filter((target) => isLocalFileTarget(target) && !existsSync(resolve(target.startsWith("/") ? servedRoot : directory, decodeURI(target.split(/[?#]/u)[0] ?? "").replace(/^\//u, ""))));
}

const mirrorFile = "🌐️zukunft-bau-entwerfen-mit-bestand.html";

/** 🪄 Registers the archived Zukunft Bau page tests. */
export function registerZukunftBauMirrorTests(kit: ProjektetageDeckTestKit): void {
  const { describe, expect, it, source } = kit;
  const packageDirectory = dirname(fileURLToPath(source.url));
  const servedRoot = resolve(packageDirectory, "../../🌐️public");
  const htmlPath = resolve(servedRoot, mirrorFile);
  const html = readFileSync(htmlPath, "utf8");
  const references = htmlReferences(html);
  const stylesheets = references.filter((reference) => reference.relation === "stylesheet" && isLocalFileTarget(reference.target)).map((reference) => resolve(dirname(htmlPath), decodeURI(reference.target)));
  describe("archived Zukunft Bau page", () => {
    it("loads no remote subresource", () => {
      expect(remoteSubresources(references)).toEqual([]);
    });
    it("resolves every local reference of the page and of its style sheets", () => {
      expect(danglingTargets(references.map((reference) => reference.target), dirname(htmlPath), servedRoot)).toEqual([]);
      expect(stylesheets.length).toBeGreaterThanOrEqual(4);
      stylesheets.forEach((sheet) => expect(danglingTargets(fontFaceUrls(readFileSync(sheet, "utf8")), dirname(sheet), servedRoot)).toEqual([]));
      stylesheets.forEach((sheet) => expect(danglingTargets(cssUrls(readFileSync(sheet, "utf8")).filter((target) => !target.startsWith("/")), dirname(sheet), servedRoot)).toEqual([]));
    });
    it("reads the same references as the browser document and style sheet", () => {
      const document = new DOMParser().parseFromString(html, "text/html");
      const browserReferences = [...document.querySelectorAll("[href],[src]")].flatMap((element) => (["href", "src"] as const).flatMap((attribute) => (element.hasAttribute(attribute) ? [{ tag: element.localName, attribute, target: element.getAttribute(attribute) ?? "", relation: (element.getAttribute("rel") ?? "").toLowerCase() }] : [])));
      expect(references.length).toBeGreaterThan(100);
      expect(references).toEqual(browserReferences);
      const layout = stylesheets.find((sheet) => sheet.endsWith("layout.css")) ?? "";
      const css = readFileSync(layout, "utf8");
      const style = document.createElement("style");
      style.textContent = css;
      document.head.append(style);
      const browserFontFaces = [...(style.sheet?.cssRules ?? [])].filter((rule) => rule.constructor.name === "CSSFontFaceRule").flatMap((rule) => cssUrls((rule as CSSFontFaceRule).style.getPropertyValue("src")));
      expect(browserFontFaces).toEqual(fontFaceUrls(css));
      expect(css.match(/@font-face/gu)?.length ?? 0).toBeGreaterThanOrEqual(1);
    });
    it("refuses remote preloads and dangling font files in synthetic pages", () => {
      const synthetic = htmlReferences('<link rel="preload" href="https://example.org/a.woff2" as="font"><link rel="canonical" href="https://example.org/"><img src="x.png"><script src="//cdn.example.org/a.js"></script>');
      expect(remoteSubresources(synthetic).map((reference) => reference.target)).toEqual(["https://example.org/a.woff2", "//cdn.example.org/a.js"]);
      expect(fontFaceUrls('@font-face{font-family:a;src:url("../Fonts/a.woff2") format("woff2"),local(a)} .x{background:url(b.png)}')).toEqual(["../Fonts/a.woff2"]);
      expect(danglingTargets(["../Fonts/a.woff2", "#x", "data:image/png;base64,AA", "https://example.org/a"], packageDirectory, servedRoot)).toEqual(["../Fonts/a.woff2"]);
    });
  });
}

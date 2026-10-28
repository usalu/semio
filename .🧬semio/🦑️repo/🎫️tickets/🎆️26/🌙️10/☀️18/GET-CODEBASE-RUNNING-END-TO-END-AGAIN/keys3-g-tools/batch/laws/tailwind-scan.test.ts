// 🧪️ keys3-g law: the Tailwind scan region of the course site and the declared cache key agree, and no excluded file can change the served CSS.
// Oracles: Tailwind's own compiler and scanner (@tailwindcss/node + @tailwindcss/oxide, the engine @tailwindcss/vite runs) for the scan region and the CSS; picomatch (third party) next to the repository's own `inputGlobRegExpV1` for the key patterns; a literal token search (no Tailwind code) explains every dropped rule.
// Graph-free. `KEYS3_ROOT` points at a tree holding the patched files (`apply.py stage <dir>`); unset = the live tree (after the batch landed). Run: bun test ./<this file>
import {beforeAll, describe, expect, test} from "bun:test";
import {createRequire} from "node:module";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join, relative, resolve, sep} from "node:path";

const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.KEYS3_ROOT ?? LIVE;
const require = createRequire(LIVE + "/package.json");
const {compile} = await import(require.resolve("@tailwindcss/node"));
const {Scanner} = await import(require.resolve("@tailwindcss/oxide"));
const picomatch = require("picomatch") as (pattern: string, options?: object) => (path: string) => boolean;
const {inputGlobRegExpV1} = await import(LIVE + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts");

const SITE = "🎓️teaching/🏛️architecture/📚️course";
const CSS = `${SITE}/🎨️.css`, PROJECT = `${SITE}/📦️packages/🟦️typescript/📋️project.json`;
const EXCLUDED_DIRECTORIES = ["🧪️tests", "🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples", "📖️stories", "🎭️e2e"];
const excluded = (path: string): boolean => path.split("/").some((part, index, parts) => index < parts.length - 1 && EXCLUDED_DIRECTORIES.includes(part)) || /\.(?:feature|mdx)$/u.test(path) || /\.(?:stories|story)\.[^/]+$/u.test(path);

type Scan = {files: string[]; css: string; candidates: Set<string>};
async function scan(cssText: string, base: string, viteRoot: string): Promise<Scan> {
  const compiler = await compile(cssText, {base, onDependency() {}, shouldRewriteUrls: true});
  const sources = (compiler.root === "none" ? [] : compiler.root === null ? [{base: viteRoot, pattern: "**/*", negated: false}] : [{...compiler.root, negated: false}]).concat(compiler.sources);
  const scanner = new Scanner({sources});
  const candidates = scanner.scan() as string[];
  return {files: [...scanner.files].map((file: string) => relative(LIVE, file).split(sep).join("/")).sort(), css: compiler.build(candidates), candidates: new Set(candidates)};
}
const rules = (css: string): Map<string, string> => {
  const out = new Map<string, string[]>(), stack: string[] = [];
  let current = "";
  for (const char of css.replace(/\/\*[\s\S]*?\*\//gu, "")) {
    if (char === "{") { stack.push(current.trim()); current = ""; }
    else if (char === "}") { if (current.trim()) out.set(stack.join(" >> "), [...(out.get(stack.join(" >> ")) ?? []), current.trim()]); current = ""; stack.pop(); }
    else if (char === ";") { if (stack.length) out.set(stack.join(" >> "), [...(out.get(stack.join(" >> ")) ?? []), current.trim()]); current = ""; }
    else current += char;
  }
  return new Map([...out].map(([key, rows]) => [key, rows.join("\n")]));
};

describe("tailwind scan narrowing", () => {
  const cssText = readFileSync(join(ROOT, CSS), "utf8");
  const baselineText = cssText.split("\n").filter((line) => !line.startsWith("@source not ")).join("\n");
  const project = JSON.parse(readFileSync(join(ROOT, PROJECT), "utf8"));
  const scanPatterns: string[] = project.namedInputs.tailwindScan;

  test("the stylesheet narrows the scan and the project declares the region", () => {
    expect(cssText.split("\n").filter((line) => line.startsWith("@source not ")).length).toBeGreaterThanOrEqual(2);
    expect(Array.isArray(scanPatterns) && scanPatterns.length >= 4).toBe(true);
    expect(project.targets.build.inputs).toContain("tailwindScan");
  });

  test("controlled fixture: the pattern forms exclude tests, fixtures, stories and features and keep everything else", async () => {
    const dir = mkdtempSync(join(import.meta.dir, ".fixture-"));
    try {
      const put = (path: string, text: string) => { mkdirSync(dirname(join(dir, path)), {recursive: true}); writeFileSync(join(dir, path), text); };
      put("site/ok/🟦️.tsx", `export const a = "used-abc flex";`);
      for (const path of ["site/🧪️tests/x/🟦️.tsx", "site/ok/🧪️tests/y.tsx", "site/🧫️fixtures/a.ts", "site/🎭️e2e/a.ts", "site/📖️stories/a.tsx", "site/ok/🧪️.story.tsx", "site/ok/b.stories.tsx", "site/ok/c.feature"]) put(path, `export const b = "only-${path.replace(/[^a-z]/gu, "")} flex";`);
      const entry = `@import "tailwindcss";\n@source "./site";\n@source not "./**/{${EXCLUDED_DIRECTORIES.join(",")}}";\n@source not "./**/*.{feature,stories.*,story.*,mdx}";\n`;
      const result = await scan(entry, dir, dir);
      expect(result.files.some((file) => file.endsWith("site/ok/🟦️.tsx"))).toBe(true);
      expect(result.files.filter((file) => file.includes("site/") && !file.endsWith("site/ok/🟦️.tsx"))).toEqual([]);
      expect(result.candidates.has("used-abc")).toBe(true);
    } finally { rmSync(dir, {recursive: true, force: true}); }
  });

  describe("the real course stylesheet", () => {
    let narrow: Scan, base: Scan;
    beforeAll(async () => { [narrow, base] = [await scan(cssText, join(LIVE, SITE), join(LIVE, SITE)), await scan(baselineText, join(LIVE, SITE), join(LIVE, SITE))]; }, 120000);

    test("no excluded file is scanned, and the baseline scanned some (the narrowing has something to do)", () => {
      expect(narrow.files.filter(excluded)).toEqual([]);
      expect(base.files.filter(excluded).length).toBeGreaterThan(100);
      expect(narrow.files.length).toBeGreaterThan(500);
    });

    test("the scan region is exactly the declared key region (picomatch and the repository matcher agree on every file)", () => {
      const positives = scanPatterns.filter((pattern) => !pattern.startsWith("!")).map((pattern) => pattern.replace("{workspaceRoot}/", ""));
      const negatives = scanPatterns.filter((pattern) => pattern.startsWith("!")).map((pattern) => pattern.slice(1).replace("{workspaceRoot}/", ""));
      const picked = (matchers: ((path: string) => boolean)[], path: string) => matchers.some((match) => match(path));
      const library = [positives.map((pattern) => inputGlobRegExpV1(pattern)), negatives.map((pattern) => inputGlobRegExpV1(pattern))].map((rows) => rows.map((expression: RegExp) => (path: string) => expression.test(path)));
      const third = [positives.map((pattern) => picomatch(pattern, {dot: true})), negatives.map((pattern) => picomatch(pattern, {dot: true}))];
      const keyed = (pair: ((path: string) => boolean)[][], path: string) => picked(pair[0]!, path) && !picked(pair[1]!, path);
      const unkeyedScanned = narrow.files.filter((file) => !keyed(library, file));
      expect(unkeyedScanned).toEqual([]);
      for (const file of [...narrow.files, ...base.files.filter(excluded)]) expect(keyed(third, file), file).toBe(keyed(library, file));
      const keyedButExcluded = base.files.filter((file) => excluded(file) && keyed(library, file));
      expect(keyedButExcluded).toEqual([]);
    });

    test("the served CSS only loses rules, and every lost rule is explained by a literal occurrence in an excluded file", () => {
      const [before, after] = [rules(base.css), rules(narrow.css)];
      const lost = [...before.keys()].filter((key) => !after.has(key));
      expect([...after.keys()].filter((key) => !before.has(key))).toEqual([]);
      const dropped = lost.filter((key) => key.startsWith("@layer utilities")).map((key) => key.split(" >> ").find((part) => part.startsWith("."))!).filter(Boolean);
      const tokens = [...new Set(dropped.map((selector) => selector.slice(1).replace(/\\(.)/gu, "$1").replace(/(?::(?:hover|focus|active|disabled)|\[[^\]]*\])+$/u, "")))];
      const text = base.files.filter(excluded).map((file) => { try { return readFileSync(join(LIVE, file), "utf8"); } catch { return ""; } }).join("\n");
      const unexplained = tokens.filter((token) => !text.includes(token.split(":").pop()!));
      expect(unexplained).toEqual([]);
      expect(lost.length).toBeGreaterThan(0);
    });
  });
});

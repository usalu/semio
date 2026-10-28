/** 🧱️ Contract: the owner tree glob of a native (Cargo) project names every source extension a non-Cargo consumer of that tree reads, so a TypeScript or vite target that depends on `^default` of a native project is keyed by the CSS, HTML, SVG, SQL and JSX files of the owner tree; the Cargo-only extensions (`rs`, `toml`) stay out because the native sources carry them. Evidence: `undeclared-g` read oracle on `@teaching/architecture-course:build` (kernel-enforced) read `🖱️ui/🧵️styles/🎨️.css` and `🖱️ui/🌐️globals/🎨️.css`, which no key named.
 * run: `bun test ./<this file>` */
import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";

const source = readFileSync(`${process.env.UNDECLARED_G_ROOT ?? "/Users/ueli/Documents/semio"}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs`, "utf8");
const set = (text: string) => new Set(text.split(","));
const generic = /: "\{(ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit,sql)\}";/u.exec(source)?.[1];
const native = /const ownerExtensions = native \? "\{([^}]*)\}" : extensions;/u.exec(source)?.[1];

test("both extension lists are found in the source", () => { expect(generic).toBeString(); expect(native).toBeString(); });

test("the native owner list is the generic list plus the native-only IDL and shader extensions, and nothing a consumer reads is missing", () => {
  const web = ["css", "scss", "html", "svg", "sql", "jsx"], nativeOnly = ["semio", "wgsl", "glsl", "h", "c", "cpp"];
  const have = set(native!);
  for (const extension of [...set(generic!), ...nativeOnly]) expect(have.has(extension), extension).toBe(true);
  for (const extension of web) expect(have.has(extension), extension).toBe(true);
  expect(have.has("rs")).toBe(false);
  expect(have.has("toml")).toBe(false);
});

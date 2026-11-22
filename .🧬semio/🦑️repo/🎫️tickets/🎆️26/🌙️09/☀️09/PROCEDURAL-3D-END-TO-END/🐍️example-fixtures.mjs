/** 🗂️ THE one resolver for the bundled generation3d examples' committed `example-geometry` fixture.
 * The fixture location is NOT restated here: it is `EXAMPLE_FIXTURE_PATH`, the constant the TypeScript
 * twin of the native lane (`📚️examples/🧪️tests/🧩️geometry/🟦️.ts`) declares and the Rust lane mirrors with
 * `include_str!`, stated once relative to each example's root. The examples root is derived from this
 * file's own position, never from an absolute path.
 *
 * @see 🐍️example-oracle.mjs, 🐍️wgpu-example-matrix-probe.mjs, 🐍️wgpu-world3d-interaction-probe.mjs
 */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const REPO_ROOT = resolve(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const EXAMPLES_SEGMENTS = ["✏️s", "🔌️plugins", "🌀️procedural", "🗿️artifacts", "🧊️generation3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "📚️examples"];

export const EXAMPLES_DIR = join(REPO_ROOT, ...EXAMPLES_SEGMENTS);
const { EXAMPLE_FIXTURE_PATH } = await import(join(EXAMPLES_DIR, "🧪️tests", "🧩️geometry", "🟦️.ts"));

/** 📥️ The parsed fixture of one example directory entry, or `null` when it carries none. */
export const readExampleFixture = (entry, dir = EXAMPLES_DIR) => {
  try {
    return JSON.parse(readFileSync(join(dir, entry, EXAMPLE_FIXTURE_PATH), "utf8"));
  } catch {
    return null;
  }
};

/** 🔎️ The fixture of the example whose directory name ends with `slug` (directories carry an emoji prefix). */
export const exampleFixtureFor = (slug, dir = EXAMPLES_DIR) => {
  try {
    const entry = readdirSync(dir).find((name) => name.endsWith(slug));
    return entry ? readExampleFixture(entry, dir) : null;
  } catch {
    return null;
  }
};

/** 📚️ Every example directory entry that carries a fixture. */
export const exampleEntries = (dir = EXAMPLES_DIR) => readdirSync(dir).filter((entry) => readExampleFixture(entry, dir) !== null);

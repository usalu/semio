/** 🧬️ Norm wire twin drift gate: every committed TypeScript twin of the norm plugin equals, byte for byte, what the framework twin generator (`repo ./📜️script.ts schema twins generate --only typescript`) produces from its JSON Schema, so a twin is never edited by hand.
 * @see ../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts */
import { expect, test } from "bun:test";
import { join } from "node:path";
import { runTwins } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";

const REPO = join(import.meta.dir, "../../../../../..");

test("every committed norm TypeScript twin equals its generated twin", () => {
  const report = runTwins(REPO, ["check", "--under", "✏️s/🔌️plugins/📕️norm", "--only", "typescript", "--strict"]);
  expect(report.problems).toEqual([]);
  expect(report.counts.drift).toBe(0);
  expect(report.counts.style).toBe(0);
}, 120_000);

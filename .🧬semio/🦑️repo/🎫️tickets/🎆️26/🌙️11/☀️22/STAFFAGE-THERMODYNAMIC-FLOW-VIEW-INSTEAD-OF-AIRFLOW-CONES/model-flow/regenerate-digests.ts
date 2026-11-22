/** 🧮️ Regenerates the fixture `🧫️fixtures/🧊️model-digests` from `digestsOf(scaledFigure())` after the 3D files changed on purpose (here: the emitter cones left and the air flow came into the thermal scenes). Run from the repo root through vitest, which resolves the test-only loaders: `bun node_modules/vitest/vitest.mjs run --configLoader native --config <this folder>/regenerate.config.ts`. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { it } from "vitest";
import { digestsOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";
import { STAFFAGE_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧰️.ts";

it("writes the digests", { timeout: 3_600_000 }, async () => {
  const digests = await digestsOf(scaledFigure());
  writeFileSync(join(STAFFAGE_ROOT, "🧫️fixtures", "🧊️model-digests", "🔣️.json"), `${JSON.stringify(digests, null, 1)}\n`);
  console.log("digests written", Object.keys(digests.glb).length, Object.keys(digests.fbx).length);
});

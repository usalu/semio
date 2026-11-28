/** ⏪️ Bun preload that restores the clamped chord (`largest / 3200`) in `qualityOf`, to measure the state before the monotone rule with the unchanged rest of the code: `bun --preload old-quality.ts <script>`. */
import { plugin } from "bun";
plugin({
  name: "old-quality",
  setup(build) {
    build.onLoad({ filter: /⏱️job\/🟦️\.ts$/u }, async ({ path }) => {
      const source = await Bun.file(path).text();
      const patched = source.replace("(largest * Math.max(scale, ROUNDNESS.referenceScale)) / (ROUNDNESS.referenceScale * ROUNDNESS.largestPerChord)", "largest / ROUNDNESS.largestPerChord");
      if (patched === source) throw new Error("old-quality: the chord expression was not found");
      return { contents: patched, loader: "ts" };
    });
  },
});

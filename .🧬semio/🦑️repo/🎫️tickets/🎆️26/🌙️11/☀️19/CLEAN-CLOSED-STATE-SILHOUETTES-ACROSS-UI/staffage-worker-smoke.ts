import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset } from "@teaching/staffage";
import { depictionOptions } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🟦️.ts";
import { drawFigure, type DrawingReply, type DrawingRequest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🏁️drawing/🟦️.ts";

const staffage = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const asset = JSON.parse(readFileSync(resolve(staffage, "🧫️fixtures/🚶️reference-walker/🔣️.json"), "utf8")) as Asset;
const worker = new Worker(process.argv[2] ?? resolve(staffage, "🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🏁️drawing/worker.ts"));
const options = depictionOptions({ view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" });
const query = { kind: "clip", clip: asset.presentation.defaultClip, timeS: 0.25 } as const;
const expected = drawFigure(asset, options, query);
const posted = [
  { kind: "asset", id: 1, asset },
  { kind: "draw", id: 1, lease: 2, options, query, wrapLoops: false },
] as const satisfies readonly DrawingRequest[];
try {
  const result = await new Promise<DrawingReply>((accept, reject) => {
    const timeout = setTimeout(() => reject(new Error("Drawing worker timed out")), 10000);
    worker.onmessage = ({ data }: MessageEvent<DrawingReply>) => { clearTimeout(timeout); accept(data); };
    worker.onerror = (event) => { clearTimeout(timeout); reject(new Error(event.message)); };
    for (const request of posted) worker.postMessage(request);
  });
  if ("error" in result) throw new Error(result.error);
  if (result.lease !== 2 || JSON.stringify(result.drawing) !== JSON.stringify(expected)) throw new Error("Worker result differs from direct drawing");
  console.log(`[DEBUG] Worker reproduced the direct SVG (${expected.markup.length} characters)`);
} finally {
  worker.terminate();
}

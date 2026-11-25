/** 📊️ Prints the samples of an idle probe as one line each and the plateau (median and maximum of the renderer memory from 60 s on). Usage: node samples.mjs <s7-samples.jsonl> */
import { readFileSync } from "node:fs";

const samples = readFileSync(process.argv[2], "utf8").trim().split("\n").map((line) => JSON.parse(line));
for (const sample of samples) {
  const state = sample.state;
  console.log([`${sample.s} s`, `renderer ${sample.rendererMB} MB (+${sample.rendererSwapMB} swapped)`, `largest ${sample.largestRendererMB}`, `chromium ${sample.chromiumMB}`, typeof state === "string" ? state : `pages ${state.pages.length} cards ${state.cards} live ${state.live} still ${state.still} svg ${state.svgNodes} elements ${state.elements}`, sample.dom ? `dom nodes ${sample.dom.nodes}` : "", sample.heap ? `heap ${sample.heap.usedMB} MB embedder ${sample.heap.embedderMB} MB` : ""].join(" | "));
}
const late = samples.filter((sample) => sample.s >= 60);
const sorted = (pick) => late.map(pick).sort((a, b) => a - b);
const report = (name, pick) => { const values = sorted(pick); if (values.length > 0) console.log(`${name} from 60 s: median ${values[Math.floor(values.length / 2)]}, min ${values[0]}, max ${values.at(-1)} (${values.length} samples)`); };
report("renderer RSS MB", (sample) => sample.rendererMB);
report("renderer RSS + swap MB", (sample) => sample.rendererMB + sample.rendererSwapMB);
report("all Chromium RSS MB", (sample) => sample.chromiumMB);
report("DOM nodes", (sample) => sample.dom?.nodes ?? 0);
report("script heap MB", (sample) => sample.heap?.usedMB ?? 0);
report("embedder heap MB", (sample) => sample.heap?.embedderMB ?? 0);

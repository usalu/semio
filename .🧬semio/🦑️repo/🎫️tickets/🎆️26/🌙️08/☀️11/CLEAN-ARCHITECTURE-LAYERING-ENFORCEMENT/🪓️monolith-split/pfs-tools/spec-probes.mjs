import fs from "node:fs";
const T = process.env.T;
const file = "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts";
const see = (list) => list.map(([p, d]) => ` * @see ${p} — ${d}`).join("\n");
const doc = (emoji, text, refs = []) => `/** ${emoji} ${text}\n *\n${see([...refs, ["../🟦️.ts", "the differential check table that assembles the topics"]])}\n */`;
const spec = {
  file,
  label: "probes",
  childRoot: "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes",
  deleteParent: false,
  children: [
    { folder: "🧰️probe-kit", doc: doc("🧰️", "Shared kit of the differential check table of `@semio-tech/print-viz-inference`: the three sampling densities every check table honours, the shape of one differential check, the sample counts of a level and the recorder of a d3 path."), stmts: ["30-52", "1677"] },
    { folder: "📐️scale-and-format", doc: doc("📐️", "Differential checks of the scale and format kernels against the d3 packages registered as their oracles.", [["../🧰️probe-kit/🟦️.ts", "the check shape and the sampling densities"]]), stmts: ["55-222"] },
    { folder: "🧮️transform-and-shape", doc: doc("🧮️", "Differential checks of the transform and shape kernels against the d3 packages registered as their oracles.", [["../🧰️probe-kit/🟦️.ts", "the check shape and the sampling densities"]]), stmts: ["274-421"] },
    { folder: "🧭️coordinate-and-hierarchy", doc: doc("🧭️", "Differential checks of the coordinate and hierarchy kernels against the d3 packages registered as their oracles.", [["../🧰️probe-kit/🟦️.ts", "the check shape and the sampling densities"]]), stmts: ["629-756"] },
    { folder: "🕸️network-and-flow", doc: doc("🕸️", "Differential checks of the network and flow kernels against the d3 packages registered as their oracles.", [["../🧰️probe-kit/🟦️.ts", "the check shape and the d3 path recorder"]]), stmts: ["890-1095"] },
    { folder: "🌍️geo-and-spatial", doc: doc("🌍️", "Differential checks of the geo and spatial kernels against the d3 packages registered as their oracles.", [["../🧰️probe-kit/🟦️.ts", "the check shape and the d3 path recorder"]]), stmts: ["1160-1314"] },
    { folder: "🖼️theme-and-render", doc: doc("🖼️", "Differential checks of the theme and render kernels: the planned chart against the TikZ the native emitter writes for the demo specification.", [["../🧰️probe-kit/🟦️.ts", "the check shape"]]), stmts: ["1446-1532"] },
  ],
};
fs.writeFileSync(T + "/spec-probes.json", JSON.stringify(spec, null, 1));

#!/usr/bin/env bun
/** 🖼️ Gives the committed models of the SVG export cases their authored views of `viewsFor`, so the export of the views has an input: the house of the export cases gets the plan of every storey, four elevations, two sections and a camera, the annotated room gets the plan of
 * each storey. The views are written after the last collection and nothing else in the file is touched: `bun r11-w12-views-fixtures.ts`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";
import { viewsFor } from "./r10-w12-views-examples.ts";

const snapshot = em(0x1f4f8) + "snapshot";
const json = em(0x1f523) + ".json";
const fixtures = child(subset, "fixtures");
const targets: [string, (kind: string) => boolean][] = [
  [join(fixtures, em(0x1f3d7) + "ifc", em(0x1f3e0) + "house", snapshot, json), () => true],
  [join(fixtures, em(0x1f4a1) + "inferences", em(0x1faa7) + "annotation-layout", em(0x1f3e0) + "room", snapshot, json), (kind) => kind === "Plan"],
];

for (const [path, keep] of targets) {
  const text = readFileSync(path, "utf8");
  const model = JSON.parse(text);
  delete model.views;
  const views = Object.fromEntries(Object.entries<any>(viewsFor(model)).filter(([, view]) => keep(view.kind)));
  const block = JSON.stringify(views, null, 2).replace(/\n/g, "\n  ");
  const stripped = text.replace(/,\n  "views": \{[\s\S]*?\n  \}(?=\n\}\s*$)/, "");
  writeFileSync(path, stripped.replace(/\n\}\s*$/, `,\n  "views": ${block}\n}\n`));
  console.log(`${path.split(/[\/]/).slice(-3, -2)[0]}: ${Object.keys(views).length} views`);
}

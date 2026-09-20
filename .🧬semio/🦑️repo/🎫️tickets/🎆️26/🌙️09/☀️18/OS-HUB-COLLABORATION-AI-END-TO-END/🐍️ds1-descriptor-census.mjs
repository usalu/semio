// 📏️ DS1 — measures what dominates every shipped plugin descriptor's bytes, and how much of it is
// verbatim-duplicate `ActionDefinition` rows inside `manifest.apps[].windowKinds[].actions`.
// Reads the JSON projection each plugin ships next to its `🛂️.descriptor.semio`.
// Run from the repo root: `bun .🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️ds1-descriptor-census.mjs`
import { readdirSync, existsSync, readFileSync } from "node:fs";

const BASE = "✏️s/🔌️plugins";
const BOUND = 4 * 1024 * 1024;

const rows = [];
for (const plugin of readdirSync(BASE)) {
  const projection = `${BASE}/${plugin}/🔣️.json`;
  const pack = `${BASE}/${plugin}/🛂️.descriptor.semio`;
  if (!existsSync(projection)) continue;
  const descriptor = JSON.parse(readFileSync(projection, "utf8"));
  const apps = descriptor.manifest?.apps ?? [];
  const seen = new Map();
  let windowKinds = 0;
  let actionBytes = 0;
  let actionRows = 0;
  let rosterRows = 0;
  let rosterBytes = 0;
  for (const app of apps) {
    // 📋️ Since the app-roster join (`AppDefinition.actions` / `window_kind_actions`) the roster is
    // stored ONCE per app; counting it keeps "actionBytes" the descriptor's total action cost before
    // and after the change, so the two censuses compare like for like.
    for (const action of app.actions ?? []) {
      const encoded = JSON.stringify(action);
      actionBytes += encoded.length;
      actionRows += 1;
      rosterRows += 1;
      rosterBytes += encoded.length;
      seen.set(encoded, (seen.get(encoded) ?? 0) + 1);
    }
    for (const windowKind of app.windowKinds ?? []) {
      windowKinds += 1;
      for (const action of windowKind.actions ?? []) {
        const encoded = JSON.stringify(action);
        actionBytes += encoded.length;
        actionRows += 1;
        seen.set(encoded, (seen.get(encoded) ?? 0) + 1);
      }
    }
  }
  let distinctBytes = 0;
  for (const encoded of seen.keys()) distinctBytes += encoded.length;
  rows.push({
    plugin,
    apps: apps.length,
    windowKinds,
    descriptorBytes: JSON.stringify(descriptor).length,
    packBytes: existsSync(pack) ? readFileSync(pack).byteLength : 0,
    actionRows,
    distinctRows: seen.size,
    actionBytes,
    rosterRows,
    rosterBytes,
    duplicateBytes: actionBytes - distinctBytes,
  });
}

rows.sort((left, right) => right.duplicateBytes - left.duplicateBytes);
const header = ["plugin", "apps", "windowKinds", "descriptorBytes", "packBytes", "overBound", "actionRows", "distinctRows", "actionBytes", "rosterRows", "rosterBytes", "duplicateBytes", "dupShareOfDescriptor"];
console.log(header.join("\t"));
for (const row of rows) {
  console.log([row.plugin, row.apps, row.windowKinds, row.descriptorBytes, row.packBytes, row.packBytes > BOUND ? "YES" : "", row.actionRows, row.distinctRows, row.actionBytes, row.rosterRows, row.rosterBytes, row.duplicateBytes, `${((100 * row.duplicateBytes) / row.descriptorBytes).toFixed(1)}%`].join("\t"));
}
const total = rows.reduce((accumulator, row) => ({ descriptorBytes: accumulator.descriptorBytes + row.descriptorBytes, duplicateBytes: accumulator.duplicateBytes + row.duplicateBytes, actionRows: accumulator.actionRows + row.actionRows, apps: accumulator.apps + row.apps }), { descriptorBytes: 0, duplicateBytes: 0, actionRows: 0, apps: 0 });
console.log(["TOTAL", total.apps, "", total.descriptorBytes, "", "", total.actionRows, "", "", "", "", total.duplicateBytes, `${((100 * total.duplicateBytes) / total.descriptorBytes).toFixed(1)}%`].join("\t"));

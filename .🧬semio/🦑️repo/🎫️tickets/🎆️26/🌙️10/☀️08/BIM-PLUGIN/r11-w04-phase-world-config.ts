#!/usr/bin/env bun
/**
 * 🎭️ Adds the `viewPhase` text field (after `hiddenStoreys`) to the four schema facets of the BIM world window config. Idempotent; the Rust field table is edited by hand.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, GQLF, JSONF, PROTOF, subset, TSF } from "./r3-f1-paths.ts";

const windows = child(child(child(child(subset, "editor"), "modes"), "edit"), "windows");
const schema = child(child(child(windows, "world"), "config"), "schema");
const edit = (file: string, change: (text: string) => string) => {
  const before = readFileSync(file, "utf8");
  const after = change(before);
  if (before !== after) writeFileSync(file, after);
  console.log(before === after ? "unchanged" : "updated", file.slice(-40));
};
const swap = (text: string, from: string, to: string) => (text.includes(from) && !text.includes(to) ? text.replace(from, to) : text);

edit(join(schema, TSF), (text) => {
  let out = swap(text, "storey visibility and section plane of one exact BIM world window", "storey visibility, phase filter and section plane of one exact BIM world window");
  out = swap(out, "  hiddenStoreys: string[];\n  sectionEnabled", "  hiddenStoreys: string[];\n  viewPhase: string;\n  sectionEnabled");
  out = swap(out, '"hiddenStoreys", "sectionEnabled"', '"hiddenStoreys", "viewPhase", "sectionEnabled"');
  return swap(out, '    hiddenStoreys: list(row.hiddenStoreys, "$.hiddenStoreys", text),\n', '    hiddenStoreys: list(row.hiddenStoreys, "$.hiddenStoreys", text),\n    viewPhase: text(row.viewPhase, "$.viewPhase"),\n');
});
edit(join(schema, GQLF), (text) => swap(text, "  hiddenStoreys: [String!]!\n  sectionEnabled", "  hiddenStoreys: [String!]!\n  viewPhase: String!\n  sectionEnabled"));
edit(join(schema, JSONF), (text) => {
  const doc = JSON.parse(text);
  if (doc.properties.viewPhase) return text;
  const properties: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(doc.properties)) {
    properties[key] = value;
    if (key === "hiddenStoreys") properties.viewPhase = { type: "string" };
  }
  doc.properties = properties;
  doc.required = doc.required.flatMap((key: string) => (key === "hiddenStoreys" ? [key, "viewPhase"] : [key]));
  return JSON.stringify(doc, null, 2) + "\n";
});
edit(join(schema, PROTOF), (text) => {
  if (text.includes("view_phase")) return text;
  return text.replace(
    "  repeated string hidden_storeys = 4;\n  bool section_enabled = 5;\n  string section_axis = 6;\n  double section_offset = 7;\n  bool framed = 8;\n",
    "  repeated string hidden_storeys = 4;\n  string view_phase = 5;\n  bool section_enabled = 6;\n  string section_axis = 7;\n  double section_offset = 8;\n  bool framed = 9;\n",
  );
});

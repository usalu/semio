#!/usr/bin/env bun
/**
 * 🎚️ Rewrites the persisted window configs of the plan and the section window for authored views: both bind to a view id (`view`), the plan's storey and cut height and the section's line and depth live in the view
 * itself. Rewrites the Rust field tables and the four schema facets of each config (idempotent).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, GQLF, JSONF, PROTOF, RS, subset, TSF } from "./r3-f1-paths.ts";

const windows = child(child(child(child(subset, "editor"), "modes"), "edit"), "windows");
const edit = (file: string, change: (text: string) => string) => {
  const before = readFileSync(file, "utf8");
  const after = change(before);
  if (before !== after) writeFileSync(file, after);
  console.log(before === after ? "unchanged" : "updated", file.slice(-60));
};
const swap = (text: string, from: string, to: string) => (text.includes(from) ? text.replace(from, to) : text);

for (const [dirName, kind] of [["plan", "Plan"], ["section", "Section"]] as const) {
  const config = child(child(windows, dirName), "config");
  const schema = child(config, "schema");
  edit(join(config, RS), (text) => {
    if (kind === "Plan") {
      const doc = swap(text, "//! 🎚️ Persisted local state for one exact BIM plan window: which storey it shows, its navigation and the cut height of its display.", "//! 🎚️ Persisted local state for one exact BIM plan window: which plan view it shows and its navigation. The storey and the cut height are authored in the view.");
      return swap(doc, "    storey: String = String::new();\n    cut_height: f64 = 1.2;\n", "    view: String = String::new();\n");
    }
    const doc = swap(text, "//! 🎚️ Persisted local state for one exact BIM section window: the section line in plan coordinates, how deep it looks and its navigation.", "//! 🎚️ Persisted local state for one exact BIM section window: which section or elevation view it shows and its navigation. The plane and the depth are authored in the view.");
    return swap(doc, "    start_x: f64 = 0.0;\n    start_y: f64 = 0.0;\n    end_x: f64 = 10.0;\n    end_y: f64 = 0.0;\n    depth: f64 = 5.0;\n", "    view: String = String::new();\n");
  });
  edit(join(schema, TSF), (text) => {
    if (kind === "Plan") {
      let out = swap(text, "/** 🎚️ Persisted navigation and storey of one exact BIM plan window. */", "/** 🎚️ Persisted plan view and navigation of one exact BIM plan window. */");
      out = swap(out, "  storey: string;\n  cutHeight: number;\n", "  view: string;\n");
      out = swap(out, '["storey", "cutHeight", "framed", "viewport"]', '["view", "framed", "viewport"]');
      return swap(out, '    storey: text(row.storey, "$.storey"),\n    cutHeight: num(row.cutHeight, "$.cutHeight"),\n', '    view: text(row.view, "$.view"),\n');
    }
    let out = swap(text, "/** 🎚️ Persisted section line, depth and navigation of one exact BIM section window. */", "/** 🎚️ Persisted section or elevation view and navigation of one exact BIM section window. */");
    out = swap(out, "  startX: number;\n  startY: number;\n  endX: number;\n  endY: number;\n  depth: number;\n", "  view: string;\n");
    out = swap(out, '["startX", "startY", "endX", "endY", "depth", "framed", "viewport"]', '["view", "framed", "viewport"]');
    return swap(out, '    startX: num(row.startX, "$.startX"),\n    startY: num(row.startY, "$.startY"),\n    endX: num(row.endX, "$.endX"),\n    endY: num(row.endY, "$.endY"),\n    depth: num(row.depth, "$.depth"),\n', '    view: text(row.view, "$.view"),\n');
  });
  edit(join(schema, JSONF), (text) => {
    const doc = JSON.parse(text);
    const gone = kind === "Plan" ? ["storey", "cutHeight"] : ["startX", "startY", "endX", "endY", "depth"];
    const properties: Record<string, unknown> = { view: { type: "string" } };
    for (const [key, value] of Object.entries(doc.properties)) if (!gone.includes(key) && key !== "view") properties[key] = value;
    doc.properties = properties;
    doc.required = ["view", ...doc.required.filter((key: string) => !gone.includes(key) && key !== "view")];
    return JSON.stringify(doc, null, 2) + "\n";
  });
  edit(join(schema, GQLF), (text) => (kind === "Plan" ? swap(text, "  storey: String!\n  cutHeight: Float!\n", "  view: String!\n") : swap(text, "  startX: Float!\n  startY: Float!\n  endX: Float!\n  endY: Float!\n  depth: Float!\n", "  view: String!\n")));
  edit(join(schema, PROTOF), (text) => {
    const fields = kind === "Plan" ? /  string storey = 1;\n  double cut_height = 2;\n  bool framed = 3;\n([^]*)viewport = 4;/ : /  double start_x = 1;\n  double start_y = 2;\n  double end_x = 3;\n  double end_y = 4;\n  double depth = 5;\n  bool framed = 6;\n([^]*)viewport = 7;/;
    return text.replace(fields, "  string view = 1;\n  bool framed = 2;\n$1viewport = 3;");
  });
}

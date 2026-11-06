const fs = require("node:fs");
const Ajv = require("ajv/dist/2020");

const root = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧫️retained-params/";
const ajv = new Ajv({ strict: true });
const read = name => JSON.parse(fs.readFileSync(root + name, "utf8"));
const cases = read("🧪️.json");
if (!ajv.validate(read("🔣️.json"), cases)) throw Error(ajv.errorsText());
const rules = {
  ingest: { frame_sample_stride: { type: "integer", minimum: 1 } },
  feature: { target_count: { type: "integer", minimum: 1 } },
  matching: { sequential_window: { type: "integer", minimum: 0 }, ratio_test: { type: "number", exclusiveMinimum: 0, maximum: 1 } },
  sfm: { ransac_iterations: { type: "integer", minimum: 0 }, ransac_threshold_px: { type: "number" } },
  dense: { max_points: { type: "integer", minimum: 0 }, confidence_threshold: { type: "number" } },
  mesh: { texture_size: { type: "integer", minimum: 0 }, tsdf_voxel_size_mm: { type: "number" } },
  motion: { enabled: { type: "boolean" }, min_track_quality: { type: "number" } },
  geo: { enabled: { type: "boolean" }, gsd_m: { type: "number", exclusiveMinimum: 0 } },
};
const defaults = { frame_sample_stride: 5, target_count: 4000, sequential_window: 8, ratio_test: 0.8, ransac_iterations: 1000, ransac_threshold_px: 2, max_points: 500000, confidence_threshold: 0.5, texture_size: 2048, tsdf_voxel_size_mm: 5, enabled: false, min_track_quality: 0.3, gsd_m: 0.05 };
const validators = Object.fromEntries(Object.entries(rules).map(([facet, properties]) => [facet, ajv.compile({ type: "object", properties, additionalProperties: false })]));
const seen = new Set();
for (const row of cases) {
  const key = `${row.facet}:${row.expected}`;
  if (seen.has(key)) throw Error(`duplicate ${key}`);
  seen.add(key);
  const actual = !validators[row.facet](row.change) ? "refused" : Object.entries(row.change).every(([field, value]) => defaults[field] === value) ? "unchanged" : "admitted";
  if (actual !== row.expected) throw Error(`${key}: got ${actual}`);
}
if (seen.size !== 24) throw Error("incomplete facet/outcome matrix");
const messages = read("💬️.json");
if (!ajv.validate(read("💬️🔣️.json"), messages)) throw Error(ajv.errorsText());
for (const row of messages) {
  const text = row.facet === "ingest" ? `Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness=${row.a}, max_frames=${row.b}, frame_sample_stride=${row.c}).` : row.facet === "feature" ? `Feature params need a positive target count and a finite non-negative edge threshold (got target_count=${row.a}, edge_threshold=${row.b}).` : `Match ratio test ${row.a} must be finite and within (0, 1].`;
  if (text !== row.message) throw Error(`message mismatch for ${JSON.stringify(row)}`);
}
console.log(`24 parameter decision vectors and ${messages.length} numeric messages agree with Ajv and independent formatting`);

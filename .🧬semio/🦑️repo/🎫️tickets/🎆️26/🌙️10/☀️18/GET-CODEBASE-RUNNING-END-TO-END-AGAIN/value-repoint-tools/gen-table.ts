import { readFileSync, writeFileSync } from "node:fs";

const ROOT = "/Users/ueli/Documents/semio";
const V = `${ROOT}/🧰️framework/🔨️modules/🌱️value`;
const files = [`${V}/🦀️.rs`, `${V}/📦️packages/🦀️rust/🦀️.rs`];
const MODULE_OWNERS: Record<string, string> = {
  fixed_key_slots: "semio_framework_value_registry",
  fixed_request_owners: "semio_framework_value_registry",
  exact_key_lookup: "semio_framework_value_registry",
  list: "semio_framework_value_persistent",
  retained_equality: "semio_framework_value_persistent",
  original_vector_concat: "semio_framework_value_persistent",
  slice_equality: "semio_framework_value_persistent",
  sort: "semio_framework_value_edit",
  indexed_edit: "semio_framework_value_edit",
  indexed_collection_commit: "semio_framework_value_edit",
  named_edit: "semio_framework_value_edit",
  graphql_int64: "semio_framework_value_schema",
  intrinsic_json: "semio_framework_value_schema",
  native_decoding: "semio_framework_value_native_owned",
};
const FUNCTIONS: Record<string, string> = { register_value_schema_exports: "semio_framework_value_schema" };
const items: Record<string, string> = {};
for (const f of files) {
  const text = readFileSync(f, "utf8");
  for (const m of text.matchAll(/pub use\s+([a-z_]+)\s*::\s*(?:\{([^}]*)\}|([A-Za-z0-9_]+))\s*;/g)) {
    if (!(m[1] in MODULE_OWNERS)) continue;
    for (const raw of (m[2] ?? m[3]).split(",")) {
      const n = raw.trim();
      if (!n) continue;
      if (items[n] && items[n] !== m[1]) throw new Error(`flat item ${n} claimed by ${items[n]} and ${m[1]}`);
      items[n] = m[1];
    }
  }
}
const out = { modules: MODULE_OWNERS, functions: FUNCTIONS, items, deriveMacroNames: ["NativeReceive", "RetainedEquality"] };
writeFileSync(`${import.meta.dir}/moved.json`, JSON.stringify(out, null, 1) + "\n");
console.log(Object.keys(MODULE_OWNERS).length, "modules", Object.keys(items).length, "flat items");

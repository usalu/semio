#!/usr/bin/env node
/** 🏷️ WP-18: the eight `delete-*-type` leaves also delete the properties and classifications keyed by the type (data belongs to its holder) and their inverses restore them. Idempotent. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { mutations } from "./r3-f1-paths.ts";

const kinds = ["wall", "slab", "roof", "ceiling", "column", "beam", "window", "door"].map((name) => `delete-${name}-type`);
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((n) => n.endsWith(suffix)));
for (const kind of kinds) {
  const leaf = join(mutations, readdirSync(mutations).find((n) => n.endsWith(kind)));
  const diffPath = join(sub(leaf, "diff"), readdirSync(sub(leaf, "diff")).find((n) => n.endsWith(".rs")));
  let diff = readFileSync(diffPath, "utf8");
  if (!diff.includes("cascade::data_diff")) {
    diff = diff.replace(/    MutationOutcome::new\(ModelDiff::(\w+)\(payload\.id\.clone\(\), Entry::Deleted\)\)/, (_, collection) => `    let mut removal = cascade::data_diff(base, &payload.id);\n    removal.${collection} = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));\n    MutationOutcome::new(removal)`);
    diff = diff.replace("use crate::{Entry,", "use crate::{Entry, KeyedDelta,").replace("use crate::{CurtainPanel, Entry,", "use crate::{CurtainPanel, Entry, KeyedDelta,");
    diff = diff.replace(/(use super::Delete\w+;\n)/, "use super::super::cascade;\n$1");
    diff = diff.replace(/^(\/\/! 🔺️ Diff constructor for `\w+`: one deleted [a-z ]+ entry)/m, "$1 together with the properties and classifications keyed by the type");
    writeFileSync(diffPath, diff);
  }
  const inversePath = join(sub(leaf, "inverse"), readdirSync(sub(leaf, "inverse")).find((n) => n.endsWith(".rs")));
  let inverse = readFileSync(inversePath, "utf8");
  if (!inverse.includes("cascade::data_rows")) {
    inverse = inverse.replace(/Some\((\w+)\) => vec!\[(ModelMutation::Create\w+\(.*\))\],/, "Some($1) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once($2)).collect(),");
    inverse = inverse.replace(/(use super::Delete\w+;\n)/, "use super::super::cascade;\n$1");
    inverse = inverse.replace(/^(\/\/! ↩️ Inverse of `\w+`: )/m, "$1the setters of the properties and classifications of the type, then ");
    writeFileSync(inversePath, inverse);
  }
  const schemaPath = join(sub(leaf, "schema"), readdirSync(sub(leaf, "schema")).find((n) => n.endsWith(".json")));
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  if (!schema["x-semio-inverse-rows"]) {
    const next = {};
    for (const [key, value] of Object.entries(schema)) {
      next[key] = value;
      if (key === "required") next["x-semio-inverse-rows"] = { bounded: 4096 };
    }
    writeFileSync(schemaPath, JSON.stringify(next, null, 2) + "\n");
  }
  console.log(kind, "ok");
}

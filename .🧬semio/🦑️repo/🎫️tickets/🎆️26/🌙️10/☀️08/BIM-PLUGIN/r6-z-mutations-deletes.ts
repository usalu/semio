/**
 * 🗑️ Routes the ten single-element delete leaves through the shared cascade (`🌊️cascade`): the element leaves together with its
 * properties and classifications (and a curtain wall with the openings it hosts), and the inverse replays one concrete create plus one
 * data setter per removed entry. Rewrites each leaf's `🔺️diff`, `↩️inverse`, doc line and payload schema, then adds the data case.
 * Usage: `bun r6-z-mutations-deletes.ts [--cases]`.
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { mutations } from "./r3-f1-paths.ts";
import { addCase, appliedCases, caseDirs, leafDirName, read, removeCase, variantOf } from "./r6-z-mutations-cases.ts";
import { field, object, parse, print, raw, string_, type Node } from "./r6-z-mutations-rawjson.ts";

type Spec = { kind: string; collection: string; noun: string; plural?: string };
const SPECS: Spec[] = [
  { kind: "delete-beam", collection: "beams", noun: "Beam" },
  { kind: "delete-column", collection: "columns", noun: "Column" },
  { kind: "delete-slab", collection: "slabs", noun: "Slab" },
  { kind: "delete-roof", collection: "roofs", noun: "Roof" },
  { kind: "delete-stair", collection: "stairs", noun: "Stair" },
  { kind: "delete-railing", collection: "railings", noun: "Railing" },
  { kind: "delete-space", collection: "spaces", noun: "Space" },
  { kind: "delete-opening", collection: "openings", noun: "Opening" },
  { kind: "delete-curtain-wall", collection: "curtain_walls", noun: "Curtain wall" },
  { kind: "delete-grid-line", collection: "grids", noun: "Grid line" },
];

const sub = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
const lower = (noun: string) => noun.toLowerCase();

function rewrite(spec: Spec) {
  const leaf = join(mutations, leafDirName(spec.kind));
  const name = variantOf(spec.kind);
  const cascadesOpenings = spec.kind === "delete-curtain-wall";
  const what = cascadesOpenings ? "the curtain wall and the openings it hosts leave in one sparse diff together with" : `the ${lower(spec.noun)} leaves in one sparse diff together with`;
  const diffDir = sub(leaf, "diff");
  writeFileSync(
    join(diffDir, readdirSync(diffDir)[0]),
    `//! 🔺️ Diff constructor for \`${name}\`: ${what} its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the ${lower(spec.noun)} leaves.

use super::super::cascade;
use super::${name};
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${name}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${spec.collection}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${spec.noun} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "${spec.noun}", Some(&payload.id))
}
`,
  );
  const inverseDir = sub(leaf, "inverse");
  writeFileSync(
    join(inverseDir, readdirSync(inverseDir)[0]),
    `//! ↩️ Inverse of \`${name}\`: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::${name};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${name}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  );
  const mutationDir = sub(leaf, "mutation");
  const mutationFile = join(mutationDir, readdirSync(mutationDir)[0]);
  let source = readFileSync(mutationFile, "utf8").replaceAll("\r\n", "\n");
  const doc = `Removes ${cascadesOpenings ? "a curtain wall together with the openings it hosts and the properties and classifications of both" : `a ${lower(spec.noun)} together with its properties and classifications`}.`;
  source = source.replace(/(\/\/! \S+ `[a-z-]+` payload\. )[^\n]*/, `$1${doc}`);
  if (cascadesOpenings) {
    source = source.replace('&format!("Delete curtain wall \\"{}\\"", self.id), &format!("Vorhangfassade \\"{}\\" löschen", self.id)', '&format!("Delete curtain wall \\"{}\\" with its openings", self.id), &format!("Vorhangfassade \\"{}\\" samt Öffnungen löschen", self.id)');
  }
  writeFileSync(mutationFile, source);
  const schemaDir = sub(leaf, "schema");
  const schemaFile = join(schemaDir, readdirSync(schemaDir)[0]);
  const schema = parse(readFileSync(schemaFile, "utf8"));
  if (schema.k === "obj" && !field(schema, "x-semio-inverse-rows")) {
    const at = schema.v.findIndex(([key]) => key === "required") + 1;
    schema.v.splice(at, 0, ["x-semio-inverse-rows", object(["bounded", raw("4096")])]);
    writeFileSync(schemaFile, print(schema) + "\n");
  }
}

const dataSets = (id: string): [string, Node][] => [
  ["properties", object([id, object(["Pset_Removal", object(["Marker", object(["Text", object(["value", string_("kept in the inverse")])])])])])],
  ["classifications", object([id, object(["system", string_("DIN 276")], ["code", string_("340")], ["title", string_("Wände")])])],
];

function withData(beforeText: string, id: string): string {
  const snapshot = parse(beforeText);
  if (snapshot.k !== "obj") throw new Error("snapshot is no object");
  for (const [key, value] of dataSets(id)) {
    const existing = snapshot.v.find(([name]) => name === key);
    if (existing && existing[1].k === "obj") existing[1].v = [...existing[1].v.filter(([entry]) => entry !== id), ...(value as { v: [string, Node][] }).v];
    else if (existing) existing[1] = value;
    else snapshot.v.push([key, value]);
  }
  return print(snapshot) + "\n";
}

if (!process.argv.includes("--cases")) {
  for (const spec of SPECS) rewrite(spec);
  console.log(`rewrote ${SPECS.length} delete leaves`);
} else {
  for (const spec of SPECS) {
    const applied = appliedCases(spec.kind);
    if (spec.kind === "delete-curtain-wall") {
      const hosted = caseDirs(spec.kind).find((dir) => dir.endsWith("hosts-an-opening"));
      if (hosted) {
        const beforeText = read(spec.kind, hosted, "before");
        const mutationText = read(spec.kind, hosted, "mutation");
        removeCase(spec.kind, hosted);
        addCase(spec.kind, "cascades-the-opening", { beforeText, mutationText, outcome: { status: "applied" } });
      }
    }
    const base = applied.find((dir) => dir.endsWith("removes")) ?? applied[0];
    const mutationText = read(spec.kind, base, "mutation");
    const id = JSON.parse(mutationText).id as string;
    addCase(spec.kind, "removes-its-data", { beforeText: withData(read(spec.kind, base, "before"), id), mutationText, outcome: { status: "applied" } });
    console.log(`${spec.kind}: cases added`);
  }
}

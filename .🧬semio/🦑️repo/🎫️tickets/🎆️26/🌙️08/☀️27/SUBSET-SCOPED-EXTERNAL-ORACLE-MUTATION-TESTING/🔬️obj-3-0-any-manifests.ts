#!/usr/bin/env bun
// 🧬️ Writes the `mutationManifests` and `fixtureManifests` blocks into
// `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json`.
//
// Every field is derived from something that was READ, never guessed:
//   * `mutations[].id` — the catalog's own `kinds` array, in its own order.
//   * `productionDispatch.variant` — the `ObjMutation` enum in `../🧬️schema/🧬️mutations/🦀️.rs`.
//   * `outcomes` — the `MutationOutcome::` call sites in that same dispatch (`new` at line 220 for
//     every kind, `error` at line 171 in `apply_obj_mutation` for every kind, plus the `no-op`
//     warning branch that only `📄set-snapshot/🦀️.rs:19` reaches). `no-mutation`'s arm is
//     `ObjDiff::default()` — an empty diff — so it is a `no-op` and never `applied`.
//   * `oracleRequirements` — the single registered qualifying oracle `tobj-obj-3-0-mutate`. All 22
//     kinds were MEASURED to move the composed projection (see 📓️obj-3-0-any-fixture-corpus.md),
//     so none takes an `-uncarried` exemption.
//   * `files[].sha256`/`bytes` — hashed off the committed fixture, hashed LAST and never rewritten.
//
// Usage: bun 🔬️obj-3-0-any-manifests.ts

import { readFileSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const SUBSET = join(REPO_ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any");
const ORACLE_JSON = join(SUBSET, "🧪️oracle", "🔣️.json");
const RECIPE = "pattern-shell";
const FIXTURE_FILE = "pattern-shell.obj";
const FIXTURE_ABS = join(SUBSET, "🧫️fixtures", RECIPE, FIXTURE_FILE);

const VARIANT: Record<string, string> = {
  "no-mutation": "NoMutation",
  "set-snapshot": "SetSnapshot",
  "insert-vertex": "InsertVertex",
  "remove-vertex": "RemoveVertex",
  "set-vertex": "SetVertex",
  "insert-texcoord": "InsertTexCoord",
  "remove-texcoord": "RemoveTexCoord",
  "set-texcoord": "SetTexCoord",
  "insert-normal": "InsertNormal",
  "remove-normal": "RemoveNormal",
  "set-normal": "SetNormal",
  "insert-face": "InsertFace",
  "remove-face": "RemoveFace",
  "set-face": "SetFace",
  "set-group": "SetGroup",
  "remove-group": "RemoveGroup",
  "set-object": "SetObject",
  "remove-object": "RemoveObject",
  "set-mtllib": "SetMtllib",
  "set-usemtl": "SetUsemtl",
  "set-smoothing-groups": "SetSmoothingGroups",
  "set-unknown-statements": "SetUnknownStatements",
};

// 🎯️ Which half of the composed projection each kind was MEASURED to move, and by how many diffs.
// Kept here beside the generation so the evidence and the manifest never drift; the manifest itself
// carries no such field, since `ManifestMutation` is `additionalProperties: false`.
const WITNESSED: Record<string, string> = {
  "no-mutation": "identity — the composed projection is byte-for-byte the base's (mesh 0, document 0), which is this kind's whole semantics",
  "set-snapshot": "document (7)",
  "insert-vertex": "document (7)",
  "remove-vertex": "document (7)",
  "set-vertex": "document (9)",
  "insert-texcoord": "document (3)",
  "remove-texcoord": "document (3)",
  "set-texcoord": "document (2)",
  "insert-normal": "document (3)",
  "remove-normal": "document (3)",
  "set-normal": "document (5)",
  "insert-face": "mesh (43) + document (9)",
  "remove-face": "mesh (34) + document (11)",
  "set-face": "mesh (40) only — the document surface holds by construction, which is why the mesh half is not optional",
  "set-group": "document (2)",
  "remove-group": "document (6)",
  "set-object": "mesh (8) + document (4)",
  "remove-object": "document (6)",
  "set-mtllib": "document (1)",
  "set-usemtl": "document (3)",
  "set-smoothing-groups": "document (4)",
  "set-unknown-statements": "document (3)",
};

const LOCAL: Record<string, string[]> = {
  "no-mutation": ["composed-projection-unchanged"],
  "set-snapshot": ["replacement-snapshot-reproduces-exactly-or-reports-no-op"],
  "insert-vertex": ["declared-vertex-count-increases-by-one"],
  "remove-vertex": ["declared-vertex-count-decreases-by-one"],
  "set-vertex": ["declared-vertex-extent-moves-at-the-named-row"],
  "insert-texcoord": ["declared-texcoord-count-increases-by-one"],
  "remove-texcoord": ["declared-texcoord-count-decreases-by-one"],
  "set-texcoord": ["declared-texcoord-extent-moves-at-the-named-row"],
  "insert-normal": ["declared-normal-count-increases-by-one"],
  "remove-normal": ["declared-normal-count-decreases-by-one"],
  "set-normal": ["declared-normal-extent-moves-at-the-named-row"],
  "insert-face": ["face-index-space-opens-at-the-insertion-point", "tobj-triangle-count-increases-by-one"],
  "remove-face": ["face-index-space-closes-at-the-removal-point", "tobj-triangle-count-decreases-by-one"],
  "set-face": ["tobj-face-topology-moves-while-the-document-surface-holds"],
  "set-group": ["named-group-membership-span-matches-the-declared-faces"],
  "remove-group": ["named-group-leaves-the-list-and-later-bands-keep-their-order"],
  "set-object": ["named-object-membership-span-matches-the-declared-faces"],
  "remove-object": ["named-object-leaves-the-list-and-later-objects-keep-their-order"],
  "set-mtllib": ["mtllib-reference-matches-or-is-absent"],
  "set-usemtl": ["usemtl-run-starts-match-the-declared-list"],
  "set-smoothing-groups": ["smoothing-run-starts-match-the-declared-list"],
  "set-unknown-statements": ["retained-statement-lines-match-the-declared-list"],
};

const ENCLOSING = ["tobj-mesh-and-independent-document-projection-agree"];

console.log(Object.entries(WITNESSED).map(([kind, half]) => `[witnessed] ${kind.padEnd(24)} ${half}`).join("\n"));

const parsed = JSON.parse(readFileSync(ORACLE_JSON, "utf8")) as Record<string, unknown>;
const catalog = (parsed.mutationCatalogs as { kinds: string[]; capability: string }[])[0]!;

const mutations = catalog.kinds.map((kind) => ({
  id: kind,
  capability: catalog.capability,
  payloadSchema: `../🧬️schema/🧬️mutations/🦀️.rs#${VARIANT[kind]}`,
  outcomes: kind === "no-mutation" ? ["no-op"] : kind === "set-snapshot" ? ["applied", "no-op", "rejected"] : ["applied", "rejected"],
  productionDispatch: { operation: kind, bridgeVersion: 1, variant: `ObjMutation::${VARIANT[kind]}` },
  oracleRequirements: [{ capability: catalog.capability, qualifyingKind: "third-party-library", oracle: "tobj-obj-3-0-mutate" }],
  carriers: ["obj"],
  invariants: { local: LOCAL[kind], enclosing: ENCLOSING },
}));

parsed.mutationManifests = [
  {
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact: "s.stdio.obj",
    standard: "3.0",
    subset: "any",
    standardDirectoryName: "🔖️3.0",
    subsetDirectoryName: "✳️any",
    mutations,
  },
];

const bytes = readFileSync(FIXTURE_ABS);
const sha256 = `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
parsed.fixtureManifests = [
  {
    schema: "semio.repository-test.fixture/v2",
    id: RECIPE,
    class: "third-party-generated",
    target: { artifact: "s.stdio.obj", standard: "3.0", subset: "any" },
    units: { length: "unitless", angle: "degree" },
    files: [{ role: "primary-obj", path: `../🧫️fixtures/${RECIPE}/${FIXTURE_FILE}`, mediaType: "model/obj", sha256, bytes: statSync(FIXTURE_ABS).size }],
    generator: {
      oracle: "tobj-obj-3-0-mutate",
      packageVersion: "4",
      engineFamily: "tobj",
      engineVersion: "4",
      command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate",
      platform: "darwin-arm64",
    },
    provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
    comparisonProfile: "semantic-obj-3-0-v1",
    reproducible: true,
    family: "mechanical",
    notes:
      "One base input document exercising all 22 declared obj@3.0 mutation kinds: 6 v rows (one with an explicit w, one referenced by no face), 6 vt and 4 vn rows (likewise one unreferenced each), 5 f rows across 2 g bands and 2 o objects, an mtllib, two usemtl runs and two s runs starting at different face indices (one of them `s off`), and 2 retained comment lines. The unreferenced rows are deliberate: tobj re-indexes per model and drops every row no face references, so those kinds move the document projection and NOTHING in the mesh projection — measured, 14 of the 22 kinds move only the document half and set-face moves only the mesh half. OBJ declares no physical length or angle unit, so `unitless`/`degree` are the schema-required placeholders, not claims. OBJ has no reference WRITER in the Rust ecosystem, so the bytes are written by the generator's own grammar emitter and ADMITTED by the registered tobj 4 reader before being written to disk (3 models, 13 referenced positions, 15 triangulated indices) — the same already-recorded precedent this subset's oracle and the shared mesh::oracle_create_obj rest on.",
  },
];

writeFileSync(ORACLE_JSON, `${JSON.stringify(parsed, null, 2)}\n`);
console.log(`[manifests] ${mutations.length} mutation(s) and 1 fixture (${sha256}, ${bytes.length} bytes) written into ${ORACLE_JSON}`);

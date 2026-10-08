/**
 * ✂️ Gives the 14 whole-payload `set-*` leaves the minimal-patch shape: the payload converts itself to its entity patch (`patch`) and
 * back (`from_patch`), the diff keeps `payload.patch().minimal(record)` and the inverse restores `payload.patch().minimal(record)
 * .negate(record)`, so a restated value never reaches the diff or the inverse. Rewrites each leaf's `🦠️mutation` (adds the two methods)
 * and `↩️inverse` (whole file); the diffs are edited by hand. Idempotent.
 */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { mutations } from "./r3-f1-paths.ts";

type Spec = { kind: string; patch: string; collection?: string; singleton?: boolean };
export const SPECS: Spec[] = [
  { kind: "set-material", patch: "MaterialPatch", collection: "materials" },
  { kind: "set-wall-type", patch: "WallTypePatch", collection: "wall_types" },
  { kind: "set-slab-type", patch: "SlabTypePatch", collection: "slab_types" },
  { kind: "set-roof-type", patch: "RoofTypePatch", collection: "roof_types" },
  { kind: "set-column-type", patch: "ColumnTypePatch", collection: "column_types" },
  { kind: "set-beam-type", patch: "BeamTypePatch", collection: "beam_types" },
  { kind: "set-window-type", patch: "WindowTypePatch", collection: "window_types" },
  { kind: "set-door-type", patch: "DoorTypePatch", collection: "door_types" },
  { kind: "set-building", patch: "BuildingPatch", collection: "buildings" },
  { kind: "set-site", patch: "SitePatch", collection: "sites" },
  { kind: "set-grid-line", patch: "GridLinePatch", collection: "grids" },
  { kind: "set-railing", patch: "RailingPatch", collection: "railings" },
  { kind: "set-space", patch: "SpacePatch", collection: "spaces" },
  { kind: "set-project-info", patch: "ProjectPatch", singleton: true },
  { kind: "set-curtain-wall", patch: "CurtainWallPatch", collection: "curtain_walls" },
];

const pascal = (kind: string) => kind.split("-").map((word) => word[0].toUpperCase() + word.slice(1)).join("");
const dirOf = (kind: string) => join(mutations, readdirSync(mutations).find((name) => name.endsWith(kind) && name.slice(0, -kind.length).length > 0 && !name.slice(0, -kind.length).includes("-"))!);
const sub = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
const read = (path: string) => readFileSync(path, "utf8").replaceAll("\r\n", "\n");

for (const spec of SPECS) {
  const leaf = dirOf(spec.kind);
  const name = pascal(spec.kind);
  const mutationFile = join(sub(leaf, "mutation"), readdirSync(sub(leaf, "mutation"))[0]);
  let source = read(mutationFile);
  const struct = source.match(/pub struct \w+ \{([\s\S]*?)\n\}/)![1];
  const fields = [...struct.matchAll(/pub (\w+): /g)].map((match) => match[1]).filter((field) => field !== "id");
  const crate = source.match(/use crate::\{([^}]*)\};/)!;
  if (!source.includes(`pub fn patch(&self)`)) {
    const names = [...new Set([...crate[1].split(",").map((item) => item.trim()), spec.patch])].sort();
    source = source.replace(crate[0], `use crate::{${names.join(", ")}};`);
    const hasId = !spec.singleton;
    const impl = `impl ${name} {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ${spec.patch} {
        ${spec.patch} { ${fields.map((field) => `${field}: self.${field}.clone()`).join(", ")}, ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(${hasId ? "id: String, " : ""}patch: ${spec.patch}) -> Self {
        Self { ${hasId ? "id, " : ""}${fields.map((field) => `${field}: patch.${field}`).join(", ")} }
    }
}

`;
    source = source.replace(/impl MutationKind<ModelSnapshot, ModelMutation> for/, impl + "impl MutationKind<ModelSnapshot, ModelMutation> for");
    writeFileSync(mutationFile, source);
  }
  const inverseDir = sub(leaf, "inverse");
  const inverseFile = join(inverseDir, readdirSync(inverseDir)[0]);
  const lookup = spec.singleton ? "&base.project" : `base.${spec.collection}.get(&payload.id)`;
  const noun = spec.kind.replace("set-", "").replaceAll("-", " ");
  const inverse = spec.singleton
    ? `//! ↩️ Inverse of \`${name}\`: one \`${name}\` restoring the base value of exactly the fields the forward really changes, none when nothing changes.

use super::${name};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &${name}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let restore = payload.patch().minimal(&base.project).negate(&base.project);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::${name}(${name}::from_patch(restore))]
}
`
    : `//! ↩️ Inverse of \`${name}\`: an absolute \`${name}\` restoring the base value of exactly the fields the forward really changes, none when the ${noun} is absent or nothing changes.

use super::${name};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &${name}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = ${lookup} else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::${name}(${name}::from_patch(payload.id.clone(), restore))]
}
`;
  writeFileSync(inverseFile, inverse);
  console.log(`${spec.kind}: ${fields.join(", ")}`);
}

/** 🛤 `change-support-collision-location` wire twin: the leaf payload `ChangeSupportCollisionLocation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportCollisionLocation {
  id: string;
  newLocation: string;
}

export const parseChangeSupportCollisionLocation: NormWireReader<ChangeSupportCollisionLocation> = normWireObject<ChangeSupportCollisionLocation>({ id: normWireRequired(normWireString), newLocation: normWireRequired(normWireString) });

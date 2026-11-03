/** ⚙ `change-support-collision-height` wire twin: the leaf payload `ChangeSupportCollisionHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportCollisionHeight {
  id: string;
  newHeight: number;
}

export const parseChangeSupportCollisionHeight: NormWireReader<ChangeSupportCollisionHeight> = normWireObject<ChangeSupportCollisionHeight>({ id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });

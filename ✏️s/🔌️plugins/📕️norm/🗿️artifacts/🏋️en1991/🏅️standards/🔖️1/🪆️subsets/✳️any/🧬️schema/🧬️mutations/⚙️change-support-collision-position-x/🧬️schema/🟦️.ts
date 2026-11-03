/** ⚙ `change-support-collision-position-x` wire twin: the leaf payload `ChangeSupportCollisionPositionX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportCollisionPositionX {
  id: string;
  newPositionX: number;
}

export const parseChangeSupportCollisionPositionX: NormWireReader<ChangeSupportCollisionPositionX> = normWireObject<ChangeSupportCollisionPositionX>({ id: normWireRequired(normWireString), newPositionX: normWireRequired(normWireNumber) });

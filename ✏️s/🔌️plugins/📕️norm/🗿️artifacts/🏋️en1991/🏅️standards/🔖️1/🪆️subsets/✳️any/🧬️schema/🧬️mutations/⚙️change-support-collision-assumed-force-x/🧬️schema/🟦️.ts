/** ⚙ `change-support-collision-assumed-force-x` wire twin: the leaf payload `ChangeSupportCollisionAssumedForceX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportCollisionAssumedForceX {
  id: string;
  newAssumedForceX: number;
}

export const parseChangeSupportCollisionAssumedForceX: NormWireReader<ChangeSupportCollisionAssumedForceX> = normWireObject<ChangeSupportCollisionAssumedForceX>({ id: normWireRequired(normWireString), newAssumedForceX: normWireRequired(normWireNumber) });

/** ⚙ `change-support-collision-assumed-force-y` wire twin: the leaf payload `ChangeSupportCollisionAssumedForceY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportCollisionAssumedForceY {
  id: string;
  newAssumedForceY: number;
}

export const parseChangeSupportCollisionAssumedForceY: NormWireReader<ChangeSupportCollisionAssumedForceY> = normWireObject<ChangeSupportCollisionAssumedForceY>({ id: normWireRequired(normWireString), newAssumedForceY: normWireRequired(normWireNumber) });

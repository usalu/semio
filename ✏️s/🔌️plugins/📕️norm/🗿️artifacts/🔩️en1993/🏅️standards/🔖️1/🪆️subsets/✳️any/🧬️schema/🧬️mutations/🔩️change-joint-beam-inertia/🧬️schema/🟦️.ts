/** 🔩️ `change-joint-beam-inertia` wire twin: the leaf payload `ChangeJointBeamInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamInertia {
  id: string;
  newBeamInertia: number;
}

export const parseChangeJointBeamInertia: NormWireReader<ChangeJointBeamInertia> = normWireObject<ChangeJointBeamInertia>({ id: normWireRequired(normWireString), newBeamInertia: normWireRequired(normWireNumber) });

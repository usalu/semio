/** 🔩️ `change-joint-beam-flange-thickness` wire twin: the leaf payload `ChangeJointBeamFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamFlangeThickness {
  id: string;
  newBeamFlangeThickness: number;
}

export const parseChangeJointBeamFlangeThickness: NormWireReader<ChangeJointBeamFlangeThickness> = normWireObject<ChangeJointBeamFlangeThickness>({ id: normWireRequired(normWireString), newBeamFlangeThickness: normWireRequired(normWireNumber) });

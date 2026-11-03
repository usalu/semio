/** 🔩️ `change-joint-beam-web-thickness` wire twin: the leaf payload `ChangeJointBeamWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamWebThickness {
  id: string;
  newBeamWebThickness: number;
}

export const parseChangeJointBeamWebThickness: NormWireReader<ChangeJointBeamWebThickness> = normWireObject<ChangeJointBeamWebThickness>({ id: normWireRequired(normWireString), newBeamWebThickness: normWireRequired(normWireNumber) });

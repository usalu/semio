/** 🔩️ `change-joint-beam-span` wire twin: the leaf payload `ChangeJointBeamSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamSpan {
  id: string;
  newBeamSpan: number;
}

export const parseChangeJointBeamSpan: NormWireReader<ChangeJointBeamSpan> = normWireObject<ChangeJointBeamSpan>({ id: normWireRequired(normWireString), newBeamSpan: normWireRequired(normWireNumber) });

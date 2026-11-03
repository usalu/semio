/** 🔩️ `change-joint-beam-web-fy` wire twin: the leaf payload `ChangeJointBeamWebFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamWebFy {
  id: string;
  newBeamWebFy: number;
}

export const parseChangeJointBeamWebFy: NormWireReader<ChangeJointBeamWebFy> = normWireObject<ChangeJointBeamWebFy>({ id: normWireRequired(normWireString), newBeamWebFy: normWireRequired(normWireNumber) });

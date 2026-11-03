/** 🔩️ `change-joint-anchor-length` wire twin: the leaf payload `ChangeJointAnchorLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointAnchorLength {
  id: string;
  newAnchorLength: number;
}

export const parseChangeJointAnchorLength: NormWireReader<ChangeJointAnchorLength> = normWireObject<ChangeJointAnchorLength>({ id: normWireRequired(normWireString), newAnchorLength: normWireRequired(normWireNumber) });

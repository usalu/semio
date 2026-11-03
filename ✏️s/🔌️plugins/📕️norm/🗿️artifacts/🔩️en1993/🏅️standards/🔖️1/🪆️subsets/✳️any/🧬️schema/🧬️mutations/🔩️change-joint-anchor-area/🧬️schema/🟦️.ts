/** 🔩️ `change-joint-anchor-area` wire twin: the leaf payload `ChangeJointAnchorArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointAnchorArea {
  id: string;
  newAnchorArea: number;
}

export const parseChangeJointAnchorArea: NormWireReader<ChangeJointAnchorArea> = normWireObject<ChangeJointAnchorArea>({ id: normWireRequired(normWireString), newAnchorArea: normWireRequired(normWireNumber) });

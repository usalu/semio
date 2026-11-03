/** 🔩️ `change-joint-column-web-shear-area` wire twin: the leaf payload `ChangeJointColumnWebShearArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnWebShearArea {
  id: string;
  newColumnWebShearArea: number;
}

export const parseChangeJointColumnWebShearArea: NormWireReader<ChangeJointColumnWebShearArea> = normWireObject<ChangeJointColumnWebShearArea>({ id: normWireRequired(normWireString), newColumnWebShearArea: normWireRequired(normWireNumber) });

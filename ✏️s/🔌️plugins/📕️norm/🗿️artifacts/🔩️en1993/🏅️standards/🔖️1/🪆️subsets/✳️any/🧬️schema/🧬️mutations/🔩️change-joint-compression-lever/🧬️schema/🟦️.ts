/** 🔩️ `change-joint-compression-lever` wire twin: the leaf payload `ChangeJointCompressionLever`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointCompressionLever {
  id: string;
  newCompressionLever: number;
}

export const parseChangeJointCompressionLever: NormWireReader<ChangeJointCompressionLever> = normWireObject<ChangeJointCompressionLever>({ id: normWireRequired(normWireString), newCompressionLever: normWireRequired(normWireNumber) });

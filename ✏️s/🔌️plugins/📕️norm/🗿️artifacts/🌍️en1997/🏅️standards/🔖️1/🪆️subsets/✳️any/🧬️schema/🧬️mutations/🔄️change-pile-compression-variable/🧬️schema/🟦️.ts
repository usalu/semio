/** 🔄️ `change-pile-compression-variable` wire twin: the leaf payload `ChangePileCompressionVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileCompressionVariable {
  mutation: "changePileCompressionVariable";
  id: string;
  newCompressionVariable: number;
}

export const parseChangePileCompressionVariable: NormWireReader<ChangePileCompressionVariable> = normWireObject<ChangePileCompressionVariable>({ mutation: normWireRequired(normWireLiteral("changePileCompressionVariable")), id: normWireRequired(normWireString), newCompressionVariable: normWireRequired(normWireNumber) });

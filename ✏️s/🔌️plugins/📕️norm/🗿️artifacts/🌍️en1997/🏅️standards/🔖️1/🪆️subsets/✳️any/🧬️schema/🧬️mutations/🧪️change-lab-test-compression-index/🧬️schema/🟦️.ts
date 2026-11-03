/** 🧪️ `change-lab-test-compression-index` wire twin: the leaf payload `ChangeLabTestCompressionIndex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestCompressionIndex {
  mutation: "changeLabTestCompressionIndex";
  id: string;
  newCompressionIndex: number;
}

export const parseChangeLabTestCompressionIndex: NormWireReader<ChangeLabTestCompressionIndex> = normWireObject<ChangeLabTestCompressionIndex>({ mutation: normWireRequired(normWireLiteral("changeLabTestCompressionIndex")), id: normWireRequired(normWireString), newCompressionIndex: normWireRequired(normWireNumber) });

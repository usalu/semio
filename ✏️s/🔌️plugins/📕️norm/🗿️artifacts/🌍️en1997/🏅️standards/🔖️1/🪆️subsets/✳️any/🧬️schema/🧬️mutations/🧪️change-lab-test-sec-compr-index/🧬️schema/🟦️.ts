/** 🧪️ `change-lab-test-sec-compr-index` wire twin: the leaf payload `ChangeLabTestSecComprIndex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestSecComprIndex {
  mutation: "changeLabTestSecComprIndex";
  id: string;
  newSecondaryCompressionIndex: number;
}

export const parseChangeLabTestSecComprIndex: NormWireReader<ChangeLabTestSecComprIndex> = normWireObject<ChangeLabTestSecComprIndex>({ mutation: normWireRequired(normWireLiteral("changeLabTestSecComprIndex")), id: normWireRequired(normWireString), newSecondaryCompressionIndex: normWireRequired(normWireNumber) });

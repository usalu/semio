/** 🧪️ `change-lab-test-void-ratio` wire twin: the leaf payload `ChangeLabTestVoidRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestVoidRatio {
  mutation: "changeLabTestVoidRatio";
  id: string;
  newVoidRatio: number;
}

export const parseChangeLabTestVoidRatio: NormWireReader<ChangeLabTestVoidRatio> = normWireObject<ChangeLabTestVoidRatio>({ mutation: normWireRequired(normWireLiteral("changeLabTestVoidRatio")), id: normWireRequired(normWireString), newVoidRatio: normWireRequired(normWireNumber) });

/** 🧪️ `change-lab-test-drainage-height` wire twin: the leaf payload `ChangeLabTestDrainageHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestDrainageHeight {
  mutation: "changeLabTestDrainageHeight";
  id: string;
  newDrainageHeight: number;
}

export const parseChangeLabTestDrainageHeight: NormWireReader<ChangeLabTestDrainageHeight> = normWireObject<ChangeLabTestDrainageHeight>({ mutation: normWireRequired(normWireLiteral("changeLabTestDrainageHeight")), id: normWireRequired(normWireString), newDrainageHeight: normWireRequired(normWireNumber) });

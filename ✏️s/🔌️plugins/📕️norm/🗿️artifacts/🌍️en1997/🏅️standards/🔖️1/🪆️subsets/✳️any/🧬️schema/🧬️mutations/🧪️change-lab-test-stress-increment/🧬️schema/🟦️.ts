/** 🧪️ `change-lab-test-stress-increment` wire twin: the leaf payload `ChangeLabTestStressIncrement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestStressIncrement {
  mutation: "changeLabTestStressIncrement";
  id: string;
  newStressIncrement: number;
}

export const parseChangeLabTestStressIncrement: NormWireReader<ChangeLabTestStressIncrement> = normWireObject<ChangeLabTestStressIncrement>({ mutation: normWireRequired(normWireLiteral("changeLabTestStressIncrement")), id: normWireRequired(normWireString), newStressIncrement: normWireRequired(normWireNumber) });

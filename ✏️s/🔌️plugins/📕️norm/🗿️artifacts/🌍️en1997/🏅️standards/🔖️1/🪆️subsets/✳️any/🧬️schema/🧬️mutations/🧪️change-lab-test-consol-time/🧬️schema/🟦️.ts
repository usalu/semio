/** 🧪️ `change-lab-test-consol-time` wire twin: the leaf payload `ChangeLabTestConsolTime`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestConsolTime {
  mutation: "changeLabTestConsolTime";
  id: string;
  newConsolidationTime: number;
}

export const parseChangeLabTestConsolTime: NormWireReader<ChangeLabTestConsolTime> = normWireObject<ChangeLabTestConsolTime>({ mutation: normWireRequired(normWireLiteral("changeLabTestConsolTime")), id: normWireRequired(normWireString), newConsolidationTime: normWireRequired(normWireNumber) });

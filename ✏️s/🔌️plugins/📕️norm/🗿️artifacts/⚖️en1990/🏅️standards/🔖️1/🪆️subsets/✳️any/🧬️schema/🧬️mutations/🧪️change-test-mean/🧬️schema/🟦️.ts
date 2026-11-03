/** 🧪 `change-test-mean` wire twin: the leaf payload `ChangeTestMean`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestMean {
  mutation: "changeTestMean";
  id: string;
  newMean: number;
}

export const parseChangeTestMean: NormWireReader<ChangeTestMean> = normWireObject<ChangeTestMean>({ mutation: normWireRequired(normWireLiteral("changeTestMean")), id: normWireRequired(normWireString), newMean: normWireRequired(normWireNumber) });

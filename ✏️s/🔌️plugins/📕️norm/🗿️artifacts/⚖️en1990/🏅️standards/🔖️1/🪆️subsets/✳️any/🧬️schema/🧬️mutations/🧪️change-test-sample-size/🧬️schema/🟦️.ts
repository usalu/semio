/** 🧪 `change-test-sample-size` wire twin: the leaf payload `ChangeTestSampleSize`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestSampleSize {
  mutation: "changeTestSampleSize";
  id: string;
  newSampleSize: number;
}

export const parseChangeTestSampleSize: NormWireReader<ChangeTestSampleSize> = normWireObject<ChangeTestSampleSize>({ mutation: normWireRequired(normWireLiteral("changeTestSampleSize")), id: normWireRequired(normWireString), newSampleSize: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });

/** 🧪 `remove-test-series` wire twin: the leaf payload `RemoveTestSeries`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTestSeries {
  mutation: "removeTestSeries";
  index: number;
}

export const parseRemoveTestSeries: NormWireReader<RemoveTestSeries> = normWireObject<RemoveTestSeries>({ mutation: normWireRequired(normWireLiteral("removeTestSeries")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

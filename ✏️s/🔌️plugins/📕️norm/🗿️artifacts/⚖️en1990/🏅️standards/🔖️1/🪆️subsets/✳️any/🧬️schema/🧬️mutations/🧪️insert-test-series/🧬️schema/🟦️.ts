/** 🧪 `insert-test-series` wire twin: the leaf payload `InsertTestSeries`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990TestSeries, parseEn1990TestSeries } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTestSeries {
  mutation: "insertTestSeries";
  index?: number;
  item: En1990TestSeries;
}

export const parseInsertTestSeries: NormWireReader<InsertTestSeries> = normWireObject<InsertTestSeries>({ mutation: normWireRequired(normWireLiteral("insertTestSeries")), index: normWireOptional(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990TestSeries) });

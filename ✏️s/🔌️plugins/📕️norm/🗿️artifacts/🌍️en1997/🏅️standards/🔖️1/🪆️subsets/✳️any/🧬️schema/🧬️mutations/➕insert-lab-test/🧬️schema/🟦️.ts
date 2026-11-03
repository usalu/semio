/** ➕ `insert-lab-test` wire twin: the leaf payload `InsertLabTest`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997LabTest, parseEn1997LabTest } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLabTest {
  mutation: "insertLabTest";
  index: number;
  labTest: En1997LabTest;
}

export const parseInsertLabTest: NormWireReader<InsertLabTest> = normWireObject<InsertLabTest>({ mutation: normWireRequired(normWireLiteral("insertLabTest")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), labTest: normWireRequired(parseEn1997LabTest) });

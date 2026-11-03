/** ➕ `insert-anchor-test` wire twin: the leaf payload `InsertAnchorTest`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997AnchorTest, parseEn1997AnchorTest } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAnchorTest {
  mutation: "insertAnchorTest";
  anchorId: string;
  index: number;
  anchorTest: En1997AnchorTest;
}

export const parseInsertAnchorTest: NormWireReader<InsertAnchorTest> = normWireObject<InsertAnchorTest>({ mutation: normWireRequired(normWireLiteral("insertAnchorTest")), anchorId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), anchorTest: normWireRequired(parseEn1997AnchorTest) });

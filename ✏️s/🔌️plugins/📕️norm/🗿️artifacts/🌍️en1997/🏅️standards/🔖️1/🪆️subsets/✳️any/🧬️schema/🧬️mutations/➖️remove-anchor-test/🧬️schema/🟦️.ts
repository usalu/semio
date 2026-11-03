/** ➖ `remove-anchor-test` wire twin: the leaf payload `RemoveAnchorTest`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAnchorTest {
  mutation: "removeAnchorTest";
  anchorId: string;
  index: number;
}

export const parseRemoveAnchorTest: NormWireReader<RemoveAnchorTest> = normWireObject<RemoveAnchorTest>({ mutation: normWireRequired(normWireLiteral("removeAnchorTest")), anchorId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

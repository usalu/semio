/** ➖ `remove-lab-test` wire twin: the leaf payload `RemoveLabTest`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLabTest {
  mutation: "removeLabTest";
  index: number;
}

export const parseRemoveLabTest: NormWireReader<RemoveLabTest> = normWireObject<RemoveLabTest>({ mutation: normWireRequired(normWireLiteral("removeLabTest")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

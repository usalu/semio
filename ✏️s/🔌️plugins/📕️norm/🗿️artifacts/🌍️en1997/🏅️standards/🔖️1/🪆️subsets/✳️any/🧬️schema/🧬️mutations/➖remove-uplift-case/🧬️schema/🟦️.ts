/** ➖ `remove-uplift-case` wire twin: the leaf payload `RemoveUpliftCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveUpliftCase {
  mutation: "removeUpliftCase";
  index: number;
}

export const parseRemoveUpliftCase: NormWireReader<RemoveUpliftCase> = normWireObject<RemoveUpliftCase>({ mutation: normWireRequired(normWireLiteral("removeUpliftCase")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

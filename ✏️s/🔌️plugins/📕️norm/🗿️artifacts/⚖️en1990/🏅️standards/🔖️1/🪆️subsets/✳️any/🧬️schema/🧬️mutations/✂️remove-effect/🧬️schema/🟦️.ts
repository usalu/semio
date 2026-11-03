/** ✂️ `remove-effect` wire twin: the leaf payload `RemoveEffect`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveEffect {
  mutation: "removeEffect";
  index: number;
}

export const parseRemoveEffect: NormWireReader<RemoveEffect> = normWireObject<RemoveEffect>({ mutation: normWireRequired(normWireLiteral("removeEffect")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

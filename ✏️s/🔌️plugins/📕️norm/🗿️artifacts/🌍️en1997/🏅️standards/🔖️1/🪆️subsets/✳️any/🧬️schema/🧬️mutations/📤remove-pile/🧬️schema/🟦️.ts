/** 📤 `remove-pile` wire twin: the leaf payload `RemovePile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePile {
  mutation: "removePile";
  index: number;
}

export const parseRemovePile: NormWireReader<RemovePile> = normWireObject<RemovePile>({ mutation: normWireRequired(normWireLiteral("removePile")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

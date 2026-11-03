/** 🧯 `remove-accidental` wire twin: the leaf payload `RemoveAccidental`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAccidental {
  mutation: "removeAccidental";
  index: number;
}

export const parseRemoveAccidental: NormWireReader<RemoveAccidental> = normWireObject<RemoveAccidental>({ mutation: normWireRequired(normWireLiteral("removeAccidental")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

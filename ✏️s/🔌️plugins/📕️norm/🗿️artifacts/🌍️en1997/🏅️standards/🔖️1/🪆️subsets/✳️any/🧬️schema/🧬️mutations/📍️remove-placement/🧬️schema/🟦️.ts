/** 📍️ `remove-placement` wire twin: the leaf payload `RemovePlacement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePlacement {
  mutation: "removePlacement";
  index: number;
}

export const parseRemovePlacement: NormWireReader<RemovePlacement> = normWireObject<RemovePlacement>({ mutation: normWireRequired(normWireLiteral("removePlacement")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

/** 🕳️ `remove-seismic` wire twin: the leaf payload `RemoveSeismic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSeismic {
  mutation: "removeSeismic";
  index: number;
}

export const parseRemoveSeismic: NormWireReader<RemoveSeismic> = normWireObject<RemoveSeismic>({ mutation: normWireRequired(normWireLiteral("removeSeismic")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

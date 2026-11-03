/** ➖️ `remove-seismic-slope` wire twin: the leaf payload `RemoveSeismicSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSeismicSlope {
  mutation: "removeSeismicSlope";
  id: string;
}

export const parseRemoveSeismicSlope: NormWireReader<RemoveSeismicSlope> = normWireObject<RemoveSeismicSlope>({ mutation: normWireRequired(normWireLiteral("removeSeismicSlope")), id: normWireRequired(normWireString) });

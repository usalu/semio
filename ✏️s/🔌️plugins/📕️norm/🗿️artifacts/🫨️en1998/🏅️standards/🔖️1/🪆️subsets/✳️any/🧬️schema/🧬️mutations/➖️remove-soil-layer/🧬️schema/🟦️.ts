/** ➖️ `remove-soil-layer` wire twin: the leaf payload `RemoveSoilLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSoilLayer {
  mutation: "removeSoilLayer";
  id: string;
}

export const parseRemoveSoilLayer: NormWireReader<RemoveSoilLayer> = normWireObject<RemoveSoilLayer>({ mutation: normWireRequired(normWireLiteral("removeSoilLayer")), id: normWireRequired(normWireString) });

/** ✏️ `change-soil-layer-vs-ms` wire twin: the leaf payload `ChangeSoilLayerVsMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilLayerVsMS {
  mutation: "changeSoilLayerVsMS";
  id: string;
  newShearWaveVelocityMS: number;
}

export const parseChangeSoilLayerVsMS: NormWireReader<ChangeSoilLayerVsMS> = normWireObject<ChangeSoilLayerVsMS>({ mutation: normWireRequired(normWireLiteral("changeSoilLayerVsMS")), id: normWireRequired(normWireString), newShearWaveVelocityMS: normWireRequired(normWireNumber) });

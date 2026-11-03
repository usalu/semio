/** ✏️ `change-soil-layer-thickness-m` wire twin: the leaf payload `ChangeSoilLayerThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilLayerThicknessM {
  mutation: "changeSoilLayerThicknessM";
  id: string;
  newThicknessM: number;
}

export const parseChangeSoilLayerThicknessM: NormWireReader<ChangeSoilLayerThicknessM> = normWireObject<ChangeSoilLayerThicknessM>({ mutation: normWireRequired(normWireLiteral("changeSoilLayerThicknessM")), id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireNumber) });

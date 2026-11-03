/** ✏️ `change-soil-layer-fines-percent` wire twin: the leaf payload `ChangeSoilLayerFinesPercent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilLayerFinesPercent {
  mutation: "changeSoilLayerFinesPercent";
  id: string;
  newFinesPercent: number;
}

export const parseChangeSoilLayerFinesPercent: NormWireReader<ChangeSoilLayerFinesPercent> = normWireObject<ChangeSoilLayerFinesPercent>({ mutation: normWireRequired(normWireLiteral("changeSoilLayerFinesPercent")), id: normWireRequired(normWireString), newFinesPercent: normWireRequired(normWireNumber) });

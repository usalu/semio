/** 🥞️ `change-layer-soil-type` wire twin: the leaf payload `ChangeLayerSoilType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SoilType, parseSoilType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLayerSoilType {
  mutation: "changeLayerSoilType";
  id: string;
  newSoilType: SoilType;
}

export const parseChangeLayerSoilType: NormWireReader<ChangeLayerSoilType> = normWireObject<ChangeLayerSoilType>({ mutation: normWireRequired(normWireLiteral("changeLayerSoilType")), id: normWireRequired(normWireString), newSoilType: normWireRequired(parseSoilType) });

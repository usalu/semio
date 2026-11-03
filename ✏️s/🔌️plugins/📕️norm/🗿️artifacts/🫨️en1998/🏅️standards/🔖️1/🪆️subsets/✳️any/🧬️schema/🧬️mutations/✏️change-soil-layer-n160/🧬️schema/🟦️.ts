/** ✏️ `change-soil-layer-n160` wire twin: the leaf payload `ChangeSoilLayerN160`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilLayerN160 {
  mutation: "changeSoilLayerN160";
  id: string;
  newN160: number;
}

export const parseChangeSoilLayerN160: NormWireReader<ChangeSoilLayerN160> = normWireObject<ChangeSoilLayerN160>({ mutation: normWireRequired(normWireLiteral("changeSoilLayerN160")), id: normWireRequired(normWireString), newN160: normWireRequired(normWireNumber) });

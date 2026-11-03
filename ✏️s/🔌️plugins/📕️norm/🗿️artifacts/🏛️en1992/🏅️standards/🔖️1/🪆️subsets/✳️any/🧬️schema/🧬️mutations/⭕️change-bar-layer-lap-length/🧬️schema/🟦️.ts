/** ⭕ `change-bar-layer-lap-length` wire twin: the leaf payload `ChangeBarLayerLapLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerLapLength {
  memberId: string;
  layerId: string;
  newLapLength: number;
}

export const parseChangeBarLayerLapLength: NormWireReader<ChangeBarLayerLapLength> = normWireObject<ChangeBarLayerLapLength>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newLapLength: normWireRequired(normWireNumber) });

/** ⭕ `change-bar-layer-lap-share` wire twin: the leaf payload `ChangeBarLayerLapShare`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerLapShare {
  memberId: string;
  layerId: string;
  newLapShare: number;
}

export const parseChangeBarLayerLapShare: NormWireReader<ChangeBarLayerLapShare> = normWireObject<ChangeBarLayerLapShare>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newLapShare: normWireRequired(normWireNumber) });

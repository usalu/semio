/** ⭕ `change-bar-layer-diameter` wire twin: the leaf payload `ChangeBarLayerDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerDiameter {
  memberId: string;
  layerId: string;
  newDiameter: number;
}

export const parseChangeBarLayerDiameter: NormWireReader<ChangeBarLayerDiameter> = normWireObject<ChangeBarLayerDiameter>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newDiameter: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });

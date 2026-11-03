/** ⭕ `change-bar-layer-aggregate-size` wire twin: the leaf payload `ChangeBarLayerAggregateSize`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerAggregateSize {
  memberId: string;
  layerId: string;
  newAggregateSize: number;
}

export const parseChangeBarLayerAggregateSize: NormWireReader<ChangeBarLayerAggregateSize> = normWireObject<ChangeBarLayerAggregateSize>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newAggregateSize: normWireRequired(normWireNumber) });

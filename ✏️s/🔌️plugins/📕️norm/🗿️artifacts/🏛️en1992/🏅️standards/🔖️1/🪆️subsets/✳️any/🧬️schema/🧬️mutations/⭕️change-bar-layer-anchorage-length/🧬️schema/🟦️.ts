/** ⭕ `change-bar-layer-anchorage-length` wire twin: the leaf payload `ChangeBarLayerAnchorageLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerAnchorageLength {
  memberId: string;
  layerId: string;
  newAnchorageLength: number;
}

export const parseChangeBarLayerAnchorageLength: NormWireReader<ChangeBarLayerAnchorageLength> = normWireObject<ChangeBarLayerAnchorageLength>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newAnchorageLength: normWireRequired(normWireNumber) });

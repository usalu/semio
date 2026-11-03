/** ⭕ `change-bar-layer-position` wire twin: the leaf payload `ChangeBarLayerPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BarPosition, parseBarPosition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBarLayerPosition {
  memberId: string;
  layerId: string;
  newPosition: BarPosition;
}

export const parseChangeBarLayerPosition: NormWireReader<ChangeBarLayerPosition> = normWireObject<ChangeBarLayerPosition>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newPosition: normWireRequired(parseBarPosition) });

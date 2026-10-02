/** 🦠️ `change-bar-layer-count` wire twin: the leaf payload `ChangeBarLayerCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBarLayerCount {
  memberId: string;
  layerId: string;
  newCount: number;
}

export const parseChangeBarLayerCount: NormWireReader<ChangeBarLayerCount> = normWireObject<ChangeBarLayerCount>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

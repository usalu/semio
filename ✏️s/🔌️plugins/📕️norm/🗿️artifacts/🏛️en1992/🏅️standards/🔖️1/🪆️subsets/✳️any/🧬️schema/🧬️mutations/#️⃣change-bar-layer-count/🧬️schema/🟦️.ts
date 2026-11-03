/** #️⃣ `change-bar-layer-count` wire twin: the leaf payload `ChangeBarLayerCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLayerCount {
  memberId: string;
  layerId: string;
  newCount: number;
}

export const parseChangeBarLayerCount: NormWireReader<ChangeBarLayerCount> = normWireObject<ChangeBarLayerCount>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });

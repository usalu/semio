/** 🔁️ `remove-fatigue-band` wire twin: the leaf payload `RemoveFatigueBand`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueBand {
  id: string;
  index: number;
}

export const parseRemoveFatigueBand: NormWireReader<RemoveFatigueBand> = normWireObject<RemoveFatigueBand>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

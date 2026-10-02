/** 📈️ `change-element-delta-uf` wire twin: the leaf payload `ChangeElementDeltaUf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementDeltaUf {
  elementId: string;
  newDeltaUF: number;
}

export const parseChangeElementDeltaUf: NormWireReader<ChangeElementDeltaUf> = normWireObject<ChangeElementDeltaUf>({ elementId: normWireRequired(normWireString), newDeltaUF: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

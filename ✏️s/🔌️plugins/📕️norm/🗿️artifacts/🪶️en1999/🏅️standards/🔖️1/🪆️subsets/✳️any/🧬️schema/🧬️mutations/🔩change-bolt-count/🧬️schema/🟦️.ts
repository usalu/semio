/** 🔩 `change-bolt-count` wire twin: the leaf payload `ChangeBoltCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBoltCount {
  mutation: "changeBoltCount";
  connectionId: string;
  newRows: number;
  newBoltsPerRow: number;
}

export const parseChangeBoltCount: NormWireReader<ChangeBoltCount> = normWireObject<ChangeBoltCount>({ mutation: normWireRequired(normWireLiteral("changeBoltCount")), connectionId: normWireRequired(normWireString), newRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBoltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

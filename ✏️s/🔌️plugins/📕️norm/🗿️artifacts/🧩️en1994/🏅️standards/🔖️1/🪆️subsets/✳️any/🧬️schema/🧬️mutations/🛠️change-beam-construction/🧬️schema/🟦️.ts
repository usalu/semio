/** 🛠️ `change-beam-construction` wire twin: the leaf payload `ChangeBeamConstruction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamConstruction {
  index: number;
  newConstruction: string;
}

export const parseChangeBeamConstruction: NormWireReader<ChangeBeamConstruction> = normWireObject<ChangeBeamConstruction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newConstruction: normWireRequired(normWireString) });

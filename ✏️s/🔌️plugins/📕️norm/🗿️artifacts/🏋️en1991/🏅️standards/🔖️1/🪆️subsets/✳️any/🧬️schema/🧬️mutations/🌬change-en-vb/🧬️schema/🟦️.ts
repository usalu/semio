/** 🌬 `change-en-vb` wire twin: the leaf payload `ChangeEnVb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEnVb {
  newEnVb: number;
}

export const parseChangeEnVb: NormWireReader<ChangeEnVb> = normWireObject<ChangeEnVb>({ newEnVb: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

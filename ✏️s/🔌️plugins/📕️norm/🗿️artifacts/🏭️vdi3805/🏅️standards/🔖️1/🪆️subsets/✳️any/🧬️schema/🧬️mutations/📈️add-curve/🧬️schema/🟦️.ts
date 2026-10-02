/** 📈️ `add-curve` wire twin: the leaf payload `AddCurve`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CharacteristicCurve, parseCharacteristicCurve } from "../../../📸️snapshot/🟦️.ts";

export interface AddCurve {
  curve: CharacteristicCurve;
}

export const parseAddCurve: NormWireReader<AddCurve> = normWireObject<AddCurve>({ curve: normWireRequired(parseCharacteristicCurve) });

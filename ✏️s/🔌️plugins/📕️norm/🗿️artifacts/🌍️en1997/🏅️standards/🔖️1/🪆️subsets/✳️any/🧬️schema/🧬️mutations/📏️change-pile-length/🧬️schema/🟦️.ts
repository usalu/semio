/** 📏️ `change-pile-length` wire twin: the leaf payload `ChangePileLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangePileLength {
  mutation: "changePileLength";
  id: string;
  newLength: number;
}

export const parseChangePileLength: NormWireReader<ChangePileLength> = normWireObject<ChangePileLength>({ mutation: normWireRequired(normWireLiteral("changePileLength")), id: normWireRequired(normWireString), newLength: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });

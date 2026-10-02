/** 📐 `change-beta-computed` wire twin: the leaf payload `ChangeBetaComputed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBetaComputed {
  mutation: "changeBetaComputed";
  newBetaComputed: number;
}

export const parseChangeBetaComputed: NormWireReader<ChangeBetaComputed> = normWireObject<ChangeBetaComputed>({ mutation: normWireRequired(normWireLiteral("changeBetaComputed")), newBetaComputed: normWireRequired(normWireNumber) });

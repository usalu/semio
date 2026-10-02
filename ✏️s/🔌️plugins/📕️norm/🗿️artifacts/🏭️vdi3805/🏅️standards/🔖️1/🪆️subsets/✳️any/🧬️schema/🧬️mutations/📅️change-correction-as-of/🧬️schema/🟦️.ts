/** 📅️ `change-correction-as-of` wire twin: the leaf payload `ChangeCorrectionAsOf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type EditionId, parseEditionId } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCorrectionAsOf {
  newCorrectionAsOf: EditionId;
}

export const parseChangeCorrectionAsOf: NormWireReader<ChangeCorrectionAsOf> = normWireObject<ChangeCorrectionAsOf>({ newCorrectionAsOf: normWireRequired(parseEditionId) });

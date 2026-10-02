/** ✂️ `retire-subject` wire twin: the leaf payload `RetireSubject`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireSubject {
  id: string;
}

export const parseRetireSubject: NormWireReader<RetireSubject> = normWireObject<RetireSubject>({ id: normWireRequired(normWireString) });

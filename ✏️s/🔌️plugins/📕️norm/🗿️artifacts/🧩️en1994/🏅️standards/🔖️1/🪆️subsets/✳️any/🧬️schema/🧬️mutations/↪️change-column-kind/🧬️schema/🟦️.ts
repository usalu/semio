/** ↪️ `change-column-kind` wire twin: the leaf payload `ChangeColumnKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeColumnKind {
  index: number;
  newKind: string;
}

export const parseChangeColumnKind: NormWireReader<ChangeColumnKind> = normWireObject<ChangeColumnKind>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newKind: normWireRequired(normWireString) });

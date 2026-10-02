/** 🏷️ `change-element-kind` wire twin: the leaf payload `ChangeElementKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementKind {
  elementId: string;
  newKind: string;
}

export const parseChangeElementKind: NormWireReader<ChangeElementKind> = normWireObject<ChangeElementKind>({ elementId: normWireRequired(normWireString), newKind: normWireRequired(normWireString) });

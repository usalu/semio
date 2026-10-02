/** 👁️ `change-supervision-level` wire twin: the leaf payload `ChangeSupervisionLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSupervisionLevel {
  mutation: "changeSupervisionLevel";
  newSupervisionLevel: string;
}

export const parseChangeSupervisionLevel: NormWireReader<ChangeSupervisionLevel> = normWireObject<ChangeSupervisionLevel>({ mutation: normWireRequired(normWireLiteral("changeSupervisionLevel")), newSupervisionLevel: normWireRequired(normWireString) });

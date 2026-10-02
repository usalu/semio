/** 🏷️ `change-project-id` wire twin: the leaf payload `ChangeProjectId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeProjectId {
  mutation: "changeProjectId";
  newProjectId: string;
}

export const parseChangeProjectId: NormWireReader<ChangeProjectId> = normWireObject<ChangeProjectId>({ mutation: normWireRequired(normWireLiteral("changeProjectId")), newProjectId: normWireRequired(normWireString) });

/** ↔️ `change-footing-width` wire twin: the leaf payload `ChangeFootingWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFootingWidth {
  mutation: "changeFootingWidth";
  id: string;
  newWidth: number;
}

export const parseChangeFootingWidth: NormWireReader<ChangeFootingWidth> = normWireObject<ChangeFootingWidth>({ mutation: normWireRequired(normWireLiteral("changeFootingWidth")), id: normWireRequired(normWireString), newWidth: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });

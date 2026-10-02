/** 🧽️ `retire-property-definition` wire twin: the leaf payload `RetirePropertyDefinition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetirePropertyDefinition {
  id: string;
}

export const parseRetirePropertyDefinition: NormWireReader<RetirePropertyDefinition> = normWireObject<RetirePropertyDefinition>({ id: normWireRequired(normWireString) });

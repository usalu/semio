/** 🧱 `change-vent-duct-class` wire twin: the leaf payload `ChangeVentDuctClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentDuctClass {
  ventId: string;
  newDuctClass: string;
}

export const parseChangeVentDuctClass: NormWireReader<ChangeVentDuctClass> = normWireObject<ChangeVentDuctClass>({ ventId: normWireRequired(normWireString), newDuctClass: normWireRequired(normWireString) });

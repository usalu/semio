/** 🗑️ `retire-geometry-object` wire twin: the leaf payload `RetireGeometryObject`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireGeometryObject {
  id: string;
}

export const parseRetireGeometryObject: NormWireReader<RetireGeometryObject> = normWireObject<RetireGeometryObject>({ id: normWireRequired(normWireString) });

/** 🧮️ `change-geometry-parameters` wire twin: the leaf payload `ChangeGeometryParameters`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireMap, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeGeometryParameters {
  id: string;
  newParameters: { [key: string]: number };
}

export const parseChangeGeometryParameters: NormWireReader<ChangeGeometryParameters> = normWireObject<ChangeGeometryParameters>({ id: normWireRequired(normWireString), newParameters: normWireRequired(normWireMap(normWireNumber)) });

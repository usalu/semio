/** 🧊️ `add-geometry` wire twin: the leaf payload `AddGeometry`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ParametricGeometry, parseParametricGeometry } from "../../../📸️snapshot/🟦️.ts";

export interface AddGeometry {
  geometry: ParametricGeometry;
}

export const parseAddGeometry: NormWireReader<AddGeometry> = normWireObject<AddGeometry>({ geometry: normWireRequired(parseParametricGeometry) });

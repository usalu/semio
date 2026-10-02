/** 📐 `change-orography-factor` wire twin: the leaf payload `ChangeOrographyFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeOrographyFactor {
  newOrographyFactor: number;
}

export const parseChangeOrographyFactor: NormWireReader<ChangeOrographyFactor> = normWireObject<ChangeOrographyFactor>({ newOrographyFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

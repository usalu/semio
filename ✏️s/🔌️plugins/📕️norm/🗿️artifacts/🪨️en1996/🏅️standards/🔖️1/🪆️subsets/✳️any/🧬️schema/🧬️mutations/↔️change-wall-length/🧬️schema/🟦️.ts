/** ↔️ `change-wall-length` wire twin: the leaf payload `ChangeWallLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWallLength {
  index: number;
  newLengthM: number;
}

export const parseChangeWallLength: NormWireReader<ChangeWallLength> = normWireObject<ChangeWallLength>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

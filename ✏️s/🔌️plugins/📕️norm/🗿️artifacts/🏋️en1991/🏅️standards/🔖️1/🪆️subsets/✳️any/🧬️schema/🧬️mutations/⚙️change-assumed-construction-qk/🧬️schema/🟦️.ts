/** ⚙️ `change-assumed-construction-qk` wire twin: the leaf payload `ChangeAssumedConstructionQk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedConstructionQk {
  newAssumedConstructionQk: number;
}

export const parseChangeAssumedConstructionQk: NormWireReader<ChangeAssumedConstructionQk> = normWireObject<ChangeAssumedConstructionQk>({ newAssumedConstructionQk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

/** ⭕ `change-silo-hydraulic-radius` wire twin: the leaf payload `ChangeSiloHydraulicRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloHydraulicRadius {
  newSiloHydraulicRadius: number;
}

export const parseChangeSiloHydraulicRadius: NormWireReader<ChangeSiloHydraulicRadius> = normWireObject<ChangeSiloHydraulicRadius>({ newSiloHydraulicRadius: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

/** ⭕ `change-silo-hydraulic-radius` wire twin: the leaf payload `ChangeSiloHydraulicRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloHydraulicRadius {
  newSiloHydraulicRadius: number;
}

export const parseChangeSiloHydraulicRadius: NormWireReader<ChangeSiloHydraulicRadius> = normWireObject<ChangeSiloHydraulicRadius>({ newSiloHydraulicRadius: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

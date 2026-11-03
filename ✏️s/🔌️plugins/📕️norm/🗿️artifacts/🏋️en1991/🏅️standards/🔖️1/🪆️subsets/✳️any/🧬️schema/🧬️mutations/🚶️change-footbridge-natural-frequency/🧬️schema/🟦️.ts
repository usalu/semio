/** 🚶 `change-footbridge-natural-frequency` wire twin: the leaf payload `ChangeFootbridgeNaturalFrequency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootbridgeNaturalFrequency {
  id: string;
  newNaturalFrequency: number;
}

export const parseChangeFootbridgeNaturalFrequency: NormWireReader<ChangeFootbridgeNaturalFrequency> = normWireObject<ChangeFootbridgeNaturalFrequency>({ id: normWireRequired(normWireString), newNaturalFrequency: normWireRequired(normWireNumber) });

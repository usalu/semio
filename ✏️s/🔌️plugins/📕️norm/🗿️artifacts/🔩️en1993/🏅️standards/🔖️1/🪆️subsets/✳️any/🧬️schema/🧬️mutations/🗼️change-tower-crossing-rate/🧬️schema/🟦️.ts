/** 🗼️ `change-tower-crossing-rate` wire twin: the leaf payload `ChangeTowerCrossingRate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerCrossingRate {
  id: string;
  newCrossingRate: number;
}

export const parseChangeTowerCrossingRate: NormWireReader<ChangeTowerCrossingRate> = normWireObject<ChangeTowerCrossingRate>({ id: normWireRequired(normWireString), newCrossingRate: normWireRequired(normWireNumber) });

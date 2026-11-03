/** 🗼️ `change-tower-strouhal` wire twin: the leaf payload `ChangeTowerStrouhal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerStrouhal {
  id: string;
  newStrouhal: number;
}

export const parseChangeTowerStrouhal: NormWireReader<ChangeTowerStrouhal> = normWireObject<ChangeTowerStrouhal>({ id: normWireRequired(normWireString), newStrouhal: normWireRequired(normWireNumber) });

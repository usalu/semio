/** 🗼️ `change-tower-dynamic-factor` wire twin: the leaf payload `ChangeTowerDynamicFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerDynamicFactor {
  id: string;
  newDynamicFactor: number;
}

export const parseChangeTowerDynamicFactor: NormWireReader<ChangeTowerDynamicFactor> = normWireObject<ChangeTowerDynamicFactor>({ id: normWireRequired(normWireString), newDynamicFactor: normWireRequired(normWireNumber) });

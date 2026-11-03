/** 🗼️ `change-tower-k-factor` wire twin: the leaf payload `ChangeTowerKFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerKFactor {
  id: string;
  newKFactor: number;
}

export const parseChangeTowerKFactor: NormWireReader<ChangeTowerKFactor> = normWireObject<ChangeTowerKFactor>({ id: normWireRequired(normWireString), newKFactor: normWireRequired(normWireNumber) });

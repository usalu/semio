/** 🗼️ `change-tower-kw-factor` wire twin: the leaf payload `ChangeTowerKwFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerKwFactor {
  id: string;
  newKwFactor: number;
}

export const parseChangeTowerKwFactor: NormWireReader<ChangeTowerKwFactor> = normWireObject<ChangeTowerKwFactor>({ id: normWireRequired(normWireString), newKwFactor: normWireRequired(normWireNumber) });

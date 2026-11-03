/** 🪢️ `change-tension-partial-factor` wire twin: the leaf payload `ChangeTensionPartialFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPartialFactor {
  id: string;
  newPartialFactor: number;
}

export const parseChangeTensionPartialFactor: NormWireReader<ChangeTensionPartialFactor> = normWireObject<ChangeTensionPartialFactor>({ id: normWireRequired(normWireString), newPartialFactor: normWireRequired(normWireNumber) });

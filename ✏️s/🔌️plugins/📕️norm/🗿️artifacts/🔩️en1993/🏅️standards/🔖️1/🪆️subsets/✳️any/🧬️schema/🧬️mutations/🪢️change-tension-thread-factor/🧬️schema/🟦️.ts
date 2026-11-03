/** 🪢️ `change-tension-thread-factor` wire twin: the leaf payload `ChangeTensionThreadFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionThreadFactor {
  id: string;
  newThreadFactor: number;
}

export const parseChangeTensionThreadFactor: NormWireReader<ChangeTensionThreadFactor> = normWireObject<ChangeTensionThreadFactor>({ id: normWireRequired(normWireString), newThreadFactor: normWireRequired(normWireNumber) });

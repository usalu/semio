/** 🪢️ `change-tension-fk` wire twin: the leaf payload `ChangeTensionFK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionFK {
  id: string;
  newFK: number;
}

export const parseChangeTensionFK: NormWireReader<ChangeTensionFK> = normWireObject<ChangeTensionFK>({ id: normWireRequired(normWireString), newFK: normWireRequired(normWireNumber) });

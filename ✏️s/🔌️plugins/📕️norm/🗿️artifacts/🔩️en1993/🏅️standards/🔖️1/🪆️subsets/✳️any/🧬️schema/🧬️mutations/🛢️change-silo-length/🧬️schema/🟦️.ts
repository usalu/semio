/** 🛢️ `change-silo-length` wire twin: the leaf payload `ChangeSiloLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloLength {
  id: string;
  newLength: number;
}

export const parseChangeSiloLength: NormWireReader<ChangeSiloLength> = normWireObject<ChangeSiloLength>({ id: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });

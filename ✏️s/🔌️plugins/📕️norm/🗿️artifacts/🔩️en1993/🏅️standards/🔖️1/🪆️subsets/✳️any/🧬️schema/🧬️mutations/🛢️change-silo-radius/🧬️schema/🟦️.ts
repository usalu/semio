/** 🛢️ `change-silo-radius` wire twin: the leaf payload `ChangeSiloRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRadius {
  id: string;
  newRadius: number;
}

export const parseChangeSiloRadius: NormWireReader<ChangeSiloRadius> = normWireObject<ChangeSiloRadius>({ id: normWireRequired(normWireString), newRadius: normWireRequired(normWireNumber) });

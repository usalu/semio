/** 🛢️ `change-silo-hoop-stress` wire twin: the leaf payload `ChangeSiloHoopStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloHoopStress {
  id: string;
  newHoopStress: number;
}

export const parseChangeSiloHoopStress: NormWireReader<ChangeSiloHoopStress> = normWireObject<ChangeSiloHoopStress>({ id: normWireRequired(normWireString), newHoopStress: normWireRequired(normWireNumber) });

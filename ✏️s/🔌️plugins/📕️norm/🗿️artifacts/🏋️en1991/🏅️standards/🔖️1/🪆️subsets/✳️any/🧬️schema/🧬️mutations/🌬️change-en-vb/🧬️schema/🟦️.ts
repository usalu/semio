/** 🌬️ `change-en-vb` wire twin: the leaf payload `ChangeEnVb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEnVb {
  newEnVb: number;
}

export const parseChangeEnVb: NormWireReader<ChangeEnVb> = normWireObject<ChangeEnVb>({ newEnVb: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

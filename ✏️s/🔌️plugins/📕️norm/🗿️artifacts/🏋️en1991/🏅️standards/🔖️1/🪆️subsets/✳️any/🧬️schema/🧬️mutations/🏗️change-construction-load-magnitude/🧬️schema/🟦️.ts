/** 🏗 `change-construction-load-magnitude` wire twin: the leaf payload `ChangeConstructionLoadMagnitude`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConstructionLoadMagnitude {
  id: string;
  newMagnitude: number;
}

export const parseChangeConstructionLoadMagnitude: NormWireReader<ChangeConstructionLoadMagnitude> = normWireObject<ChangeConstructionLoadMagnitude>({ id: normWireRequired(normWireString), newMagnitude: normWireRequired(normWireNumber) });

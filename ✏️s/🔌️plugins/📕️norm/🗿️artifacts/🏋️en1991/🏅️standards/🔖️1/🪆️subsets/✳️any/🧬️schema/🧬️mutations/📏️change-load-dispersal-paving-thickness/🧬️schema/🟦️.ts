/** 📏 `change-load-dispersal-paving-thickness` wire twin: the leaf payload `ChangeLoadDispersalPavingThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadDispersalPavingThickness {
  id: string;
  newPavementThickness: number;
}

export const parseChangeLoadDispersalPavingThickness: NormWireReader<ChangeLoadDispersalPavingThickness> = normWireObject<ChangeLoadDispersalPavingThickness>({ id: normWireRequired(normWireString), newPavementThickness: normWireRequired(normWireNumber) });

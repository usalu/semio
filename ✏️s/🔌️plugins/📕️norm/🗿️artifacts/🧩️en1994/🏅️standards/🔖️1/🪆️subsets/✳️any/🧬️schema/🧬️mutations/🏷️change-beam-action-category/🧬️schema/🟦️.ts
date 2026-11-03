/** 🏷️ `change-beam-action-category` wire twin: the leaf payload `ChangeBeamActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamActionCategory {
  beamId: string;
  id: string;
  newCategory: string;
}

export const parseChangeBeamActionCategory: NormWireReader<ChangeBeamActionCategory> = normWireObject<ChangeBeamActionCategory>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });

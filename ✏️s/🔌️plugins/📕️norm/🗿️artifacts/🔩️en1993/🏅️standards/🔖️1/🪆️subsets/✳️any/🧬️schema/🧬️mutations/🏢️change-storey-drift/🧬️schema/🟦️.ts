/** 🏢️ `change-storey-drift` wire twin: the leaf payload `ChangeStoreyDrift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyDrift {
  id: string;
  newDrift: number;
}

export const parseChangeStoreyDrift: NormWireReader<ChangeStoreyDrift> = normWireObject<ChangeStoreyDrift>({ id: normWireRequired(normWireString), newDrift: normWireRequired(normWireNumber) });

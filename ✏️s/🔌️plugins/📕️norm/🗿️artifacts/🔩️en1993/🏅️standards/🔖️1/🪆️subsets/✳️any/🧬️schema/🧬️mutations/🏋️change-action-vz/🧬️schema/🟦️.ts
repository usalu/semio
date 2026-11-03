/** 🏋️ `change-action-vz` wire twin: the leaf payload `ChangeActionVz`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVz {
  id: string;
  newActionVz: number;
}

export const parseChangeActionVz: NormWireReader<ChangeActionVz> = normWireObject<ChangeActionVz>({ id: normWireRequired(normWireString), newActionVz: normWireRequired(normWireNumber) });

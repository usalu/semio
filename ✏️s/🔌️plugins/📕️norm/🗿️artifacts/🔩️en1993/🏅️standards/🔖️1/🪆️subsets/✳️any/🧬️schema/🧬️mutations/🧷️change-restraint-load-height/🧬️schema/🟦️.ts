/** 🧷️ `change-restraint-load-height` wire twin: the leaf payload `ChangeRestraintLoadHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintLoadHeight {
  id: string;
  newLoadHeight: number;
}

export const parseChangeRestraintLoadHeight: NormWireReader<ChangeRestraintLoadHeight> = normWireObject<ChangeRestraintLoadHeight>({ id: normWireRequired(normWireString), newLoadHeight: normWireRequired(normWireNumber) });

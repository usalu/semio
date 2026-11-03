/** 🧷️ `change-restraint-intermediate-position` wire twin: the leaf payload `ChangeRestraintIntermediatePosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintIntermediatePosition {
  id: string;
  newIntermediatePosition: number;
}

export const parseChangeRestraintIntermediatePosition: NormWireReader<ChangeRestraintIntermediatePosition> = normWireObject<ChangeRestraintIntermediatePosition>({ id: normWireRequired(normWireString), newIntermediatePosition: normWireRequired(normWireNumber) });

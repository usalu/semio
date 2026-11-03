/** 🧷️ `change-restraint-hinge-to-restraint` wire twin: the leaf payload `ChangeRestraintHingeToRestraint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintHingeToRestraint {
  id: string;
  newHingeToRestraint: number;
}

export const parseChangeRestraintHingeToRestraint: NormWireReader<ChangeRestraintHingeToRestraint> = normWireObject<ChangeRestraintHingeToRestraint>({ id: normWireRequired(normWireString), newHingeToRestraint: normWireRequired(normWireNumber) });

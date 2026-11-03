/** 🧷️ `change-rest-hinge-to-rest` wire twin: the leaf payload `ChangeRestHingeToRest`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestHingeToRest {
  id: string;
  newHingeToRestraint: number;
}

export const parseChangeRestHingeToRest: NormWireReader<ChangeRestHingeToRest> = normWireObject<ChangeRestHingeToRest>({ id: normWireRequired(normWireString), newHingeToRestraint: normWireRequired(normWireNumber) });

/** 🧷️ `change-restraint-lat-spring-stiffness` wire twin: the leaf payload `ChangeRestraintLatSpringStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintLatSpringStiffness {
  id: string;
  newLateralSpringStiffness: number;
}

export const parseChangeRestraintLatSpringStiffness: NormWireReader<ChangeRestraintLatSpringStiffness> = normWireObject<ChangeRestraintLatSpringStiffness>({ id: normWireRequired(normWireString), newLateralSpringStiffness: normWireRequired(normWireNumber) });

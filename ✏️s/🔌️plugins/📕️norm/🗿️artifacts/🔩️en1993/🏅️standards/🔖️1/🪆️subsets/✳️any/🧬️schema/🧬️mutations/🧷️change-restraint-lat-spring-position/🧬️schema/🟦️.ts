/** 🧷️ `change-restraint-lat-spring-position` wire twin: the leaf payload `ChangeRestraintLatSpringPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintLatSpringPosition {
  id: string;
  newLateralSpringPosition: number;
}

export const parseChangeRestraintLatSpringPosition: NormWireReader<ChangeRestraintLatSpringPosition> = normWireObject<ChangeRestraintLatSpringPosition>({ id: normWireRequired(normWireString), newLateralSpringPosition: normWireRequired(normWireNumber) });

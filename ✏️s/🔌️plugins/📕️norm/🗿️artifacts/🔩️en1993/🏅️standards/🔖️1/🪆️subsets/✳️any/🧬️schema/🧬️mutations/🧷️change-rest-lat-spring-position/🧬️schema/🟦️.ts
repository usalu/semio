/** 🧷️ `change-rest-lat-spring-position` wire twin: the leaf payload `ChangeRestLatSpringPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestLatSpringPosition {
  id: string;
  newLateralSpringPosition: number;
}

export const parseChangeRestLatSpringPosition: NormWireReader<ChangeRestLatSpringPosition> = normWireObject<ChangeRestLatSpringPosition>({ id: normWireRequired(normWireString), newLateralSpringPosition: normWireRequired(normWireNumber) });

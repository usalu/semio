/** 🧷️ `change-restraint-torsional-spring` wire twin: the leaf payload `ChangeRestraintTorsionalSpring`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintTorsionalSpring {
  id: string;
  newTorsionalSpring: number;
}

export const parseChangeRestraintTorsionalSpring: NormWireReader<ChangeRestraintTorsionalSpring> = normWireObject<ChangeRestraintTorsionalSpring>({ id: normWireRequired(normWireString), newTorsionalSpring: normWireRequired(normWireNumber) });

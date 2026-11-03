/** 💥 `change-impact-action-mass` wire twin: the leaf payload `ChangeImpactActionMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionMass {
  id: string;
  newMass: number;
}

export const parseChangeImpactActionMass: NormWireReader<ChangeImpactActionMass> = normWireObject<ChangeImpactActionMass>({ id: normWireRequired(normWireString), newMass: normWireRequired(normWireNumber) });

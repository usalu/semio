/** 💥 `change-impact-action-assumed-force` wire twin: the leaf payload `ChangeImpactActionAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeImpactActionAssumedForce: NormWireReader<ChangeImpactActionAssumedForce> = normWireObject<ChangeImpactActionAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireNumber) });

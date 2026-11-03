/** 📐 `change-imperfection-assumed-force` wire twin: the leaf payload `ChangeImperfectionAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImperfectionAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeImperfectionAssumedForce: NormWireReader<ChangeImperfectionAssumedForce> = normWireObject<ChangeImperfectionAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireNumber) });

/** ⚙ `change-snow-guard-assumed-force` wire twin: the leaf payload `ChangeSnowGuardAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSnowGuardAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeSnowGuardAssumedForce: NormWireReader<ChangeSnowGuardAssumedForce> = normWireObject<ChangeSnowGuardAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireNumber) });

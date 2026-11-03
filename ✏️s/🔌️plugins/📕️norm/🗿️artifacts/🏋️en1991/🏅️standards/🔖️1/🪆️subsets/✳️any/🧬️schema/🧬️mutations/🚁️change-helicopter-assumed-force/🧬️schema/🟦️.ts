/** 🚁 `change-helicopter-assumed-force` wire twin: the leaf payload `ChangeHelicopterAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHelicopterAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeHelicopterAssumedForce: NormWireReader<ChangeHelicopterAssumedForce> = normWireObject<ChangeHelicopterAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireNumber) });

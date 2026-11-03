/** 🔆 `change-assumed-h-net` wire twin: the leaf payload `ChangeAssumedHNet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedHNet {
  newAssumedHNet: number;
}

export const parseChangeAssumedHNet: NormWireReader<ChangeAssumedHNet> = normWireObject<ChangeAssumedHNet>({ newAssumedHNet: normWireRequired(normWireNumber) });

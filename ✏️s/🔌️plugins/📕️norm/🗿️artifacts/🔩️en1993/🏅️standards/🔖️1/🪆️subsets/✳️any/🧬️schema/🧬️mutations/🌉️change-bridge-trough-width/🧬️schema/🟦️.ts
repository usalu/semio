/** 🌉️ `change-bridge-trough-width` wire twin: the leaf payload `ChangeBridgeTroughWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeTroughWidth {
  id: string;
  newTroughWidth: number;
}

export const parseChangeBridgeTroughWidth: NormWireReader<ChangeBridgeTroughWidth> = normWireObject<ChangeBridgeTroughWidth>({ id: normWireRequired(normWireString), newTroughWidth: normWireRequired(normWireNumber) });

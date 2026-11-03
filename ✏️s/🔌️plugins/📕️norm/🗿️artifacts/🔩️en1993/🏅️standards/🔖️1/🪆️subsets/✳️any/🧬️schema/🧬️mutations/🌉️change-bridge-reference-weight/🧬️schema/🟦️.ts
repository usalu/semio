/** 🌉️ `change-bridge-reference-weight` wire twin: the leaf payload `ChangeBridgeReferenceWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeReferenceWeight {
  id: string;
  newReferenceWeight: number;
}

export const parseChangeBridgeReferenceWeight: NormWireReader<ChangeBridgeReferenceWeight> = normWireObject<ChangeBridgeReferenceWeight>({ id: normWireRequired(normWireString), newReferenceWeight: normWireRequired(normWireNumber) });

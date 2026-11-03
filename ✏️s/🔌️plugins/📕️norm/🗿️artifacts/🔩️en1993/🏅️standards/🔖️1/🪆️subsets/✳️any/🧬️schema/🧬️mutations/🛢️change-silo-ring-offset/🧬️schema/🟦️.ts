/** 🛢️ `change-silo-ring-offset` wire twin: the leaf payload `ChangeSiloRingOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRingOffset {
  id: string;
  newRingOffset: number;
}

export const parseChangeSiloRingOffset: NormWireReader<ChangeSiloRingOffset> = normWireObject<ChangeSiloRingOffset>({ id: normWireRequired(normWireString), newRingOffset: normWireRequired(normWireNumber) });

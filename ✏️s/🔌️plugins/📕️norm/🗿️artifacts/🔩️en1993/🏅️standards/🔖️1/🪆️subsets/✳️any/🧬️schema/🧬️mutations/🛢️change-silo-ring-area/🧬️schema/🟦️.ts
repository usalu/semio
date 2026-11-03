/** 🛢️ `change-silo-ring-area` wire twin: the leaf payload `ChangeSiloRingArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRingArea {
  id: string;
  newRingArea: number;
}

export const parseChangeSiloRingArea: NormWireReader<ChangeSiloRingArea> = normWireObject<ChangeSiloRingArea>({ id: normWireRequired(normWireString), newRingArea: normWireRequired(normWireNumber) });

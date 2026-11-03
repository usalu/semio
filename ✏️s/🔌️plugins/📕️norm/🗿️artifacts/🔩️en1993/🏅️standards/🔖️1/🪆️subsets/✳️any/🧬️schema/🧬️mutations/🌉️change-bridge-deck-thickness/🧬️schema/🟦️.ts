/** 🌉️ `change-bridge-deck-thickness` wire twin: the leaf payload `ChangeBridgeDeckThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckThickness {
  id: string;
  newDeckThickness: number;
}

export const parseChangeBridgeDeckThickness: NormWireReader<ChangeBridgeDeckThickness> = normWireObject<ChangeBridgeDeckThickness>({ id: normWireRequired(normWireString), newDeckThickness: normWireRequired(normWireNumber) });

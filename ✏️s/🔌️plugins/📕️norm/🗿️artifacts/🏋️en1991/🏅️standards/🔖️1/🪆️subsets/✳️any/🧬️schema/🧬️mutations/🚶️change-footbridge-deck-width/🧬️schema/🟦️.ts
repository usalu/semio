/** 🚶 `change-footbridge-deck-width` wire twin: the leaf payload `ChangeFootbridgeDeckWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootbridgeDeckWidth {
  id: string;
  newDeckWidth: number;
}

export const parseChangeFootbridgeDeckWidth: NormWireReader<ChangeFootbridgeDeckWidth> = normWireObject<ChangeFootbridgeDeckWidth>({ id: normWireRequired(normWireString), newDeckWidth: normWireRequired(normWireNumber) });

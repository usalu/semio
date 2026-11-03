/** 🌉 `change-wind-bridge-deck-width` wire twin: the leaf payload `ChangeWindBridgeDeckWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeDeckWidth {
  id: string;
  newDeckWidth: number;
}

export const parseChangeWindBridgeDeckWidth: NormWireReader<ChangeWindBridgeDeckWidth> = normWireObject<ChangeWindBridgeDeckWidth>({ id: normWireRequired(normWireString), newDeckWidth: normWireRequired(normWireNumber) });

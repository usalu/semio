/** 🌉 `change-wind-bridge-deck-depth` wire twin: the leaf payload `ChangeWindBridgeDeckDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeDeckDepth {
  id: string;
  newDeckDepth: number;
}

export const parseChangeWindBridgeDeckDepth: NormWireReader<ChangeWindBridgeDeckDepth> = normWireObject<ChangeWindBridgeDeckDepth>({ id: normWireRequired(normWireString), newDeckDepth: normWireRequired(normWireNumber) });

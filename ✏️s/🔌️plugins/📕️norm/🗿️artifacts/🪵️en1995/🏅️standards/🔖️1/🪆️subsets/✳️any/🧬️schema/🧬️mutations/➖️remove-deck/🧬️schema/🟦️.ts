/** ➖️ `remove-deck` wire twin: the leaf payload `RemoveDeck`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveDeck {
  index: number;
}

export const parseRemoveDeck: NormWireReader<RemoveDeck> = normWireObject<RemoveDeck>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

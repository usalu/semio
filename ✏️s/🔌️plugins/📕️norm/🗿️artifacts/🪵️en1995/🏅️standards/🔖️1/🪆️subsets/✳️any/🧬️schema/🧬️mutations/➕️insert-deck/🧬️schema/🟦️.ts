/** ➕️ `insert-deck` wire twin: the leaf payload `InsertDeck`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Deck, parseDeck } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDeck {
  index: number;
  deck: Deck;
}

export const parseInsertDeck: NormWireReader<InsertDeck> = normWireObject<InsertDeck>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), deck: normWireRequired(parseDeck) });

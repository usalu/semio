/** 🏝️ `change-coast-or-island` wire twin: the leaf payload `ChangeCoastOrIsland`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCoastOrIsland {
  newCoastOrIsland: boolean;
}

export const parseChangeCoastOrIsland: NormWireReader<ChangeCoastOrIsland> = normWireObject<ChangeCoastOrIsland>({ newCoastOrIsland: normWireRequired(normWireBoolean) });

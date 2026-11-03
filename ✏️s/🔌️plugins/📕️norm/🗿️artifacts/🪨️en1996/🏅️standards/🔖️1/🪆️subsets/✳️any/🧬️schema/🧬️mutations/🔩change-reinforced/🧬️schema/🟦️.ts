/** 🔩 `change-reinforced` wire twin: the leaf payload `ChangeReinforced`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforced {
  id: string;
  newReinforced: boolean;
}

export const parseChangeReinforced: NormWireReader<ChangeReinforced> = normWireObject<ChangeReinforced>({ id: normWireRequired(normWireString), newReinforced: normWireRequired(normWireBoolean) });

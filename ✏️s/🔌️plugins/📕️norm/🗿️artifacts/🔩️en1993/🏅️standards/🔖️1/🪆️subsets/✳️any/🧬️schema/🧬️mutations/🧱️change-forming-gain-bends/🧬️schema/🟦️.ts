/** 🧱️ `change-forming-gain-bends` wire twin: the leaf payload `ChangeFormingGainBends`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFormingGainBends {
  id: string;
  newBends: number;
}

export const parseChangeFormingGainBends: NormWireReader<ChangeFormingGainBends> = normWireObject<ChangeFormingGainBends>({ id: normWireRequired(normWireString), newBends: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

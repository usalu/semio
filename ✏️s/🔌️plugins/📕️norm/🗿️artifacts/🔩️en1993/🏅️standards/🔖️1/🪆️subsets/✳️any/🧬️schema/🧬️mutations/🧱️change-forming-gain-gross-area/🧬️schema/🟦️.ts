/** 🧱️ `change-forming-gain-gross-area` wire twin: the leaf payload `ChangeFormingGainGrossArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFormingGainGrossArea {
  id: string;
  newGrossArea: number;
}

export const parseChangeFormingGainGrossArea: NormWireReader<ChangeFormingGainGrossArea> = normWireObject<ChangeFormingGainGrossArea>({ id: normWireRequired(normWireString), newGrossArea: normWireRequired(normWireNumber) });

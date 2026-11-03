/** 📏️ `change-forming-gain-forming` wire twin: the leaf payload `ChangeFormingGainForming`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FormingProcess, parseFormingProcess } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFormingGainForming {
  id: string;
  newForming: FormingProcess;
}

export const parseChangeFormingGainForming: NormWireReader<ChangeFormingGainForming> = normWireObject<ChangeFormingGainForming>({ id: normWireRequired(normWireString), newForming: normWireRequired(parseFormingProcess) });

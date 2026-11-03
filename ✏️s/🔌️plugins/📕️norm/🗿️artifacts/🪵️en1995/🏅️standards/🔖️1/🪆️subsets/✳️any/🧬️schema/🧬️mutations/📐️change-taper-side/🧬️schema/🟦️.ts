/** 📐️ `change-taper-side` wire twin: the leaf payload `ChangeTaperSide`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TensionSide, parseTensionSide } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTaperSide {
  taperId: string;
  newValue: TensionSide;
}

export const parseChangeTaperSide: NormWireReader<ChangeTaperSide> = normWireObject<ChangeTaperSide>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(parseTensionSide) });

/** 📏️ `change-buckling-length-sway` wire twin: the leaf payload `ChangeBucklingLengthSway`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameSway, parseFrameSway } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBucklingLengthSway {
  id: string;
  newSway: FrameSway;
}

export const parseChangeBucklingLengthSway: NormWireReader<ChangeBucklingLengthSway> = normWireObject<ChangeBucklingLengthSway>({ id: normWireRequired(normWireString), newSway: normWireRequired(parseFrameSway) });

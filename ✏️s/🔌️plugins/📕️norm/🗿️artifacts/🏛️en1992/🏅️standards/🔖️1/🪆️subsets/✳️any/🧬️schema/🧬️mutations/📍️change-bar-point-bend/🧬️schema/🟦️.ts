/** 📍 `change-bar-point-bend` wire twin: the leaf payload `ChangeBarPointBend`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BarBend, parseBarBend } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBarPointBend {
  memberId: string;
  barId: string;
  newBend: BarBend;
}

export const parseChangeBarPointBend: NormWireReader<ChangeBarPointBend> = normWireObject<ChangeBarPointBend>({ memberId: normWireRequired(normWireString), barId: normWireRequired(normWireString), newBend: normWireRequired(parseBarBend) });

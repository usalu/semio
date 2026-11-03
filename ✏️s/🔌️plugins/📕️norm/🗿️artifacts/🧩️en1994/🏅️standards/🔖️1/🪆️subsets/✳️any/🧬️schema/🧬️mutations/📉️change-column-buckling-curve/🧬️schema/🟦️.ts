/** 📉️ `change-column-buckling-curve` wire twin: the leaf payload `ChangeColumnBucklingCurve`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BucklingCurve, parseBucklingCurve } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColumnBucklingCurve {
  id: string;
  newBucklingCurve: BucklingCurve;
}

export const parseChangeColumnBucklingCurve: NormWireReader<ChangeColumnBucklingCurve> = normWireObject<ChangeColumnBucklingCurve>({ id: normWireRequired(normWireString), newBucklingCurve: normWireRequired(parseBucklingCurve) });

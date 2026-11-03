/** 📉 `change-fire-curve` wire twin: the leaf payload `ChangeFireCurve`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1991FireCurve, parseEn1991FireCurve } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireCurve {
  newFireCurve: En1991FireCurve;
}

export const parseChangeFireCurve: NormWireReader<ChangeFireCurve> = normWireObject<ChangeFireCurve>({ newFireCurve: normWireRequired(parseEn1991FireCurve) });

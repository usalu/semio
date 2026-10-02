/** 📉 `change-fire-curve` wire twin: the leaf payload `ChangeFireCurve`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1991FireCurve, parseEn1991FireCurve } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireCurve {
  newFireCurve: En1991FireCurve;
}

export const parseChangeFireCurve: NormWireReader<ChangeFireCurve> = normWireObject<ChangeFireCurve>({ newFireCurve: normWireRequired(parseEn1991FireCurve) });

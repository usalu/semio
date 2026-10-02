/** 🔥 `change-fire-mode` wire twin: the leaf payload `ChangeFireMode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1991FireMode, parseEn1991FireMode } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireMode {
  newFireMode: En1991FireMode;
}

export const parseChangeFireMode: NormWireReader<ChangeFireMode> = normWireObject<ChangeFireMode>({ newFireMode: normWireRequired(parseEn1991FireMode) });

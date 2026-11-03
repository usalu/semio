/** 🔥 `change-fire-mode` wire twin: the leaf payload `ChangeFireMode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1991FireMode, parseEn1991FireMode } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireMode {
  newFireMode: En1991FireMode;
}

export const parseChangeFireMode: NormWireReader<ChangeFireMode> = normWireObject<ChangeFireMode>({ newFireMode: normWireRequired(parseEn1991FireMode) });

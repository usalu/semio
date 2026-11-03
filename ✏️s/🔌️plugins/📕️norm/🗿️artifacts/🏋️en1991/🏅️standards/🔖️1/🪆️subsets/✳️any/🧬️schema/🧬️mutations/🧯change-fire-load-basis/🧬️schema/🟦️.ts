/** 🧯 `change-fire-load-basis` wire twin: the leaf payload `ChangeFireLoadBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1991FireLoadBasis, parseEn1991FireLoadBasis } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireLoadBasis {
  newFireLoadBasis: En1991FireLoadBasis;
}

export const parseChangeFireLoadBasis: NormWireReader<ChangeFireLoadBasis> = normWireObject<ChangeFireLoadBasis>({ newFireLoadBasis: normWireRequired(parseEn1991FireLoadBasis) });

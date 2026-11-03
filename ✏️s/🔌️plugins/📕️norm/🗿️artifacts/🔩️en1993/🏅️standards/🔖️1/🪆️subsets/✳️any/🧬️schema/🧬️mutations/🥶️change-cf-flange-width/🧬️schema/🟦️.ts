/** 🥶️ `change-cf-flange-width` wire twin: the leaf payload `ChangeCfFlangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfFlangeWidth {
  id: string;
  newFlangeWidth: number;
}

export const parseChangeCfFlangeWidth: NormWireReader<ChangeCfFlangeWidth> = normWireObject<ChangeCfFlangeWidth>({ id: normWireRequired(normWireString), newFlangeWidth: normWireRequired(normWireNumber) });

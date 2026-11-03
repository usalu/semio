/** 🥶️ `change-cf-opposite-flange-width` wire twin: the leaf payload `ChangeCfOppositeFlangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfOppositeFlangeWidth {
  id: string;
  newOppositeFlangeWidth: number;
}

export const parseChangeCfOppositeFlangeWidth: NormWireReader<ChangeCfOppositeFlangeWidth> = normWireObject<ChangeCfOppositeFlangeWidth>({ id: normWireRequired(normWireString), newOppositeFlangeWidth: normWireRequired(normWireNumber) });

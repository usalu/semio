/** ✏️ `change-system-alpha-u-over-alpha1` wire twin: the leaf payload `ChangeSystemAlphaUOverAlpha1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSystemAlphaUOverAlpha1 {
  mutation: "changeSystemAlphaUOverAlpha1";
  buildingId: string;
  id: string;
  newAlphaUOverAlpha1: number;
}

export const parseChangeSystemAlphaUOverAlpha1: NormWireReader<ChangeSystemAlphaUOverAlpha1> = normWireObject<ChangeSystemAlphaUOverAlpha1>({ mutation: normWireRequired(normWireLiteral("changeSystemAlphaUOverAlpha1")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newAlphaUOverAlpha1: normWireRequired(normWireNumber) });

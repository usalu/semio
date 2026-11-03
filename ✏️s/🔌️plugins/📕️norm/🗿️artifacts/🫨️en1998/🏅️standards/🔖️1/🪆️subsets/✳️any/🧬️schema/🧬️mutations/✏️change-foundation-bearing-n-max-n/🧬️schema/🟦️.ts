/** ✏️ `change-foundation-bearing-n-max-n` wire twin: the leaf payload `ChangeFoundationBearingNMaxN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBearingNMaxN {
  mutation: "changeFoundationBearingNMaxN";
  id: string;
  newBearingNMaxN: number;
}

export const parseChangeFoundationBearingNMaxN: NormWireReader<ChangeFoundationBearingNMaxN> = normWireObject<ChangeFoundationBearingNMaxN>({ mutation: normWireRequired(normWireLiteral("changeFoundationBearingNMaxN")), id: normWireRequired(normWireString), newBearingNMaxN: normWireRequired(normWireNumber) });

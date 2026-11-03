/** ✏️ `change-foundation-pile-head-shear-n` wire twin: the leaf payload `ChangeFoundationPileHeadShearN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileHeadShearN {
  mutation: "changeFoundationPileHeadShearN";
  id: string;
  newPileHeadShearN: number;
}

export const parseChangeFoundationPileHeadShearN: NormWireReader<ChangeFoundationPileHeadShearN> = normWireObject<ChangeFoundationPileHeadShearN>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileHeadShearN")), id: normWireRequired(normWireString), newPileHeadShearN: normWireRequired(normWireNumber) });

/** ✏️ `change-foundation-pile-head-fixed` wire twin: the leaf payload `ChangeFoundationPileHeadFixed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileHeadFixed {
  mutation: "changeFoundationPileHeadFixed";
  id: string;
  newPileHeadFixed: number;
}

export const parseChangeFoundationPileHeadFixed: NormWireReader<ChangeFoundationPileHeadFixed> = normWireObject<ChangeFoundationPileHeadFixed>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileHeadFixed")), id: normWireRequired(normWireString), newPileHeadFixed: normWireRequired(normWireNumber) });

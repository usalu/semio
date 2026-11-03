/** ✏️ `change-foundation-pile-up-vs-ms` wire twin: the leaf payload `ChangeFoundationPileUpVsMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileUpVsMS {
  mutation: "changeFoundationPileUpVsMS";
  id: string;
  newPileUpperVsMS: number;
}

export const parseChangeFoundationPileUpVsMS: NormWireReader<ChangeFoundationPileUpVsMS> = normWireObject<ChangeFoundationPileUpVsMS>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileUpVsMS")), id: normWireRequired(normWireString), newPileUpperVsMS: normWireRequired(normWireNumber) });

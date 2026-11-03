/** ✏️ `change-foundation-pile-iface-m` wire twin: the leaf payload `ChangeFoundationPileIfaceM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileIfaceM {
  mutation: "changeFoundationPileIfaceM";
  id: string;
  newPileInterfaceDepthM: number;
}

export const parseChangeFoundationPileIfaceM: NormWireReader<ChangeFoundationPileIfaceM> = normWireObject<ChangeFoundationPileIfaceM>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileIfaceM")), id: normWireRequired(normWireString), newPileInterfaceDepthM: normWireRequired(normWireNumber) });

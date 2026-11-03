/** ✏️ `change-foundation-pile-ks-nm3` wire twin: the leaf payload `ChangeFoundationPileKsNM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileKsNM3 {
  mutation: "changeFoundationPileKsNM3";
  id: string;
  newPileSubgradeModulusNM3: number;
}

export const parseChangeFoundationPileKsNM3: NormWireReader<ChangeFoundationPileKsNM3> = normWireObject<ChangeFoundationPileKsNM3>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileKsNM3")), id: normWireRequired(normWireString), newPileSubgradeModulusNM3: normWireRequired(normWireNumber) });

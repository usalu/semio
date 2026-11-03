/** ✏️ `change-foundation-pile-modulus-pa` wire twin: the leaf payload `ChangeFoundationPileModulusPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileModulusPa {
  mutation: "changeFoundationPileModulusPa";
  id: string;
  newPileModulusPa: number;
}

export const parseChangeFoundationPileModulusPa: NormWireReader<ChangeFoundationPileModulusPa> = normWireObject<ChangeFoundationPileModulusPa>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileModulusPa")), id: normWireRequired(normWireString), newPileModulusPa: normWireRequired(normWireNumber) });

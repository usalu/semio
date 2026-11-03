/** ✏️ `change-foundation-k-foundation` wire twin: the leaf payload `ChangeFoundationKFoundation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationKFoundation {
  mutation: "changeFoundationKFoundation";
  id: string;
  newKFoundation: number;
}

export const parseChangeFoundationKFoundation: NormWireReader<ChangeFoundationKFoundation> = normWireObject<ChangeFoundationKFoundation>({ mutation: normWireRequired(normWireLiteral("changeFoundationKFoundation")), id: normWireRequired(normWireString), newKFoundation: normWireRequired(normWireNumber) });

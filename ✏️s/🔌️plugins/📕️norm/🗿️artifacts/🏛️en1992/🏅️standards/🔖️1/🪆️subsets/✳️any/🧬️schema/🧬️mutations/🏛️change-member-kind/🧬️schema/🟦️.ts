/** 🏛️ `change-member-kind` wire twin: the leaf payload `ChangeMemberKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberKind {
  memberId: string;
  newKind: "Beam" | "Slab" | "Column" | "Wall" | "FlatSlab" | "RibbedSlab" | "TensionMember" | "Bridge" | "LiquidRetaining";
}

export const parseChangeMemberKind: NormWireReader<ChangeMemberKind> = normWireObject<ChangeMemberKind>({ memberId: normWireRequired(normWireString), newKind: normWireRequired(normWireLiteral("Beam", "Slab", "Column", "Wall", "FlatSlab", "RibbedSlab", "TensionMember", "Bridge", "LiquidRetaining")) });

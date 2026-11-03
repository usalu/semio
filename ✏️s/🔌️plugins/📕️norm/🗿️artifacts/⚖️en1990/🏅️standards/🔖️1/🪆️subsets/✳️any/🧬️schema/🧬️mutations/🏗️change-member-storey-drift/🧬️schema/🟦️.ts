/** 🏗️ `change-member-storey-drift` wire twin: the leaf payload `ChangeMemberStoreyDrift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStoreyDrift {
  mutation: "changeMemberStoreyDrift";
  id: string;
  newStoreyDrift: number;
}

export const parseChangeMemberStoreyDrift: NormWireReader<ChangeMemberStoreyDrift> = normWireObject<ChangeMemberStoreyDrift>({ mutation: normWireRequired(normWireLiteral("changeMemberStoreyDrift")), id: normWireRequired(normWireString), newStoreyDrift: normWireRequired(normWireNumber) });

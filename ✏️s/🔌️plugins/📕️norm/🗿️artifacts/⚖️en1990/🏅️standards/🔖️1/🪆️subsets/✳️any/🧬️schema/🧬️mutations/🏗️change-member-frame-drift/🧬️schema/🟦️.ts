/** 🏗️ `change-member-frame-drift` wire twin: the leaf payload `ChangeMemberFrameDrift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberFrameDrift {
  mutation: "changeMemberFrameDrift";
  id: string;
  newFrameDrift: number;
}

export const parseChangeMemberFrameDrift: NormWireReader<ChangeMemberFrameDrift> = normWireObject<ChangeMemberFrameDrift>({ mutation: normWireRequired(normWireLiteral("changeMemberFrameDrift")), id: normWireRequired(normWireString), newFrameDrift: normWireRequired(normWireNumber) });

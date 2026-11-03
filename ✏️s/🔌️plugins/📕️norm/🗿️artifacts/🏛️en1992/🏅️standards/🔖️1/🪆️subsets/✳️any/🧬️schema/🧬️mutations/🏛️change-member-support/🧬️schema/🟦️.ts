/** 🏛️ `change-member-support` wire twin: the leaf payload `ChangeMemberSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSupport {
  memberId: string;
  newSupport: "SimplySupported" | "Continuous" | "Cantilever" | "Fixed";
}

export const parseChangeMemberSupport: NormWireReader<ChangeMemberSupport> = normWireObject<ChangeMemberSupport>({ memberId: normWireRequired(normWireString), newSupport: normWireRequired(normWireLiteral("SimplySupported", "Continuous", "Cantilever", "Fixed")) });

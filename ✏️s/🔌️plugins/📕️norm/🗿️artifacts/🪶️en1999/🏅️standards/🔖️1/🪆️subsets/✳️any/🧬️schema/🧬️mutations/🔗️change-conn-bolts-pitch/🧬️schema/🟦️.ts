/** 🔗️ `change-conn-bolts-pitch` wire twin: the leaf payload `ChangeConnBoltsPitch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsPitch {
  mutation: "changeConnBoltsPitch";
  connectionId: string;
  newBoltsPitch: number;
}

export const parseChangeConnBoltsPitch: NormWireReader<ChangeConnBoltsPitch> = normWireObject<ChangeConnBoltsPitch>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsPitch")), connectionId: normWireRequired(normWireString), newBoltsPitch: normWireRequired(normWireNumber) });

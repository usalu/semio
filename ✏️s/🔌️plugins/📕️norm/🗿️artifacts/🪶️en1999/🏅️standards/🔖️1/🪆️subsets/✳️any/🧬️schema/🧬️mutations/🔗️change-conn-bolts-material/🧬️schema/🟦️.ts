/** 🔗️ `change-conn-bolts-material` wire twin: the leaf payload `ChangeConnBoltsMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsMaterial {
  mutation: "changeConnBoltsMaterial";
  connectionId: string;
  newBoltsMaterial: string;
}

export const parseChangeConnBoltsMaterial: NormWireReader<ChangeConnBoltsMaterial> = normWireObject<ChangeConnBoltsMaterial>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsMaterial")), connectionId: normWireRequired(normWireString), newBoltsMaterial: normWireRequired(normWireString) });

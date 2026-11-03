/** 🔗️ `change-conn-bolts-diameter` wire twin: the leaf payload `ChangeConnBoltsDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsDiameter {
  mutation: "changeConnBoltsDiameter";
  connectionId: string;
  newBoltsDiameter: number;
}

export const parseChangeConnBoltsDiameter: NormWireReader<ChangeConnBoltsDiameter> = normWireObject<ChangeConnBoltsDiameter>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsDiameter")), connectionId: normWireRequired(normWireString), newBoltsDiameter: normWireRequired(normWireNumber) });

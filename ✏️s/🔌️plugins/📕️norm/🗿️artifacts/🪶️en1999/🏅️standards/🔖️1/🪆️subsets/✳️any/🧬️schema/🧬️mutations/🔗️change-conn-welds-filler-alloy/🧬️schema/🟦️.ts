/** 🔗️ `change-conn-welds-filler-alloy` wire twin: the leaf payload `ChangeConnWeldsFillerAlloy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnWeldsFillerAlloy {
  mutation: "changeConnWeldsFillerAlloy";
  connectionId: string;
  newWeldsFillerAlloy: string;
}

export const parseChangeConnWeldsFillerAlloy: NormWireReader<ChangeConnWeldsFillerAlloy> = normWireObject<ChangeConnWeldsFillerAlloy>({ mutation: normWireRequired(normWireLiteral("changeConnWeldsFillerAlloy")), connectionId: normWireRequired(normWireString), newWeldsFillerAlloy: normWireRequired(normWireString) });

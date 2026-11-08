/** 🗂️ `remove-fire-insulation` wire twin: the leaf payload `RemoveFireInsulation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFireInsulation {
  mutation: "removeFireInsulation";
  id: string;
}

export const parseRemoveFireInsulation: NormWireReader<RemoveFireInsulation> = normWireObject<RemoveFireInsulation>({ mutation: normWireRequired(normWireLiteral("removeFireInsulation")), id: normWireRequired(normWireString) });

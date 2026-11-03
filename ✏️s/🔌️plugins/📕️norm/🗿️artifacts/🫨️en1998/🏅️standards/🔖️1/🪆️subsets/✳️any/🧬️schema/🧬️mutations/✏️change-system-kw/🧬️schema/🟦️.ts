/** ✏️ `change-system-kw` wire twin: the leaf payload `ChangeSystemKW`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSystemKW {
  mutation: "changeSystemKW";
  buildingId: string;
  id: string;
  newKW: number;
}

export const parseChangeSystemKW: NormWireReader<ChangeSystemKW> = normWireObject<ChangeSystemKW>({ mutation: normWireRequired(normWireLiteral("changeSystemKW")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newKW: normWireRequired(normWireNumber) });

/** ✏️ `change-bridge-pier-cracked-inertia-ratio` wire twin: the leaf payload `ChangeBridgePierCrackedInertiaRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePierCrackedInertiaRatio {
  mutation: "changeBridgePierCrackedInertiaRatio";
  id: string;
  newPierCrackedInertiaRatio: number;
}

export const parseChangeBridgePierCrackedInertiaRatio: NormWireReader<ChangeBridgePierCrackedInertiaRatio> = normWireObject<ChangeBridgePierCrackedInertiaRatio>({ mutation: normWireRequired(normWireLiteral("changeBridgePierCrackedInertiaRatio")), id: normWireRequired(normWireString), newPierCrackedInertiaRatio: normWireRequired(normWireNumber) });

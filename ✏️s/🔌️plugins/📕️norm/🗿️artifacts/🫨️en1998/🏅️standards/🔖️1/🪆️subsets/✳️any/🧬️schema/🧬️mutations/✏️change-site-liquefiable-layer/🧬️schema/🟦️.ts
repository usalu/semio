/** ✏️ `change-site-liquefiable-layer` wire twin: the leaf payload `ChangeSiteLiquefiableLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteLiquefiableLayer {
  mutation: "changeSiteLiquefiableLayer";
  newLiquefiableLayer: boolean;
}

export const parseChangeSiteLiquefiableLayer: NormWireReader<ChangeSiteLiquefiableLayer> = normWireObject<ChangeSiteLiquefiableLayer>({ mutation: normWireRequired(normWireLiteral("changeSiteLiquefiableLayer")), newLiquefiableLayer: normWireRequired(normWireBoolean) });

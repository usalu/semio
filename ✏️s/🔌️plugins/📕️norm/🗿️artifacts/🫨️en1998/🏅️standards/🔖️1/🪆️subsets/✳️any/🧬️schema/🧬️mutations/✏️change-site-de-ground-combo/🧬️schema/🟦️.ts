/** ✏️ `change-site-de-ground-combo` wire twin: the leaf payload `ChangeSiteDeGroundCombo`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteDeGroundCombo {
  mutation: "changeSiteDeGroundCombo";
  newDeGroundCombo: string;
}

export const parseChangeSiteDeGroundCombo: NormWireReader<ChangeSiteDeGroundCombo> = normWireObject<ChangeSiteDeGroundCombo>({ mutation: normWireRequired(normWireLiteral("changeSiteDeGroundCombo")), newDeGroundCombo: normWireRequired(normWireString) });

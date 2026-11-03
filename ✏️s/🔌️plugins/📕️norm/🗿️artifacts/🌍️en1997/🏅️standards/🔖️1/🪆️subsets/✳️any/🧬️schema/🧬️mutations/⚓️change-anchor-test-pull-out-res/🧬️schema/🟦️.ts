/** ⚓️ `change-anchor-test-pull-out-res` wire twin: the leaf payload `ChangeAnchorTestPullOutRes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorTestPullOutRes {
  mutation: "changeAnchorTestPullOutRes";
  anchorId: string;
  id: string;
  newPullOutResistance: number;
}

export const parseChangeAnchorTestPullOutRes: NormWireReader<ChangeAnchorTestPullOutRes> = normWireObject<ChangeAnchorTestPullOutRes>({ mutation: normWireRequired(normWireLiteral("changeAnchorTestPullOutRes")), anchorId: normWireRequired(normWireString), id: normWireRequired(normWireString), newPullOutResistance: normWireRequired(normWireNumber) });

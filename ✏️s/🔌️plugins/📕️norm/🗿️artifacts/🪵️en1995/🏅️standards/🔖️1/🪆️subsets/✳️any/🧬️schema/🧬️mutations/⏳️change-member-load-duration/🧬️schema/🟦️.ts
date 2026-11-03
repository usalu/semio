/** ⏳️ `change-member-load-duration` wire twin: the leaf payload `ChangeMemberLoadDuration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LoadDurationClass, parseLoadDurationClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberLoadDuration {
  memberId: string;
  actionId: string;
  newValue: LoadDurationClass;
}

export const parseChangeMemberLoadDuration: NormWireReader<ChangeMemberLoadDuration> = normWireObject<ChangeMemberLoadDuration>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(parseLoadDurationClass) });

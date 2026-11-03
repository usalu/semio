/** 📏️ `change-member-load-application` wire twin: the leaf payload `ChangeMemberLoadApplication`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LoadApplication, parseLoadApplication } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberLoadApplication {
  id: string;
  newLoadApplication: LoadApplication;
}

export const parseChangeMemberLoadApplication: NormWireReader<ChangeMemberLoadApplication> = normWireObject<ChangeMemberLoadApplication>({ id: normWireRequired(normWireString), newLoadApplication: normWireRequired(parseLoadApplication) });

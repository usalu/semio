/** 📏️ `change-buckling-length-restraint-b` wire twin: the leaf payload `ChangeBucklingLengthRestraintB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type NodeBRestraint, parseNodeBRestraint } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBucklingLengthRestraintB {
  id: string;
  newRestraintB: NodeBRestraint;
}

export const parseChangeBucklingLengthRestraintB: NormWireReader<ChangeBucklingLengthRestraintB> = normWireObject<ChangeBucklingLengthRestraintB>({ id: normWireRequired(normWireString), newRestraintB: normWireRequired(parseNodeBRestraint) });

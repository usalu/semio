/** 📝️ `En1995Mutation` wire twin: the serialized text form this facet's grammar and codec speak, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireAny, type NormWireReader } from "../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeMemberB, parseChangeMemberB } from "../↔️change-member-b/🧬️schema/🟦️.ts";
import { type ChangeMemberH, parseChangeMemberH } from "../↕️change-member-h/🧬️schema/🟦️.ts";
import { type InsertConnection, parseInsertConnection } from "../➕️insert-connection/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "../➕️insert-member/🧬️schema/🟦️.ts";
import { parseRemoveConnection, type RemoveConnection } from "../➖️remove-connection/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "../➖️remove-member/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "../🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeConnectionNumber, parseChangeConnectionNumber } from "../🔢️change-connection-number/🧬️schema/🟦️.ts";
import { type ChangeMemberStrengthClass, parseChangeMemberStrengthClass } from "../🛡️change-member-strength-class/🧬️schema/🟦️.ts";

export type En1995Mutation = ChangeAnnex | InsertMember | RemoveMember | ChangeMemberH | ChangeMemberB | ChangeMemberStrengthClass | ChangeConnectionNumber | InsertConnection | RemoveConnection;

export const parseEn1995Mutation: NormWireReader<En1995Mutation> = normWireAny<[ChangeAnnex, InsertMember, RemoveMember, ChangeMemberH, ChangeMemberB, ChangeMemberStrengthClass, ChangeConnectionNumber, InsertConnection, RemoveConnection]>(parseChangeAnnex, parseInsertMember, parseRemoveMember, parseChangeMemberH, parseChangeMemberB, parseChangeMemberStrengthClass, parseChangeConnectionNumber, parseInsertConnection, parseRemoveConnection);

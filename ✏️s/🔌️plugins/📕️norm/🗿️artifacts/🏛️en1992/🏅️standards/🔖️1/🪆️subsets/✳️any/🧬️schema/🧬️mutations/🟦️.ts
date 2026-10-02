/** 🧺️ `En1992Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeBarLayerCount, parseChangeBarLayerCount } from "./#️⃣change-bar-layer-count/🧬️schema/🟦️.ts";
import { type ChangeMemberWidth, parseChangeMemberWidth } from "./↔️change-member-width/🧬️schema/🟦️.ts";
import { type ChangeMemberHeight, parseChangeMemberHeight } from "./↕️change-member-height/🧬️schema/🟦️.ts";
import { type ChangeActionVk, parseChangeActionVk } from "./↘️change-action-vk/🧬️schema/🟦️.ts";
import { type InsertAnchor, parseInsertAnchor } from "./⚓️insert-anchor/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { type ChangeActionMk, parseChangeActionMk } from "./⤴️change-action-mk/🧬️schema/🟦️.ts";
import { type ChangeBarLayerDiameter, parseChangeBarLayerDiameter } from "./⭕change-bar-layer-diameter/🧬️schema/🟦️.ts";
import { type ChangeMemberSpan, parseChangeMemberSpan } from "./🌉️change-member-span/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeMemberExposure, parseChangeMemberExposure } from "./🌦️change-member-exposure/🧬️schema/🟦️.ts";
import { type ChangeActionNk, parseChangeActionNk } from "./🏋️change-action-nk/🧬️schema/🟦️.ts";
import { type ChangeTitle, parseChangeTitle } from "./🏷️change-title/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLife, parseChangeDesignWorkingLife } from "./📅️change-design-working-life/🧬️schema/🟦️.ts";
import { type ChangeAnchorHEf, parseChangeAnchorHEf } from "./📍change-anchor-h-ef/🧬️schema/🟦️.ts";
import { type ChangeDeltaCDev, parseChangeDeltaCDev } from "./📏️change-delta-c-dev/🧬️schema/🟦️.ts";
import { type ChangeMemberEffectiveDepth, parseChangeMemberEffectiveDepth } from "./📐️change-member-effective-depth/🧬️schema/🟦️.ts";
import { parseReorderMembers, type ReorderMembers } from "./🔀️reorder-members/🧬️schema/🟦️.ts";
import { type ChangeMemberAxisDistance, parseChangeMemberAxisDistance } from "./🔥change-member-axis-distance/🧬️schema/🟦️.ts";
import { type ChangeMemberFireRating, parseChangeMemberFireRating } from "./🔥️change-member-fire-rating/🧬️schema/🟦️.ts";
import { type ChangeReinforcementFYk, parseChangeReinforcementFYk } from "./🔩change-reinforcement-f-yk/🧬️schema/🟦️.ts";
import { parseRemoveAnchor, type RemoveAnchor } from "./🗑️remove-anchor/🧬️schema/🟦️.ts";
import { type ChangeCementType, parseChangeCementType } from "./🧪change-cement-type/🧬️schema/🟦️.ts";
import { type ChangeConcreteFCk, parseChangeConcreteFCk } from "./🧱change-concrete-f-ck/🧬️schema/🟦️.ts";
import { type ChangeAnchorAs, parseChangeAnchorAs } from "./🧷change-anchor-as/🧬️schema/🟦️.ts";
import { type ChangeMemberStirrupSpacing, parseChangeMemberStirrupSpacing } from "./🪢change-member-stirrup-spacing/🧬️schema/🟦️.ts";
import { type ChangeMemberCover, parseChangeMemberCover } from "./🛡️change-member-cover/🧬️schema/🟦️.ts";

export type En1992Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeTitle: ChangeTitle }
  | { ChangeDesignWorkingLife: ChangeDesignWorkingLife }
  | { ChangeDeltaCDev: ChangeDeltaCDev }
  | { ChangeCementType: ChangeCementType }
  | { ChangeConcreteFCk: ChangeConcreteFCk }
  | { ChangeReinforcementFYk: ChangeReinforcementFYk }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { ReorderMembers: ReorderMembers }
  | { ChangeMemberWidth: ChangeMemberWidth }
  | { ChangeMemberHeight: ChangeMemberHeight }
  | { ChangeMemberEffectiveDepth: ChangeMemberEffectiveDepth }
  | { ChangeMemberCover: ChangeMemberCover }
  | { ChangeMemberExposure: ChangeMemberExposure }
  | { ChangeMemberSpan: ChangeMemberSpan }
  | { ChangeMemberStirrupSpacing: ChangeMemberStirrupSpacing }
  | { ChangeMemberAxisDistance: ChangeMemberAxisDistance }
  | { ChangeMemberFireRating: ChangeMemberFireRating }
  | { ChangeBarLayerCount: ChangeBarLayerCount }
  | { ChangeBarLayerDiameter: ChangeBarLayerDiameter }
  | { ChangeActionMk: ChangeActionMk }
  | { ChangeActionNk: ChangeActionNk }
  | { ChangeActionVk: ChangeActionVk }
  | { InsertAnchor: InsertAnchor }
  | { RemoveAnchor: RemoveAnchor }
  | { ChangeAnchorHEf: ChangeAnchorHEf }
  | { ChangeAnchorAs: ChangeAnchorAs };

export const parseEn1992Mutation: NormWireReader<En1992Mutation> = normWireExternal<En1992Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeTitle: parseChangeTitle,
  ChangeDesignWorkingLife: parseChangeDesignWorkingLife,
  ChangeDeltaCDev: parseChangeDeltaCDev,
  ChangeCementType: parseChangeCementType,
  ChangeConcreteFCk: parseChangeConcreteFCk,
  ChangeReinforcementFYk: parseChangeReinforcementFYk,
  InsertMember: parseInsertMember,
  RemoveMember: parseRemoveMember,
  ReorderMembers: parseReorderMembers,
  ChangeMemberWidth: parseChangeMemberWidth,
  ChangeMemberHeight: parseChangeMemberHeight,
  ChangeMemberEffectiveDepth: parseChangeMemberEffectiveDepth,
  ChangeMemberCover: parseChangeMemberCover,
  ChangeMemberExposure: parseChangeMemberExposure,
  ChangeMemberSpan: parseChangeMemberSpan,
  ChangeMemberStirrupSpacing: parseChangeMemberStirrupSpacing,
  ChangeMemberAxisDistance: parseChangeMemberAxisDistance,
  ChangeMemberFireRating: parseChangeMemberFireRating,
  ChangeBarLayerCount: parseChangeBarLayerCount,
  ChangeBarLayerDiameter: parseChangeBarLayerDiameter,
  ChangeActionMk: parseChangeActionMk,
  ChangeActionNk: parseChangeActionNk,
  ChangeActionVk: parseChangeActionVk,
  InsertAnchor: parseInsertAnchor,
  RemoveAnchor: parseRemoveAnchor,
  ChangeAnchorHEf: parseChangeAnchorHEf,
  ChangeAnchorAs: parseChangeAnchorAs,
});

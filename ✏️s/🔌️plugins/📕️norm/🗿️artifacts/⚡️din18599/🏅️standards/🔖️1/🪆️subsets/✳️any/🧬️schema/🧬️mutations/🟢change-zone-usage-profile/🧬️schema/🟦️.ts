/** 🟢 `change-zone-usage-profile` wire twin: the leaf payload `ChangeZoneUsageProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599UsageProfile, parseDin18599UsageProfile } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeZoneUsageProfile {
  mutation: "changezoneusageprofile";
  zoneId: string;
  newUsageProfile: Din18599UsageProfile;
}

export const parseChangeZoneUsageProfile: NormWireReader<ChangeZoneUsageProfile> = normWireObject<ChangeZoneUsageProfile>({ mutation: normWireRequired(normWireLiteral("changezoneusageprofile")), zoneId: normWireRequired(normWireString), newUsageProfile: normWireRequired(parseDin18599UsageProfile) });

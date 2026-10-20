/** 🗃️ Content-addressed Cargo compile-unit cache: per-agent lanes, per-unit single-flight claims, host-wide compile slots, bounded cleanup and per-unit admission memos.
 * Cargo runs `bun 📜️script.ts <rustc arguments>` in place of rustc; a hit restores the recorded outputs, a miss compiles once across all lanes through the memory-admitting scheduler.
 * @see ./🧬️schema/🔣️.json */
export {UNIT_CACHE_POLICY_V1,parseRustcUnitV1,rustcBypassV1,unitKeyV1} from "./🔏️key/🟦️.ts";
export {acquireSlotV1,acquireUnitClaimV1,tryClaimV1} from "./🔒️lock/🟦️.ts";
export {lookupUnitV1,publishUnitV1,restoreUnitV1} from "./💾️store/🟦️.ts";
export {acquireLaneV1,resolveLaneIdV1,systemAncestryV1,DEFAULT_LANE_ID_V1} from "./🧵️lane/🟦️.ts";
export {collectGarbageV1,planGcV1} from "./🧹️gc/🟦️.ts";
export {focusFromCargoArgumentsV1,unitCacheEnvironmentV1,unitCacheRootV1} from "./🚪️launch/🟦️.ts";
export {unitAdmissionMemoV1} from "./📖️admission/🟦️.ts";

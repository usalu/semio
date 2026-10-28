/** ⏲️ The limits of the cache collector, read from the sibling schema document (`🔣️.json`, definition `Limits`). */
import schema from "./🔣️.json" with {type: "json"};

/** 🧹️ The waits, budgets, intervals and progress cadences of the cache collector; the schema document is their single source. */
export const CACHE_GC_LIMITS_V1 = Object.freeze({...schema.definitions.Limits.const});

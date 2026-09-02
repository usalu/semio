/** 📦️ forms facet WASM facades — mirrors the declaration-tree taxonomy (ticket
 * 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). `🪓️decomposer` and the flat artifact-level
 * `🧬️schema`/`🚪️io` targets this file pointed at pre-migration never existed in the current tree
 * (confirmed: zero matching directories) — replaced with the real standard/subset-scoped paths;
 * the native-codec facets (`📸️snapshot`/`🔺️diff`/`🧬️mutations`) moved from `🧬️schema` to `🚪️io`
 * (design.md §1 CORRECTION). */
export * as forms_schema from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts";
export * as forms_snapshot from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export * as forms_snapshot_text from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🟦️.ts";
export * as forms_snapshot_binary from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🟦️.ts";
export * as forms_diff from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
export * as forms_diff_text from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🟦️.ts";
export * as forms_diff_binary from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🟦️.ts";
export * as forms_mutations from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export * as forms_mutations_text from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🟦️.ts";
export * as forms_mutations_binary from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🟦️.ts";
export * as forms_io from "../../🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts";

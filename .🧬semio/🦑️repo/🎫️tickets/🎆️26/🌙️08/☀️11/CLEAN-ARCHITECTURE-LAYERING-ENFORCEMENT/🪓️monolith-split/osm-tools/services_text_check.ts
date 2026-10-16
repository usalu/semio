import { policyReadRustPolicySource } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
const text = policyReadRustPolicySource("/Users/ueli/Documents/semio", "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs");
const start = text.indexOf("//#region 📄️FixedFilePage"), end = text.indexOf("//#endregion 📄️FixedFilePage");
console.log("fixed file page test name present:", text.includes("fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page"));
console.log("FixedFilePage slice bytes:", end - start, "(start", start, "end", end, ")");

# EPW Paid Frame and Native Corpus Audit

Read-only source audit, 2026-10-14. No test execution or runtime qualification. Current Source94825 remains the root-owned invocation. Paths below are relative to `/Users/ueli/Documents/semio`.

## Concrete construction issue

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:333–347`: `EpwSnapshot::default()` allocates its schema at336 through `STDIO_EPW_DOCUMENT_SCHEMA.into()`. Construct the retained receiving snapshot explicitly with an empty schema String and empty nested defaults; copy the authored schema through the original Port. `EpwRecord::default()` at63–66 contains empty Strings and can be pushed into already admitted record backing after paid item work. Current record API has borrowed `field_at`111 and consuming `set_field_at`154, but no mutable borrowed field accessor. A direct `field_at_mut -> Option<&mut String>` is the appropriate copy destination; avoid a temporary `[String;35]`, consuming setter candidate, or JSON reconstruction.

## Frame contract and admission boundaries

Framework owner: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs`.

Five ID-index Vecs plus period/record order Vecs are sufficient for this schema. Keep them and the complete partial snapshot/projection in the retained Frame before any fallible copy. Admit each backing with Port.vector29, each append/swap/copy with Port.work28, and texts through Port.text30–36. Count multiplication is checked against overflow/isize. Original wallet admissions are demands against the existing grant, not authority to replace or replenish it. Minimum nonzero grant is not a success guarantee: schema tokens, ID sorting, comparisons, rows/cells and text each spend separate items. All zero and partial failures must leave the actual retained Frame available for paid retirement.

Port.work28 checkpoints both SQL and native controls before item mutation. Port.text30–36 preserves the actual SQL observer refusal instead of collapsing it into a generic native cancellation; retain this path for long NUL text. Do not construct error strings, temporary vectors or cloned metadata on refusal. Schema validation87–99 borrows declaration tokens and validates five-table census/declarations; it does not validate row widths, cell types, IDs, parents or ordinals. check_database75–79 checks aggregate ceilings and debits reconstruction scalar bytes once on R. Actual reconstructed text also needs SQL reconstruction-byte admission before the retained original text copy; a borrowed directional validator must not debit text reconstruction bytes for comparisons that allocate nothing.

The four hooks must enforce the same domain semantics under their actual Decode/Encode native controls. Metadata validation must precede admission of output backing, using borrowed dialect/standard/subset comparisons; wrong metadata must not copy a newly allocated dialect string in the measured region. Keep the complete public failure and original context; no cold fallback or demand-derived fresh control.

## SQL identity and semantic mapping

EPW SQL owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql:1–5`.

Table widths are document8, location10, data_periods2, data_period7, record38. Every row requires rowid == Integer cell0 and per-table unique i64 ID. Three singleton tables require exactly one row and ID1. Period/record child IDs are arbitrary signed i64, including0 and negatives: do not cast them to usize or reject nonpositive identities. Both parent columns require1. Dense ordinals require checked nonnegative conversion and exactly0..len after order sorting; never confuse sorted ID positions with ordinal positions. records_per_hour requires Integer0..4294967295 before u32 conversion.

Exact payload coverage is document columns1..7, location1..9, data_periods1, period3..6 and record3..37. All lexical cells require Text; empty/NUL strings remain legal. No lexical Option exists in the typed model; NULL is invalid. Validate seven document strings, nine location strings, all four strings of every period and all35 strings of every record with paid borrowed comparisons. Compare lengths/census and records_per_hour too. A complete empty snapshot still has the three singleton rows.

## Minimum concrete Native corpus

Reuse the five neutral fixture cases (complete, signed zero/negative, empty, all35 empty record fields, long NUL repeated257), all56 valid-different edits and existing11 SQL-invalid edits. Keep independent Bun SQLite expectations as the language-neutral semantic oracle, while Rust constructs owned typed SQL inputs for cases SQLite itself cannot represent.

Add a compact table-driven typed malformed corpus for each of the five tables: remove one trailing cell, add one cell, mismatch rowid/cell0, duplicate identity. This provides20 probes and covers the current record-only width and period-only alias/duplicate gaps without five bespoke laws. Add singleton ID0/wrong Integer alias values and missing/extra singleton row probes; child i64 boundary IDs remain accepted when unique. Include wrong scalar type for every scalar category (ID, parent, ordinal, u32), u32 overflow/negative, period negative/duplicate/gapped ordinals, and lexical NULL/Integer in each table carrying Text. The existing malformed11 cases cover several of these and should be reused rather than duplicated.

Mount original paid P/R, Decode/Encode borrowed validators, zero-grant refusal/no mutation, cumulative-grant conservation across all operations, cancellation during long text, and physical paid retirement of success/partial output. Validation of valid-different edits must reject equality against the original snapshot while R returns the independently edited snapshot. Neither copied fixture assertions nor source hook-name guards qualify those Native branches.

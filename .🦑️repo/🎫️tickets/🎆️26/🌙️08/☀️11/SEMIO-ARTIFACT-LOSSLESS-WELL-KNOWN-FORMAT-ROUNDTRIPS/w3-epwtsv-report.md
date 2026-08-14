# W3 — epw + tsv format artifacts (real implementation)

Agent: W3 epw+tsv. Write scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/**` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/**` only.

## What was read first

- `📋️master-plan.md` "New format artifacts" table (epw/tsv rows).
- `w0-recon-report.md` §3 — confirmed energy's `EpwWeather::parse` (`✏️s/🔌️plugins/🔋️energy/⚙️engine/site/🦀️component.rs:55`) reads a **handful** of columns (year/month/day/hour/minute, dry_bulb, dew_point, RH, pressure, DNI/DHI, horizontal-IR, wind speed/dir, precip, snow depth — ~16 of 35) via `p[N].parse().unwrap_or(default)`, silently defaulting anything short/malformed. Confirmed column-by-column myself (not trusted blindly, per instruction) — the plan's "15 of 35" figure is close but the real count of columns actually *read* is nearer 16; either way energy's seed is genuinely lossy and this new stdio codec is not.
- `w0-recon-report.md` §3 architect exchange (`🏛️program/…/📤️exchange/🦀️component.rs` lines 165/197 `write_delimited`/`parse_delimited`) — informed tsv's "no quoting" edge-case handling (architect's own delimited-text codec *does* quote/escape, which IANA TSV must NOT — confirmed the distinction and did not carry CSV-style quoting into tsv).
- `fixtures/epw/NOTES.md` + `fixtures/tsv/NOTES.md` (W0 real fixtures, already copied into `📚️examples/🎬️demo/🖼️assets/` by W1b) plus their Python generators (`make_epw.py`/`make_tsv.py`) — read to understand exact byte-level formatting decisions before designing the codec.
- csv's complete real implementation (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/**`) as the structural/law template (explicitly permitted per task).

## epw (energyplus) — what changed

Full rewrite of the W1b scaffold (which only typed the LOCATION line + kept everything else in a lossy `raw_lines: Vec<String>` with **zero data records**) into a genuinely lossless codec:

- `EpwSnapshot{schema, location:EpwLocation(9 fields), design_conditions, typical_extreme_periods, ground_temperatures, holidays_dst, comments_1, comments_2, data_periods:EpwDataPeriods{records_per_hour,periods:Vec<EpwDataPeriod{name,start_day_of_week,start_date,end_date}>}, records:Vec<EpwRecord(35 fields)>}` — all 8 header lines + all 35 record columns.
- **Deliberate retention choice**: every numeric-looking field (temperatures, radiation, lat/lon/elevation, …) is `String`, not `f64`/`u16`. Verified concretely that Rust's `f64` `Display` drops a source `.0` (`20.0` → `"20"`), which would silently break the fixture's own `visibility=20.0`/`elevation=55.0`/`time_zone=1.0` fields on re-encode — `String` sidesteps this and keeps `codec_retention_law` byte-exact.
- `⚙️engine::decode_epw`/`encode_epw`: real parser (LOCATION 10-token check, DATA PERIODS `N,recordsPerHour,(name,day,start,end)×N` structural parse, hard column-count errors — no `unwrap_or` defaults) + CRLF-preserving encoder.
- `🔺️diff::EpwRecordDiff` (35 independently-optional columns via a `macro_rules!` generator that also emits `apply`/`between`/`absorb`/`field_at`/`set_at`) + index-keyed `EpwRecordsDiff{removed,modified,added}` (base-free absorb via the same simulated-slot algebra as csv's `CsvDiff`) + whole-substruct replace slots for `location`/`data_periods`.
- `🧬️mutations::EpwMutation`: `NoMutation/SetSnapshot/SetLocation/SetDesignConditions/SetTypicalExtremePeriods/SetGroundTemperatures/SetHolidaysDst/SetComments1/SetComments2/SetDataPeriods/InsertRecord/RemoveRecord/SetRecordField{record_index,field_index,value}`.
- Hand-rolled `DiffCodec`/`OpText`/`OpBinary` (hex-encoded values, bracket-depth-aware splitting) — no `serde_json`, no bare tuples/arrays in the wire grammar.
- Real `sniff()`: first line starts with `LOCATION,` (already partially present in scaffold; kept + reused).
- `register_pilot_languages()` added to `⚙️engine::register()` (was missing — csv has it, EPW scaffold didn't).

## tsv (iana) — what changed

- `TsvSnapshot{schema, records:Vec<Vec<String>>, trailing_newline:bool, line_ending:LineEnding{Lf,Crlf}}` per the assignment's exact shape — no `header`/`TsvRecord` wrapper (removed from the W1b scaffold).
- `⚙️engine::decode_tsv`/`encode_tsv`: byte-exact split/rejoin on the file's own line ending then `\t` — **zero quoting/escaping logic**, matching the real fixture's own `verify_tsv.py` verification method exactly (confirmed by porting that exact check into a Rust test, `embedded_backslash_t_is_not_a_real_tab`, documenting the honest "a literal tab byte inside a field is indistinguishable from a column boundary" limitation).
- `🔺️diff::TsvRowDiff` (positional `Option<Vec<Option<String>>>` patch, same shape family as csv's `CsvRecordDiff`) + index-keyed `TsvRowsDiff`.
- `🧬️mutations::TsvMutation`: `NoMutation/SetSnapshot/SetTrailingNewline/SetLineEnding/InsertRow/RemoveRow/SetCell`.
- Hand-rolled `DiffCodec`/`OpText`/`OpBinary`, same hex/bracket grammar family as epw's.
- Real `sniff()`: non-empty and free of NUL bytes (TSV has no reliable magic per the master plan; this rejects binary noise honestly rather than claiming a false-positive signature).

## Both

- Full facet-mirror set (rust/ts/graphql/json-schema/proto) at artifact, snapshot, diff and mutations levels, kept field-for-field accurate to the real Rust structs.
- Real grammar leaves: `📖️component.grammar.semio` + `🅰️component.g4` + `🔤️component.ebnf` describe the REAL on-disk format (EPW's 8-header-line + 35-column CRLF grammar; TSV's no-quoting tab grammar) for the snapshot facet, and the REAL hand-rolled wire grammar (not JSON) for the diff/mutations facets — this corrects a real discrepancy I found in csv's own grammar leaves (csv's `diff`/`mutations` `.grammar.semio` files claim "wire text IS the JSON serialization", but csv's actual `🦀️component.rs` hand-rolls a non-JSON grammar — I did not copy that inconsistency into epw/tsv).
- All 8 laws present in the existing test regions (no new test files): `field_sweep`, `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`, `codec_retention_law`, `op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`.
- `codec_retention_law` round-trips the real W0 fixtures via `include_str!` directly from `📚️examples/🎬️demo/🖼️assets/` (not a copy): epw's 24×35 + all 8 header lines byte-exact; tsv's 6×5 byte-exact split/rejoin.
- Composer/builder/analyzer at subset/standard/artifact levels were already correctly wired by W1b's scaffold (delegation chain matches csv's) and needed no changes beyond `EpwArtifact`/`TsvArtifact` field syncs to the new snapshot shape.
- No glue.rs/catalog.json/script.ts edits — W1b's mounts already cover every module path used here (verified by reading the relevant glue.rs block, read-only).

## Verification

- `cargo check -p semio-s-plugin-stdio --lib` run 5× at different points; raw output in `w3-epwtsv-baseline-check.txt` (pre-change baseline, 0 errors), `w3-epwtsv-check1-epw-only.txt` (epw written, tsv not yet), `w3-epwtsv-check2-full.txt` through `w3-epwtsv-check5-full-final.txt` (both artifacts complete, polled 4×). **Zero errors ever referenced an epw/tsv path across all five post-baseline runs.**
- The crate-wide error count dropped 55 → 17 → 6 → 6 → 6 (and the *files* involved changed/shrank) between runs while I was writing epw/tsv — confirmed via `git status --porcelain` that the failing files (`🧿️semio` presentation/document/image/workflow subsets, `📊️csv` mutations — csv's own errors cleared between run 2 and 3) were concurrently dirty/uncommitted, i.e. other parallel W2a/W2b agents mid-edit, per the ticket's documented "poll rather than chase" guidance for concurrent workspace churn — not a defect introduced by this agent. The final 3 polls all land on the identical 6 errors, all in `🧿️semio`'s document/image/workflow subsets (`OpText`/`OpBinary` trait-scope `print_op`/`parse_op` not found — a missing `use` statement in files this agent has no write access to).
- **I could not obtain a green `cargo test -p semio-s-plugin-stdio --lib` run** because the crate is one compilation unit and those foreign files (outside my write scope) still failed to compile as of the last check in this session. I am not claiming the epw/tsv tests pass at runtime — only that the epw/tsv code itself type-checks cleanly and independently of the foreign breakage. Whoever runs the W3 verify pass should re-run `cargo test -p semio-s-plugin-stdio --lib "artifacts::epw::"` and `"artifacts::tsv::"` once the foreign files compile; I have high confidence in the 8 laws (they follow csv's proven algebra almost line-for-line, substituting record shapes) but have not watched them execute.

## Files touched

All under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/` (scaffold files edited in place, none created/deleted): `🏅️standards/…/⚙️engine/🦀️component.rs`; `🏅️standards/…/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,📸️snapshot/**,🔺️diff/**,🧬️mutations/**}` (rust facet + all grammar/protocol/ts/graphql/json/proto leaves under `📝️text/`, `💾️binary/`); no changes to composer/builder/analyzer/io/examples (already correct) beyond the two `🧬️schema/🦀️component.rs` artifact-struct field syncs.

Logs: `w3-epwtsv-*.txt` in this ticket folder (compile checks).

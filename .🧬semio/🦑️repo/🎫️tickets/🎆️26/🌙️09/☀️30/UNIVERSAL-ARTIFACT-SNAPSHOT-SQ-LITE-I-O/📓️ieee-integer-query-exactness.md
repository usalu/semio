# IEEE Integer Query Exactness

## Authored Boundary

The query scalar and its signed IEEE word companion must describe the same exact number. SQLite INTEGER cells retain the full signed 64-bit identity; converting them to floating point before comparing can conceal a disagreement. The shared binary64 and binary32 readers currently perform this lossy comparison. This is a static finding until the staged assertions run.

## Language-Neutral Cases

The handcrafted `🎯️integer-query.json` fixture contains eight signed INTEGER cases. It includes exact small integers, the exact 2^53 boundary, integers immediately outside that exact boundary, the last exactly representable binary64 INTEGER before 2^63, both i64 extremes, and a negative extreme neighbor. Each case specifies the native binary64 word, native binary32 word, and separate admission outcome at each width.

Source laws create an actual independent Bun SQLite file with no-affinity query columns, retain its physical INTEGER cells, import that file through the repository engine, and apply both shared readers. Independent DataView and BigInt comparisons validate each expected result. Native laws use the same fixture and independent SQLite/DataView oracle, then apply the shared Rust readers to the corresponding typed row.

## Execution

Source baseline session 94901 is running through the existing registered `@semio-tech/framework-rs:test-snapshot-sqlite-source` route. Native laws are staged and will run through the existing registered native route after the current artifact lane drains. Production readers are unchanged; no result is asserted before execution. No new script or runtime dependency is introduced.

### 12:58 UTC — Fixture Header Prerequisite

First Source execution selected all 35 laws: 34 passed and the new law stopped at repository SQLite identity/version header admission before either IEEE reader was called (328 assertions, 3.71 seconds Bun, 32.8 seconds Nx). This is not an IEEE behavior failure. The independent fixture now declares the required application identity and snapshot version. Its query columns use the existing BLOB no-affinity declaration to preserve actual INTEGER storage and permit supported explicit DDL parsing. Production readers remain unchanged; repeat the corrected baseline.

### Corrected Source Baseline

Corrected independent file executed all 35 Source laws: 34 passed and the new IEEE law failed at the required `toThrow` assertion. INTEGER 9007199254740993 was wrongly admitted against binary64 word 4340000000000000 (exact numeric value 9007199254740992). Independent SQLite storage and DataView/BigInt expectations had passed first. This is an authentic Source behavior failure: 336 assertions, 1.91 seconds Bun, 25.1 seconds uncached Nx. Added a signed INTEGER range and exact reverse BigInt comparison in the Source shared reader, applying at both native widths. Rust shared reader remains unchanged until its own staged negative law executes.

### Source Verification

The repaired Source reader executed all 35 selected laws with zero failures, 352 assertions, 3.15 seconds Bun and 27.1 seconds uncached Nx. The eight-case independent SQLite/DataView law now passes for both binary64 and binary32 widths, including exact i64 minimum and rejection of the rounded i64 maximum. Rust production remains unchanged and its own staged law is still awaiting the native lane.

### Native Oracle Output Prerequisite

Native run 6f65c18f-d7b9-44d6-8506-501cc793962b selected 28 SQLite laws: 27 passed, one failed, zero skipped, 111 ms. The independent SQLite/DataView oracle succeeded, but forced-color console output wrapped its numeric count in ANSI escapes. The test stopped before calling either shared IEEE reader, so this is not an IEEE behavior failure. Changed only oracle stdout to raw `Bun.write`. Rust reader remains unchanged until the corrected law executes.

### Authentic Native Failure and Repair

Corrected native law executed in root lane 35644: 28 selected tests, 27 passed, one authentic IEEE failure, zero skipped, 122 ms. Its independent SQLite/DataView oracle first validated all eight expected integer identities. The Rust reader then wrongly accepted query INTEGER 9007199254740993 against the exact binary64 value 9007199254740992. Added a direct physical query-cell comparison: REAL must equal the authored IEEE value; INTEGER must be within the signed range, integral and reversible to the same signed word before admission. This covers both binary64 and binary32 readers while retaining signed-zero word identity and i64 minimum. Native verification will follow after the current artifact lane drains; no passing native result is claimed yet.

## Native Exact INTEGER Queries Verified on 2026-10-02 at 13:32 UTC

Nextest `3cd3290d-ffd6-416e-8f4c-aa10a3ca7a84` executed and passed all 28 shared native snapshot laws, zero skipped, in 158 ms. The new eight-case INTEGER query law ran after independent Bun SQLite/DataView validation; the previous genuine rounded-integer failure is repaired by matching physical INTEGER cells only when reverse conversion recovers the exact signed64 value and the IEEE value is integral and within the representable signed range. Source separately passed all 35 laws with 352 assertions. These are shared scalar/codec results, not proof that each artifact's separate custom scalar reader is correct.

# Exact Native Typing Case Coverage

The edit and original-commit oracles previously counted matching log lines. A duplicated known case could replace an omitted case while preserving that total. Both validators now build an exact seen-name set, reject unknown names before comparison, reject a repeated name, preserve the independent JSONPatch payload comparison, and finally reject every missing expected name explicitly. Duplicate case names in the neutral fixture are also refused instead of overwriting a Map entry.

SQLite selection and output construction are unchanged. The edit reference reran with ten cases and the commit reference with eight; both passed with nativeCompared false and zero native records. No Rust source, native job, source cap or case output was changed.

The retained typing-oracle-hostile.ts runner requires an actual native log. It first validates the original native corpus successfully, then derives five negative inputs from those actual records: a duplicate replacing an omitted case with the same total count, an extra duplicate, an omitted case, a renamed unknown case, and a changed actual output. Each must fail for its specific intended reason. It creates no positive synthetic records and never treats absent native output as a pass.

Actual-native hostile execution remains pending runtime records from the sole compiler lane. Run the managed Bun script with edit or commit followed by that actual suite's retained native log. The active joint gate selects the edit package; the original-commit native suite is a separate queued gate. Do not claim either hostile check executed until its actual native corpus is available.

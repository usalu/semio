# Remaining fixture custody

Retirement tickets failure law imports adjacent schema.json and fixture.json. Both exist, paths are 119 and 120 UTF-8 bytes absolute, and the closed schema requires the exact five fixture fields. A scoped search of the tickets subtree found no Rust include_str or fixture_text consumer of that JSON; the native law owner has been asked to verify the actual consumer. This does not establish that no consumer exists elsewhere.

NativeReceive derive receiving tests include the existing adjacent fixture at tests/fixtures/🔣️.json (131 UTF-8 bytes absolute). Its two root fields are cases and primaryPathCases, both read by Rust tests. A scoped search of the receiving subtree found no corresponding schema definition or schema reference. Enum corpus uses fixture_text and an independent transparent schema; this audit did not resolve fixture-reader manifest-relative custody.

No nativeSources or repo_text declaration appeared in scoped searches of these two fixture subtrees or the lane subtree. This does not prove that an owning package does not declare them elsewhere; complete manifest and runtime hashing verification remains open. No build or oracle run was performed.

## Native consumer confirmation

The store replay operation native test `original_retirement_preserves_child_and_work_commit_failures` uses include_str with a relative path that resolves to the exact failure fixture above. It reads primary, workRefusal and actualWork. Native reports its compile is still blocked upstream.

# Go Test Dispatch Follow-Up

## Concrete Remaining Route

The completed Go lane and Terra audit verified registered build/test/dev scripts and bootstrap routes. A coordinator read of the native CLI test dispatcher found an additional route not covered by that bounded audit: `⌨️cli/🧩️component/🐹️.go` still invokes native Go directly in `runBundleTests`, `runFileTests`, `runSectionTests`, and `runDefinitionTest`. Those branches construct `go test ./...` or a semantic file-directory descendant pattern and do not provide the canonical compiler overlay.

The audit independently established that the relocated coordinator package has no direct root Go files without its overlay. The native CLI dispatcher therefore cannot execute the relocated tree correctly if one of these Go branches is reached. This is a concrete route gap inferred from the live dispatch code and the native compiler evidence, not a claim that the route has been exercised end to end in this follow-up.

## Bounded Repair

Trace whether each dispatcher branch is reachable from current CLI/MCP/launch test requests. Replace reachable Go test dispatch with the shared canonical source/test plan via the registered Bun/Nx task route, including exact owner package selection and file/section/definition filters. Do not duplicate the overlay planner in a new language implementation without a portable shared contract. Preserve progress, cancellation and test output events. Remove genuinely dead legacy routes if proven unused, rather than retain a compatibility path. Other language dispatcher branches are outside this bounded repair unless the shared entrypoint necessarily changes their routing.

Add a portable dispatcher fixture first and validate against native Go execution for an owner containing only semantic anonymous source/test leaves, including a local replaced module. Test at least the reachable bundle and definition/file selection path through the actual dispatcher rather than only inspecting a command string. Use a narrow test filter and temporary fixtures under this ticket; do not rerun the full known-failing CLI suite without need. New permanent routing code must remain in the existing 📜️script.ts and shared library conventions; launch seed and derived entries must remain aligned. No modifying Git commands, AGENTS edits, migration scripts, compatibility wrappers, or ticket/goal lifecycle changes.

## Related Current Assertion

The broad CLI report also records `TestCollectGoTestsInSection` failing to find Alpha. The live test creates its own temporary foo_test.go with VS16-decorated region markers; the collector strips one non-ASCII code point before calling Flat. This is separate from native source-directory projection. Investigate only as needed to exercise section dispatch, using the exact current parser/fixture and Unicode contract; no pre-existing baseline claim has been established. No test pass or implementation fix is claimed by this note.

## Deliverable

Retain 📓️sol-go-test-dispatch-2026-09-12.md with exact reachability evidence, owned files, portable/native red-green output and limits. Disposable files stay under 🗑️generated/sol-go-test-dispatch. This concrete integration repair takes the next freed execution slot, followed by its independent Terra check combined with the pending UI/generated audit.

## Region Marker Evidence

Further live source inspection shows Flat deliberately preserves every non-ASCII rune, including U+FE0F. The section collector removes only the first emoji code point from the leading marker, leaving its variation selector. Its subsequent flattened string can therefore differ from Alpha even though the human-visible marker was intended only as a kind prefix. Treat this as a localized section-marker parsing concern: preserve Flat’s domain identity contract and use the repository’s proper leading-grapheme handling if section dispatch remains an active route. A native runtime reproduction belongs in the executor’s portable test; this note records source evidence only.

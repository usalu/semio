# EN 1992 and EN 1994 Edit Rule Fixtures

The admitted norm JavaScript suite recorded 12,090 passes and three failures. The insert-binding failure reported missing EN 1992 anchors because the fixture still named the original edit-rules module. The rule is declared in the extracted table together with the other EN 1992 insert rules. The same source relocation affected EN 1994. The source scan still found 236 declarations, with 24 now matching only by the extracted table path.

Updated the 24 source witnesses in `✏️s/🔌️plugins/📕️norm/🧪️tests/📥️insert-bindings/🧫️cases/🔣️.json`. The recorded mutation kinds, selectors, index and item keys, and payload schema paths remain the same. Both focused insert-binding tests passed against the strict Ajv oracle, with 715 assertions.

The other two failures were inspector-admission witnesses for the same EN 1992 and EN 1994 extraction. The shared tree already named the correct edit-rules crate in both cases by the time of inspection. I verified all 17 inspector-admission tests passed, with 51 assertions. No compiler, browser or complete JavaScript suite was started for this check.

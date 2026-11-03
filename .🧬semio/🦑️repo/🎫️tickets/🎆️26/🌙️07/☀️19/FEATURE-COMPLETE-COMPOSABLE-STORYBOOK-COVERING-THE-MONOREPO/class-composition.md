# Independent Text Size and Color Composition

Selected avatars exposed a shared class merger defect: every `text-[…]` class was treated as font size. Adding an arbitrary selected-state text color erased `text-xs`, raising initials to the inherited16px/24px typography.

The repository-owned classifier now distinguishes explicit length/math/dimension text-size utilities from arbitrary colors, supports typed CSS-variable lengths, and recognizes semantic `text-tiny`/`text-2xs`. No component-specific reordering mask or runtime dependency was added.

Extended the existing language-neutral class-conflict feature and registered its new scenario. Focused test first reproduced11 failures; after repair all37 feature vectors pass. A pinned, isolated, test-only `tailwind-merge@3.4.0` independently produced the same output for all12 typography vectors, with the two repository semantic font-size names supplied through its theme extension. This is an oracle for that typography subset, not for the repository's unrelated custom surface/group families.

Reproduction input: `test-class-composition-oracle.ts` in this ticket. Run through `repo tools exec --light bun -- <script>`. It installs only under this ticket's generated/o directory, executes49 tests/61 assertions and removes the isolated package afterward. Root manifests/lock are unchanged by this oracle.

Final browser acceptance additionally compares selected/default avatar font sizes, line heights and actual text clearance against generated CSS.

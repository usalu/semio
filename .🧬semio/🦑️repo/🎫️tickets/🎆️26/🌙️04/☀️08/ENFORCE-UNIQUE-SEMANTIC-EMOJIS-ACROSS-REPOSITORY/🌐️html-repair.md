# HTML Artifact Emoji Repair

## Scope and baseline

This review covers only `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html` and its exact mount in the Stdio Rust oracle barrel. It is disjoint from the previously completed Vite output-alias work. The pre-repair read-only statute audit found 177 files, 97 directories, and 30 findings: 24 missing identities in eight fixture triplets, two UI option/config collisions, one test/oracle collision, two presentation-selector defects, and one production raw-text/support collision.

No automatic naming tool, generated rename plan, Git mutation, compatibility alias, or payload transformation was used. All 31 physical changes were literal destination-absent `mv -n` operations.

## Handpicked decisions

- The eight applied fixtures mirror their production operations: `➕insert-node-applied`, `➖remove-node-applied`, `🔖set-attribute-applied`, `💬set-comment-applied`, `📜set-doctype-applied`, `🏷️set-element-name-applied`, `⌨️set-raw-text-applied`, and `✍️set-text-applied`.
- Every fixture pair uses `⬅️before.html` and `➡️after.html` to distinguish input from expected output while leaving HTML bytes unchanged.
- Both UI windows use `☑️options`, distinct from sibling `🎚️config`; the independent parse5 comparison owner uses `🔮️oracle`, distinct from sibling `🧪️tests`.
- `✍set-text → ✍️set-text` and `🏷set-element-name → 🏷️set-element-name` complete the existing meaningful emoji graphemes.
- `📝set-raw-text → ⌨️set-raw-text` identifies verbatim typed/source text and avoids colliding with the adjacent `📝️text` serialization support tree.
- `🟤️set-snapshot → 📸️set-snapshot` replaces a color placeholder with the meaning of capturing/replacing the complete HTML document state.

The wire-level mutation IDs remain unchanged. Exact fixture catalog paths, production directory coordinates, descriptor owners/emojis, Rust `#[path]` mounts, and the Stdio oracle-barrel mount were repaired to the new physical identities.

## Verification

- All subtree JSON files parse with `jq`.
- The exact old-coordinate scan returns zero matches.
- The post-repair read-only ticket audit reports 177 files, 97 directories, and zero `missing`, `generic`, `presentation`, `spacing`, `duplicate`, `multiple`, `reserved-emoji`, or independent-oracle findings.
- The exact central mapping requested from the root lane is owner `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any` to `🔮️oracle`.
- After the root lane added that exact override, `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.html` passed with 8 fixtures and 0 file problems.

## Read-only deferred inventory

JPEG was audited without modification before selecting this bounded subtree. Its two overlapping subset authorities and applied/base fixture twins require a separate full case-by-case review; no JPEG name or reference was changed here.

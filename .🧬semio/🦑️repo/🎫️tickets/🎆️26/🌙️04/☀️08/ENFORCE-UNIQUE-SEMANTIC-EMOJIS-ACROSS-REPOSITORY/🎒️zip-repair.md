# ZIP Handpicked Emoji Repair

The previously moved `🎒️zip` tree was rechecked and its unfinished reference repair completed. The two subsets are `🧱️base` and `🌐️iso21320`; local oracles are `🔮️oracle`, and full-archive replacement is `📸️set-snapshot`. Fixture directories use the authored operation: add `➕️`, remove `➖️`, rename `🏷️`, archive comment `💬️`, data `✍️`, deflate `💨️`, stored member `📦️`, snapshot `📸️`. Before/after carriers use `⬅️` and `➡️`.

Updated subset directory fields, oracle include paths, documentation, and exact Stdio registry/oracle mounts. The existing historical ZIP generator input in the September 2 mutation-fixture ticket was patched with these explicit directory names and carriers; its semantic IDs and fixture payloads remain unchanged. No selection algorithm or renaming script was run.

Verification on 2026-09-05:

- Read-only scoped statute audit: 255 files, 172 directories, 427 governed entries; all eight finding categories zero.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.zip`: 12 fixtures, zero file problems, exit 0.
- All 32 JSON files parsed. All 12 manifests and 24 referenced files resolved and matched recorded byte length and SHA-256.
- Central oracle override paths were supplied to the root agent.

The semantic follow-up replaced the arbitrary wolf/herb mutation-suite identities with `🔀️` and corrected the catalog's README-expansion scenario directory to the existing `📖️extends-the-readme-and-adds-a-version-member`. All eighteen catalog scenario directories across ZIP, PNG, JSON, BCF, and XLSX were then checked for physical existence successfully.

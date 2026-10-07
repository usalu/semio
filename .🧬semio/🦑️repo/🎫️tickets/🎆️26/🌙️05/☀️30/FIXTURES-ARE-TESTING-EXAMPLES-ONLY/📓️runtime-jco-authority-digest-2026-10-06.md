# JCO authority digest repair

The authority catalog current bytes are exactly the previously sealed0b43c775 document with only the JCO guest path prefix changed from OS/fixtures/jcoprobe/guest to OS/testing/jcoprobe/guest. Verified against read-only git revision d4597d0410a parent: all JSON keys, array lengths, nonpath values, source hashes/sizes, adapter content and canonical counts are unchanged. The current exact digest is 2c3f40479689fb2348cc4d3f94a45a3e046ef0bf26b74dc5df1867979c432cee.

Updated only the nested-Cargo contract authorityCatalogSha256 and appended an exact reversible 12-path move record in the existing frozen seal evidence input. No source admission or source/count assertion was removed.

Acceptance dialect finding belongs the separate newly authored acceptance protocol (`repo-test-acceptance/v1`, checkResult/goalPlan/goalSummary); it is unrelated to our renamed evidence wire schema. Read-only history identifies dfe2687f7db as its introducing owner. Its2020-12 dialect is preserved here, per the parent request to avoid unrelated changes.

Focused verification: exact nested-cargo collision and historical seal reversal suites51PASS646assertions. HTML source-pair normalization1PASS49assertions. The selected current-JCO proof exposed another directly related reader: its malformed destination examples remain plain inputs at OS/fixtures/jcoprobe/destination-cases.json, while only executable testing ownership moved. Corrected that one test reader to the unchanged plain-example input path, preserving all malformed-input refusals and current actual package/adaptor checks. Its recheck is pending.

Final selected current-JCO owner proof passed1test17assertions (67unselected skipped) through actual private Nx exec→ticket owner dispatch→library permanent script. All three focused jobs settled. No acceptance/schema/domain/dependency-baseline edits. Whitespace check passed on the three touched source leaves.

# Authored Command Tokenizer Diagnostic

This bounded read-only scan used the actual current literalCommandTokensV1 implementation and declaring-owned independent string-argv parser over current closed-registry project manifests. It did not open the FactStore, execute target commands or modify production Source.

The unchanged registry contained 1696 owners. The scan read 693 declared JSON project manifests, 3445 targets and 3508 effective base/configured recipes. There were no missing declarations and no independent-argv disagreement among commands the strict tokenizer accepted.

Two authored recipes were refused: @semio-tech/framework-os:canonical-architecture and @semio-tech/framework-os-dev:browser-host-staging-check. Each embeds a quoted test-name pattern after an unquoted --testNamePattern= prefix. The independent argv parser treats each as one literal argument; the owned tokenizer intentionally admits only complete quoted tokens and rejects a quote inside an unquoted token.

These are concrete current authored-command refusals, but the diagnostic alone does not establish which command caused the live ALL failure. Exact commands, Source hashes and independent argv are retained in 🗑️generated/authored-command-tokenizer.json; commands/📜️script.ts reproduces the scan. Strict literal grammar and producer Source authority remain unchanged.

The retained two-vector --embedded-quotes proof passed. Both rejected authored forms produce the expected argv through string-argv; JSON-quoting each complete token is already admitted by the strict owned tokenizer and preserves the same argv, including the regex pipe characters as data. This establishes an asset-only canonical spelling without changing the grammar. Core correlated the sole unmatched first-pass owner with @semio-tech/framework-os; the second recipe is the same proven lexical feature, not a separately observed live failure. Permanent Source and tests remain unchanged pending release.

After Core confirmed the owned ALL3 parent and child had stopped, exactly the two command string values were hand-normalized to whole-token quoting. Immediate reread and parsed equality checks preserved every unrelated project field; exact before/after values and Source hashes are in 🗑️generated/authored-command-corrections.json. No parser grammar or runtime implementation changed.

The final full authored census passed: 1696 owners, 693 declared JSON manifests, 3445 targets, 3508 recipes, zero strict rejections, zero independent argv disagreements, zero missing manifests, and unchanged registry bytes. The retained two-vector proof also passed after correction. Core received the source-settled signal immediately. This is focused Source validation; it does not claim a successful ALL publication or final all-task planner/Stdio acceptance.

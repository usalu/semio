# Dependabot Configuration Gate Review

Read-only review of actual configuration, go.work and derived-config checker, 2026-10-29. No Source edits or check reruns by reviewer.

The ten diagnostics comprise six configuration drift findings and four checker false positives. All32 go.work module directories currently have go.mod. Remove the stale CLI 🚀️bin directory from the gomod list; add the five missing module roots listed in the log: filesystem durability Go, workspace sandbox Go, launcher Go, launcher Go main and client CLI Go main. Preserve all existing other ecosystem scopes and weekly schedules.

The four configured composite action directories are real and each contains action.yml: repo-launcher, devcontainer-repo, devcontainer-suites and cache-proof. A fifth real action directory native-repo also contains action.yml and should be included. The checker currently recognizes github-actions only by a nested .github/workflows directory, which is wrong for these configured composite roots. Fix the owner predicate rather than delete valid coverage.

Primary implementation evidence: [Dependabot GitHub Actions file fetcher](https://raw.githubusercontent.com/dependabot/dependabot-core/main/github_actions/lib/dependabot/github_actions/file_fetcher.rb), workflow_files lines145–164, special-cases repository root and otherwise scans YAML in the configured directory itself. Current [GitHub options prose](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference#directories-or-directory) recommends root; the actual fetcher explicitly models nonroot configured scopes. Preserve the real composite coverage rather than infer that the brief prose forbids it.

Neutral regression recommendation: closed fixture for root workflows, action.yml, action.yaml, missing path, empty directory and directory masquerading as action.yml. Use actual ordinary-file checks and independent Python pathlib/SQLite truth-table outcomes, plus Ajv fixture validation. Ensure no merely existing directory is accepted as a manifest. An actual YAML root/updated dirs check should compare every go.work member and all five authored composite directories without modifying package policy.

Root owns the test-first predicate/config corrections. The failing generate-config log establishes current red; no green result claimed here.


## Corrected Owner and Configuration Review

Re-read all four changed files. actionManifests now mirrors the primary fetcher’s distinction: repository root accepts ordinary action.yml/action.yaml or immediate ordinary YAML files in .github/workflows; an explicitly configured nonroot directory accepts immediate ordinary .yml/.yaml files. It does not recursively admit nested YAML or treat empty/directories-as-manifests as valid. Directory checks use no-follow lstat and Dirent.isFile excludes symlink/nonfile entries. Missing configured paths retain the existing does-not-exist diagnostic before manifest checks.

The matching primary pattern is documented in [Dependabot Actions constants](https://raw.githubusercontent.com/dependabot/dependabot-core/main/github_actions/lib/dependabot/github_actions/constants.rb): YAML suffixes and the two exact root action names. Together with [file fetcher workflow_files](https://raw.githubusercontent.com/dependabot/dependabot-core/main/github_actions/lib/dependabot/github_actions/file_fetcher.rb), this supports the implemented root/nonroot distinction. No broad existence shortcut was introduced.

Configuration retains existing scope, removes stale bin, adds all five missing go.work modules and native-repo action scope. Twelve language-neutral vectors exercise root workflows/root actions, unrelated YAML, empty/nonfile entries, configured composite/general YAML, nested YAML and unrelated files. The test’s independent SQLite file-table query selects ordinary files and root/nonroot membership; existing external YAML/TOML parser parity remains. Tests use Node assert, so Bun expect count is not the assertion metric. Root reports actual causal red, final2tests terminal0 and generate config --check terminal0. No test rerun by reviewer and no concrete remaining scoped blocker found.

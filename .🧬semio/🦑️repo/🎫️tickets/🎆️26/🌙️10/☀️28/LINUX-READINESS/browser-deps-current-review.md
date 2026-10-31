# Current Browser Dependency Source Review

Static read-only review, 2026-10-29. No tests, installers, native probes, lease/process mutations or Source edits were performed. Own-validator boolean assertions and old Bash-test relocation are already acknowledged owner work and are not newly diagnosed here.

## Source Wiring and Behavior

The native browsers branch now merges boundedEnvironment with repoToolCacheEnv, calls prepareBrowserDependenciesV1, and supplies its existing run closure. That closure passes the unchanged phase.leaseControl.signal, storage and phase to the existing phase-aware dependency runner and checks budget/progress. Required system installation is awaited before Chromium download. Bash no longer owns ensure_browser_libraries/linux_browser_packages or the ignored post-setup installer. This resolves the previously identified graph wiring gap in Source.

Linux policy preserves all prior exact dnf/pacman/zypper packages and arguments, including mozilla-nspr and both openSUSE Xvfb packages. apt remains pinned Playwright install-deps chromium. Nonroot uses resolved sudo -n --; apt explicitly wraps resolved env DEBIAN_FRONTEND=noninteractive and the runtime CLI. Root apt also receives DEBIAN_FRONTEND. Missing supported manager, CLI, sudo or env rejects before download. Installer errors propagate. This is source evidence, not successful distribution package resolution or Chromium startup.

Cancellation: helper checks signal initially, before/after Linux installer and after download. The real native closure retains exact original signal and native phase. The command-port type itself does not carry signal or Original; this is safe only because production supplies the phase-bound closure. Current recording tests prove helper ordering/failure behavior but do not verify production closure identity or genuinely issued Original. Do not claim those tests prove native authority preservation.

Offline: Linux refuses all provisioning before external calls. macOS/Windows continue existing download behavior even with offline=true; current helper's explicit offline contract is Linux-only. If the intended contract is no network with original offline authority on **all** platforms, move/check offline before the platform branch and add Win/mac offline cases. Current fixtures cover only Linux offline, so they do not establish global offline compliance.

Cross-platform: Linux-specific process.getuid is optional; helper skips Linux packages on Win/mac. Host tool lookup accepts only absolute PATH entries, verifies X_OK and regular files, and returns null for missing/inaccessible executables; Linux privilege handling never requires a shell. The Windows/mac fixtures test pure adapter facts, not actual host behavior. The static shell-relative test URL resolves to the current existing Bash file.

## Concrete Schema/Test Gaps

1. Policy schema permits three rows all naming dnf, omitting pacman/zypper. Implementation then uses row!.arguments for a selected absent manager. Require exactly one row for each manager (tuple/closed ordered rows is smallest), and pin each row's permitted installer argv. Current data is correct; schema currently fails to enforce its operational assumptions.
2. Policy schema accepts empty argument/package strings and arbitrary installer flags. Use nonempty bounded strings and existing package-name regex, preserving case and hyphen/underscore names. These are authored policy invariants rather than externally supplied arbitrary argv.
3. Neutral fixture cases have no own schema validation. Add closed Case/Fixture/ExpectedCommand definitions, including platform, available managers/tools, root/cli/offline booleans, cancel enum, fail boolean, expected array and nullable error; validate same fixture with own validator and Ajv before replay. Current third-party const comparison validates expected output equality but cannot catch malformed/omitted case fields.
4. Add installer returns-success-after-cancellation case: injected execute aborts signal and returns normally; prove helper's own post-step signal check rejects and blocks download. Existing during case aborts **and throws**, so only proves callback rejection propagation.
5. Add download-stage failure/cancellation cases to distinguish them from first-install failure, and check exact environment identity/retained keys at both steps. Current fail flag throws on first call only.
6. No host-port executable resolver test exists yet. A bounded temporary fixture can cover absolute PATH, relative/empty exclusion, nonexecutable file and directory rejection, precedence, and missing CLI without executing tools, on platforms where executable permission applies.
7. Canonical branch assertion is currently textual contains, so it proves presence but not that helper really precedes/allows the download. Production helper owns both steps, which makes current behavior reviewable. For stronger native branch integration use genuine native grant; do not fabricate phase to fit an empty-capability law.

Own validateJsonSchemaSubset returns an error array, so success must equal [], failure must have positive length. Owner is already correcting current boolean expectations.

## Minimal Portable Contribution: Actual Closed Values

Shared closed schema is `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🧬️schema/🔣️.json`, definitions.Suite and definitions.Capabilities. Current capabilities only allow artifacts:true and command:string; **{} is valid**, no added fields/grants are permitted in a contribution. typecheck values are none/strict/strict-indexed; budgetMs1000..1800000; platforms darwin/linux/win32. platformCauses must be absent for the portable row.

Append this explicit row to `📚️library/⚡️caching/🗂️suites/🔣️.json`:

~~~json
{
  "id": "browser-dependencies",
  "budgetMs": 15000,
  "typecheck": "none",
  "files": [
    "🚀️bootstrap/📦️dependencies/🏗️native/🌐️system/🧪️tests/🟦️.ts"
  ],
  "capabilities": {},
  "platforms": ["darwin", "linux", "win32"]
}
~~~

The framework routeProcessTestFilesV1 actually gives unfiltered explicit file ownership precedence over deepest tree matches, so this row supersedes broad bootstrap ownership without deleting that tree. Do not add duplicate explicit owners. No tools/scratch/fileLaunchers are required for recording-only tests. No Native capability grant should be introduced.

If changing async cases to genuine framework laws, declareBuildTestEnvelopeV1 with owner test:browser-dependencies, maximumElapsedMilliseconds15000 and capabilities:{} before the first callback, and use registerBuildTestLawV1 with an existing closed law plan. The genuine helper tests can receive an empty Original; direct prepareDependencies cannot, as documented in browser-deps-test-seam.md. Current cases simply use Bun test and a plain AbortController; those are valid pure helper tests but should be described as such.

The closed row schema does not by itself establish the repo-lib target publication; let current suite contribution generation own target creation and verify registration through existing owner checks when runtime work resumes.

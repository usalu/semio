# Cargo Nonmember Dependency Test Selection

Read-only primary-source research; no Cargo operation or membership change executed.

The proposed WFC-manifest selection of semio-framework-job-headless cannot execute its own tests with current dev-dependencies. Cargo can resolve an explicitly selected dependency package, but its compile front end explicitly rejects a test-mode selected nonmember that requires dev dependencies. Neutral currently declares serde_json and semio-framework-value-testing-allocator under dev-dependencies and is a framework workspace member, not a WFC workspace member.

Primary proof: [Cargo compile front end, pinned official Cargo source](https://rust.googlesource.com/cargo/+/58bab51ca444e109a21ae9e0f17c27e38173a660/src/cargo/ops/cargo_compile/mod.rs#328). At328–336 it checks is_any_test, !ws.is_member(pkg), and any dependency !is_transitive, then refuses with the diagnostic “cannot be tested because it requires dev-dependencies and is not a member of the workspace”. The preceding311–318 explicitly resolves requested package specs from the dependency graph, so successful package lookup does not imply permission to test that package's dev targets.

[Official cargo-test documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html#package-selection) describes package selection and --workspace, but does not itself state this detailed nonmember restriction. The implementation supplies the decisive condition. This source review is not an empirical result from the locally pinned compiler; no unsupported experimental gate was launched.

Conclusion: keep the separately selected neutral package gate under its own legitimate framework manifest. Do not add neutral to the WFC coupled four-package test selector expecting its native laws to run, remove its dev-dependencies, or move workspace membership as a workaround. Parent's existing neutral4662 and WFC coupled core gates remain unchanged; no scheduling intervention by auditor.

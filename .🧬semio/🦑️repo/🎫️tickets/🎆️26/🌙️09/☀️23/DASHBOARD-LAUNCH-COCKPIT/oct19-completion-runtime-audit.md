# October 19 Completion Runtime Audit

## Current Acceptance Flow Reviewed

This bounded completion audit read the current dashboard owner targets, package script, installation/launch helpers, dry-run authority test, journey crate manifest, Rust journey workspace/terminal consumers and the owned Cargo test-executable runner. It did not repeat the complete authority gate, native provisioning, blocked empty-directory removal or host privilege check. The parent's current facts remain authoritative: root launch catalog absent, empty `.vscode` directory present, prototype installation absent, and host token lacks `SeManageVolumePrivilege`.

The current install target depends on the normal build owner. That owner captures source ownership, invokes the declared native compile route, and publishes through immutable executable/tools custody. Run/daemon/preferences use current execution options and the leased installed reader; stale installations trigger the owning rebuild path. The authority owner selects a dashboard executable with its required options before supplying `SEMIO_DASHBOARD_BIN` to actual native dry-runs. Journey/load targets keep the installed-tool callback open while Bun suites and the owned Rust test-executable route consume the explicit selected executable. These are current source routes; their successful full native execution is not established here.

## Concrete Current Defects Repaired

The Rust journey workspace called `Path::new` in `fixture_script()` but imported only `PathBuf`. This is an actual Rust compile blocker. After adding the missing import, executing the exact function found a second defect: it walked four parents from `journeys/📦️packages/🦀️rust`, reaching `dashboard/🧫️fixtures`, while current shared fixtures live under `dashboard/🧪️tests/🧫️fixtures`. The adjacent `journey_fixture()` compile-time include had the same wrong four-parent path and could not compile. Both paths now walk three parents, matching the already-correct `Workspace::new` fixture-directory reader.

The Rust `binary()` selector also retained an obsolete mode-less version-2 installation-record fallback. The current owned journey route already supplies the selected executable under reader custody via `SEMIO_TEST_CLI`. The fallback was removed rather than adapted: absent or empty input now fails with an explicit owning-target diagnostic. The selector does not claim to independently admit bytes or acquire custody; those remain the current owning route's responsibility.

All current Rust `binary()` consumers use this workspace selector for CLI and PTY launches. No alternative publisher, fake installation, legacy compatibility reader or native storage bypass was added. Unit vectors use a synthetic path only as selector data and never execute or publish it.

## Schema-First Tests and Red/Green Evidence

The shared journey schema/fixture gained `nativeSources`: expected authored script path, required environment identity, three absent/empty/explicit selection cases and the nonempty executable-value contract. New source-only tests are registered through an import in the existing authority entry, so the current dashboard `TestScript` authority lane includes them; they are not an unreachable one-shot helper.

Executed red stages:

1. Compiling the exact `fixture_script` body with its actual standard-library imports failed with Rust **E0433: `Path` is not in scope**. The pre-retirement binary selector also failed the intentionally standard-library-only probe because it still depended on the record-reader/macro fallback; that probe failure is not presented as a full-crate dependency failure.
2. After fixing the import and replacing the selector, the actual script function compiled but failed at runtime with **NotFound** while canonicalizing the wrong four-parent path.
3. The probe was extended to compile the actual `journey_fixture` `include_str!` expression with the real journey crate's `CARGO_MANIFEST_DIR`. Rust independently refused its wrong four-parent include path.

After the two path corrections, **two tests passed, zero failed, 17 assertions**. The actual Rust function resolved the immutable authored script; independent fast-glob located the expected shared path and Node file reading agreed on its bytes. Rust's actual compile-time include emitted JSON equal to the shared fixture parsed by JavaScript and admitted by Ajv. The actual binary selector refused absent and empty input and retained the explicit Unicode/space path, agreeing with the independent Ajv value contract.

The final registered focused authority run passed **seven tests, zero failures, 31 filtered, 193 assertions, 4.82 seconds**. It includes current source-discovery/schema/AST/filesystem checks, actual registry refresh and form normalization, the two new Rust journey source checks, current test-dispatch context vectors and the existing launcher DOM/clamp/input oracle. Its output is temporarily retained at `🗑️generated/completion-runtime-focused.log`.

Read-only stable rustfmt parsing also accepted the complete current journey workspace file using edition 2024 and `skip_children=true`; stdout was discarded and source was not reformatted.

## Exact Proof Boundary and Remaining Finding

The native probes compile selected actual production/test-helper bodies and the actual include expression with their actual standard-library imports. They do not compile the entire Cargo journey crate, link portable-pty/vt100/serde, build the dashboard, admit/publish an installed dashboard, start its daemon or drive a real PTY. The syntax parser is weaker evidence still. Full installation, native dry-runs and real terminal acceptance remain required and unverified while the host/store admission is blocked. The prior full non-native authority result remains **34 pass, one strict empty-directory failure, one installed-native test filtered, 1,614 assertions**; it was not rerun merely to repeat those known facts.

Independent read-only review found one additional current handoff issue after these repairs: the Cargo executable runner merges `{...consumerEnvironment, ...boundedEnvironment}`, so an inherited stale `SEMIO_TEST_CLI` retained in compiler inputs can override the journey owner's explicit currently selected executable. This was reported to the parent for a separate bounded consumer/runtime-key fix and regression. No duplicate change was made here. The helper fixes passing does not resolve or conceal that finding.

No goal/ticket state was changed by this agent. No competing course/IDE/document files, AGENTS, Git state, worktrees, native receipts or storage gates were edited.

## Exact Files Changed

Relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`:

```text
🧪️tests/🧭️journeys/🏗️workspace/🦀️.rs
🧬️schema/🧭️journeys/🔣️.json
🧪️tests/🧫️fixtures/🧭️journeys/🔣️.json
🧪️tests/🧭️journeys/🧩️sources/🟦️.ts
🧪️tests/🧭️journeys/🥒️.feature
🧪️tests/🧭️authority/🟦️.ts
```

This retained audit is the only newly authored ticket file. Generated probe/compiler outputs stayed under the ticket's `🗑️generated` directory and were removed after each test through bounded cleanup. Existing authored inputs and reports remain retained.

## Cargo Consumer Environment Precedence Repair

The parent authorized fixing the additional handoff finding. The actual Cargo executable runner was spreading `{...consumerEnvironment, ...boundedEnvironment}`. Compiler input collection retains ambient process values, so an inherited stale `SEMIO_TEST_CLI` could overwrite the current leased dashboard path deliberately supplied by `battleWithInstalledDashboard`. Simply reversing the whole spread would let consumer values replace compiler toolchain/storage, capacity and invocation policy.

The repair is a private projection inside the existing runner module. Its schema owns exactly four current Rust-journey consumer keys:

| Consumer key | Actual current reader |
| --- | --- |
| `SEMIO_TEST_CLI` | Journey workspace binary selector |
| `SEMIO_TEST_ARTIFACT_DIR` | Workspace, terminal, measurement and step artifact output |
| `SEMIO_KEEP_WORKSPACES` | Workspace-drop retention presence flag |
| `SEMIO_TYPE_BUDGET_MS` | Rust load-journey typing budget |

The compiler environment remains the base. Only these four keys are erased and reprojected from the consumer argument; missing or explicitly undefined values erase ambient runtime state rather than resurrecting it. Empty keep-workspaces values remain present, matching the Rust presence check. Other compiler values are untouched. Unknown/protected consumer values are ignored rather than gaining generic override authority. This preserves PATH/HOME, temporary directories, Cargo homes/targets/jobserver/job limits, Rust toolchain/flags, mode, native storage settings, compiler capacity and Original fields. The existing `scriptInvocationEnvironment` is still applied last with the same received invocation. The default consumer argument is now `{}`; the actual dashboard caller already supplies its explicitly selected environment, and the finite Cargo fixture needs no journey selectors.

Windows matching applies case-insensitively only to these four owned names. Every compiler-side case alias is erased. An explicitly supplied canonical uppercase consumer key wins over stale lowercase/mixed-case entries retained by a process-environment spread. Without a canonical key, one unambiguous value is canonicalized; conflicting alias values fail closed. POSIX remains case-sensitive. Own properties alone supply consumer values. Selected values follow the schema's bounded, NUL-free text contract. No new public generic environment API or broad environment-casing rewrite was added.

### Executed Red/Green Evidence

Before the production fix, an isolated-port execution of the **actual complete `runRepositoryCargoTestExecutableV1` body** reached its final consumer command and captured `SEMIO_TEST_CLI=stale` where the shared vector required `current`: **zero passed, one failed, 15 assertions**. Thus the regression reproduces the actual bad spread at the process boundary, rather than only testing a replacement model.

After the fix, **two tests passed, zero failed, 114 assertions**. Ten language-neutral adversarial vectors cover current-vs-stale selectors, all protected compiler fields, the other real runtime keys, explicit empty keep behavior, absent/undefined erasure, Windows canonical selection and alias conflicts, POSIX case distinction, NUL refusal and nontext refusal. An independent SQLite relation performs key membership, case matching, rank selection and compiler/runtime projection; Ajv independently admits/refuses the closed schema values. The actual private production projection agrees with both those outputs and the handwritten expected fixture.

The actual-runner port test also exercises the omitted consumer argument. It verifies no ambient selector survives, compilation still receives compiler state, final execution receives the explicit consumer selector, compiler policy remains unchanged, the same received invocation is reapplied, the selected consumer runs inside the reader callback, and rechecks still occur. Its boundary ports isolate storage/compiler/publication/process effects; test-only identity markers are data in those ports, not fabricated Original envelopes or native receipts. This is actual orchestration/projection evidence, **not native admission, Cargo compilation, immutable publication, child-process or PTY runtime evidence**.

The focused command was:

```powershell
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️.ts'
```

Both the existing Cargo executable test entry and the dashboard authority entry import this focused test. Parent and independent read-only reviews found no further concrete issue in the four-key contract, current consumers, Windows aliases or preserved compiler policy. The final-suite coordinator was notified after that review; it owns the once-only full current authority rerun. No native privilege/cleanup retry or source gate exemption was attempted.

Additional exact changed paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules`:

```text
📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts
📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧬️schema/🔣️.json
📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🔣️.json
📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️.ts
📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🟦️.ts
🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts
```

The previously pending environment finding is repaired by this section; whole native execution and the host/policy blockers remain unverified requirements. No goal or ticket state was changed by this agent.

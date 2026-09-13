# Norm Python Oracle Source-Owner Pre-Audit

> **Current status — accepted.** The completed, current-source evidence is in [Final focused acceptance](#final-focused-acceptance): ordinary and isolated registered routes are both 6/137, and the production Python host CLI completed the selected EN1991 case 65/65. The following initial analysis and red findings are retained as historical planning/proof context; the DIN16798 duplicate-ID limit remains current.

## Scope and result

This is a read-only pre-extraction audit of the one shared Norm Python mutation engine currently at:

```
✏️s/🔌️plugins/📕️norm/🔮️oracles/📦️packages/🐍️python/🗣️vocabulary.py
```

The implementation is an execution concern, not a language-package concern. Its 874 lines contain the shared mutation, inverse, carrier and handler engine: 42 top-level functions and two classes. Its only non-stdlib import is the first-party test-host protocol `semio_repo_test`; the other imports are `copy`, `json` and `re`. The current Python directory has no `pyproject.toml`, `uv.lock` or requirements file.

The neutral destination should follow the established Energy oracle pattern:

```
✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py
```

This removes the semantic implementation filename and locates the file by what it does. It does not turn the code into an installable package or add an external runtime dependency.

## Current execution and import boundary

The root contribution manifest is:

```
✏️s/🔌️plugins/📕️norm/🔮️oracles/🔣️.json
```

Its Python host declaration currently identifies package `semio_norm_vocabulary` and the local path:

```
✏️s/🔌️plugins/📕️norm/🔮️oracles/📦️packages/🐍️python
```

The test-platform registry loader selects that declaration by ancestor ownership through `oracleHostPackagesFor`. The Python host materializer in:

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts
```

uses a local `path` entry only to prepend the directory to `PYTHONPATH`. It never installs a local path. Its provisioning branch returns the selected interpreter immediately when no pathless external distribution exists, so there is no pip, uv or download operation in this Norm route.

Module selection is separate. Each subset adapter imports a module string itself; the host does not infer it from the source file:

```python
_vocabulary = import_module("🗣️vocabulary")
```

The exact current consumers are the 15 implementation files under:

```
✏️s/🔌️plugins/📕️norm/🗿️artifacts/
  {⚖️en1990,⚡️din18599,🌍️en1997,🌬️din16798,🏋️en1991,🏛️en1992,🏭️vdi3805,
   📇️iso16757,🔩️en1993,🧩️en1994,🧱️din4108,🪨️en1996,🪵️en1995,🪶️en1999,🫨️en1998}
  /🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*/🐍️.py
```

Every one resolves the same two public engine members: `Subset` and `build_adapter` are re-bound from the imported module, and the per-subset file supplies only its kinds, vectors, carrier asset and envelope. The current engine's `build_adapter` registers exactly oracle-role handlers; a selected EN 1991 adapter has 65 such handlers.

The identifier `semio_norm_vocabulary` also appears in the root contribution and in the 15 adapter docstrings. It is a logical host-package identifier, not the old filename or import selector. It may remain if it still identifies the same in-repository implementation; the code-level module selector and path are the references that must rebind.

The move must therefore make three atomic source-data changes:

1. Move the engine to `🔮️oracles/🏃️execution/🐍️.py`.
2. Rebind the root manifest's `path` to `🔮️oracles/🏃️execution` and declare `module: "🐍️"`, as the existing Energy execution owner does.
3. Rebind all 15 adapter import selectors to `import_module("🐍️")`. No compatibility `🗣️vocabulary.py` shim should remain.

The registry schema permits both `path` and `module`. The generic host consumes the path to set `PYTHONPATH`; it does not consume the selector for local paths. A dedicated source-owner oracle must consequently check both registry fields and all 15 adapter imports. Otherwise one half of this closure can silently become stale.

## Installed interpreter probe

I ran a focused host-protocol import probe with the system-installed Python at:

```
/Applications/Xcode.app/Contents/Developer/usr/bin/python3
```

The probe loaded the repository Python host, added the declared local directory to `sys.path`, then loaded the live EN 1991 adapter through the host's actual `_load_adapter` routine. It passed: the adapter resolved `🗣️vocabulary` at the declared old path and registered 65 oracle handlers, all oracle-role entries.

The exact output is retained while this extraction is active:

[import-boundary.json](🗑️generated/terra-norm-python-oracle-audit/import-boundary.json)

The probe used `PYTHONDONTWRITEBYTECODE=1`, did not install or provision any package, did not invoke a service, and read no production implementation. It proves the current host import and registration boundary only; it does not exercise a mutation vector or cross-language parity.

There is no third-party Python library in this engine to qualify. The appropriate post-move runtime evidence is the repository's existing first-party host route, scoped to one already committed Norm case:

```sh
SEMIO_AGENT_ID=norm-oracle-owner SEMIO_TEST_OUTPUT_SCOPE=norm-oracle/owner bun nx run @semio-tech/repo-test-domain:test-oracle --skip-nx-cache --   exhaustive --case 🏋️mutate-en1991-1 --implementation python
```

It should run after the source-owner seam is repaired, with its normal generated test artifacts isolated to the supplied task scope. The route uses the current registered Bun router `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts oracle` and executes the same Python host; it should perform no external package provisioning because Norm declares only a local `path` entry. This command has not been run in this pre-audit to avoid duplicating the executor's later focused runtime proof.

## Registered consumer and input closure

The actual registered route is:

```
@semio-tech/repo-test-domain:test-oracle
  -> bun ./📜️script.ts oracle
  -> materializePythonHost(...)
  -> PYTHONPATH=<Norm local host path>
  -> 🖥️host/🐍️.py --adapter <selected Norm 🐍️.py>
```

`test-parity` and `test` use the same host path for a complete oracle-plus-subject run. The checked launch seed and generated launch file currently register the generic `test-parity` command, rather than a Norm-case-specific oracle command. A source-owner target added for this move needs a matching seed and generated launch entry; an existing generic parity launch must not be relabelled as proof of the new source boundary.

The current `@semio-tech/norm-plugin:test` route is a Cargo route. Its project default input list is the Norm Rust tree plus its package root, and it does not execute this Python engine. It is neither a valid Python-runtime acceptance route nor evidence that the moved Python file invalidates an Nx task.

The generic `test-oracle`, `test-parity` and `test` targets have no explicit outside-project Norm source input list. `test-discover` explicitly names all `**/🐍️.py` files, but it only discovers cases and does not execute an oracle. Whether the generic execution targets' effective Nx inputs include this dynamically discovered external source has not been established here. Before acceptance, inspect their resolved inputs after the move or add the necessary explicit owner/adapter/manifest inputs. A manual `--skip-nx-cache` pass alone would not prove normal cache invalidation.

The existing portable test-platform control proves generic ancestor host-package selection and the generic `module` override rule, but it does not name the Norm path, its 15 adapters, or this engine's module selector. I found no existing Norm-specific portable source-owner fixture or selected native host assertion for the old path. Those controls need to be added with the move; generic test-platform coverage alone does not establish the Norm closure.

## Required acceptance controls

A focused portable source-owner control should prove all of the following from live paths, without a body-hash snapshot:

- exactly one Norm shared execution leaf exists at `🔮️oracles/🏃️execution/🐍️.py`, and the old `📦️packages/🐍️python/🗣️vocabulary.py` path is absent;
- the root contribution manifest identifies the execution directory and `module: "🐍️"`;
- all 15 committed subset Python adapters import exactly that anonymous module and expose only their subset data plus the shared `Subset`/`build_adapter` binding;
- there are no adapter-local copies of the shared engine's mutation functions;
- the generic test-platform path still obtains the declared local directory through ancestor contribution lookup and supplies it to `PYTHONPATH`;
- the registered target's effective inputs include the manifest, execution owner and all selected adapter sources; and
- a moved-source host probe still loads one adapter and observes its expected oracle registrations, followed by the bounded registered case route above.

## Limits

This audit did not move source, modify registry data, run Nx, run a parity suite, run Cargo, use a virtual environment, provision a third-party library or execute a service. It is preparation evidence for the source-owner move and not acceptance of that move.

## Current execution-boundary finding (2026-09-13)

The move is not acceptable yet. `materializePythonHost` launches the committed host as a Python script and provides local oracle roots through `PYTHONPATH`. Python puts the script directory (`.../🧪️test/🖥️host`) at `sys.path[0]` ahead of that variable. Since both the host and the moved Norm engine have the anonymous module filename `🐍️.py`, the adapter’s `import_module("🐍️")` resolves the host itself instead of `🔮️oracles/🏃️execution/🐍️.py`; the host does not export `Subset`.

This is a domain-neutral host-materialization defect, not a Norm selector defect. The required repair is an explicit ordered local-module-root contract from `materializePythonHost` to the Python host, which installs declared local roots ahead of the host script directory before adapter loading. A hostile same-basename control must prove that the declared local source wins. `PYTHONPATH` alone and the imported-host inspection control are insufficient evidence.

Root also found a separate existing DIN16798 feature duplicate: 126 expanded rows but 125 distinct scenario identities. The 15 adapter maps have exactly the 125 distinct committed identities, with 799 handlers across all standards. Native source ownership may establish distinct-identity registration and the selected EN1991 run, but it must not claim full feature validity or a full 15-case parity/contract run while that duplicate persists.

## Host repair inspection

The generic repair is structurally correct: `pythonHostArguments` emits repeated `--local-source` roots in declaration order, and the committed host validates, de-duplicates, and places them before its script directory before loading an adapter. The composition target includes both the host source and a same-basename local fixture in its Nx inputs.

The reported focused native hostile control is still a helper-boundary proof: it loads the host by `importlib.util.spec_from_file_location`, calls `_prioritize_local_source_paths`, then imports the fixture marker. It does not invoke `run_main` or parse the `--local-source` command line. The selected Norm host invocation must remain the acceptance evidence for the actual CLI boundary.

## Generic host acceptance

Root’s selected Norm invocation now uses production `pythonHostArguments` and invokes the committed host through `argparse`/`run_main`. It resolves the declared `🔮️oracles/🏃️execution/🐍️.py` rather than the host’s same-named leaf and reaches native dispatch. This closes the generic Python import-boundary defect.

The run proceeds through the selected EN1991 plan but currently has 64 lookup errors from a separate stale Norm fixture coordinate; one identity case proceeds. That source-data failure is outside host selection and keeps the Norm native gate red. It must be repaired and rerun before accepting the Norm move.

## Selected native acceptance

Root’s corrected selected native CLI control is green: 1 test passed, 5 filtered, 9 assertions, 9.86 seconds (the EN1991 case itself: 9.474 seconds). It built the real 65-scenario plan, invoked the committed Python host through `argparse` and `run_main` with production `pythonHostArguments` and `--local-source`, and emitted 65 passed Python/oracle rows with the exact scenario IDs. The sole stale vector URI had an obsolete `/🧪️tests/` segment; its coordinate was rebased without changing mutation logic. Receipt: `🗑️generated/coordinator/norm-oracle-ownership/native-selected.log`.

This establishes the concrete host/module selection and one actual artifact case. It does not establish full fifteen-artifact parity or strict full-feature validity; the retained DIN16798 duplicate-identity limit remains separate.

## Final focused acceptance

I independently inspected the current Python owner, manifest, fixture, schema, source-ownership test, package target, launch entries, and the retained direct and isolated Nx receipts. The former package-named `🗣️vocabulary.py` leaf is absent. The anonymous implementation owner is `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`; every one of the fifteen adapter declarations resolves to it with `package: "semio_norm_vocabulary"` and `module: "🐍️"`.

The acceptance control exercises the production `pythonHostArguments` path and invokes the actual host CLI through `argparse`/`run_main`, with ordered `--local-source` arguments. This is the required boundary for anonymous `🐍️.py` leaves: the declared local source wins ahead of the host script directory, while `PYTHONPATH` remains the descendant-import mechanism. The dedicated selected EN1991 case completed 65 of 65 scenario rows through that path.

The ordinary package route passed 6 tests and 137 assertions in 15.16 seconds. The current isolated registered route, `@semio-tech/norm-js:test-oracle-source --skip-nx-cache`, also passed 6/137; its test phase took 15.20 seconds, Nx took 16.3 seconds, its critical task was 15.7 seconds, and its cache was skipped. The target declares the implementation-only `normOracleSources` set separately from its test/control, host, manifest, feature, fixture, taxonomy, and launch inputs. Current attribution records 27 live paths and the one removed former source path, for 28 identities.

This is a bounded acceptance of the source-owner, host-selection, source-as-data, target-input, and selected real-CLI seams. It does not establish complete semantic parity of all fifteen imported standards. In particular, DIN16798 presently expands to 126 rows but has 125 distinct identities because `@id-mutate-change-bedrooms` is duplicated. The fifteen native maps exactly cover those distinct identities (799 total handlers); the control deliberately tests that identity-set contract and retains the duplicate as a separate Gherkin contract defect.

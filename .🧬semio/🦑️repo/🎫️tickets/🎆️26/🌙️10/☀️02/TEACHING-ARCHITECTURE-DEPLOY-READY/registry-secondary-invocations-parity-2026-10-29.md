# Secondary Registry Invocation Parity — Go And Rust

The parent assigned a small multi-implementation registry addition while the actual cold caller1293 retains execution priority. This is a read-only contract trace; no Go/Rust Source edits have occurred before the parent publishes the schema/types and notifies the shared Linux owner.

The proposed closed row is invocations:[{command:"bunx",prefix:["x"]}]. It means the same tool's primary executable, with the declared prefix before caller arguments. There is no target, cross-tool chain or arbitrary sibling-executable fallback. Tool-id calls still choose commands[0] without the secondary prefix. A sibling command such as rustc remains its ordinary command unless the registry explicitly declares its invocation.

## Exact Owners

Go: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🌿️git/🐹️.go. registeredTool54–67 currently reads id,commands,locators and environment. routeRegisteredTool166–185 returns only executable/environment; ToolCommandContext255–261 passes caller arguments directly to exec.CommandContext. RunTool213–246 selects commands[0], and its current API need not change.

Rust: same workspace owner/🦀️.rs. RegisteredTool810–816 and parse_tool_registry830–839 currently omit invocations. route_registered_tool906–913 returns path/environment. spawn_command792–807 consumes that projection; its public Command return lets callers append arguments. run_tool941–961 already resolves tool IDs to commands[0].

Existing owning tests are workspace/🧪️tests/🔬️unit/🐹️.go and 🦀️.rs. Rust has a registry projection law at43–49 and actual registered Git/unknown-id execution law at53–60; Go's matching unit owner includes registered tool execution. No native build or Go execution has been started for this work.

## Coherent Implementation After Schema Publication

Add the exact invocation projection to each partial registry reader. A small pure selector should distinguish an explicitly mapped secondary command from a normal owned command, returning the primary command and immutable prefix for mapped cases. Route resolution must then resolve that selected primary using the existing registry environment/PATH/locator policy. The execution adapter carries executable,prefix,environment and appends the prefix before caller arguments. It must not resolve an ambient bunx sibling for the declared mapping.

Preserve explicit-path bypass, unknown-command behavior, missing-tool behavior, guard-off policy, sanitized environment and tool marker, cancellation and caller arguments exactly. Rust must append the prefix after any existing Windows cmd /c wrapper's target and before future caller args. Go must allocate the combined argument slice without mutating the caller's backing slice. No wrapper script, API compatibility alias or dependency is needed.

The neutral registry/argv corpus and expected output should be shared with the parent's TypeScript/schema oracle. Positive cases should cover bunx→primary bun +x and tool-id→primary bun with no prefix; negative/unmodified cases should cover explicit paths, unknown commands and undeclared sibling rustc. Actual own tests should inspect complete argv and sanitized environment using the existing owning fixtures; pure projection results should match the independent implementation. Compilation scope and any unrelated native Source-cap refusal must be reported honestly rather than worked around.

The Go/Rust adapters currently support their own existing environment/PATH/directory locator projection, not the full TypeScript SDK provisioner; the addition must preserve that boundary and must not claim new cross-platform provisioning from a prefix field alone.

## Schema/Test-Only Checkpoint And Actual Resolver RED

Parent authorized only test/schema/corpus changes while OS-MCP production closures are active. Production Tool registry DATA/schema/types, resolver/run, Go and Rust remain untouched. The existing resolver test owner now imports `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve/📣️invocations/{🔣️.json,🧬️schema/🔣️.json}`. Its closed declaration is exactly command+nonemptyfiniteprefix; target/chain/empty-prefix shapes are rejected by independent Ajv. Ninehostresolution cases exercise the ACTUAL current resolveToolV1 using structurally extended fixtureentries, without pretending the unpublished production registry accepts the field.

Actual focused RED3637 settled exit1: **15passed/7failed,22tests,64assertions**. Failures are exactly mappedinvocation cases environment,runtime,pinnedprovision,path,Windows.exe,Windows.cmd,interpreterprefix. The current secondary-path-only resolver eitherreturns missing orselectsambientbunx instead of qualifiedprimarybun+x. Tool-ID primary and genuine rustc sibling stillpass; all11preexisting resolver/pin/environment laws pass. Log `🗑️generated/tool-invocations-resolver-red.log`. An initial harness-only TS nonnulltoken required transpiling the extracted actual capacityexpression before VM evaluation; corrected run excludes that harnessfailure and retains the genuine7resolverfailures.

The independent Lodash oracle selects the declared invocation and expectedprefix. Actualproduction fullArguments AST initializer is evaluated on neutralvectors and compared with independent Lodash concat: prefix→callerargs→suffix, including Windowscmdwrapper order. The actual requested/capacity initializers retain options.command=alias and select the alias test capacityrule, not primarybun. These casespass. This is pureprojection/expression evidence, not nativeWindowsprocess execution or Go/Rustparity; those exactadaptertestsremainpending.

No newruntime dependency, ambientPATH change, fakeOriginal, actualCLI assertion weakening, aliasfallback or implementationclaim. Existing realCLI bunx execution and ordinarydiscovery stillneed their separate coherentproductioncheckpoint. Fullcourseunit28199 remains retained and42persistencereads waitmanaged4262 readiness.

## Coherent Production and Windows Boundary Checkpoint

Parent authorized simultaneous exact sharedSource edits. Support schema/types/TSresolver and Go/Rust projected readers were staged first; BunDATA declaration was published onlyafter allthree adapters accepted the newfield. Registry rejects foreign/primary/duplicate command mappings and closedrow/null/type/prefix errors. TS resolvesdeclaredaliases through qualifiedprimarylocators, composes existingprefix+aliasprefix and preservessuffix; genuine siblings and requestedcommand capacity remain. Go/Rust primarytool-ID routes stayprimary; unmapped siblings no longer accidentallyuse a primary-only tooloverride. Allothercurrentpeercontent was retained through targetedprecondition replacements; noGitmutation/Sourcecap changes.

Ordinary library workspace-contract nowexplicitlyimports unchangedreal tool-registry exec/signal laws. Actualfocused70825 exit0:43tests/4files278assertions, including realunchanged bunx --version output1.4.2 and Nodeprimary/unknownrefusal/signalSQLite laws. Current Windowscontract extension focused52704 exit0:44tests/4files295assertions. Complete Sourceledger+SHA is `🗑️generated/tool-invocations-source-checkpoint.json`.

Windows materialissue was addressed through existing system/libraryinterfaces: Rust now hands the actualbatchtarget to supported std::process::Command so its ownbatchencoder/refusals coverlateargs; no manualcmd/raw_arg. Go uses a closedprivatebatch fragment encoder with explicit typed ToolCapabilityError refusal for quotes,percent,controls,invalidtargetaliases and commandlinebound. Accepted spaces/Unicode/carets/metacharacters/empty/trailingslash fragmentvectors match existing test-onlycross-spawn7.0.6; noexternalruntime dependency. OnlyregisteredWindowsbatchroute sets the Windows-specific SysProcAttr.CmdLine with /d /v:off /s /c. Nativeexecutableargv, explicitpath/guardoff behavior, callerarray, env and contextcancellation remain original. Fullrecordertestsfor realWindows were authored in bothnativeunitowners; they have NOTexecuted onLinux. Neutral/Sourceevidence isnotphysicalWindowsproof.

Goowning15820 is oneordinarynormal repo-workspace-go:test invocation selecting the3newnative laws; existingWindowsargument law nowalso drives theactualGoencoder/sharedoraclecorpus. It remains queued, notcompiled/passed. Rustfollowup waits thatactualterminal; no overlappingnativejob was added. Fullunfilteredteachingunit28199 is separatelyretained. Registeredgofmt attempt refused missinggofmt/PATH; no formatterpass, PATHshim, manualinstall orrawcompilerfallback isclaimed.

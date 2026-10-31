# Native Direct Script Original Admission

The existing native public script interface recognizes the owning package router independently of build graph and JavaScript readiness. It does not, however, issue the genuine Script Original that this router requires. Therefore no independent standalone execution of the reserved host suite is established by that interface. No test or script was run for this trace.

## Actual Native Launcher

The canonical Go launcher, not the Rust domain launcher, owns build and script process launch. Go entry/🐹️.go:119–135 recognizes only neutral leading watch/cwd options and a .ts/.mjs/.js script. Plan:170–171 chooses kind script before ordinary build routing. Host.Execute:418–423 splits that script from its unchanged arguments; at443–448 it resolves the pinned runtime and hands off directly to script+args with h.Environ(), runtime PATH and fixed runtime environment. It does not execute the ordinary build CLI, graph, dependency ensure or Dependencies store opener. It also does not create SEMIO_SCRIPT_PROCESS_INVOCATION, a policy, capabilities, or an Original. The launcher registry fixed runtime environment contains only BUN_RUNTIME_TRANSPILER_CACHE_PATH=0.

The native script spelling is repo <absolute owning package 📜️script.ts> test original-reserved-host, or the exact authored command repo ./📜️script.ts test original-reserved-host in the owning package cwd. Leading --service=off is not a neutral script prefix and must not be assumed to select this route; the direct script branch does not use resident build forwarding anyway. This describes recognized native syntax, not a command authorized or proven to execute this test standalone.

## Receiver Requires an Existing Genuine Parent

The owning package router ends at library/packages/typescript/📜️script.ts:34 with receiveScriptProcessInvocation(process.env, original => runScriptMain(router,{invocation:original})). Its defining process/routing/invocation/process/🟦️.ts:254 rejects absent SCRIPT_PROCESS_INVOCATION_ENV with Original parent Script process invocation required. At actual process birth it also requires the wire to be the actual process environment value and rejects a second received birth. It validates the envelope and enters withScriptProcessEnvelope before running the router. It does not synthesize an Original for a direct router birth.

Thus a direct native script may carry an already-issued real parent when an authentic declared task executor launches it; that is why the exact options.command is correct inside repo:script execution. The native interface alone supplies no such parent for a fresh standalone request. A handwritten environment envelope, direct Bun invocation, test-generated stand-in, or treating the options.command text as capability issuance would bypass the requirement rather than prove it. Module imports could fail even earlier if unavailable, but reaching the receiver still necessarily requires that existing parent.

## Consequence

The canonical declared target remains the rightful available owner and is dependency/graph gated as recorded in original-host-source-route.md. The standalone native script interface avoids those guards but does not supply the missing Original, so it cannot currently seal this independent genuine-owner gate. An additional authorized genuine issuer/owner path would require its own separately assigned design and tests; no such path or workaround was added. Existing pure direct-tool results must retain their limited provenance and must not be relabeled a genuine Script execution. No Source edit, fake environment, direct tool execution, setup retry or Native effect ran.

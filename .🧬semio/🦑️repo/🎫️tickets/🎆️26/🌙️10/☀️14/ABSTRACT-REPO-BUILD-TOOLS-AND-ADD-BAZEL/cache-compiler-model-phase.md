# Current Compiler Phase Model

The explicit phase migration changed the actual dispatcher to `createRequestedIssuedResourceControlV1(phaseControl,120000)`. Its permanent AST model was still binding the former factory and omitted the third extracted-method parameter.

Before editing, the current Source-role capture receipt `tooling-closures-propagation-current.json` was checked:115reader/190runtime/70compiler entries, zero intersections with the changed permanent test path. `cache-compiler-model-role-intersection.json` retains the exact receiptSHA and path evidence. No production Source was changed.

Actual RED `cache-compiler-model-phase-red.log`:0pass/2fail, `ReferenceError: phaseControl is not defined`; this was a fixture-port failure, not a production compiler-order regression. The permanent test and its matching ticket-only model now pass a genuine caller phase, bind the same canonical requested-control factory, and authenticate each observed control as its refinement. The unchanged language-neutral three-case schema/corpus is validated by Ajv; SQLite independently checks1/3/1 Bun stages finish before each120000ms compiler admission.

Actual GREEN `cache-compiler-model-phase-current.log`:1pass/0fail/442ms. This executes the current extracted dispatcher branches with controlled Bun/compiler ports and actual issued controls. It performs no Native/SDK/store work and no real120s compiler wait. It does not qualify the full SDKsnapshot/pruning/journal target. The focused compiler result is retained separately; no transpilation-as-typecheck claim.


Focused exact-root compiler `--strict-compiler-model` completed with status0 and no diagnostics (`cache-compiler-model-focused-typescript.log`). The earlier generic `--strict` attempt failed on transitive resource-leases/browser fixture diagnostics outside the changed model; that log is retained as `cache-compiler-model-phase-typescript.log` and is not reported as a full strict pass. Both checks were Source-only under the unchanged genuine60-second diagnostic owner.

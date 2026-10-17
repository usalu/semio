# Command Audit Literal Execution Review — 2026-10-17

Parent requested a read-only review of its current literal-program fix and seven-vector exact-AST audit regression. Parent retains implementation and SQL/cwd-test edits. No audited source/test files were modified. No Native/cache operation was performed.

## Scope and Current Correction

The current audit constructs literal operations directly from `execution.program` and `execution.args`, avoiding shell parsing and interpolation of literal data; authored commands still pass through `string-argv`. Literal cwd does not expand authored placeholders; authored cwd expands workspace/project roots. The existing seven vectors cover authored single/list recipes,literal argv including empty/quotes/placeholders/Unicode,literal executable boundaries,authored/literal cwd and noop. The test extracts the actual entire production graph loop with the TypeScript AST and executes it against modeled inspection/filesystem ports. It does not exercise target configurations or argument-forwarding controls.

## Material Valid-Execution Gaps

1. **Literal disabled forwarding is ignored.** Audit always inspects `execution.args` in literal form. Actual execution suppresses shell arguments when global forwarding is false (unless an individual authored operation explicitly enables it). A schema-valid literal `{kind:"command",program:"node",args:["missing/📜️script.ts"],forwardAllArgs:false}` really executes `node` alone; the audit inspects the dropped script and can reject a valid recipe. Configured forwarding overrides are similarly missing from the seven vectors. The audit's seen-key also omits forwarding semantics,so an otherwise identical configured recipe may reuse the base observation.
2. **Authored literal defaults are omitted.** Audit parses only authored command text,without appended `execution.args`. Actual execution appends those literal defaults when forwarding is enabled. A schema-valid authored `command:"node"` with args containing a script reference executes that reference,but the audit never inspects it. Any fix must preserve per-operation `forwardAllArgs` overrides: an explicit per-item true can override global false,and false can disable only that item.
3. **Owner-argument transport must remain separate.** Actual backend intentionally suppresses shell defaults when `execution.ownerArguments` is true,transporting effective arguments through the existing environment protocol. Literal audit currently treats those defaults as shell argv. This is another reason not to append every default unconditionally.
4. **Noop cwd is checked despite no execution.** Audit verifies cwd existence before skipping `execution.kind!=="command"`. Schema admits noop executions with cwd. Actual Bazel execution returns for noop before resolving cwd; an absent unused directory can therefore produce a spurious audit failure. The seven-vector noop case has default cwd and does not expose this boundary.

The target configuration reader disallows changing command form and literal program identity,so shallow base/configuration execution merging does not itself establish an additional mixed-form defect. No unsupported graph configuration was used to justify these findings.

## Actual Bounded Proof

A read-only Bun inline check extracted the exact current audit graph loop and exact `buildExecutionArgumentsV1` production function with the TypeScript AST. Ajv2020 validated both base-false and configured-false graph inputs against the actual neutral schema. The actual loop and actual argument function executed without Native/process effects. Terminal0 printed:

```text
[DEBUG] Read-only actual audit forwarding discrepancy {"configured":false,"schemaAdmitted":true,"audit":[["node","missing/📜️script.ts"]],"actual":["node"]}
[DEBUG] Read-only actual audit forwarding discrepancy {"configured":true,"schemaAdmitted":true,"audit":[["node","missing/📜️script.ts"]],"actual":["node"]}
```

The configured input has a base recipe and a release `forwardAllArgs:false` override. Its shared seen-key collapses the identical raw audit command,so the single observation is not a separate release callback receipt; comparison to the selected release arguments still shows the forwarding distinction is absent from audit identity. The concrete base-false discrepancy alone proves the spurious-reference behavior. Filesystem existence and inspection ports were modeled; no nonexistent script was executed. Authored/per-item/owner-argument/noop findings are Source inspection evidence,not newly run backend acceptance.

Actual Bazel Source confirms it computes defaults using `buildExecutionArgumentsV1({...execution,forwardAllArgs:true},task.args)`,then decides forwarding per operation using `operation.forwardAllArgs ?? execution.forwardAllArgs`,and additionally gates shell forwarding on `!execution.ownerArguments`. Any repair should follow that rule rather than globally dropping defaults before inspecting authored operation overrides.

## Reviewed Checkpoint

Parent may change these shared inputs after this observation.

| Path | SHA256 |
| --- | --- |
| `command-audit/📜️script.ts` | `8f0cabe15a699a320fc8deccaa29368a3458a084c17ca82f77cede4010c68c3a` |
| `command-audit/🧪️tests/🟦️.ts` | `8f28d1a607e90e2f966aa7b11994b15f6a3b14d063be4c7c0b20f6c8f2f0b86d` |
| `command-audit/🧪️tests/🔣️.json` | `cd5cc777141df05c28bdd58c638a650312b7de9ea0dcb3b296d185af1438b0c7` |
| `command-audit/🧪️tests/🧬️schema.json` | `0310eaca92df051bb3c0363faacb1bab5baac51fb4b190683901b20f38804adc` |

## Implementation Follow-Up

Parent subsequently assigned the complete operation/default/noop repair. Its schema-first22-case implementation and exact actual Source/strict results are recorded in `2026-10-17-command-audit-effective-operations.md`. This review retains its earlier audited snapshot and evidence rather than rewriting those findings as current failures.

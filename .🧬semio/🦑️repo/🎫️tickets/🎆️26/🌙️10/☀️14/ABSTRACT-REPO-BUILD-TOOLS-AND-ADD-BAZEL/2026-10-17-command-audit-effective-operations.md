# Command Audit Effective Operation Repair — 2026-10-17

Parent assigned ownership of `command-audit/📜️script.ts` and its existing Source tests after the read-only review in `2026-10-17-command-audit-interface-review.md`. No Native/cache operation was performed. The normal carried immutable qualification graph route remains the non-Source entry; its real whole-graph run has not been repeated by this subtask.

## Schema-First Contract and Repair

The fixture schema was expanded first from exactly7cases to a closed10+-case contract with target executions,configurations,expected effective argv/cwd observations and modeled missing directories. Twenty-two language-neutral cases were authored before changing the actual audit loop. They retain the original authored/literal/empty/quote/Unicode/placeholder/cwd/noop cases and add disabled literal forwarding,authored literal defaults,global true/false,per-command true/false overrides,owner-argument transport,configured args/forwarding/command/cwd,noop unused cwd and identical configured-operation deduplication.

Production now resolves configurations through actual `resolveBuildTargetV1` and reads declared defaults through actual `buildExecutionArgumentsV1`. It requests defaults with forwarding enabled,then applies the actual operation-specific `forwardAllArgs ?? execution.forwardAllArgs` decision. Thus an explicit per-command true can override global false. `ownerArguments` suppresses shell-argv defaults as the backend does. Authored commands retain `string-argv` parsing,then receive the literal defaults; literal programs directly preserve their executable and argument boundaries without shell parsing or authored interpolation. Noop executions are skipped before cwd checks. Effective argv plus cwd now identifies an already-inspected operation,so configuration changes in defaults/forwarding do not disappear behind identical source command text.

The actual AST-loop regression uses both real production APIs above. Every graph is admitted by Ajv2020 against the actual canonical neutral graph schema. Independent SQLite derives configured executions,per-operation/global/owner-argument forwarding,cwd token behavior and literal argv from raw target JSON; it combines the third-party authored parser's tokens with literal defaults. It does not reserialize the fixture's expected argv as an oracle. Every SQLite observation is independently compared with the authored language-neutral expectation and then with the actual production loop. Filesystem existence,inspection and progress/control ports are explicit models. No script payload or backend command executes in this Source law.

## Source Entry and Registration

The existing helper now supports standalone `--source` and `--strict` before entering the carried-pin route. Each creates one genuine60-second Original with workspace-only capabilities and runs the exact Bun test file as an owned child under that Original's remaining budget. `--strict` first runs TypeScript on the helper and test root under the same Original. Neither mode requests a Native grant,opens a protected cache or needs an immutable graph pin. The existing EN/DE ticket-helper launch configurations already list `command-audit` and provide an explicit mode/arguments input,so these modes are executable through that registered route; no duplicate launch entry or separate script file was added. Generated output remains caller-owned by the ticket environment; these Source laws create no retained physical fixture.

## Exact Test Evidence

- Parent's earlier semantic RED is preserved in `🗑️generated/coordination-17/command-audit-semantic-red-loop.log`: original actual loop rejects the literal program recipe with `Missing command operation`. It records0pass/1fail/11assertions/558ms. Earlier missing parser/module-parent-path attempts were harness failures,not production-regression evidence. This subtask read the exact log and does not relabel those attempts.
- New22-case actual RED before production repair: `literal-disabled` unexpectedly retained `missing/📜️script.ts` in argv although the independently derived expected argv was only `node`. Terminal1,0pass/1fail/34assertions/989ms. Earlier cases,including the literal boundary vectors,had passed before this concrete failure.
- Corrected direct Source law: terminal0,1pass/0fail/90assertions/1.336s. Actual loop observed22cases,7configuration overrides and29distinct effective operations.
- Actual standalone `--source` entry: terminal0,1pass/0fail/90assertions/1.200s; diagnostic confirms the genuine Original maximum60000ms and `strict:false`.
- First actual `--strict` run found two own Source type errors: missing ScriptInvocation callback annotation and an optional configurations object passed to Object.keys. It returned terminal1 with owned child status2. Both were corrected; no unrelated check was suppressed.
- Final actual `--strict` entry: terminal0. Strict TypeScript completed,then the same owned Source law passed1test/90assertions/2.98s under the same unchanged60-second Original. Runtime diagnostics:

```text
[DEBUG] Actual qualification audit loop {"cases":22,"configured":7,"operations":29,"canonicalGraphSchema":true,"actualCallerAndConfiguration":true,"sqlite":true,"stringArgv":true,"nativeEffects":0}
[DEBUG] Command audit Source Original {"maximumElapsedMilliseconds":60000,"strict":true,"nativeEffects":0}
```

Final scoped whitespace validation returned0 after the type-only corrections. No public command,whole immutable graph audit,Native image,cache publication or backend lifecycle was executed. The audit remains a declared-command/reference inspection,not a shell-environment execution emulator or a new physical acceptance claim. Existing broad ticket failures remain historical and are not counted as passing.

## Final File Checkpoint

These observed shared bytes may change after this report.

| Repository Path | Bytes | SHA256 |
| --- | ---: | --- |
| `command-audit/📜️script.ts` | 7737 | `085f3b508a79685d4bcc18d4d026cdd948862640cf6e88d6444ced5abdcd7846` |
| `command-audit/🧪️tests/🧬️schema.json` | 1589 | `4c2da491e6032443c819f3232daacb69d0a9e39babb825349e6f615433796c5c` |
| `command-audit/🧪️tests/🔣️.json` | 10547 | `8d5b56dd5058123bd4bc86e61b1ece14e334300fbdb31f0ae704e7195c5a4e5c` |
| `command-audit/🧪️tests/🟦️.ts` | 5875 | `002de4680d674ca67c8cf2b00d19b89502557b450c88d93af7d2fbd555ba8b83` |

# Build GUI Initial Authority Read-Only Audit

Local read-only source reasoning. No execution, compiler, GUI action, source change, lock/quota/custody change or runtime qualification.

## Confirmed Current Failure Path

`.vscode/launch.json:40843` declares native child 1200000 ms for verify-jack-native-composition. Rows 57885/57904 declare native child 180000 ms for Record Original Receiving Source/Native. Each embeds `${workspaceFolder}` inside the JSON string held in SEMIO_SCRIPT_CAPABILITIES. Windows substitution with a path such as `C:\work\semio` inserts backslashes into JSON string contents without an authored JSON escape; `\w` is invalid JSON. This is source reasoning, not an executed Windows probe. A plain environment path such as CARGO_TARGET_DIR does not have the additional JSON parsing boundary.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:76–80` parses SEMIO_SCRIPT_CAPABILITIES directly and admits authored grants before the initial envelope. `runBuildCliV1:187–194` then creates that original envelope or retains an incoming one.

Replacing the embedded workspace with `.` alone is currently unsafe: `📥️caller/🟦️.ts:27–31` compares resolved workspace identity but returns the unnormalized declared workspaceRoot. `runOwnedBuildInvocationV1` line 93 preserves native grant only if **original.workspaceRoot === root**. Thus a successfully admitted `.` grant is replaced by freshly configured factory authority at this handoff. The exact 180000/1200000 child bounds can become the configured factory default. This is a confirmed source branch mismatch, not a runtime result.

`issueBuildTaskEnvironmentV1` caller lines 46–48 resolves root/artifact comparisons, but it subsequently passes the original artifactDirectory into native owner admission at 54. Initial normalization must happen before the first envelope so all downstream strict and physical identity checks see the same canonical paths without reissuing caller authority.

Both authoring and caller schemas admit nonempty path strings up to 256 chars. Neither requires `launches`, although authoring admission unconditionally calls `declared.launches.some(...)` at 30. A schema-admitted object without launches therefore throws TypeError rather than a defined admission refusal. Current affected rows include launches; this is an independent schema/code gap.

## Smallest Clean First-Party Declaration

Use the existing single structured authoring declaration and its defining schema; handcraft the affected GUI JSON strings with literal relative path declarations: workspaceRoot `.` and native.artifactDirectory `.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture`. Keep exact argument/selection/launch arrays, transport maxima, offline selection, build mode and the explicit child bounds (DSL 180000, composition 1200000). Keep ordinary CARGO_TARGET_DIR `${workspaceFolder}/...` outside JSON.

At **initial authoring admission**, resolve workspaceRoot against the explicitly supplied CLI root, verify identity against that root, resolve native artifactDirectory against the admitted canonical workspace root, and return the canonical absolute paths alongside the unchanged exact resource objects. Validate the resulting canonical caller shape before creating the envelope. This is one authored declaration and one admission boundary, not a second runtime API or a compatibility mode. Fix schema launches requirement to match actual admission law.

For already-issued envelopes, retain source/resource custody and refuse changed root identity rather than regenerating factory resources. The CLI initial declaration must be normalized before `createScriptProcessEnvelope`; changing only later comparisons is insufficient and obscures which resource owner was issued. Factory 3600000 ms remains only an undeclared/default factory choice and must not substitute for explicit GUI grants.

## Existing Test and Required Evidence

Build graph test `🏗️build/🧪️tests/🕸️graph/🟦️.ts:222–230` proves authored custom transport/child equality through readAuthoring and createBootstrap using an **absolute** root. It does not exercise relative declarations, runOwnedBuildInvocation handoff, exact native task issuance or a Windows path substitution. Extend the defining portable vectors with canonical root plus relative path declaration, exact DSL/composition resource bounds, absent-launches refusal and Windows path serialization. Drive the complete initial CLI → original envelope → orchestration → native task issuance chain and assert exact child bounds and canonical identity survive. These probes remain pending.

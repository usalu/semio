# Neutral Interface Audit 51

Read-only current-source inspection on 2026-10-13 after reading ROOT-COMPILER-OWNERSHIP-47.md, RECEIPT-VERIFICATION-49.md and ROOT-COMPILER-RECEIVING-50.md. No tests, compiler tasks or browser runtime were executed by this auditor. Prior reported green runs are not independently certified. Only this report was created; no source, GUI, AGENTS, Git state or worktree was changed.

## Actionable Findings

### P1: Original Deadline Does Not Bound Compiler Admission Waits

`🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🟦️.ts:24` passes only the original signal into withCompilerState. It checks remaining incoming budget at line 26, after acquisition. `🏗️build/🗃️state/🟦️.ts:30–31` supplies no deadline to capacity or state leases. Resource lease defaults to Infinity (`🔒️leases/🟦️.ts:100`); capacity acquisition has no deadline input (`🏗️build/🎟️capacity/🟦️.ts:21,28,38`). ScriptControl represents remaining time separately from cancellation (`🧭️routing/📥️invocation/🟦️.ts:9–13`). The actual compiler test itself uses a finite remainingMilliseconds callback with a separate controller (`🏗️build/🟦️typescript/🧪️tests/🟦️.ts:72–74`); expiry need not abort that signal. A contended request can wait past its original finite deadline until another holder releases or caller aborts. The later budget check cannot terminate the wait.

Next scope: General Process compiler state/capacity admission. Check original remaining authority throughout every admission polling stage, preserving original signal identity. Add finite-deadline contention laws with an intentionally un-aborted caller signal for both host capacity and state exclusion.

### P2: Expensive Admission Waits Lose Original Progress and Continuation

State acquisition calls at lines 30–31 omit onWait; the state interface at line 25 has no progress or continuation port. TypeScript advances original invocation before and after acquisition (`🏗️build/🟦️typescript/🟦️.ts:18,25`) but cannot publish or yield through caller control during contention. Capacity shared-core waiting reports only optional progress (`🏗️build/🎟️capacity/🟦️.ts:38`) and receives none here. Resource waiting can print its own console messages instead of calling original publisher (`🔒️leases/🟦️.ts:101–105`). Signal identity is retained; awaited original progress/continuation during expensive admission is not.

Next scope: the same neutral admission protocol. Route awaited wait progress and continuation through original invocation, test asynchronous publishers under contention, and preserve original deadline rather than resetting it.

### P2: State Path Admission Checks a Directory Instead of Materialized Paths

`🏗️build/🗃️state/🟦️.ts:20` admits state directories up to 256 UTF-16 units; TypeScript appends `/state.tsbuildinfo` (`🏗️build/🟦️typescript/🟦️.ts:26`). Admitted directories near the ceiling produce paths exceeding 256. The state lease appends `/leases/` plus a 64-character hash and `.sqlite` to root (`🏗️build/🗃️state/🟦️.ts:31`; `🔒️leases/🟦️.ts:44`). Inherited capacity skips the separately checked queue path entirely (`🏗️build/🎟️capacity/🟦️.ts:25–26`), exposing those longer paths directly. Relative roots are checked lexically without their absolute cwd prefix; schema accepts nonempty roots up to 256.

Next scope: validate resolved absolute materialized paths before filesystem creation, accounting for compiler leaves and SQLite sidecars. Add absolute/relative boundary laws and inherited-capacity coverage. Queue-path refusal does not certify all generated paths.

## Receipt Assessment

Neutral identity schema and public interface remain first-party. Kernel root imports neutral receipt identity and shared text, with no direct OS receipt parser import. fetchPackageDescriptor requires explicit source, passes exact signal/callback, validates returned identity and binds plugin/package (`🎠️kernel/🟦️.ts:51–59`). Actual OS source imports the sibling wire parser and invokes parseComponentReceiptV1 before returning four-field neutral identity (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️receipt/🟦️.ts:1–20`). Abort checks bracket fetch/parse. No receipt extraction regression was established by this audit.

Shared text still consumes full stream before descriptor capacity check, and has no reader release/cancel on callback failure (`🎠️kernel/📇️descriptor/📥️text/🟦️.ts:3–13`; kernel root line 89). Promise.all also leaves the surviving fetch active when its companion rejects. These belong to existing shared descriptor lifecycle scope; they do not establish that identity extraction weakened wire validation. Kernel wildcard publication and fault taxonomy remain separate open work.

## Practical Next Scope

Correct compiler admission authority/progress and complete materialized-path bounds in General Process; preserve other owners' Public/Catalogue work. Run strict neutral compiler receiving with new contention/path laws, then repeat existing whole Specific integration when justified by those changes. This source inspection establishes no broad General acceptance, browser Wgpu acceptance, or global deletion-graph result.

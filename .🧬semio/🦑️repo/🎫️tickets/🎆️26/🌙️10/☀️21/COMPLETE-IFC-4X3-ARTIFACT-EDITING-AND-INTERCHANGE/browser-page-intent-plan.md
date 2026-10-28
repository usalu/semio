# Browser Resident Page Intent Plan

Read-only guard/page contract checkpoint2026-10-21. Existing session admits resident edit controls only; page is separate reserved action framework.stageActionFormPage with fixed PageBinding. No production/UI test edits or mount claim.

## Exact authorization

Intercept original UiIntent before ActionDescriptor flattening. Resolve exact current completed surface/revision/node/nodeKey/trigger in the actual resident source; require active owner/window/action/source token,enabled/editable button,activate trigger,exact controller+PAGE_ACTION+version1,and no raw input (null/undefined only). Read token+destination solely from authored resident page binding; ignore event.args entirely. Never turn event-supplied choice.path/offset into fixed authority. Shared applyStagedFormPageBinding validates closed page/u64/path and status/current/binding token equality only AFTER this origin proof.

Discriminate resident page versus edit binding by exact reserved action,not guessed payload shape. Reserved invalid page action must refuse/consume and never fall through to app action dispatch. Page button disabled/read-only authority remains current resident data. A status Extension alone contains no page binding inventory.

## Atomic transition

Before mutation validate sequence as exactbigint1..MAXu64 and greater than previous accepted sequence for exact source adapter. Require next generation exists;MAXu64 refuses without changing args,page,proof,seq. Require page binding token equal current session token AND current status;unlike preserve-text edits,page cannot use old layout proof while newer local generation is awaiting refresh. This prevents navigating with a stale page inventory after typed drafts changed dependencies.

Return a structural layout replacement with next generation and fixed validated destination. Preserve args object/content exactly;page navigation is not command edit/reset/default application. Retain page in next request. Clear old proof immediately,disable Execute and old descendants until new completed tree/status matches new token. Superseded responses cannot restore old page/readiness or overwrite drafts. Reset/revise/close should define page reset policy explicitly (recommended reset pagination to initial destination on command args reset/source replacement;no hidden args mutation). New request completion acknowledges page only for exact owner/generation.

Path/offset arithmetic remains decimalu64;page choices16/32/16 fixed sizes do not justify Number coercion of arbitrary offset. Host accepts source-owned absolute destination rather than adding untrusted delta. Out-of-range destination may render empty bounded page but cannot enumerate unseen choices or certify fullform ready from visible subset alone.

## Neutral implementation matrix

Positive fixed field/diagnostic/choice page;exactMAXu64 offsets;generationMAX-1→MAX;argsreference+deepwords unchanged;request carriespage;old edit/page proof rejected after transition;new matchingpublication restoresreadiness. Bad surface/revision/node/key/trigger/controller/action/version;disabled/readonly;forged args/path/offset ignored;rawstring/map input rejected;missing/malformed page binding;status/binding/current token mismatch;old source adapter;seq0/negative/overflow/replay;generationMAX refusal;closed/retired owner;two windows sameaction isolated.

Race cases: type partialdraft then page before freshstatus (refuse without losingdraft);pageA then oldresponse;pageA thenpageB oldbutton(refuse);reset/source replacement duringpagepending;old seq afternewpagepublication;fullchoicepath protocolkeys;u64overflow/leadingzero/sparse/accessorarray in fixed page. Every refusal preserves exact prior args/page/token/proof/seq. Accepted navigation clears proof and never invokes command reducer. Independent JSON Patch comparison should be zero operations against args,not synthetic edits to make a page look like a field.

Native PageBinding destination and browser guard must share same neutral vectors;authentic native event/source close and actual owned renderer port remain separate gates. No second form renderer or fulltreeprojection.

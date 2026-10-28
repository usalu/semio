# Incomplete Identity Draft Producer Contract Audit

Read-only TS/native interface review, with independent neutral input identity-draft-audit-vectors.json. No native implementation approval or publication readiness claim. Scope extends stable existing-carrier producer only after test-first native worker checkpoint.

## Separate requirements query from canonical identity admission

TS completeForm first builds application with identities excluded, runs domain checks and identityRequirementsAsync. Only its final application(...,true) parses identity IDs and attempts actual template diff. Native should preserve that separation: original raw request remains exact centrally owned carrier; funded query carrier omits/empties identity roster without decoding blank/partial strings. Stop at actual Keys→IdentitySet requirements. Do not fabricate temporary identity numbers/GlobalIds, and do not reuse Planner.Done readiness for unfinished rows.

Canonical parsing is a terminal validation operation, not a rendering prerequisite. Blank, partial18e, whitespace,signs,leadingzero,zero,max+1,duplicate/source-colliding IDs must remain editable exact text with readyfalse and bounded diagnostic. Valid maxu64 may parse after exact checked syntax/range; precision must never pass through Number. Required GlobalId blank/partial/wronglength/invalidalphabet/sourcecollision/duplicate remains raw editable string, no RNG or auto replacement. Nonrooted role has no required GUID field; do not invent one from other carrier types.

## Stable rendered authority

Requirements key is readonly canonical planner role, binding paths point to original identities/index/id or guid and preserve original row order. Missing roles need explicit add/reset binding or immutable requirement rows with bound source-owned identity path; do not silently append generated identities. Duplicate/unknown role rows must not disappear from diagnostic/readiness merely because requirements lookup filters them. TS currently builds editable rows only for known requirement keys but final diff still rejects; native should keep exact raw owner and explicit diagnostic for malformed/unrecognized roster, even if it refuses unsupported rendering.

Actual declared Entry/Identity fields retain typed/raw scalar shapes; structural entry changes reset obsolete identities under authored source bindings. Page offset counts stable completed fields, max16content/2Page; previous/next token stays exact, page binding does not edit args. A text draft generation invalidates Execute until current source status responds; preserving old text proof never authorizes latest ready.

Diagnostics use full original paths, bilingual labels, bounded16 visible entries and exact decimal offset/total; total must count all discovered issues without cold whole-list cloning. If current supported slice cannot render every valid carrier (nonstring/>512B/depth>16), explicit resource/representation refusal is honest; do not mark ready or normalize missing words away.

## Full continuation gate and original custody

Only after exact role roster/count, canonical positiveu64/range, source/duplicate IDs and required GlobalId lexical/collision checks may canonical receiver/planner identity continuation run. A successful private planner remains prerequisite, not mutation publication: no Wire/Store/gate enable. Required-Unset/unknown fields must not be treated as successful merely because a draft can render. Original snapshot lease/full32hash/request/token/window context authority persists across query/draft/status/body work; no detached new query wallet.

Raw carrier, partial query copy, requirements metadata, diagnostics, paths, bindings and source page must remain original-funded owners across denied grants/cancellation/error. Child error retained_progress must post before propagation (previous clone-error finding applies); child Complete cannot finish parent close while any partial owner remains. Source hash/window change invalidates old work before readiness/publication. Actual native execution/ABI original receiver/context lease/host mount remain separate gates.

## Test-first next slice

Consume neutral ID vectors plus six existing identity-query populations and prepared-source path roster. Tests first assert render exact raw words/readonly role/readyfalse/no emitted mutations; then complete one ID/GUID at a time and prove no canonical decode before syntax complete. Include duplicate roles/IDs/source collisions, one partial among multiple ready rows, pagination containing partial field, cancellation at each allocation/path/diagnostic/query prefix and source/token substitution. Independent IfcOpenShell15population oracle validates final explicit IDs/rooted requirements only; it cannot validate UI draft authority or original receipts. Add actual shared reducer edits/JSONPatch comparisons for raw text preservation and structural resets.

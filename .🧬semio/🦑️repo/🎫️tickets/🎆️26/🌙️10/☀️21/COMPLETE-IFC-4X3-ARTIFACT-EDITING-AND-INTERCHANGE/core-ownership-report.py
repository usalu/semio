from pathlib import Path
T=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE')
p=T/'editing-implementation.md';s=p.read_text(encoding='utf-8').replace('23 focused Bun tests, 191 assertions pass','31 focused Bun tests, 244 assertions pass');s+='''

## Owned Capture Re-audit Follow-up

The subsequent P1 caller-mutation race and P2 base-free coordinate admission findings are resolved in `core-ownership-fixes.md`. Caller data is now incrementally captured into bounded private ownership before structural admission; application, inverse construction and event recording use only that authority. Captured command recording agrees with its replay diff. Accepted results remain structurally valid under progress-callback mutation of original inputs, and failed capture/cancellation leaves history unchanged. The optional shared `complex` field now admits omission in the IFC4x3 JSON Schema and canonicalizes to an empty collection in owned graphs. The editing suite has 31 passing tests and 244 assertions; focused strict TypeScript, the retained 1,864-law reproduction and the new ownership reproduction all pass. The capture contract promises a validated owned result; exact start-of-call capture requires caller inputs to remain unchanged until completion.
''';p.write_text(s,encoding='utf-8',newline='\n')
p=T/'core-fixes.md';s=p.read_text(encoding='utf-8');s+='''

## Subsequent Ownership Re-audit

A follow-up audit found a caller-alias race while the original implementation yielded between admission and owned copying. `core-ownership-fixes.md` supersedes that ownership boundary: bounded private capture precedes admission, captured command/event payloads stay consistent, final owned postconditions hold, and base-free coordinate/identity admission is strict. The latest focused suite has 31 passing tests and 244 assertions, with strict TypeScript and both retained reproductions passing.
''';p.write_text(s,encoding='utf-8',newline='\n')
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts');s=p.read_text(encoding='utf-8').replace('applyIfcDiff,applyIfcDiffAsync,applyIfcDiffTransaction,applyIfcDiffTransactionAsync,','applyIfcDiff,applyIfcDiffAsync,');p.write_text(s,encoding='utf-8',newline='\n')

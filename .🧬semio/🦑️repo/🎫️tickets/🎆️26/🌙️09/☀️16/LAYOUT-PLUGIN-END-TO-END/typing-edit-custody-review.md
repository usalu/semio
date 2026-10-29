# Typing Edit Custody Review

Read-only observation while the original typing-commit native request waits. No production source edited.

The current production RenderCtx::settle_tool_operation in plugin/instance/render/staging/tool-machine still handles Stage::Typing by taking the actual registry tag and artifact mutation Vec, allocating ActorId from actor.into(), calling the Clone-bound TypingLedger::send with A::typing_fold, then using steps.into_iter().find_map to return one committed result. It only checks a small final_grant header; this branch does not consume a receipt for its actual allocations, ledger cloning, fold work, or retired tag/tool/step residues. The actual tag and leaves have already moved when send can refuse.

This is the real published operation settlement branch, not an unused helper: Stage::Capture selects Stage::Typing when the actual operation registry has ToolTag::Typing. The branch is separate from the new OriginalTypingCommit/OriginalTypingDispatch owner, which handles commit and missing-window publication. The new commit native tests do not prove original custody for this edit inlet.

The defining ledger implementation and typing_fold are explicitly Clone-based reference execution. A genuine retained typing edit/fold producer must preserve actual original tag, metadata, leaf Vec, previous row, returned steps and refusal/cancellation owners; opaque IntoIter retirement and a native grant header check cannot supply that protocol. This requires coordinator scope/ownership assignment; sources remain frozen for the current native request.

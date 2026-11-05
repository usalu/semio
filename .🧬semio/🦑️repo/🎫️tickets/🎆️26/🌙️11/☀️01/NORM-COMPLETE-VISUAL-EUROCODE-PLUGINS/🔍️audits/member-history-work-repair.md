# Member History Computational Work Repair

The current MemberHistoryReceiving owns an actual consuming forwarded native decoder receipt, retirement recipient and guarded borrowed StepContext. It needs a work payer on that SAME control, without replacing its original ledger/recipient.

Canonical precedent is job/context/native decode ActualWork: public NativeDecodeWorkRecipient.admits only preflights observed+additional against actual remaining work and original semantic/retirement yield/cancel rules. NativeDecodeControl.with_work_recipient preserves actual work through a scope; capture its cumulative delta before detaching. Do not debit from admits.

The outer adapter must compare aggregate reported work to both captured original fuel and native observed work, preserve original typed failure/physical receipt, post max(reported,observed) once, and independently post physical progress. Omitted native work is an invariant refusal; it is not silently normalized into a success.

Important newly observed existing seam: composition/open/🦀️.rs receive_member_close currently bills copied_items+copied_bytes as fuel before consuming physical progress. Updated Member code MUST NOT call that inference helper and then debit actual work again. Either use a genuine physical-only receiving path locally or change the canonical shared API and all its consumers coherently; never erase other callers' work by removing their charge without adapting them.

Native worker owns actual History producer/children/retirement and public SPR driver. Six agreed canonical outcomes: admit<Option<Self>>, step/close<HistorySourceStep>, take_ready/fault/partial<bool>, all RetainedWorkOutcome with original HistoryFault. Root owns codec pair, Member receiving budget/reconciliation, and document archive close consumers. Low owns bounded direct native test adaptations and source/caller audits.

No Member/codec/production work changes authored yet in this repair; producer is in progress. Existing compile failure is real: codec pair87/127 assumes .work/.result but the current producer returns Result and lacks a complete reporter. Tests must prove actual source work, zero denied work, underreport refusal, independent original error preservation, closure and absence of duplicate debit; constants or progress-derived work are not repairs.

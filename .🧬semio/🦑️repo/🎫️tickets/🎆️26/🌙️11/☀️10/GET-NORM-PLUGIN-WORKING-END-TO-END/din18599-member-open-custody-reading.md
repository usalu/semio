# DIN 18599 Member Open Custody

The mounted `w1P` replay reached `OpeningMembers` and reported an original member snapshot custody refusal on turn 24092. The receipt was one item and one copy byte. The member opener adds a one-byte phase receipt when an action refuses, so the displayed progress is consistent with an empty child effect. The frontier trace alone gives several possible sources.

In the record member opener, `receive_step` maps `NativeReceivingFailure` through `flat`. `NativeReceivingFailure` supplies a retained progress implementation and inherits the trait's empty `cause`. Thus any error from `receiver.advance` emerges as the generic member custody refusal even when the lower failure contains an exact cause. Capturing the enum before the flattening boundary will distinguish this from a transfer or phase-refusal.

`take_step` requires an output after a completed poll. The native output helper returns a completed poll containing `Progress(default)` with no output when its copy, depth or Work is unavailable. The caller can remain at `TakeSnapshot` for that result and advance only after the value is placed. A report that claims complete transfer without filling its slot remains a custody failure.

The success transition enters `CloseReceiving`. The current quote then calls `NativeReceivingOriginal::close_demands` before the close-start flag is set. That method refuses pre-close queries. `receiver_close` already selects `begin_close_into` when the flag is clear, so opening and abandoned cleanup need to quote the same `begin_close_demand` before asking for ordinary close demand.

For pending receiving, the opener retains `(body, units)` only when the top poll is `Pending`. The local child also retains its exact grant, and batched `advance` can return `Complete(Progress)` after one earlier completed action and one later pending action. Compare the resident `advance_pending` witness, the retained request and the actual issued scope grant before clearing the parent's selection.

This is a read-only review of `store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs` and the native original receiver, protocol and failure modules. No Energy production files were edited. Exact phase attribution remains with the focused climate-member fixture.

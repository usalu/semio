# MCP App Consumer Integration

The current PluginArtifactChannel still stores raw GuestInstance values and Arc<GuestRelayWakeDriver>, and its inference lane encodes requests into Vec and decodes results after close. Those are actual remaining migrations, not ready behavior.

The canonical first-party NativeHomeAdmission<T> admits receiver state before guest startup, then transfers an OriginalNativeLease<NativeInstanceHome<T>> into a stable caller slot. NativeInstanceHome uses the immutable cache-origin resolver and implements Source-funded physical closure. GuestPoll<TurnResult> implements NativeInstanceRetirement; a reusable app receiver should own one optional Poll at a time, preserve scalar/input/output failures, and close each Poll fully before clearing its slot. Its app lifetime Driver must remain by value; a command may not outlive that Source.

A complete command path must: borrow actual typed Event input through EventJsonSource/PollJsonSource; admit a purpose-scoped guest subloan; retain the actual VM receipt and full failure; reconcile its Source SRE/header and issuer ledger under the same validated bank; perform exact Deallocate handback and genuine Source acknowledgment; only then resume the frozen Poll; deliver borrowed TurnResult inside its original Source; apply addressed shell/backbone/lifecycle responses under paid cursors; close native output/decoder before returning. Core transport still deliberately refuses frozen resume until the missing settlement witness exists.

Inference migration must preserve registered contributor/dependency authorization, metadata identity, dependency ordering, exact guest echoes and live revision/generation freshness. Calling the cold operation directly would skip these existing router responsibilities. Pending result payload mismatch is fixed separately and does not implement this migration.

Native compiler sessions remain outstanding; no actual consumer or server-ready claim is made.

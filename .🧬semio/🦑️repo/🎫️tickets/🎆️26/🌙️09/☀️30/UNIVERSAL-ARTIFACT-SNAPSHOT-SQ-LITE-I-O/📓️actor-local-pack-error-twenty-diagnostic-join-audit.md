# Actor Local Pack Error Twenty Diagnostic Join Audit

Independent readback of `physical-command-caller-owning-native-after.log`: actual semio-framework-actor lib compilation fails with20 diagnostics; no candidate assertions ran. These are ten source mistakes producing two diagnostics each (unknown PackRefusal and nonexistent Refusal variant), not20 independent caller changes.

Actor `🦀️.rs:221` declares its own public module pack. Its local PackError224 is Copy/Clone terminal wire codec error with seven variants: Truncated(usize,static str), InvalidTag, InvalidUtf8, OverlongVarint, InvalidLifecycle, InvalidUiPatchReceipt, InvalidColdPair. Display preserves actual offset/what; Error248 has no external source. Actor Cargo has no Core Pack dependency. There is no Actor PackRefusal declaration or From<Core PackError> conversion in this owner. Its `pack` name refers to local actor codec, not external container Pack.

Exact malformed truncation sites: primitive readers254,271,281,291,301,322,344,363 and actual TurnResult receipt readers3061,3075. Restore these to actual local PackError::Truncated(pos,what) (qualified local pack::PackError in TurnResult). Do not import Core PackRefusal, add Refusal variant or fabricate source conversion: these byte-slice terminal errors have no external I/O cause. Preserve lifecycle/UI decode map_err and checked receipt offset arithmetic unchanged. This narrow coherent ten-site join repairs nominal authority rather than altering Actor error API.

Other existing local PackError construction sites use the declared variants correctly. Owning native caller gate must rerun after actual source correction; direct control harness success does not cover Actor compilation. No production edits or execution by this auditor.

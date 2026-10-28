# Whole Archive Load Hold Source Audit

Read-only current source2026-10-21. No confirmed new P1/P2 in the wrapper/coalescer promise scope. No full runtime/native terminal proof.

Generic loadDocumentArchive now begins source-ingestion hold before port retirement/lane mutation. Owned branch verifies exact entry again before handoff. Cold branch uses actual active owner when available and wraps lane.replace plus finally lane.close; truly unmounted bootstrap has no previously ready source cache. loadDocumentPair delegates this whole wrapper.

DocumentAttachmentLane.replace returns its queued operation promise,including prior detach and awaited apply. close is queued behind that operation and awaits detach. replaceAttachedDocument awaits prior port retirement,checks exact before load,awaits load,then rebinds if still exact; load failure is retained and rethrown after rebind. Thus cold fulfilled wrapper waits close too; owned fulfilled wrapper waits actual attempted load/bind work represented by these port promises.

LatestDocumentReplacement distinguishes queued supersession from active attempt. Replacing a waiting #next resolves that request false immediately,because it never started apply. Active request resolves only after await operation.apply returns; superseded active errors can resolve false but only after apply rejected/finished. Releasing that request's source hold early for queued false is sound: active and newest requests retain their own holds. Empty/no-longer-current callbacks perform no load and can settle false. No global watchdog is used here.

This establishes promise sequencing,not the underlying plugin.loadAppDocumentArchive's native terminal semantics. Its actual ACK-owned loader must ensure resolve/reject means no untracked source-changing work remains. An ambiguous transport fault remains unknown-held because ingestion never admit(undefined) on rejection. Superseded active errors are intentionally swallowed to false by the existing coalescer; if a loader rejection can leave live mutation work,the coalescer contract would need an explicit unresolved outcome rather than assuming false is harmless. No such native behavior was executed here.

Root's deferred load-vs-close test and actual loader runtime evidence should verify old port retirement→load terminal→rebind/close ordering,queued supersession,active supersession,cancel,loadfault,ownerclose and overlapping remote holds. No production edits/test duplication. Remaining extension/service/terminal-effect source mutation routes are separate coverage.

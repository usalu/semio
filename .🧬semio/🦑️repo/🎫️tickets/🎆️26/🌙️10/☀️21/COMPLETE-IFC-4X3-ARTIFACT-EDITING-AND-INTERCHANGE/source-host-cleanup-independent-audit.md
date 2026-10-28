# Landed Source Host Cleanup Independent Audit

Read-only current source2026-10-21. No new confirmed P1/P2 in reviewed local cleanup/background adapter. No browser/native execution claim.

Base ShellCore cleanup now clears actual owner ref and refresh forwarder before retireAll. Register hook installs a stable actual observer forwarder in effect and cleanup clears only if still that exact function. Document-transfer effect reacquires owner from exact current document identity during setup,assigns ref,requests current plugin observation,and clears ref on cleanup only if still that exact owner. This addresses earlier captured-inactive-owner effect replay concern at source level. Existing render assignment remains a preview and must not be sole lifecycle authority.

Effect ordering dependency is explicit: refresh forwarder setup must precede document-transfer setup that invokes it. Both are hooks within actual ShellHost composition; a no-op forwarder safely suppresses stale work but can defer initial observation if a composition changes ordering. Integration should assert setup→cleanup→setup without assuming an extra render. Cleanup exact-function/exact-owner guards prevent an older effect clearing a newer registration.

source-revision/document.ts acquires the same real LocalDocumentOwner identity (pluginId,appId,instance,runtimeKey,clientInstanceId),then owner/plugin-scoped source and opaque SourceWork. It does not borrow focused owner ref. Thus background ingestion can hold its own active cache while focus changes. The adapter trusts that identity/actual plugin were obtained from current admitted open entry; it does not independently check entry membership or prevent stale callers from acquiring a new owner. Call sites must check exact open entry before adapter creation and after awaits,then retire on close.

Capacity refusal now latches exact owner/plugin until new lifetime,preventing missed work during capacity exhaustion from becoming ready merely when a slot frees. Registry retireAll resets weakmaps for fresh host setup; retired handles remain refused. No eviction of live work is introduced.

Root reports39tests136assert/strictpass across source suites; not rerun here. Remaining remote/archive/service/extension call-site wiring and late queued callback tests are separate gates. A surviving callback must not reacquire an owner after true unmount; base refclear,forwarder no-op and real owner retirement are all needed. Fullbrowser/native form proof remains false.

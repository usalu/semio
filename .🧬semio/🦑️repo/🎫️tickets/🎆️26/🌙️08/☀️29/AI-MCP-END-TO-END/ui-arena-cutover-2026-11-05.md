# UI Arena Cutover

The existing `UiArenaLifetime` and the guest actor's `OriginalGuestUiArena` open the real backing under the admitted Source and publish it through `UiArenaRegistry`. The normal UI collection path still uses a separate `LazyLock<Mutex<UiValueArena>>`; cutting over only `initialized_arena` would leave handles split between distinct stores.

The replacement must be one coordinated change across collection-arena `initialized_arena` and `with_ui_value_arena`, values alias cloning, original receiving's two direct locks, typed copy's two locks, typed comparison's lock, and retirement's three locks. Every path must borrow the same registered backing; contention must retain existing yield behavior. The registration owner must remain live until every collection and queued handback has retired, and a poisoned borrow must be recovered by its actual `UiArenaRegistration` rather than silently creating another backing.

Existing tests directly lock the old global in values observer, retirement and unit tests, including a poison-recovery expectation. They must migrate to an explicitly funded arena fixture and test that the exact owner recovers its borrow. The host must register its own process lifetime before any collection construction; guest actor admission already opens and registers its arena before plugin initialization. Neither native process bootstrap nor the old global cutover is yet qualified.
# Borrowed encoding pages

The registered Source-backed arena now exposes active collection head/count and borrowed page/next access for encoder traversal. Each page stores its actual epoch-qualified owner, so lookup rejects a foreign collection even when its page kind matches. No value is cloned or transferred. The portable Ajv/SQLite owner and linked-page law passed 1 test with 5 assertions. A native law and seven downstream serial Source fixture migrations are authored and remain uncompiled. The larger page is charged through the existing `size_of::<UiPageSlot>()` opening and admission accounting, which still needs native conservation verification.


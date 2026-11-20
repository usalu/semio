# Registry follow during concurrent edits

The graph’s second live registry follow reported that the registry had changed. The current registry owned the exact PDF page-editor root. The follow event alone does not identify the author of the registry change. [Live registry evidence](audit-brep-registry-follow-0438.md)

The previous retry logic treated any registry change as sufficient reason to skip scoped repair. A private physical fixture reproduced the failure when a peer added an unrelated row while the current graph needed a different Cargo root. The next whole attempt still lacked the required owner. The new path checks each named root against the current registry, reconciles only roots still requiring work and preserves the existing retry bound. SQLite supplied the independently sorted expected roots. The second attempt found both the required owner and the peer's exact unrelated row.

Registry publication also rechecks the captured registry immediately before replacing it. Each preparation uses a unique same-directory file and removes it on exit. A private fixture changed the real registry after its baseline read and before publication. The stale update refused, preserved the peer's exact bytes and left no temporary file. A subsequent scoped reconciliation worked from the new bytes. The pre-publication comparison detects changes up to its read. Independent machines can still change their replica after that comparison.

The final farm, registry and pending-crate run passed 42 tests with 263 assertions.

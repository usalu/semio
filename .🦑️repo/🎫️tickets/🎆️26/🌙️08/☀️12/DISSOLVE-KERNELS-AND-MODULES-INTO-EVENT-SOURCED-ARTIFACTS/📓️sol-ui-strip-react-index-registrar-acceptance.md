# UI Strip React Index Registrar Acceptance

After Terra deleted the zero-consumer Strip component and its accepted-dirty story, the coordinator rehashed the shared React package index at `7872a8bcbcf3990d623d0dc4486e8b16e199c7cd0f053fb9c76ab2b0cd9d2eb6` on HEAD `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.

The coordinator removed only the adjacent five-line Strip import/export region using `apply_patch`. Final index SHA-256: `57388b35c4d4b2d1bb272577e01ae839837c1632b8c1329c4c3c87fd38b50f4e`.

The cumulative ordinary index diff is exactly fifteen deletion lines for accepted Card, Band, and Strip registrar removals. Cached diff is empty; scoped `git diff --check` and the Strip identifier/path scan pass. Terra was signalled to complete the active/excluded stale census and JavaScript Nx gates.

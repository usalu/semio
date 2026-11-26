# Result F101: dead key closure.derived-provider-differs

- Defined only in `🏗️build/📖️texts/🔣️.json` (en + de in one block). The catalog schema is generic (`🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🧬️schema/🔣️json/🟦️.ts`) and names no keys, so nothing else needed editing.
- No code emits it: the key string occurs nowhere in the library except the JSON. Nearest key `closure.provider-designation-differs` (`🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:2214`) is a different check (project root vs designation path, not a derived walk), so it is not a replacement and was not used.
- Action: removed the key (en + de) from `🔣️.json`, atomic temp+rename, lock `landing.lock.texts` held only for the write. Script kept at `🧰️scripts/F101/remove-derived-provider-differs.py`. Catalog now 183 messages.
- Checks after the write: dangling-imports rc 0; `repo typecheck` on the texts folder passed (46 files, 0 errors); `repo test-file .../🏗️build/📖️texts/🧪️tests/🟦️.ts`: 3 pass, 0 fail (965 expects).
- Deviation: the lock was released before typecheck and test (per the 14:55 hygiene addendum, no lock across a check).
- Not verified: a whole-repo grep outside the library timed out; the key test only scans library sources, so that scope is what matters.

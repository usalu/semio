# Windows Checkout Audit

The checked commit had no filename component above NTFS's 255 UTF-16-unit limit and no Windows-illegal component. The reported Git for Windows failure was instead the legacy full-path limit: seven ticket paths were 257–276 UTF-16 units before the checkout root was added.

The six deepest paths were stale `CACHEDIR.TAG` markers retained under `🕰️misplaced-cache-evidence`; their parent folders contain no source or audit input. The obsolete closed ticket was shortened to `WINDOWS-NATIVE-FLOWS`, and the long window-contract ticket was shortened to `WINDOW-APP-PANEL-CONTRACTS`.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔬️index.test.ts` now validates Windows-illegal components against the schema-backed fixture and asserts every ticket file fits the 259-unit legacy Windows limit when checked out under `C:\Users\username\source\repos\semio`.

The complete repository still has legitimate non-ticket paths longer than this legacy limit. Windows developers must enable Git's `core.longpaths` setting and the operating system's long-path support to check out the entire tree.

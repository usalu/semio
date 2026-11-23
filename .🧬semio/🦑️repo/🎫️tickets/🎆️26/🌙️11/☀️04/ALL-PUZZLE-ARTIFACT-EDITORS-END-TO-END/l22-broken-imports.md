# L22 Graph Blocker: Broken Relative Import

- The 3D activation failed in graph load with `BUILD_SOURCE_UNPARSEABLE`. The new untracked script `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🥣️companion/📦️packages/🦀️rust/📜️script.ts` (another lane, 15:00) imported `../`×7 + `🏃️process/…`. That resolves to the removed `🧰️framework/🏃️process`. The process modules now live at `🧰️framework/🔨️modules/🏃️process`.
- Fix: both imports now use `../`×6. Both targets were verified to exist.
- Repo-wide scan with `l22-broken-imports.mjs` (bun, roots 🧰️framework ✏️s 🌎️hub). After dropping `.d.ts` files, generated bindings, `\u`-escaped specifiers and import text inside test fixtures, no other unresolved import of the moved `🏃️process` path remains.

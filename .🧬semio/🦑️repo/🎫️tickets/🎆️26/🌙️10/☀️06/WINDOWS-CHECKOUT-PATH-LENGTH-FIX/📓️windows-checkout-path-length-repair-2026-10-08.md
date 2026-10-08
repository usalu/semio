# Windows Checkout Path Length Repair

## Problem

`git pull` at `C:\git\semio` failed with `Filename too long` while creating ticket fixture directories such as `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/cargo-inputs/📥️current-source-closure-16/.../🧪️tests`.

Windows `MAX_PATH` is 260 UTF-16 units including the terminating NUL, so a path string may be at most 259 units. `CreateDirectoryW` rejects a directory at 248 units. `C:\git\semio\` is 13 units, so a repo-relative file may be 246 units and a directory 234.

## What was removed

Unimportant ticket copies that exceeded that budget, and ticket directories over 10MB that the workspace clean already treats as disposable:

- 193 tracked ticket paths over the stock checkout budget (file 213 units, directory 201 units below a 46-unit clone root). All of them sat under `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`, `PRINT-VISUALIZATION-LIBRARY`, and `FIXTURES-ARE-TESTING-EXAMPLES-ONLY`, mostly as nested copies of `🧰️framework`.
- 11 `cargo-inputs` snapshot directories over 10MB (`📥️next-source-admission-chain` at 292MB, `📥️next-source-admission-process`, `📥️current-native-world`, `📥️next-dependency-cargo-projection`, and `📥️current-successor-16` through `22`). The cargo-inputs script does not name these directories.

`bun ./📜️script.ts clean` did not finish. `git ls-files --others -i --exclude-standard -z` exceeded its 120s budget and the command exited before it deleted anything.

## Result

Tracked tree after the removal: no file reaches 260 UTF-16 units under `C:\git\semio`, and no directory reaches 248. A `git pull` into `C:\git\semio` can create every tracked path.

One live path remains over the stricter 46-unit clone root (`C:\Users\abcdefghijklmnopqrst\Documents\semio\`):

`✏️s/🔌️plugins/🌍️gis/.../🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json` at 217 units (directory 208). It fits `C:\git\semio` (absolute file 230, directory 221). `🧾️wire-witness` is the taxonomy slug `^wire-witness$`, so that segment stays.

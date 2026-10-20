# Windows Checkout Path Length Repair

`git pull` at `C:\git\semio` stopped on `Filename too long` while creating

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/cargo-fix-tools/staged/files/…/🖱️store-cursor-operations`.

That directory is 250 UTF-16 units under `C:\git\semio\`. Win32 `CreateDirectoryW` rejects a directory at 248 units (`MAX_PATH` 260, including the terminating NUL, minus 12 for an 8.3 name). The file inside was still under the 259-unit file limit, so a file-only check does not catch it.

## Budget

| Check | Maximum |
| --- | --- |
| `C:\git\semio\` + file | 259 UTF-16 units |
| `C:\git\semio\` + directory | 247 UTF-16 units |
| `C:\Users\username\source\repos\semio\` + ticket file | 260 UTF-16 units |

Counted with Python `utf-16-le` and checked against Node string length (`win32.join`). Both report the same longest file, 239 units.

Also scanned: `<>:"\|?*`, control characters, trailing dots or spaces, `CON`/`PRN`/`AUX`/`NUL`/`COM1`–`COM9`/`LPT1`–`LPT9`, and case-fold collisions. None.

## Change

81 staged snapshots under `GET-CODEBASE-RUNNING-END-TO-END-AGAIN` were past at least one budget. Their bytes now live in a sibling `w/` directory. `w/map.json` maps the original repo-relative path to a 12-hex file name. `winpath.py` resolves that alias for the apply scripts that read `staged/<logical path>`.

After the index update: 0 tracked paths over any budget. Longest file 239, longest directory 230. The failing `store-cursor-operations` directory is not tracked.

The checkout test now also rejects a directory whose `C:\git\semio` join is longer than 247.

## Later staged snapshots

Seventeen paths staged after that pass overflowed again: fifteen under `storage-seam-tools/staged/files/` (two of those directories are 248 and 249 units at `C:\git\semio`) and two under `pm-seam-tools/staged/new/`. Their bytes moved into the same `w/` alias layout. `storage-seam-tools/apply.py` resolves the alias, and `mkgroups.py` plus `pm-seam-tools/mkratchet.py` write through `place` so a later regeneration does not recreate the long path.

Rescan of the index after that move: 0 paths over the file, directory, or ticket-checkout budget. Longest file 239, longest directory 230.

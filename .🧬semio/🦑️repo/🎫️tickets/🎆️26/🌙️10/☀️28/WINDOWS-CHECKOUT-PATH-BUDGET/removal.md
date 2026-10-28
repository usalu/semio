# Windows Checkout Path Budget

Stock Windows checkout allows a repo-relative file of 213 UTF-16 units and a repo-relative directory of 201 UTF-16 units (`MAX_PATH` 260, one unit for the terminator, a 46-unit clone root, and 12 units held back for `CreateDirectoryW`).

228 tracked files under `🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` were nested copies of `framework` sources. Their paths ran from 214 to 270 units, and their parent directories ran past 201. They are removed from the index. The live sources stay in place. Audits and scripts in that ticket stay.

After removal, every tracked ticket path fits: the longest ticket file is 213 units and the longest ticket directory is 201 units.

Eleven product fixture directories are still 202 units, one over the directory budget. They are not ticket copies. Shortening them means renaming fixture case folders that tests and schemas address by name.

# Windows Clone

Git for Windows creates directories with `CreateDirectoryW`, which rejects a path of 248 UTF-16 code units or more when `core.longpaths` is off. Files fail at 260. The reported clone root `C:\git\semio` (12 code units) makes a directory fail at relative length 235 and a file fail at relative length 247.

Removed 53 tracked paths under this ticket that cross that line. They are payload and splice copies of source trees, nested under the ticket folder. The product tree itself stays under the limit at this clone root (longest directory 243).

After removal the index has no illegal path. The longest remaining directory is 247 and the longest remaining file is 259, so `C:\git\semio` fits with no spare characters. A longer clone root still fails.

These deletions are staged. `git pull` on Windows keeps failing until this change is committed and pushed, because `3b2f1181d27` still contains the long paths.

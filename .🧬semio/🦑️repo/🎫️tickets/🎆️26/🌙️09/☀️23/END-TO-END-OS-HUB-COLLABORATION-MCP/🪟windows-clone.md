# Windows Clone

`git pull` of `0ed485479f5` still contains the `🔬️program-unit` directory. That directory is 256 UTF-16 code units at `C:\git\semio`. Git for Windows rejects a directory at 248 (`CreateDirectoryW`, `MAX_PATH - 12`) and a file at 260.

Removed 20 tracked ticket payload and splice copies:

- 9 directories at 248 or more, including `🔬️program-unit`
- 6 directories at 247, one code unit under that cutoff
- 5 directories at 244–246 in the same splice tree

No reserved device names, trailing dots or spaces, illegal characters, or path segments over 255 code units are in the tree. The longest remaining directory is a product path at 243. The longest remaining file is 252.

The `unable to rmdir '♻️mit-bestand/🔎️recherche'` line is a warning. Pull continues past it. The fatal error is the long directory.

These deletions are staged. `0ed485479f5` on the remote still has the long paths, so Windows `git pull` keeps failing until this change is committed and pushed.

# Misplaced Git Message Placeholders

The three empty, regular files `COMMIT_EDITMSG`, `MERGE_MSG` and `SQUASH_MSG` were found at the workspace root, not inside Git metadata. Read-only `git rev-parse --git-dir` returned `.git`; repository hooks and source consumers address these names inside that metadata directory. All three root files were confirmed empty immediately before their recoverable move here. Their names and bytes are preserved; no files under `.git` and no Git index/configuration were modified.

These are retained recovery inputs, not generated output. No blanket naming exemption was added to hide the misplaced originals.

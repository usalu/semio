# Closure ancestry fix

Root cause: retiringAncestryV1 walked dirname(path) up to `/` and stat'ed each directory via named(), which enforces the image-root boundary and threw for directories above image.root.

Review of the fix `proof.directories.has(parent)?named(proof,parent):(check(proof),lstatSync(parent,{bigint:true}))`:
- The image root is the first entry of proof.directories (directories() starts at root), so it and everything below still go through named().
- Directories above the root are compared against proof.ancestors (filled by ancestry() from `/` to root); a missing expectation throws. Ignored control directories are also in proof.ancestors.
- recheck() already validated every proof.ancestors entry with lstat before each member, so ancestor drift is refused twice.
- Other dirname walks in the closures module (missingLeaf, retirement catalog, group planning loop, pairs) stay within the root or only look up proof.directories; no sibling has the defect.

Tests: added "graph optional group retirement ancestors above the image root" to closures/tests/groups: captures a proof, changes the mode of the directory above the image root, expects the "ancestor changed" refusal and that the group is retained, restores the mode, then retires successfully. The existing `complete` scenarios cover the success path beneath the nested root.

How to run: these files use registerIsolatedBuildTestV1 and need the original parent Script invocation, so they run only via `repo run repo:test-storage-graph-processes` (suite storage-graph-processes), not bare bun test.

Results: not obtained. Log anc-storage-graph-processes.log stayed in "Reading the project graph slice" for over an hour, queued behind other fleet graph loads.

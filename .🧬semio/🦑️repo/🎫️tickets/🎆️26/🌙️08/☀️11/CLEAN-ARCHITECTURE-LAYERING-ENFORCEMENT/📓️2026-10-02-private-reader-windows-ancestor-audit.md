# Private Reader Windows Ancestor Audit

Read-only actual guard inspection and Node/Bun path-semantic examples; no source edit, Cargo/Nx/compiler execution or native Windows filesystem claim.

Current `assertPrivateReaderInput` uses platform `relative(owner,target)` for workspace containment, then climbs `dirname(current)` until string equality `current===owner`. On Windows, relative resolves paths case-insensitively while string equality preserves spelling. This mismatch admits an in-workspace case-variant path yet never reaches the stop condition.

Both Bun win32 and Node win32 independently produce the following actual path results:

| Owner | Input | relative containment | Ancestor stop |
|---|---|---|---|
| `C:\Repo` | `c:\repo\folder\fixture.json` | `folder\fixture.json`, accepted | never exact-matches `C:\Repo`; reaches `c:\` fixed point |
| `C:\Repo` | `..\repo\folder\fixture.json` | `folder\fixture.json`, accepted | never exact-matches `C:\Repo`; reaches `C:\` fixed point |
| `\\Host\Share\Repo` | `\\host\share\repo\file.json` | `file.json`, accepted | reaches UNC share-root fixed point |
| `C:\Repo` | `folder\fixture.json` | `folder\fixture.json`, accepted | exact owner reached normally |

The semantic case-variant owner ancestor has `win32.relative(owner,current)===""`. These tests ran path operations on the current macOS host. Actual Windows lstat success on a normal case-insensitive filesystem would allow the loop to continue indefinitely at the root; missing/inaccessible files may throw sooner, but that is not a correct termination proof. The path utility's public custom workspace/input interface permits absolute and normalized relative variants even though authored witness paths normally retain owner spelling.

Use the same platform path identity for termination as for admission: after checking current node with lstat, stop when `relative(owner,current)===""`. Then calculate parent and explicitly refuse if `parent===current` before reaching the admitted owner. This preserves case-sensitive POSIX semantics and case-insensitive drive/UNC Windows semantics without universally lowercasing paths. Add pure path fixtures for all examples and an actual Windows-case filesystem check when running there; retained real symlink/junction, directory/missing/escape tests must continue to refuse.

Existing lstat ancestor walk checks the target, each intervening ancestor and workspace owner itself. A regular target reached through a symlinked directory is caught when that directory is inspected; a final symlink fails isFile. The loop intentionally stops at the declared workspace owner, so it does not certify parents above that owner. Read/check sequencing is not an atomic filesystem snapshot; current SHA checks supply content binding, not a global concurrent mutation lock.

Current restoration now calls `sources.assertFile(witness.source)` before checking helper/input bindings, closing the previous caller physical-identity omission. That correction does not address the separate Windows termination mismatch documented here.

# Watcher Initial Policy Investigation

The authored Nx 23.2 patch passes the closed repository exclusion policy as additional globs and disables use_ignores. Pinned watcher.rs builds the event filter from those globs but enumerate_watch_paths receives only use_ignores. Enabling it would load ambient .gitignore and hide structural Source. PathNotFound already warns and skips; no change is warranted.

Upstream evidence: https://raw.githubusercontent.com/nrwl/nx/23.2.0/packages/nx/src/native/watch/watcher.rs and https://raw.githubusercontent.com/nrwl/nx/23.2.0/packages/nx/src/native/walker.rs . Initial enumeration, watch registration and later subtree discovery must all obey the same exact closed root/name policy. This discrepancy alone does not establish the cause of the complete 120-second daemon startup failure.

No Source edits yet. Existing repo-watch std-only backend already prunes exact rooted/name exclusions on Linux and filters them on macOS/Windows. Its existing supervisor uses physical fences in .🧬semio, which is excluded by the workspace closed policy, and degrades to recursive portable watching on failure. Reuse must preserve flush barriers, fail closed on loss, retain authentic service controls, and avoid a broad policy exemption.

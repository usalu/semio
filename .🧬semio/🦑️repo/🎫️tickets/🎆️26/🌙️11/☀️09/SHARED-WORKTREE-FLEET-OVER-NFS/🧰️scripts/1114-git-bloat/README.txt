Landing 2 of package W (after the graph run writes graph-run-V.done), under 1113-landing.lock, then all module tests:
  python3 patch-git-config-and-sweep.py <module root>   (🌿️git/🟦️.ts config keys + sweepGitGarbageV1, and its tests in 🌿️git/🧪️tests/🟦️.ts)
  python3 patch-maintain-sweep.py <module root>         (daemon maintain() calls sweepGitGarbageV1)
README.md gets one sentence in the Configuration paragraph and in the Repacks paragraph (keys, sweep).
The working copy with the tests already passing (46 tests in the git suite) was in the scratchpad W directory and is deleted; the patch scripts reproduce it.

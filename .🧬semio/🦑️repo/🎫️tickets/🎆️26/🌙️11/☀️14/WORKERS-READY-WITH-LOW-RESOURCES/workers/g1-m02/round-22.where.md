## The Missing Fact of Round 19 (copied for the coordinator, round 22 try 1, 05:33:21Z)

Service log `host-*.log` of the try, verbatim:

```
2026-11-15T05:33:21.623Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=86077 ppid=86049
2026-11-15T05:33:21.624Z shutdown access-denied
```

Bootstrap log: `The graph service could not answer (lost the service exited: access-denied (code 4), 453 MB resident, slice workers held 56 MB [log: … build:implementation-source · Reading the sources of the build implementation | Internal error: Relative filesystem fstatat failed (EPERM) …])`, `native_rc=1` (not 4: my own shell can read the worktree).

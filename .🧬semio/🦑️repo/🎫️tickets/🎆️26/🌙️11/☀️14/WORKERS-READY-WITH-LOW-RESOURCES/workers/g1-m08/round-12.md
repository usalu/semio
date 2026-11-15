# Round 12 — g1-m08 — Read-only pause report

Observed 2026-11-15T01:33:23Z. No bootstrap was run in this round. Last native outcome remains round 11 try 4, native_rc=1; ready=false. No native process was running when round 12 arrived; no SIGINT was needed. All observations below were read-only and collected once. No resident service was stopped.

## 1. Last-try graph service generation / stale / fleet lines

Last try used the log(s) named below. The service reused its log across requests and host incarnations; this is the complete matching-line snapshot of that log, not an inferred attribution of every line to the last request. Generation numbers and exact words are retained.

/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-ac6bfe8f337c.log

~~~text
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 94 taken from the fleet, 6 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/print (about 3628 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 520
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: refreshing 614 projects (generation 62)
graph: project @semio-tech/framework-job-rs (language:cargo) grew by 2696 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 413 taken from the fleet, 1 too heavy for this machine (workspace (about 13780 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 201
graph: nothing to wait for any longer: the fleet was asked for 201 stale projects and delivered 0 within 901 s; 201 remain, among them the too heavy project(s) workspace (about 13780 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 190 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 27770 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 5 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 424
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 413 taken from the fleet, 1 too heavy for this machine (workspace (about 13896 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 201
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: refreshing 614 projects (generation 62)
graph: project @semio-tech/framework-job-rs (language:cargo) grew by 2513 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 201 taken from the fleet, 12 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 413
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: refreshing 614 projects (generation 62)
graph: project @semio-tech/framework-os-kernel (language:cargo) grew by 2816 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 614 stale projects, 413 taken from the fleet, 1 too heavy for this machine (workspace (about 13896 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 201
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: refreshing 614 projects (generation 62)
graph: project @semio-tech/framework-process (project) grew by 2148 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/framework-rs (language:cargo) grew by 1855 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/framework-replication-rs (language:cargo) grew by 4198 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 62; verifying the tree once.
graph: 566 stale projects, 198 taken from the fleet, 10 too heavy for this machine (@semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-rs (about 8496 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 4 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 368
graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 10 s
graph: No verified change journal cursor is recorded for generation 68; verifying the tree once.
graph: refreshing 2 projects (generation 69)
graph: project @semio-tech/plugin-registry (project) grew by 3092 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 68; verifying the tree once.
graph: 2 stale projects, 0 taken from the fleet, 2 too heavy for this machine (workspace (about 13896 MB), @semio-tech/plugin-registry (about 6184 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 2
graph: No verified change journal cursor is recorded for generation 68; verifying the tree once.
graph: refreshing 2 projects (generation 69)
graph: generation 69 published, 2 projects refreshed in 107 s (0 kept their edges)
graph: fact index of generation 69: 740266 facts of 5101 projects, 84.7 MB expanded (16.5 % of the 512 MB limit), 13.6 MB packed
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 12 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
graph: refreshing 2 projects (generation 75)
graph: generation 75 published, 2 projects refreshed in 69 s (0 kept their edges)
graph: fact index of generation 75: 760113 facts of 5101 projects, 87.6 MB expanded (17.1 % of the 512 MB limit), 13.9 MB packed
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 10 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
graph: refreshing 2 projects (generation 75)
graph: generation 75 published, 2 projects refreshed in 68 s (0 kept their edges)
graph: fact index of generation 75: 760113 facts of 5101 projects, 87.6 MB expanded (17.1 % of the 512 MB limit), 13.9 MB packed
~~~

## 2. Newest three host logs and replica digest

Exact ls -la output:
~~~text
-rw-r--r--  1 ueli  staff   138037 Nov 14 22:13 /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-6f286e53e68a.log
-rw-r--r--  1 ueli  staff   641417 Nov 15 00:18 /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-9888631afea4.log
-rw-r--r--@ 1 ueli  staff  2089224 Nov 15 02:15 /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-ac6bfe8f337c.log

~~~

Exact byte sizes and UTC mtimes:
~~~json
[
  {
    "path": "/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-ac6bfe8f337c.log",
    "size_bytes": 2089224,
    "mtime_utc": "2026-11-15T01:15:26.314811+00:00"
  },
  {
    "path": "/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-9888631afea4.log",
    "size_bytes": 641417,
    "mtime_utc": "2026-11-14T23:18:12.894286+00:00"
  },
  {
    "path": "/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-6f286e53e68a.log",
    "size_bytes": 138037,
    "mtime_utc": "2026-11-14T21:13:58.678568+00:00"
  }
]
~~~

Replica code digest from health: 52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f; replica PID 92340.

## 3. Machine state now

Memory pressure level:
~~~text
1

~~~

Swap in use, exact sysctl output:
~~~text
vm.swapusage: total = 3072.00M  used = 1624.00M  free = 1448.00M  (encrypted)

~~~

Five largest processes (RSS order), with PID, PPID, age and resident size:

| PID | PPID | Age | RSS KiB | Executable |
|---:|---:|---|---:|---|
| 92340 | 92339 | 00:47 | 1521008 | /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun |
| 6959 | 6930 | 22:17:30 | 632544 | /Applications/ChatGPT.app/Contents/Frameworks/Codex |
| 92713 | 6961 | 00:04 | 412944 | /Library/Developer/CommandLineTools/usr/bin/git |
| 6930 | 1 | 22:17:32 | 288192 | /Applications/ChatGPT.app/Contents/MacOS/ChatGPT |
| 6956 | 6930 | 22:17:31 | 211920 | /Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex |

Full ps lines including commands, verbatim:
~~~text
92340 92339       00:47 1521008 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts --boot-child run
 6959  6930    22:17:30 632544 /Applications/ChatGPT.app/Contents/Frameworks/Codex Framework.framework/Versions/157.0.8103.78/Helpers/Codex (Aperitif Renderer).app/Contents/MacOS/Codex (Aperitif Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Codex --standard-schemes=app,codex-sandbox --service-worker-schemes=codex-sandbox --secure-schemes=app,codex-sandbox --fetch-schemes=app,codex-sandbox --cors-schemes=codex-sandbox --streaming-schemes=app,codex-sandbox --code-cache-schemes=app --owl-scoped-user-agent-additional-hosts=openai.com,chatgpt.com,chatgpt.site,chatgpt-team.site --owl-scoped-user-agent-prefix=CodexBrowser --start-stack-profiler --enable-blink-features=WebMCP,FocusgroupV2 --disable-blink-features=ReplacedNormalFlowStackingInlinePaint --lang=en-US --num-raster-threads=4 --enable-zero-copy --enable-gpu-memory-buffer-compositor-resources --enable-main-frame-before-activation --renderer-client-id=8 --launch-time-ticks=18241780293 --shared-files --metrics-shmem-handle=1752395122,r,6121802591936840752,1743079678216674005,1572864 --field-trial-handle=1718379636,r,3089663580297264058,9954380167774086212,262144 --enable-features=DevToolsWebMCPSupport,DocumentPolicyIncludeJSCallStacksInCrashReports,ReportStuckThrottle --disable-features=DropInputEventsWhilePaintHolding,OwlNativeCaret,WebUIOmniboxAimPopup,WebUIOmniboxPopup --variations-seed-version --pseudonymization-salt-handle=1935764596,r,15594513949212912552,12137183214413531065,4 --trace-process-track-uuid=3190708993808206286 --seatbelt-client=176 --force-fieldtrials=WebRTC-Sctp-Snap/Enabled/WebRTC-IceHandshakeDtls/Enabled/WebRTC-ForceDtls13/Enabled/
92713  6961       00:04 412944 /Library/Developer/CommandLineTools/usr/bin/git -c attr.tree= -c core.attributesFile= -c safe.bareRepository=explicit -c core.hooksPath=/dev/null -c core.fsmonitor= status --renames --porcelain=v1 -z --untracked-files=normal
 6930     1    22:17:32 288192 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT
 6956  6930    22:17:31 211920 /Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex -c features.code_mode_host=true app-server --analytics-default-enabled -c plugins.codex-app-tools@openai-bundled.mcp_servers.codex_app.enabled=true
~~~

Remaining service-host or project-show processes, with their exact ps age/RSS/parents:
~~~text
None observed.
~~~

No permission, code, cache or service changes were made. The paused worker will check for round 13 every 60 seconds, for at most six hours.

Report completed 2026-11-15T01:33:23Z.

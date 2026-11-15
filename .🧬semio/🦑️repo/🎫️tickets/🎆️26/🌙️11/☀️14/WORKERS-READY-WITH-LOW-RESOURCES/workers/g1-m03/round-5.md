# g1-m03 Round 5

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 100 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: three tries. **Try 3 got past the graph step** (703 of 703 owners read: 221 taken from the fleet at its start, 154 more arrived, the rest inferred here under the memory bound) and ran `workspace:setup` to the end: 12 tasks succeeded, 7 failed, 4 not run. `native_rc=1`, marker absent, `ready` false. No hard stop in any try. Memory pressure: level 1 in all samples of tries 1 and 2; in try 3 level 2 in two single samples (19:21:42Z during the nx tasks and 19:36:45Z), never two in a row, so below the stop rule of five in a row, and never level 4.
- No further try was started after try 3: the round reruns only after `native_rc=75` or a refusal text, and try 3 ended in nx task failures. Three of the seven failures are `stale build.closure.source-differs` (a source file changed during the run), so a rerun may get further; this worker waits for the next round or a `next.md`.

## 1. Before the run

    Pre-run facts, 2026-11-14T17:56:29Z
    - replica code digest (.cache/f/health.json): 39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759, store ok, daemon pid 35298
    - `ps -axo pid,ppid,etime,rss,args` lines of `repo show projects` or `repo service host` processes (arguments shortened to the part after the script path):
        pid 8384 ppid 1 age 01:01:06 rss 64224 KiB: repo service host run
        pid 8386 ppid 8384 age 01:01:04 rss 352272 KiB: \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await
      (2 process(es))
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T17:49:52.347Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
        2026-11-14T17:49:52.394Z worker sandbox: darwin-sandbox confined network denied
        2026-11-14T17:49:52.397Z worker registered: 7bd0830f-4828-4e0c-ab8b-141d774a4efb cache http://192.168.178.70:6094
        2026-11-14T17:53:21.859Z agent code changed, restarting
        2026-11-14T17:53:21.862Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-03-lan.fritz.box-36654
        2026-11-14T17:53:23.833Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
        2026-11-14T17:53:23.892Z worker sandbox: darwin-sandbox confined network denied
        2026-11-14T17:53:23.897Z worker registered: fdcad750-9461-48da-8378-daf75a29023b cache http://192.168.178.70:6094
    - memory pressure level 1, free disk 100Gi

## 2. and 3. Tries

| Try | Log | Start | End | `native_rc` | Wanted | Arrived | Missing | Seconds |
|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-5.log` | 17:56:30Z | 17:57:16Z | 1 | not reached | not reached | not reached | not reached |
| 2 | `~/native-bootstrap-round-5-try-2.log` | 17:59:30Z | 18:27:17Z | 75 | 645 (703 stale, 58 taken from the fleet at the start) | 221 | 424 | 1498 |
| 3 | `~/native-bootstrap-round-5-try-3.log` | 18:29:34Z | 19:37:23Z | 1 (nx task failures) | 482 (703 stale, 221 taken from the fleet at the start) | 154 seen as the last "waiting for the fleet" line | not printed (no refusal; the graph step then went on with local inference) | not measured |

Waits: 120 s after try 1 and 120 s after try 2 (`round-5.monitor.txt`).

Try 1 ended after 46 s, not with a refusal and not by a code change, with (verbatim):

```
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6feebb05efe3.log.
end 2026-11-14T17:57:16Z
native_rc=1
```

The graph service log of that moment (`host-6feebb05efe3.log`): `2026-11-14T17:57:16.281Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)` at `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148`. It was the first request to a graph service that had just been started for a new code version (`Starting the repo graph service for this host.`). Try 2, two minutes later, used the same service without this error. No privacy setting was changed on this machine.

Refusal of try 2 in the bootstrap log (verbatim):

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 645 and delivered 221 within 1498 s, 424 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## 5. Graph service log per try (verbatim: `waiting for the fleet`, `taken from the fleet`, `inferred here`, `refreshing N projects`, `asking the host for`, `stale facts`; then the last 30 lines when the try ended)

### Try 1

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6feebb05efe3.log (lines after 0)
-- lines with: waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts
-- last 30 lines when the try ended
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] build:implementation-source · Reading the sources of the build implementation
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[repo] still working after 5 s; last phase started: source.served
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6548 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T17:57:16.281Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
[repo] still working after 10 s; last phase started: source.served
[repository:command] still running after 10 s
[repo] still working after 20 s; last phase started: source.served
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-f74179f31e9f.log (lines after 3927)
-- lines with: waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts
-- last 30 lines when the try ended
[repository:command] still running after 3551 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repo] still working after 3560 s; last phase started: source.served
[repository:command] still running after 3561 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 3571 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 3581 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repo] still working after 3590 s; last phase started: source.served
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
[repository:command] still running after 3591 s
[repository:command] still running after 3601 s
[repository:command] still running after 3611 s
[repo] still working after 3620 s; last phase started: source.served
[repository:command] still running after 3621 s
[repository:command] still running after 3631 s
[repository:command] still running after 3641 s
[repo] still working after 3650 s; last phase started: source.served
[repository:command] still running after 3651 s
[repository:command] still running after 3661 s
[repository:command] still running after 3671 s
[repo] still working after 3680 s; last phase started: source.served
[repository:command] still running after 3681 s
[repository:command] still running after 3691 s
[repository:command] still running after 3701 s
[repo] still working after 3710 s; last phase started: source.served
[repository:command] still running after 3711 s
[repository:command] still running after 3721 s
```

### Try 2

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6feebb05efe3.log (lines after 163)
-- lines with: waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts
graph: 703 stale projects, 58 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 645
graph: waiting for the fleet: 0 of 645 projects arrived
graph: waiting for the fleet: 77 of 645 projects arrived
graph: waiting for the fleet: 125 of 645 projects arrived
graph: waiting for the fleet: 173 of 645 projects arrived
graph: waiting for the fleet: 221 of 645 projects arrived
-- last 30 lines when the try ended
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1700 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1710 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1720 s
[repo] still working after 1730 s; last phase started: source.served
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1730 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1740 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1750 s
[repo] still working after 1760 s; last phase started: source.served
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1760 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1770 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1780 s
[repo] still working after 1790 s; last phase started: source.served
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1790 s
graph: waiting for the fleet: 221 of 645 projects arrived
[repository:command] still running after 1800 s
graph: nothing to wait for any longer: asked the fleet for 645 stale projects and received 221 within 1498 s, 424 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:27:16.966Z graph-wanted-expired {"wanted":"645","arrived":"221","seconds":"1498","missing":"424","local":"192"}
[repository:command] still running after 1810 s
[repo] still working after 1820 s; last phase started: source.served
[repository:command] still running after 1820 s
```

### Try 3

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6feebb05efe3.log (lines after 1361)
-- lines with: waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts
-- last 30 lines when the try ended
[repository:command] still running after 1840 s
[repo] still working after 1850 s; last phase started: source.served
[repository:command] still running after 1850 s
[repository:command] still running after 1860 s
[repository:command] still running after 1870 s
[repo] still working after 1880 s; last phase started: source.served
[repository:command] still running after 1880 s
[repository:command] still running after 1890 s
[repository:command] still running after 1900 s
[repo] still working after 1910 s; last phase started: source.served
[repository:command] still running after 1910 s
[repository:command] still running after 1920 s
[repository:command] still running after 1930 s
[repo] still working after 1940 s; last phase started: source.served
[repository:command] still running after 1940 s
[repository:command] still running after 1950 s
[repository:command] still running after 1960 s
[repo] still working after 1970 s; last phase started: source.served
[repository:command] still running after 1970 s
[repository:command] still running after 1980 s
[repository:command] still running after 1990 s
[repo] still working after 2000 s; last phase started: source.served
[repository:command] still running after 2000 s
[repository:command] still running after 2010 s
[repository:command] still running after 2020 s
[repo] still working after 2030 s; last phase started: source.served
[repository:command] still running after 2030 s
[repository:command] still running after 2040 s
2026-11-14T18:31:20.656Z shutdown superseded
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-a561d8014168.log (lines after 0)
-- lines with: waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts
graph: 703 stale projects, 221 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 482
graph: waiting for the fleet: 0 of 482 projects arrived
graph: waiting for the fleet: 106 of 482 projects arrived
graph: waiting for the fleet: 154 of 482 projects arrived
graph: waiting for the fleet: 202 of 482 projects arrived
graph: waiting for the fleet: 250 of 482 projects arrived
graph: waiting for the fleet: 298 of 482 projects arrived
graph: refreshing 703 projects (generation 45)
graph: 177 stale facts (analysis 67, file 43, field 38, readdir 27, digest 2) leave 703 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: asking the host for up to 7 worker processes for 47 projects; projects are inferred in this process meanwhile
graph: asking the host for up to 4 worker processes for 28 projects; projects are inferred in this process meanwhile
graph: 3158 derived results asked again after 0 stale facts: 2117 unchanged, 1041 changed
-- last 30 lines when the try ended
[repository:command] still running after 3831 s
[repository:command] still running after 3841 s
[repository:command] still running after 3851 s
[repo] still working after 3860 s; last phase started: source.served
[repository:command] still running after 3861 s
[repository:command] still running after 3871 s
[repository:command] still running after 3881 s
[repo] still working after 3890 s; last phase started: source.served
[repository:command] still running after 3891 s
[repository:command] still running after 3901 s
[repository:command] still running after 3911 s
[repo] still working after 3920 s; last phase started: source.served
[repository:command] still running after 3921 s
[repository:command] still running after 3931 s
[repository:command] still running after 3941 s
[repo] still working after 3950 s; last phase started: source.served
[repository:command] still running after 3951 s
[repository:command] still running after 3961 s
[repository:command] still running after 3971 s
[repo] still working after 3980 s; last phase started: source.served
[repository:command] still running after 3981 s
[repository:command] still running after 3991 s
[repository:command] still running after 4001 s
[repo] still working after 4010 s; last phase started: source.served
[repository:command] still running after 4011 s
[repository:command] still running after 4021 s
[repository:command] still running after 4031 s
[repo] still working after 4040 s; last phase started: source.served
[repository:command] still running after 4041 s
[repository:command] still running after 4051 s
```

Other `graph:` lines of try 3 that describe the local inference (first occurrence of each distinct beginning):

```
graph: No verified change journal cursor is recorded for generation 44; verifying the tree once.
graph: nothing to wait for any longer: asked the fleet for 645 stale projects and received 221 within 1498 s, 424 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
```

## 6. Failed tasks of try 3 (`nx run workspace:setup --parallel=2`, run duration 16m 20s, cache 0/19)

Succeeded (12): `ui-rs:generate`, `assets:build`, `ui-styling-tokens:generate`, `framework-schema:generate`, `dsl-derive-rs:generate-schema-index`, `repo-mcp:build`, `repo:generator-inputs`, `ui-styling-py:deps`, `repo-test-dotnet:deps`, `energy-oracle-py:deps`, `repo-mcp:install`, `framework-graph:generate` (eleven `✔` lines are in the log; nx counts 12).

Failed (7): `@semio-tech/repo-lib:cpp-setup`, `@semio-tech/plugin-registry:generate`, `workspace:deps-build-tools`, `@semio-tech/framework-os-mcp-rs:build`, `workspace:deps-cargo`, `workspace:deps-trunk`, `workspace:deps-tools`.

Not run because their dependencies failed (4): `workspace:setup`, `workspace:deps-cpp`, `workspace:deps-wasm`, `workspace:prepare`.

Started and in neither list of nx: `workspace:deps-js-all`, `workspace:deps-wasm-opt`, `workspace:deps-go`, `workspace:setup-git`, `workspace:deps-browsers`, `workspace:deps-dotnet`, `workspace:deps-python`.

Final lines: `Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=2`, `repository:command failed with exit code 1`.

### `@semio-tech/repo-lib:cpp-setup`

First error line (line 2822 of the log without the JSON progress lines):

```
error: Single-link regular artifact source required
```

The 15 lines before it:

```
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
[script] nx:received-graph · Waiting for access to native-storage:[16777232,2522775,12883894272,16777264] (exclusive): position 2 of 2, behind the request of process 89077 (exclusive, queued 0 s ago) · holder: progressing, nx-sdk-derived-namespace:/Users/ueli/Documents/semio/.cache/b/t/m/nx/6184766c3a255f4a39f67c911be
[script] nx:received-graph · Waiting for access to native-storage:[16777232,2532232,1609605120,16777268] (exclusive): position 2 of 2, behind the request of process 89077 (exclusive, queued 0 s ago) · holder: progressing, native-storage:[16777232,2522775,12883894272,16777264]
> repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/📥️owner-command/📜️script.ts" owner-command --cwd "." -- repo "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts" cpp setup

15 | const descriptorLimit=schema.properties.maximumDescriptors.const,auxiliaryDescriptors=schema.properties.auxiliaryDescriptors.const,chunkBytes=schema.properties.chunkBytes.const,maximumFiles=schema.properties.maximumFiles.const,maximumBytes=schema.properties.maximumPublicationBytes.const;
16 | const scopes=new WeakMap<ArtifactSourceSetV1,Scope>();let reservedDescriptors=0;
17 | function check(options:Options){options.signal?.throwIfAborted();if(options.control)checkResourceLeaseControl(options.control);}
18 | function scopeFor(token:ArtifactSourceSetV1){const scope=scopes.get(token);if(!scope?.live||!scope.accepting)throw Error("Live admitted artifact source set required");check(scope.options);return scope;}
19 | function same(a:Identity,b:Identity){return a.dev===b.dev&&a.ino===b.ino&&a.size===b.size&&a.mode===b.mode&&a.mtimeNs===b.mtimeNs&&a.ctimeNs===b.ctimeNs&&a.nlink===b.nlink;}
20 | function regular(row:Awaited<ReturnType<FileHandle["stat"]>>|Awaited<ReturnType<typeof lstat>>){if(!row.isFile()||row.isSymbolicLink()||BigInt(row.nlink)!==1n)throw Error("Single-link regular artifact source required");}
                                                                                                                                                                          ^
```

### `@semio-tech/plugin-registry:generate`

First error line (line 2876 of the log without the JSON progress lines):

```
SyntaxError: export 'nativeBuildUnitGraphInputRequestsV1' not found in '../../../../../📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts'
```

The 15 lines before it:

```
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
> repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/📥️owner-command/📜️script.ts" owner-command --cwd "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" -- repo "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script

```

### `workspace:deps-build-tools`

First error line (line 3014 of the log without the JSON progress lines):

```
Internal error: Genuine dependency funding parent is busy
```

The 15 lines before it:

```
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
[repository:command] still running after 320 s
> repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts" native owner-command --manifest "Cargo.toml" --cwd "." -- "repo" "/Users/ueli/Documents/semio/📜️script.ts" "setup" "git"
> repo "$SEMIO_NX_COMMAND_RUNNER" c33bdbae0baf0c33b0a4ecaa123008729204ea2cd8cb8031b6cf7ea2bbcd823f
[script] build:source-admission · Admitting the sources of the build
[script] build:implementation-source · Reading the sources of the build implementation
```

### `@semio-tech/framework-os-mcp-rs:build`

First error line (line 7393 of the log without the JSON progress lines):

```
error: extern location for equivalent does not exist: /Users/ueli/Documents/semio/.cache/b/r/m/u/l/q0c9bae/prototype/build/equivalent/0bf2463dafad4359/out/libequivalent-0bf2463dafad4359.rmeta
```

The 15 lines before it:

```
   |
   = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
   |
39 - pub const FRAMEWORK_RESERVED_CONTRIBUTIONS: [manifest::ReservedContribution; 2] = [semio_framework_time_travel_actions::RESERVED, semio_framework_tool_run_actions::RESERVED];
39 + pub const FRAMEWORK_RESERVED_CONTRIBUTIONS: [ReservedContribution; 2] = [semio_framework_time_travel_actions::RESERVED, semio_framework_tool_run_actions::RESERVED];
   |

warning: `semio-framework` (lib) generated 1 warning (run `cargo fix --lib -p semio-framework` to apply 1 suggestion)
   Compiling heck v0.5.0
   Compiling anyhow v1.0.104
   Compiling semver v1.0.28
   Compiling bitflags v2.13.2
   Compiling equivalent v1.0.3
   Compiling indexmap v2.14.2
```

### `workspace:deps-cargo`

First error line (line 7485 of the log without the JSON progress lines):

```
RepoError: stale build.closure.source-differs {"path":"🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts","expectedSha256":"ffd49c0985745451d3594878764ffa7530288ec347b5c34a1ac6b6ab07602835","observedSha256":"88cc24fb11604397e4f9d6a041c1373c949f388f80b6e3cbe833c71917648742"}
```

The 15 lines before it:

```
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[repository:command] still running after 1120 s
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
59 | `&&t[d]!=="\r")t[d]=" "},o=0,s=(a)=>{let c=0,d="";while(o<e.length){let l=e[o],u=e[o+1];if(l==="/"&&u==="/"){let m=e.indexOf(`
60 | `,o),p=m<0?e.length:m;n(o,p),o=p;continue}if(l==="/"&&u==="*"){let m=e.indexOf("*/",o+2),p=m<0?e.length:m+2;n(o,p),o=p;continue}if(l==='"'||l==="'"){let m=o++;while(o<e.length&&e[o]!==l&&e[o]!==`
61 | `){if(e[o]==="\\")o++;o++}r.push({start:m,end:o+1,value:e.slice(m+1,o)}),n(m+1,o),o++,d="x";continue}if(l==="`"){let m=o++,p=o;while(o<e.length&&e[o]!=="`"){if(e[o]==="\\"){o+=2;continue}if(e[o]==="$"&&e[o+1]==="{"){n(p,o),o+=2,s(!0),p=o;continue}o++}n(p,o),i.push({start:m,end:o+1,body:e.slice(m+1,o)}),o++,d="x";continue}if(l==="/"&&(d===""||oEe.test(d)||sEe.test(d))){let m=o++,p=!1;while(o<e
62 | `){let f=e[o];if(f==="\\"){o+=2;continue}if(f==="[")p=!0;else if(f==="]")p=!1;else if(f==="/"&&!p)break;o++}n(m+1,o),o++,d="x";continue}if(a){if(l==="{")c++;else if(l==="}"){if(c===0){o++;return}c--}}if(!/\s/u.test(l))d=(d+l).slice(-24);o++}};return s(!1),{code:t.join(""),strings:r,templates:i}}function iw(e,t){let r=e.strings.find((i)=>i.start>=t&&/^\s*$/u.test(e.code.slice(t,i.start)));retu
63 | `).slice(-6).join(" | ").slice(0,600)});try{e.rename(n,t.directory)}catch(d){if(e.remove(n),!e.exists(t.file))throw d}}function fj(e){let t=FEe(e.host,e.read(e.host.source));if(e.exists(t.file))return t.file;let r=`${t.directory}.lock`,i=sr.lock,n=e.now()+i.waitSeconds*1000;e.mkdir(D_(t.directory));for(;;){if(e.lock(r))try{if(!e.exists(t.file))jEe(e,t);return t.file}finally{e.remove(r)}if(e.e
64 | `)}catch{}}})}function I_(e=hj()){let t=e.host.env[sr.addon.environment.addon],r=t!==void 0&&t!==""?t:fj(e),i;try{i=kEe(import.meta.url)(r)}catch(o){throw zl("nativeLoadFailed",{file:r,reason:String(o.message).slice(0,400)})}let n=sr.primitives.filter((o)=>typeof i[o]!=="function");if(n.length>0)throw zl("nativeCoreIncomplete",{file:r,missing:n.join(", ")});return i}var sr;var iE=w(()=>{on();
 | ... truncated 
```

### `workspace:deps-trunk`

First error line (line 7530 of the log without the JSON progress lines):

```
RepoError: stale build.closure.source-differs {"path":"🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts","expectedSha256":"ffd49c0985745451d3594878764ffa7530288ec347b5c34a1ac6b6ab07602835","observedSha256":"88cc24fb11604397e4f9d6a041c1373c949f388f80b6e3cbe833c71917648742"}
```

The 15 lines before it:

```
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
59 | `&&t[d]!=="\r")t[d]=" "},o=0,s=(a)=>{let c=0,d="";while(o<e.length){let l=e[o],u=e[o+1];if(l==="/"&&u==="/"){let m=e.indexOf(`
60 | `,o),p=m<0?e.length:m;n(o,p),o=p;continue}if(l==="/"&&u==="*"){let m=e.indexOf("*/",o+2),p=m<0?e.length:m+2;n(o,p),o=p;continue}if(l==='"'||l==="'"){let m=o++;while(o<e.length&&e[o]!==l&&e[o]!==`
61 | `){if(e[o]==="\\")o++;o++}r.push({start:m,end:o+1,value:e.slice(m+1,o)}),n(m+1,o),o++,d="x";continue}if(l==="`"){let m=o++,p=o;while(o<e.length&&e[o]!=="`"){if(e[o]==="\\"){o+=2;continue}if(e[o]==="$"&&e[o+1]==="{"){n(p,o),o+=2,s(!0),p=o;continue}o++}n(p,o),i.push({start:m,end:o+1,body:e.slice(m+1,o)}),o++,d="x";continue}if(l==="/"&&(d===""||oEe.test(d)||sEe.test(d))){let m=o++,p=!1;while(o<e
62 | `){let f=e[o];if(f==="\\"){o+=2;continue}if(f==="[")p=!0;else if(f==="]")p=!1;else if(f==="/"&&!p)break;o++}n(m+1,o),o++,d="x";continue}if(a){if(l==="{")c++;else if(l==="}"){if(c===0){o++;return}c--}}if(!/\s/u.test(l))d=(d+l).slice(-24);o++}};return s(!1),{code:t.join(""),strings:r,templates:i}}function iw(e,t){let r=e.strings.find((i)=>i.start>=t&&/^\s*$/u.test(e.code.slice(t,i.start)));retu
63 | `).slice(-6).join(" | ").slice(0,600)});try{e.rename(n,t.directory)}catch(d){if(e.remove(n),!e.exists(t.file))throw d}}function fj(e){let t=FEe(e.host,e.read(e.host.source));if(e.exists(t.file))return t.file;let r=`${t.directory}.lock`,i=sr.lock,n=e.now()+i.waitSeconds*1000;e.mkdir(D_(t.directory));for(;;){if(e.lock(r))try{if(!e.exists(t.file))jEe(e,t);return t.file}finally{e.remove(r)}if(e.e
64 | `)}catch{}}})}function I_(e=hj()){let t=e.host.env[sr.addon.environment.addon],r=t!==void 0&&t!==""?t:fj(e),i;try{i=kEe(import.meta.url)(r)}catch(o){throw zl("nativeLoadFailed",{file:r,reason:String(o.message).slice(0,400)})}let n=sr.primitives.filter((o)=>typeof i[o]!=="function");if(n.length>0)throw zl("nativeCoreIncomplete",{file:r,missing:n.join(", ")});return i}var sr;var iE=w(()=>{on();
 | ... truncated 
```

### `workspace:deps-tools`

First error line (line 7569 of the log without the JSON progress lines):

```
RepoError: stale build.closure.source-differs {"path":"🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts","expectedSha256":"ffd49c0985745451d3594878764ffa7530288ec347b5c34a1ac6b6ab07602835","observedSha256":"88cc24fb11604397e4f9d6a041c1373c949f388f80b6e3cbe833c71917648742"}
```

The 15 lines before it:

```
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
59 | `&&t[d]!=="\r")t[d]=" "},o=0,s=(a)=>{let c=0,d="";while(o<e.length){let l=e[o],u=e[o+1];if(l==="/"&&u==="/"){let m=e.indexOf(`
60 | `,o),p=m<0?e.length:m;n(o,p),o=p;continue}if(l==="/"&&u==="*"){let m=e.indexOf("*/",o+2),p=m<0?e.length:m+2;n(o,p),o=p;continue}if(l==='"'||l==="'"){let m=o++;while(o<e.length&&e[o]!==l&&e[o]!==`
61 | `){if(e[o]==="\\")o++;o++}r.push({start:m,end:o+1,value:e.slice(m+1,o)}),n(m+1,o),o++,d="x";continue}if(l==="`"){let m=o++,p=o;while(o<e.length&&e[o]!=="`"){if(e[o]==="\\"){o+=2;continue}if(e[o]==="$"&&e[o+1]==="{"){n(p,o),o+=2,s(!0),p=o;continue}o++}n(p,o),i.push({start:m,end:o+1,body:e.slice(m+1,o)}),o++,d="x";continue}if(l==="/"&&(d===""||oEe.test(d)||sEe.test(d))){let m=o++,p=!1;while(o<e
62 | `){let f=e[o];if(f==="\\"){o+=2;continue}if(f==="[")p=!0;else if(f==="]")p=!1;else if(f==="/"&&!p)break;o++}n(m+1,o),o++,d="x";continue}if(a){if(l==="{")c++;else if(l==="}"){if(c===0){o++;return}c--}}if(!/\s/u.test(l))d=(d+l).slice(-24);o++}};return s(!1),{code:t.join(""),strings:r,templates:i}}function iw(e,t){let r=e.strings.find((i)=>i.start>=t&&/^\s*$/u.test(e.code.slice(t,i.start)));retu
63 | `).slice(-6).join(" | ").slice(0,600)});try{e.rename(n,t.directory)}catch(d){if(e.remove(n),!e.exists(t.file))throw d}}function fj(e){let t=FEe(e.host,e.read(e.host.source));if(e.exists(t.file))return t.file;let r=`${t.directory}.lock`,i=sr.lock,n=e.now()+i.waitSeconds*1000;e.mkdir(D_(t.directory));for(;;){if(e.lock(r))try{if(!e.exists(t.file))jEe(e,t);return t.file}finally{e.remove(r)}if(e.e
64 | `)}catch{}}})}function I_(e=hj()){let t=e.host.env[sr.addon.environment.addon],r=t!==void 0&&t!==""?t:fj(e),i;try{i=kEe(import.meta.url)(r)}catch(o){throw zl("nativeLoadFailed",{file:r,reason:String(o.message).slice(0,400)})}let n=sr.primitives.filter((o)=>typeof i[o]!=="function");if(n.length>0)throw zl("nativeCoreIncomplete",{file:r,missing:n.join(", ")});return i}var sr;var iE=w(()=>{on();
 | ... truncated 
```

The `stale build.closure.source-differs` of the last three tasks names `🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts` (expected sha256 `ffd49c09…`, observed `88cc24fb…`): the file changed in the shared tree while the run was going.

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4563 MB because of its assembly phase
[capacity] admitted run with 2 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 19% of the default from 15 earlier run(s) of its class, at least 4563 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 11 earlier run(s) of its class, at least 5882 MB because of its assembly phase
[capacity] admitted dependencies with 5372 of 5882 MB: only 5372 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 11 earlier run(s) of its class, at least 5882 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6989 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4563 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5417 MB reserved (7499 MB available, 2048 MB floor)
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4563 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 12 earlier run(s) of its class, at least 5882 MB because of its assembly phase
[capacity] admitted dependencies with 5287 of 5882 MB: only 5287 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 12 earlier run(s) of its class, at least 5882 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6624 MB available, 2048 MB floor)
```

Try 3:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4558 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 14% of the default from 15 earlier run(s) of its class, at least 4558 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 12 earlier run(s) of its class, at least 5883 MB because of its assembly phase
[capacity] admitted dependencies with 4752 of 5883 MB: only 4752 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 12 earlier run(s) of its class, at least 5883 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6131 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (249 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 1623 MB), graph-farm (running, 1 job(s), 1536 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (-3379 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 10496 MB), graph-farm (running, 1 job(s), 1536 MB)
[capacity] run: work has to run, waiting 12s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (-3379 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 10496 MB), graph-farm (running, 1 job(s), 1536 MB)
[capacity] run: work has to run, waiting 18s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (542 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10496 MB)
[capacity] run: work has to run, waiting 23s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (535 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3623 MB)
[capacity] run: work has to run, waiting 29s to grow the reservation from 2048 MB to 12565 MB: not enough free memory (745 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3623 MB)
[capacity] run: reservation grown from 2048 MB to 3949 MB with 1 of 8 job(s) after 34s: the host cannot carry more right now (4015 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
[capacity] run: grown from 1 to 2 of 8 job(s) after 2355s, reservation 3949 MB -> 3949 MB: memory freed up and nothing of ours waits (3236 MB available, 2048 MB floor); work that starts from now on uses the new jobs
[capacity] run: sized at 91% of the default from 3 earlier run(s) of its class
```

## Samples (every 30 s, `ps -axo rss,pid,ppid,args | sort -nr | head -3`)

| Try | Samples | Pressure level other than 1 | Largest process |
|---|---|---|---|
| 1 | 2 | 0 | 2385936 KiB, pid 40690, ppid 47709 |
| 2 | 56 | 0 | 3552400 KiB, pid 42223, ppid 40854 |
| 3 | 136 | 2 | 6341856 KiB, pid 57930, ppid 57846 |

The largest process of try 3 (6341856 KiB, 6.05 GiB) is a child of the graph service's worker (ppid 57846) during the local inference; no process above 8 GB. The `git` processes of 2.3 to 2.5 GB of the Claude desktop app (round 3 report) were present again.

## Wall time per phase

| Phase | Try 1 | Try 2 | Try 3 |
|---|---|---|---|
| whole run | 46 s | 27 min 47 s | 67 min 49 s |
| waiting for the fleet | not reached | 1498 s | not measured exactly: the last "waiting for the fleet" line is `154 of 482`; by the `[repo] still working` counter the local inference (`owners N of 703`) was running at about 1570 s of `workspace:setup` |
| graph step in total (until `owners 703 of 703`) | not reached | not finished | about 50 min (18:29:34Z start, 703 of 703 seen between 19:15Z and 19:25Z) |
| nx tasks of `workspace:setup` | not reached | not reached | 16 min 20 s (nx run duration) |
| launcher, toolchain, native repo command, dependency scopes | not measured separately | not measured separately | not measured separately |

## Files

`run-round-5.sh` (runner), `round-5.monitor.txt`, `round-5.prerun.txt`, `round-5.try-<k>.samples.txt`, `round-5.try-<k>.service.txt`, `round-5.tail.txt` (last 200 lines of try 3), `round-5.try-1.tail.txt`, `round-5.try-2.tail.txt`.

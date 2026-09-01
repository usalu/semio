# Independent UI Builder And Evidence R3–R4

## Actual Result

Corrected R4:20PASS/678 skipped/698 total,one selected file of5,3.63s,start03:21:11,Nx0. All69 selected source/fixture/config hashes match before and after. This is the combined11 builder plus9 evidence cohort, not a full renderer or strict-type pass.

## R3 Routing Failure

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedResidentBuilder|OwnedResidentEvidence'
```

Nx forwarded the unquoted regex pipe to its shell, which attempted to run `OwnedResidentEvidence` as a command and exited1. The left side also started; no test summary was retained, so this attempt has no credited runtime result and is not a semantic RED. Its exact output is preserved below. No production source or budget was changed. All69 selected hashes remained stable across that failed invocation.

## R4 Corrected Invocation

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t "OwnedResidentBuilder|OwnedResidentEvidence"'
```

The inner quotes survived Nx forwarding, keeping the alternation inside the Vitest selector. The same canonical Bun/Nx test-long route and existing budget ran once with the corrected invocation. A post-run tool expression contained an invalid token and was rejected before execution; the following valid hash command then captured the unchanged source before both holds were released.

## Source Ownership And Executed Scope

UI and the Demonstrator acknowledged the narrow holds before capture and maintained them through both attempts. Root released both immediately after terminal and post-hash capture. The manifest is selected source stability only, not full import/ancestry or whole-tree proof. No native compiler was launched.

The coordinator read the complete20 test bodies and the R84–R87 source report. The actual laws include schema/AST/independent arithmetic;13-phase builder admission; every one-byte-short refusal;14 builder admission cancellation prefixes;11-phase private evidence admission;12 evidence prefixes without replacement tokens; exact source evidence detach/settlement and separate752 registration refund; the17-grant two-way builder binding and separate856 refund; payload872 retirement; before/after source faults; parent/cell quarantine; exact blocked/rejected/full/over-grant child propagation; and four revocation frontiers0/3/6/9.

An untransferred original source fragment remains owned by its source after builder-only cancellation. The body-empty test has no mounted pages/readers; it does not declare arbitrary content empty. The recorded refunds are logical registration conservation, not facade garbage-collection or physical heap measurements. Faulted owners remain charged.

Copied-evidence/page/reader streaming, source-side metadata admission, canonical raw receiver/worker credit, native InputAck, final controller/native parent retirement, full strict and full app/runtime gates remain open. Peer fullactor175PASS/3FAIL and UI strict41 are separate delegated snapshots; this run skipped678 tests.

## Exact R3 Tool Capture

```json
{
  "command": "NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedResidentBuilder|OwnedResidentEvidence'",
  "run": {
    "chunk_id": "dd4daf",
    "wall_time_seconds": 1.003401875,
    "session_id": 89370,
    "original_token_count": 0,
    "output": ""
  },
  "poll": {
    "chunk_id": "10e803",
    "wall_time_seconds": 0.000001792,
    "exit_code": 1,
    "original_token_count": 1808,
    "output": "\n> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedResidentBuilder|OwnedResidentEvidence\n\n> bun ./📜️script.ts test long --run -t OwnedResidentBuilder|OwnedResidentEvidence\r\n\r\n/bin/sh: OwnedResidentEvidence: command not found\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\nWarning: command \"bun ./📜️script.ts test long --run -t OwnedResidentBuilder|OwnedResidentEvidence\" exited with non-zero status code\n\n\n NX   Running target test-long for project @semio-tech/framework-renderer-react failed\n\nFailed tasks:\n\n- @semio-tech/framework-renderer-react:test-long\n\nHint: run the command with --verbose for more details.\n\n"
  }
}
```

## Exact R4 Tool Capture

```json
{
  "command": "NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t \"OwnedResidentBuilder|OwnedResidentEvidence\"'",
  "run": {
    "chunk_id": "3d0a48",
    "wall_time_seconds": 1.00377375,
    "session_id": 64329,
    "original_token_count": 0,
    "output": ""
  },
  "poll": {
    "chunk_id": "df5de2",
    "wall_time_seconds": 0.00000175,
    "exit_code": 0,
    "original_token_count": 1833,
    "output": "\n> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t \"OwnedResidentBuilder|OwnedResidentEvidence\"\n\n> bun ./📜️script.ts test long --run -t \"OwnedResidentBuilder|OwnedResidentEvidence\"\r\n\r\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\n RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\u001b[0m\u001b[33mWarning\u001b[0m\u001b[2m:\u001b[0m \u001b[1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mwarnOnDeactivatedColors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m33\u001b[0m\u001b[2m:\u001b[33m24\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mgetColorDepth\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:tty\u001b[0m\u001b[2m:\u001b[0m\u001b[33m42\u001b[0m\u001b[2m:\u001b[33m39\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mshouldColorize\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m14\u001b[0m\u001b[2m:\u001b[33m109\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mrefresh\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m18\u001b[0m\u001b[2m:\u001b[33m31\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:util/colors\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:util/colors\u001b[0m\u001b[2m:\u001b[0m\u001b[33m24\u001b[0m\u001b[2m:\u001b[33m16\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3minternal:assert/assertion_error\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36minternal:assert/assertion_error\u001b[0m\u001b[2m:\u001b[0m\u001b[33m2\u001b[0m\u001b[2m:\u001b[33m187\u001b[0m\u001b[2m)\u001b[0m\n\u001b[0m      \u001b[2mat \u001b[0m\u001b[0m\u001b[1m\u001b[3mloadAssertionError\u001b[0m\u001b[2m (\u001b[0m\u001b[0m\u001b[36mnode:assert\u001b[0m\u001b[2m:\u001b[0m\u001b[33m28\u001b[0m\u001b[2m:\u001b[33m96\u001b[0m\u001b[2m)\u001b[0m\n\n\n Test Files  1 passed | 4 skipped (5)\n      Tests  20 passed | 678 skipped (698)\n   Start at  03:21:11\n   Duration  3.63s (transform 6.66s, setup 0ms, import 9.04s, tests 715ms, environment 1.96s)\n\n\n\n\n NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react\n\n\n"
  }
}
```

## R4 Readable Terminal

```text
> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t "OwnedResidentBuilder|OwnedResidentEvidence"

> bun ./📜️script.ts test long --run -t "OwnedResidentBuilder|OwnedResidentEvidence"

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  1 passed | 4 skipped (5)
      Tests  20 passed | 678 skipped (698)
   Start at  03:21:11
   Duration  3.63s (transform 6.66s, setup 0ms, import 9.04s, tests 715ms, environment 1.96s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

## Pre-Capture Hashes

```text
8df81492f42dfa1232a718e917149b209d7151a72d5bea397f354091290f55ad  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
bf3e61ae9dc23e53987a69b38031fe7564f5c6520700abb5bc77c0d2183faa88  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧪️fixture.json
fec64c912116aad6f6ee7336829daeb8d9866cad9e7dde3b135434dc6628bb0c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧪️schema.json
d91db7dbec3337a54d65db2885f1af8066efd7d7b3f26f011fdf12cfd4de5eab  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️contract.json
513f30617f342a696fc6f784773939b2e0b8f8684bd2cfe692dcf55dfde333c3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️schema.json
7060883d2207777e65dc8dfa5a4d06ed7dd75123f15682c2c441de9f9ccfee26  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️fixture.json
dedd8aae066a8854171ac28c009dddff19a1dddfd48fcf2851edfbccbf29e3e1  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️schema.json
53a601ea75ddd11f05372fce7eea0e1bfd639063c3e308b2e2b023abd53b2329  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️contract.json
5f117157e8432ac6c4f30d77662b40553e4a84d818db70d3fda7ae50c2d8e916  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema.json
5edeb104796ee6c8231bc87648a447cb34fc13e5849a768fad8e78f02165cd51  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
644a09582afc6a48341fd03ff043642f191db01f4814125f44d793e716fac05e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧪️fixture.json
e8c7dfa9e77c0d4cb73d2345e1111a5c3e5552ac9869989aa570e2170c3bc823  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧪️schema.json
2f964e47d6a673ef6f2cb209e8d5071611680de8bfebe4cc919b1f6d3bb61178  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️contract.json
0b2853c860eb05b957520dd97ff24a4902f69e95c66598d11c7add7f470c4098  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️schema.json
e93ea7a4f1ad39703b126a9c9847cc63f9c1afce9ae062cb2eb453309bf4827f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
3c6cc4246e1e6a841d0158e103056bc2706843d5866294791e8e5a77affafcc8  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
f6ad9100dfc187ef6430e1c5b9fea6172743930d969cfa86a19144f17fa90e9e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️component.ts
87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
cbe9a8cba5f138a4892f0c751de5f6693d61a84635228cc5de3bb1deef5bca21  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
115d3ca312e424de2ae3fd6c8573f37e5baf056500c279a1d662bd01ed6f68e4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
7afa3143644036f4f6486bc8be860a37fbe12e9afdfb8f635253f5e3abf03089  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
a5b6b7d300351971f3c0fa505f62131b4d6931bfbf4ee876c6eb93a1c4cd9097  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts
ff50668ea54d4fc985fa0ed1db123862361e4d9c506d0bb9ec44c6698305faec  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
9f21bd7e7468a091b24c6a03e1f5661a849156b883185fa18450f05d4b5b12e3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️fixture.json
942592b42a3c444f663e9319ecc2e1b0b0a23fa934c991f3e251a79ce1a5fb5d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️schema.json
fd740759266dc3ba8c1a086dd970ba9b4da6ba71e89ee9a97690506f0b1e9766  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️contract.json
76ee6ef761569101fa6e122c9178721c3b75f708260d868f0bec597efc068dea  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json
0d5537911a8182d0b880225ba6dbcf7d6ddd035392ea786e164b556e28eae575  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture.json
b738c919a3c18f6a402892899ea8be0c092c087264b1a58a408786e45e9b775d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️schema.json
aceb2a84ec6e05202c55299f22fd283c61a5887b1a5cd4c3ea337053ff6fc797  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️contract.json
3e6e680878c616a06acd6d5a1bdfdbd3a6acbfc510e8eaf83985071218845c56  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json
55a104f4877689e212d04aaa925e8ce91a6beeb687a11edbd1126e0bf97c89c6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
69b8029194e89ba34647d23f857a5bc85e3921648a1d8fb74caf7317db26020d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️fixture.json
a2bf880f8e691a9ae9b775185c2e7ed0e2b36c412c278d172352465e7da50e57  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema.json
45fc4a27d7722cb8d878a97b1b7a81696e514561f8fa552f2d6883e76073d360  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧪️fixture.json
c8014ad5443e2d0b307de3f3768d8daf76714961222dc75867d0ca58254379fb  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema.json
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
7372c8b826126375744e453dde9f496b6bdd949974d88feaaf1b9b4999bad583  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json
86c17e26345b106a223e5856dbf555821ecc5838d6329a83d33ed58ecb24d69e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️schema.json
ce4b106f44e41a4774bd013ea238c0ac78d1e200abb35e5d7e7174d277b79f02  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️contract.json
632633312bf19e05328cdba8066dfb8815d8ab5160004562b2a4e2ed4801d6b4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json
2dd1bedd0a96206c8c48b2b1f516e4ac03851e2f8f46910fb762577274160184  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json
b022c923c784c2767cd62541c5fdf04b88b3141f670ba45a1044097e6e3e75c0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️schema.json
31fb4de0189614f3300da19511ee45bf0363bde92ccbbb94f74db18a19f550f6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️contract.json
391ed28331efd821ae5202530bfc302a60c9e6167eeccf69799a6a932e9b592c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json
bb65777fe21df516694632dd3ebbb60cb37e5a7588f622bde03d1ea7e25e9ef5  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json
7ccdea4203b34e05fb8aa8dbaffd71516704cde46e47179615280640dbbecddc  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️schema.json
9d4b0070c5cf760870c0f4d42cda545493f92b13affcd51c7e923f81f719d0ad  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️contract.json
06a7c498de6d1830f8f04c1156799127c37eea24f08f0586149f91e1069a9c57  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json
0e38f10aeac6d258fa5faae63015f6eae334259d5c0f1918a5a42d89d8abc8b2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
c6665ba2b2f4d69d292e58290c08a3204df0c7ba3896a80c0477e3fc06611fbd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json
e75dfa18930ba21b975352267e4f016d7935cfd3bdb6493912762a3de12f47d0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️schema.json
07678b9544a0fa2d6c025bc6a1ae527c70cb9e73430187750255225ee3297854  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️contract.json
be24d686a32e88b61e1db8235bc2b1dad0a5752d4604ccc644f0dd0b5dc0fe6c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json
340c765cd3432edb5c5426d0efca7ada23c6e04951c00b4464a9391eecddd569  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts
12d126fce98d7f9d309cdf83c370f761654ecf93795beface442175bb9810769  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json
6283d9e72b10476030d23564668d76af0bc882579f128b58a455c19f68d5f2c3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️schema.json
202113fa2f1cc8cf24de89bb192697259dd4a507825b0158c4a27262dc35fa7c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️contract.json
745742fc3f60c06e11ef022a2e3a3ec98672fbec32c1fd6953721c45c6509933  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json
0cbd82c7433272a3f63d1947590dc67e50f61b8a2ee72aad05d0f079b8978e9b  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
c3a2942b485b2da3b78ea2aca1240e53735e72969ab344137c89864b51c6b744  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json
5ed62d04c64c8e5156b32f8e7bb88cde6c057a43057376de58a94da4d742a21b  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts
e0a3b40a20620d710fdb5723a80e1f99e52ed2675b2b2b7e05eddf2df7268c59  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts
f1c847a7d62c8c801b184dc9510dda8365416d222aaf660c8b89d775ad9459e7  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

## Post-Capture Hashes

```text
8df81492f42dfa1232a718e917149b209d7151a72d5bea397f354091290f55ad  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
bf3e61ae9dc23e53987a69b38031fe7564f5c6520700abb5bc77c0d2183faa88  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧪️fixture.json
fec64c912116aad6f6ee7336829daeb8d9866cad9e7dde3b135434dc6628bb0c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧪️schema.json
d91db7dbec3337a54d65db2885f1af8066efd7d7b3f26f011fdf12cfd4de5eab  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️contract.json
513f30617f342a696fc6f784773939b2e0b8f8684bd2cfe692dcf55dfde333c3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️schema.json
7060883d2207777e65dc8dfa5a4d06ed7dd75123f15682c2c441de9f9ccfee26  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️fixture.json
dedd8aae066a8854171ac28c009dddff19a1dddfd48fcf2851edfbccbf29e3e1  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧪️schema.json
53a601ea75ddd11f05372fce7eea0e1bfd639063c3e308b2e2b023abd53b2329  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️contract.json
5f117157e8432ac6c4f30d77662b40553e4a84d818db70d3fda7ae50c2d8e916  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema.json
5edeb104796ee6c8231bc87648a447cb34fc13e5849a768fad8e78f02165cd51  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
644a09582afc6a48341fd03ff043642f191db01f4814125f44d793e716fac05e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧪️fixture.json
e8c7dfa9e77c0d4cb73d2345e1111a5c3e5552ac9869989aa570e2170c3bc823  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧪️schema.json
2f964e47d6a673ef6f2cb209e8d5071611680de8bfebe4cc919b1f6d3bb61178  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️contract.json
0b2853c860eb05b957520dd97ff24a4902f69e95c66598d11c7add7f470c4098  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️schema.json
e93ea7a4f1ad39703b126a9c9847cc63f9c1afce9ae062cb2eb453309bf4827f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
3c6cc4246e1e6a841d0158e103056bc2706843d5866294791e8e5a77affafcc8  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
f6ad9100dfc187ef6430e1c5b9fea6172743930d969cfa86a19144f17fa90e9e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️component.ts
87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
cbe9a8cba5f138a4892f0c751de5f6693d61a84635228cc5de3bb1deef5bca21  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
115d3ca312e424de2ae3fd6c8573f37e5baf056500c279a1d662bd01ed6f68e4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
7afa3143644036f4f6486bc8be860a37fbe12e9afdfb8f635253f5e3abf03089  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
a5b6b7d300351971f3c0fa505f62131b4d6931bfbf4ee876c6eb93a1c4cd9097  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts
ff50668ea54d4fc985fa0ed1db123862361e4d9c506d0bb9ec44c6698305faec  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
9f21bd7e7468a091b24c6a03e1f5661a849156b883185fa18450f05d4b5b12e3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️fixture.json
942592b42a3c444f663e9319ecc2e1b0b0a23fa934c991f3e251a79ce1a5fb5d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️schema.json
fd740759266dc3ba8c1a086dd970ba9b4da6ba71e89ee9a97690506f0b1e9766  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️contract.json
76ee6ef761569101fa6e122c9178721c3b75f708260d868f0bec597efc068dea  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json
0d5537911a8182d0b880225ba6dbcf7d6ddd035392ea786e164b556e28eae575  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture.json
b738c919a3c18f6a402892899ea8be0c092c087264b1a58a408786e45e9b775d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️schema.json
aceb2a84ec6e05202c55299f22fd283c61a5887b1a5cd4c3ea337053ff6fc797  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️contract.json
3e6e680878c616a06acd6d5a1bdfdbd3a6acbfc510e8eaf83985071218845c56  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json
55a104f4877689e212d04aaa925e8ce91a6beeb687a11edbd1126e0bf97c89c6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
69b8029194e89ba34647d23f857a5bc85e3921648a1d8fb74caf7317db26020d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️fixture.json
a2bf880f8e691a9ae9b775185c2e7ed0e2b36c412c278d172352465e7da50e57  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema.json
45fc4a27d7722cb8d878a97b1b7a81696e514561f8fa552f2d6883e76073d360  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧪️fixture.json
c8014ad5443e2d0b307de3f3768d8daf76714961222dc75867d0ca58254379fb  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema.json
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
7372c8b826126375744e453dde9f496b6bdd949974d88feaaf1b9b4999bad583  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json
86c17e26345b106a223e5856dbf555821ecc5838d6329a83d33ed58ecb24d69e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️schema.json
ce4b106f44e41a4774bd013ea238c0ac78d1e200abb35e5d7e7174d277b79f02  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️contract.json
632633312bf19e05328cdba8066dfb8815d8ab5160004562b2a4e2ed4801d6b4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json
2dd1bedd0a96206c8c48b2b1f516e4ac03851e2f8f46910fb762577274160184  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json
b022c923c784c2767cd62541c5fdf04b88b3141f670ba45a1044097e6e3e75c0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️schema.json
31fb4de0189614f3300da19511ee45bf0363bde92ccbbb94f74db18a19f550f6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️contract.json
391ed28331efd821ae5202530bfc302a60c9e6167eeccf69799a6a932e9b592c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json
bb65777fe21df516694632dd3ebbb60cb37e5a7588f622bde03d1ea7e25e9ef5  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json
7ccdea4203b34e05fb8aa8dbaffd71516704cde46e47179615280640dbbecddc  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️schema.json
9d4b0070c5cf760870c0f4d42cda545493f92b13affcd51c7e923f81f719d0ad  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️contract.json
06a7c498de6d1830f8f04c1156799127c37eea24f08f0586149f91e1069a9c57  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json
0e38f10aeac6d258fa5faae63015f6eae334259d5c0f1918a5a42d89d8abc8b2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
c6665ba2b2f4d69d292e58290c08a3204df0c7ba3896a80c0477e3fc06611fbd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json
e75dfa18930ba21b975352267e4f016d7935cfd3bdb6493912762a3de12f47d0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️schema.json
07678b9544a0fa2d6c025bc6a1ae527c70cb9e73430187750255225ee3297854  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️contract.json
be24d686a32e88b61e1db8235bc2b1dad0a5752d4604ccc644f0dd0b5dc0fe6c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json
340c765cd3432edb5c5426d0efca7ada23c6e04951c00b4464a9391eecddd569  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts
12d126fce98d7f9d309cdf83c370f761654ecf93795beface442175bb9810769  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json
6283d9e72b10476030d23564668d76af0bc882579f128b58a455c19f68d5f2c3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️schema.json
202113fa2f1cc8cf24de89bb192697259dd4a507825b0158c4a27262dc35fa7c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️contract.json
745742fc3f60c06e11ef022a2e3a3ec98672fbec32c1fd6953721c45c6509933  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json
0cbd82c7433272a3f63d1947590dc67e50f61b8a2ee72aad05d0f079b8978e9b  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
c3a2942b485b2da3b78ea2aca1240e53735e72969ab344137c89864b51c6b744  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json
5ed62d04c64c8e5156b32f8e7bb88cde6c057a43057376de58a94da4d742a21b  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts
e0a3b40a20620d710fdb5723a80e1f99e52ed2675b2b2b7e05eddf2df7268c59  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts
f1c847a7d62c8c801b184dc9510dda8365416d222aaf660c8b89d775ad9459e7  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

## Continuation

UI is released to the separately declared page/storage-owner and early-reader TDD. The peer may update its stale actor cancellation caller to the actual payload-owned evidence/binding close, preserving the copied-page failures. No cleanup, output publication, quota change or ticket/goal completion follows.


# Prefix1fa Native Log Watch

Owner Cargo reports sole actual live TTY46021 after11:02:12.977Z precheck; only Cargo polls/settles it. Log expected dev16777229/ino484808951/0600/nlink1. Independent observer reads only ordinary bounded raw-log extents; no terminal inferred from log. No Native/process effects.

## First Actual Observation

{
  "at": "2026-10-21T11:03:11.413Z",
  "extent": [
    0,
    1355
  ],
  "size": 1355,
  "mtime": "1792580586499143257",
  "fdNameStable": true,
  "tail": [
    "[capacity] waiting 22s for exec: an earlier request is still waiting (need 3584 MB; -120 MB spare above the 6553 MB floor, load 27.72; 6 running, 6 ahead)",
    "[capacity] waiting 28s for exec: an earlier request is still waiting (need 3584 MB; -2678 MB spare above the 6553 MB floor, load 27.97; 6 running, 6 ahead)",
    "[capacity] waiting 34s for exec: not enough free memory (need 3584 MB; -1934 MB spare above the 6553 MB floor, load 29.09; 6 running, 6 ahead)",
    "[capacity] waiting 39s for exec: not enough free memory (need 3584 MB; -2206 MB spare above the 6553 MB floor, load 29.33; 6 running, 6 ahead)",
    "[capacity] waiting 45s for exec: not enough free memory (need 3584 MB; -2942 MB spare above the 6553 MB floor, load 29.46; 6 running, 6 ahead)"
  ],
  "relevant": []
}

## Material Sequential Observation

{
  "at": "2026-10-21T11:48:25.675Z",
  "extent": [
    66577,
    97844
  ],
  "size": 97844,
  "fdNameStable": true,
  "stage": "queue",
  "markers": [],
  "errors": [],
  "tail": [
    "[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 15 s)",
    "[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"lease-queue:{\\\"version\\\":1,\\\"position\\\":8,\\\"queued\\\":8,\\\"holderMode\\\":\\\"shared\\\",\\\"holderPid\\\":81882,\\\"holderSeconds\\\":3620,\\\"resource\\\":\\\"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g\\\"}\",\"mode\":\"shared\",\"elapsedMs\":0}}",
    "[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 8 of 8, behind the request of process 81882 (shared, queued 3620 s ago)"
  ]
}

## Material Sequential Observation

{
  "at": "2026-10-21T12:22:54.921Z",
  "extent": [
    210448,
    217762
  ],
  "size": 217762,
  "fdNameStable": true,
  "stage": "queue:native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g",
  "markers": [],
  "errors": [
    "error: Original resource loan deadline exhausted"
  ],
  "tail": [
    "      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:142:5706)",
    "",
    "Bun v1.3.14 (macOS arm64)"
  ]
}

## Final Frozen Result

Cargo actual owner poll confirmed TTY46021 exit1, without cancellation. Terminal attribution is owner report, not inferred from log. No live job remains for this observer.

Independent full32768-byte descriptor stream at2026-10-21T12:23:37.549Z: 217762B/SHA256f53d266804ea194f50079480d535852be1c25153475d6a76d6a6b0c1b1bed4a6. Original epoch `16777229:484808951:33152:1:217762:1792585346677462786:1792585346677462786`, mode0600/nlink1. Stable before/afterFD and named epoch, EOF exactly at size; retained ancestry unchanged. Sequential observed extents also cover0–217762 and stream SHA256f53d266804ea194f50079480d535852be1c25153475d6a76d6a6b0c1b1bed4a6; matches frozen fullstream=true.

All five exact marker families count0 across full frozen stream: captured-neutral genuine unit; fresh capture prefix genuine unit; prefix stage; late Source sampled; prefix complete. Thus no unit/prefix/sample/owner success is evidenced by this run. No futurepath absence audit performed by observer; owner handles that separately.

Exact refusal:

```text
error: Original resource loan deadline exhausted
      at remaining (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:214:643)
      at checkResourceLeaseControl (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts:18:29)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:142:5706)

Bun v1.3.14 (macOS arm64)
```

Bounded startup evidence: initial hostcapacity wait, then graph image lease queue (shared/exclusive request telemetry). No verified kernel holder, cycle or process ownership inferred from diagnostic queue PID. No retry, cleanup, Source/test evaluation or TTY access performed. Watch finalized.

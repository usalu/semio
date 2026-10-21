# Guest Own7 Unit2 Log Watch

Cargo reports actual sole owned TTY57283 launched current own7 facd93fe31f2ae08561312024e510590d6e0d16e91a80e7aea1dcc9aa948d9ac. Cargo alone polls/settles. Original logdev16777229/ino456693168/0600/nlink1. Reviewer performs bounded sequential nofollow log reads only; actual terminal requires owner confirmation.

Initial observation:

```json
{
  "size": 9400,
  "out": [
    {
      "extent": [
        0,
        9400
      ],
      "tail": "\",\"elapsedMs\":372.93274999999994}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:45056\",\"mode\":\"shared\",\"elapsedMs\":412.59912500000064}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:49152\",\"mode\":\"shared\",\"elapsedMs\":451.02691699999923}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:53248\",\"mode\":\"shared\",\"elapsedMs\":490.9272920000003}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:57344\",\"mode\":\"shared\",\"elapsedMs\":529.3847920000007}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:61440\",\"mode\":\"shared\",\"elapsedMs\":569.8344580000012}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:65536\",\"mode\":\"shared\",\"elapsedMs\":610.2883330000004}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:69632\",\"mode\":\"shared\",\"elapsedMs\":651.228833000001}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:73728\",\"mode\":\"shared\",\"elapsedMs\":692.1895829999994}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"dependencies:generation-verify:77824\",\"mode\":\"shared\",\"elapsedMs\":724.3965830000016}}\n",
      "markers": []
    }
  ]
}
```

Sequential extent[0,9400), same original identity. Latest bounded observation:

```json
{
  "size": 37211,
  "out": [
    {
      "extent": [
        31552,
        37211
      ],
      "tail": "nts/semio:shard:7\",\"mode\":\"exclusive\",\"elapsedMs\":167662}}\n[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 168 s)\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"lease-queue:{\\\"version\\\":1,\\\"position\\\":2,\\\"queued\\\":2,\\\"holderMode\\\":\\\"exclusive\\\",\\\"holderPid\\\":8301,\\\"holderSeconds\\\":404,\\\"resource\\\":\\\"repository-graph-facts:/Users/ueli/Documents/semio:shard:7\\\"}\",\"mode\":\"exclusive\",\"elapsedMs\":0}}\n[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 8301 (exclusive, queued 404 s ago)\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"resource-directory-admission:v1\",\"mode\":\"shared\",\"elapsedMs\":0}}\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"repository-graph-facts:/Users/ueli/Documents/semio:shard:7\",\"mode\":\"exclusive\",\"elapsedMs\":183154}}\n[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 183 s)\n[script] {\"owner\":\"repository:build\",\"command\":\"repository:command\",\"stage\":\"running\",\"activity\":{\"resource\":\"lease-queue:{\\\"version\\\":1,\\\"position\\\":2,\\\"queued\\\":2,\\\"holderMode\\\":\\\"exclusive\\\",\\\"holderPid\\\":8301,\\\"holderSeconds\\\":419,\\\"resource\\\":\\\"repository-graph-facts:/Users/ueli/Documents/semio:shard:7\\\"}\",\"mode\":\"exclusive\",\"elapsedMs\":0}}\n[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 8301 (exclusive, queued 419 s ago)\n",
      "markers": []
    }
  ]
}
```

Sequential examined[0,9400), original identity unchanged; latest progress still graph-facts shard7 queue2of2, no unit marker/error in examined extents.

Sequential examined[0,9400), original identity unchanged. Latest shard7 queue2of3; no unit marker/error in examined extent.

Sequential examined[0,9400), same original identity; graph-facts shard7 wait remains latest stage, no completion/error marker examined.

Sequential examined[0,9400), same original identity; no unit completion/error marker examined. Latest stage remains shard7 wait.

Sequential examined[0,9400), same original identity; latest startup still shard7 wait, no unit completion/error marker examined.

Sequential examined[0,9400), same original log identity; latest shard7 queue2of3, no unit marker/error examined.

Sequential examined[0,9400), same original identity, latest shard7 wait. Exact acceptance requires one complete current marker ticket83/payload84/source76/phases19/oracles SQLite+Python hashlib+actual Python physical receiver plus actual owner0 and seven postpins; none inferred from startup.

Sequential examined[0,9400), same original identity; no unit marker/error observed. Latest graph-facts wait unchanged.

Sequential examined[0,9400), original identity unchanged. No unit completion/error marker examined; latest shard7 wait unchanged.

Sequential examined[0,9400), same original identity; no marker/error examined, graph-facts wait unchanged.

Sequential examined[0,9400), same original identity; no unit marker/error examined, latest shard7 wait unchanged.

Sequential examined[0,9400), same original identity; no unit marker/error observed, latest shard7 queue unchanged.

Sequential examined[0,9400), same original identity. Material transition: shard7 readiness wait followed by native image shared5of6. Separate static call-order audit establishes temporary shard release before image reopen; no unit marker/error examined.

Sequential examined[0,9400), same original identity; image shared5of6 is latest stage, no unit marker/error examined.

Concrete refusal in[259901,261348): Resource lease timed out: resource-directory-admission:v1 at acquireUngatedResourceLease process/leases/🟦️.ts228:63, Bun1.3.14. No own7 completion marker in full examined extent; awaiting owner actual terminal.

## Owner Confirmed Final Terminal

Root attributes Cargo actual TTY57283 exit1, no cancellation. Full frozen261348B/SHA25657a895535199bf04554c5c16101f09cd267bf0a226ea0b9f76b447f82fd05a89 independently matches. Full sequential extent[0,261348) examined, exact own7 markerCount0. Refusal is directory admission timeout228, not an own7 test result. No unit collection/pass claimed.

All seven postpins unchanged under current raw ownsealfacd93fe31f2ae08561312024e510590d6e0d16e91a80e7aea1dcc9aa948d9ac:

```json
{
  "bytes": 261348,
  "sha256": "57a895535199bf04554c5c16101f09cd267bf0a226ea0b9f76b447f82fd05a89",
  "markerCount": 0,
  "seal": "facd93fe31f2ae08561312024e510590d6e0d16e91a80e7aea1dcc9aa948d9ac",
  "post": [
    {
      "file": "schema.json",
      "bytes": 20432,
      "sha256": "eb30727ef0c11e6dc3d1ff3d58a53cfa792f42116997466e8fda59694b2c7f8c",
      "matches": true
    },
    {
      "file": "cases.json",
      "bytes": 7297,
      "sha256": "3d060f22f969b1601d886245ba01809b86e927030f7acf191f2d20d57f40a825",
      "matches": true
    },
    {
      "file": "core.ts",
      "bytes": 13502,
      "sha256": "d7632ccddb32435d9369dc8e84898bf23167a2224ed3eed33b17ef819a239653",
      "matches": true
    },
    {
      "file": "guest.py",
      "bytes": 20695,
      "sha256": "86f0957445bf5f5a13dba50059c01e56dde70d65355306a5a0d45ecbb9b97499",
      "matches": true
    },
    {
      "file": "entry.ts",
      "bytes": 11867,
      "sha256": "8f05146298e5b561aa964751810955208bb581686ff25b255da66f4d6ade1e1d",
      "matches": true
    },
    {
      "file": "tests.ts",
      "bytes": 8439,
      "sha256": "9bbaab5eeb85c304a411899b4b67271448e5fd667bff882b9b25ff8bf56c36ed",
      "matches": true
    },
    {
      "file": "oracle.py",
      "bytes": 3259,
      "sha256": "75fb2f0b74be2ee8d0095167667ac48362da07d6740d2795f0d37c22ee2dd823",
      "matches": true
    }
  ]
}
```

Watch ended; no live native job or TTY poll by this reviewer.

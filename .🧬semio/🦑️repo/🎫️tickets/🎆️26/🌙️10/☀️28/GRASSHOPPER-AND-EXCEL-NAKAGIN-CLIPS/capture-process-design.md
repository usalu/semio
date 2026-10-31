# Capture Worker Process Interface

The agreed Path + arguments API is saved and integration-ready. Three real-process tests passed in 14.648 seconds under the normal 2 GB / 2 CPU reservation. AJV independently accepted all three portable cases and rejected three invalid mutations.

The runtime helper launches the ticket worker through repo → uv → verify. A queue and daemon stdout pump keep source checkpoints running while the child is silent. Any BaseException delegates the owned tree to the repository first-party terminateOwnedProcessTree interface, with bounded waits and an audit of all initial descendants. No Popen context manager or platform-specific termination implementation was added.

The stopper checks the live root PID, exact parent PID, ticket marker, verify command, label token and process age through doctorPortsV1 before signalling. Both live foreign-parent requests were rejected without any signal; the same trees remained live until the authorized cleanup. A fresh independent doctor-port snapshot then proved the worker and grandchild PIDs absent. KeyboardInterrupt and a source-drift exception both retain their original exception types.

## Interface

run_worker(root, ticket, output, worker, label, checkpoint, *, arguments=(), interval=5.0)

Worker is a Python Path inside the ticket. Output must be inside its generated directory. Checkpoints receive label, phase, pid and elapsed_seconds. Launch and complete phases may add callbacks beyond the timed running events. The helper returns zero only after successful worker completion and a final checkpoint.

## Evidence

The schema-first missing-helper import failure was observed before implementation. The first actual tree run successfully cleaned both trees but the test reader used the Windows default encoding for UTF-8 JSON; that test defect was corrected explicitly. The final suite is green. No production model, renderer, kernel, native or existing capture driver was changed by this task.

Final owned-tree audit summaries:

```json
[
  {
    "label": "capture-test-keyboard",
    "initial": [
      43584,
      4024,
      55780,
      40772,
      19636,
      45488,
      55964,
      28824,
      50276,
      11480,
      37456,
      53872,
      27092
    ],
    "remaining": [],
    "success": true
  },
  {
    "label": "capture-test-source-drift",
    "initial": [
      1720,
      26400,
      24584,
      14868,
      11632,
      16224,
      56828,
      364,
      55520,
      51460,
      15312,
      43124,
      50564
    ],
    "remaining": [],
    "success": true
  }
]
```

Current implementation and test byte hashes:

```json
{
  "capture-worker-process.py": "e7bdce1946e93e0758e7559fc5563f3fe469955450cb8a767ef7269c71e1cee4",
  "stop-capture-worker.ts": "7c8589cc25e993699f0d6c6f81f89a98a814ad7617dd9a6d234f10abf6ec82a1",
  "capture-process-fixture.py": "02992ee7a8e5e83ba1f218095fb2f3eb270bc4818611fc8fd9c0be021cab82a1",
  "probe-capture-processes.ts": "b490cfd2c3e1c26e7976ef111222bcc5a1d7a42cbaa27dc6503c1e8d3cf64282",
  "test-capture-worker-process.py": "fa93b9bc73a4841449c24c7965010d976851ae3e0801aac55eb3817439d7ab3a",
  "capture-process-cases.json": "ddc9c6d35cb754b00c4bb0cc99592a034e76be04e8d1c73c6caf3504dcb9c261",
  "capture-process.schema.json": "ace30be8aff44bd8257ebbbbb8927d716a6d58b96fa182f537f6f63c704d1ca1",
  "validate-capture-process.cjs": "18fc1c12c4542e51317ed975e54926a47bbc2cc7b0e6cd0093684c0ce232c156"
}
```

The retained runtime result is generated/capture-process-tests/result.json. Actual OS termination has been exercised on Windows; POSIX process-group traversal remains delegated to the same existing cross-platform first-party implementation rather than reimplemented here.

The integrated controller found a reaped-child boundary: cleanup now returns immediately when poll() reports an exited child, preventing unnecessary adapter launch or a stale-PID signal. Both live-tree cleanup cases were rerun after this correction and remained green. Parent integration separately tests reaped-child failure and drift.

## Final Four-Test Process Gate

Root reran the completed helper after the explicit exited-root/held-pipe diagnostic was added. Four tests passed in 19.844 seconds; the repository command exited0. The added bounded pipe fixture fails explicitly with cleanup_verified=false and no stale root signal. Both actual live-root child/grandchild trees were terminated and independently checked absent. The final controller fixture suite also passed4 tests in0.724s, including all9workers, exact Pillow pixels and6 rejected child-envelope identities. Earlier3-test/e7bdce hash blocks above are historical. Current helper byte hash is170a8ec3b93e3ce0199291b24cc05eadf7cba5d9752acac9667efb5b71a7bb12.

The process result records counts rather than the complete rendering runtime envelope; the actual capture workers separately enforce their full declared source/runtime endpoints. Actual Windows OS cancellation is tested. POSIX behavior remains delegated to the existing first-party implementation.

```json
{
  "result": {
    "tests": 4,
    "success": true,
    "failures": 0,
    "errors": 0
  },
  "input_sha256_after_run": {
    "capture-worker-process.py": "170a8ec3b93e3ce0199291b24cc05eadf7cba5d9752acac9667efb5b71a7bb12",
    "stop-capture-worker.ts": "7c8589cc25e993699f0d6c6f81f89a98a814ad7617dd9a6d234f10abf6ec82a1",
    "capture-process-fixture.py": "02992ee7a8e5e83ba1f218095fb2f3eb270bc4818611fc8fd9c0be021cab82a1",
    "probe-capture-processes.ts": "b490cfd2c3e1c26e7976ef111222bcc5a1d7a42cbaa27dc6503c1e8d3cf64282",
    "test-capture-worker-process.py": "913358020b119706b3ff8a832d878f7ba30216f9cc822b3edbb79d7a170eb57b",
    "capture-process-cases.json": "b7938ffdd38776a54edaa4555c206759fd1e48bf8ba48c115f4805e7a76ee43f",
    "capture-process.schema.json": "5ccd9dc7440d01708db4eca5a0ebf8b3012e9fad66c6ec6d5ce11e86e9ff6b94",
    "validate-capture-process.cjs": "18fc1c12c4542e51317ed975e54926a47bbc2cc7b0e6cd0093684c0ce232c156"
  },
  "tree_outcomes": [
    {
      "success": false,
      "root_pid": 987654321,
      "parent_pid": 56052,
      "label": "capture-test-held-pipe",
      "reason": "exited-root-held-pipe",
      "remaining": null,
      "cleanup_verified": false,
      "initial_descendants": null,
      "signalled": null
    },
    {
      "success": false,
      "root_pid": null,
      "parent_pid": null,
      "label": null,
      "reason": null,
      "remaining": [],
      "cleanup_verified": null,
      "initial_descendants": [
        {
          "pid": 18956,
          "parent": 56052,
          "ageSeconds": 4,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.EXE tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 49388,
          "parent": 18956,
          "ageSeconds": 4,
          "rssMegabytes": 11,
          "cpuPercent": 0,
          "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
        },
        {
          "pid": 35016,
          "parent": 18956,
          "ageSeconds": 4,
          "rssMegabytes": 61,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 8508,
          "parent": 35016,
          "ageSeconds": 3,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 6512,
          "parent": 8508,
          "ageSeconds": 3,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 27708,
          "parent": 6512,
          "ageSeconds": 3,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 16588,
          "parent": 27708,
          "ageSeconds": 3,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 24752,
          "parent": 16588,
          "ageSeconds": 2,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 25780,
          "parent": 24752,
          "ageSeconds": 2,
          "rssMegabytes": 62,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 30404,
          "parent": 25780,
          "ageSeconds": 2,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 6200,
          "parent": 30404,
          "ageSeconds": 2,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 42812,
          "parent": 6200,
          "ageSeconds": 2,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 24588,
          "parent": 42812,
          "ageSeconds": 2,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        }
      ],
      "signalled": []
    },
    {
      "success": true,
      "root_pid": null,
      "parent_pid": null,
      "label": null,
      "reason": null,
      "remaining": [],
      "cleanup_verified": null,
      "initial_descendants": [
        {
          "pid": 18956,
          "parent": 56052,
          "ageSeconds": 5,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.EXE tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 49388,
          "parent": 18956,
          "ageSeconds": 5,
          "rssMegabytes": 11,
          "cpuPercent": 0,
          "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
        },
        {
          "pid": 35016,
          "parent": 18956,
          "ageSeconds": 5,
          "rssMegabytes": 61,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 8508,
          "parent": 35016,
          "ageSeconds": 5,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 6512,
          "parent": 8508,
          "ageSeconds": 5,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 27708,
          "parent": 6512,
          "ageSeconds": 5,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 16588,
          "parent": 27708,
          "ageSeconds": 5,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json --leaf-label capture-test-keyboard-leaf"
        },
        {
          "pid": 24752,
          "parent": 16588,
          "ageSeconds": 3,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 25780,
          "parent": 24752,
          "ageSeconds": 3,
          "rssMegabytes": 62,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 30404,
          "parent": 25780,
          "ageSeconds": 3,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 6200,
          "parent": 30404,
          "ageSeconds": 3,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 42812,
          "parent": 6200,
          "ageSeconds": 3,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        },
        {
          "pid": 24588,
          "parent": 42812,
          "ageSeconds": 3,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-keyboard-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\keyboard-leaf.json"
        }
      ],
      "signalled": [
        18956
      ]
    },
    {
      "success": false,
      "root_pid": null,
      "parent_pid": null,
      "label": null,
      "reason": null,
      "remaining": [],
      "cleanup_verified": null,
      "initial_descendants": [
        {
          "pid": 27624,
          "parent": 56052,
          "ageSeconds": 4,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.EXE tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 50588,
          "parent": 27624,
          "ageSeconds": 4,
          "rssMegabytes": 11,
          "cpuPercent": 0,
          "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
        },
        {
          "pid": 48236,
          "parent": 27624,
          "ageSeconds": 4,
          "rssMegabytes": 60,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 23888,
          "parent": 48236,
          "ageSeconds": 4,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 13552,
          "parent": 23888,
          "ageSeconds": 4,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 16704,
          "parent": 13552,
          "ageSeconds": 4,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 40540,
          "parent": 16704,
          "ageSeconds": 4,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 33608,
          "parent": 40540,
          "ageSeconds": 2,
          "rssMegabytes": 13,
          "cpuPercent": 0,
          "command": "repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 55508,
          "parent": 33608,
          "ageSeconds": 2,
          "rssMegabytes": 62,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 28684,
          "parent": 55508,
          "ageSeconds": 2,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 21492,
          "parent": 28684,
          "ageSeconds": 2,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 29828,
          "parent": 21492,
          "ageSeconds": 2,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 31724,
          "parent": 29828,
          "ageSeconds": 2,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        }
      ],
      "signalled": []
    },
    {
      "success": true,
      "root_pid": null,
      "parent_pid": null,
      "label": null,
      "reason": null,
      "remaining": [],
      "cleanup_verified": null,
      "initial_descendants": [
        {
          "pid": 27624,
          "parent": 56052,
          "ageSeconds": 5,
          "rssMegabytes": 12,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.EXE tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 50588,
          "parent": 27624,
          "ageSeconds": 5,
          "rssMegabytes": 11,
          "cpuPercent": 0,
          "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
        },
        {
          "pid": 48236,
          "parent": 27624,
          "ageSeconds": 5,
          "rssMegabytes": 60,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 23888,
          "parent": 48236,
          "ageSeconds": 5,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 13552,
          "parent": 23888,
          "ageSeconds": 5,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 16704,
          "parent": 13552,
          "ageSeconds": 5,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 40540,
          "parent": 16704,
          "ageSeconds": 5,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift --mode tree --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-worker.json --leaf-record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json --leaf-label capture-test-source-drift-leaf"
        },
        {
          "pid": 33608,
          "parent": 40540,
          "ageSeconds": 3,
          "rssMegabytes": 13,
          "cpuPercent": 0,
          "command": "repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 55508,
          "parent": 33608,
          "ageSeconds": 3,
          "rssMegabytes": 62,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 28684,
          "parent": 55508,
          "ageSeconds": 3,
          "rssMegabytes": 7,
          "cpuPercent": 0,
          "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 21492,
          "parent": 28684,
          "ageSeconds": 3,
          "rssMegabytes": 26,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 29828,
          "parent": 21492,
          "ageSeconds": 3,
          "rssMegabytes": 4,
          "cpuPercent": 0,
          "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        },
        {
          "pid": 31724,
          "parent": 29828,
          "ageSeconds": 3,
          "rssMegabytes": 104,
          "cpuPercent": 0,
          "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-process-fixture.py --label capture-test-source-drift-leaf --mode leaf --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\capture-process-tests\\source-drift-leaf.json"
        }
      ],
      "signalled": [
        27624
      ]
    }
  ]
}
```

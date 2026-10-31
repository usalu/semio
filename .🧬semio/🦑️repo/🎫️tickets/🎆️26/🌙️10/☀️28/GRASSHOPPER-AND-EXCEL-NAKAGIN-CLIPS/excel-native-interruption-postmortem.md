# Excel Native Interruption Postmortem

The original owned exec session 31115 became unavailable after this agent ended its turn at root's request to free an audit slot. The original process tree and command labels are absent in the subsequent first-party doctor facts. The last worker log is ordinary progress at 5001 seconds with no terminal result; the run is an incomplete non-pass and establishes no native pair success. Agent-turn completion and disappearance are temporally correlated, but no termination journal establishes a definitive mechanism. No cancellation, signal, pause or private-cache mutation was issued by this worker.

Root authorized replacement only after absence verification and preservation. The original contracts remain a terminal 19-test pass, with complete permanent evidence in final-native-excel-contracts-proof.md. All fixed-path Excel output directories were preserved under generated/historical-xl-0850 before replacement, including the complete contract result/checks and unfinished worker outputs. No other owner's files were moved.

The corrective coordination rule is to retain the owning agent turn and original exec handle through the entire terminal batch, without ending the agent turn to free an audit slot. A fresh 24 GB official reservation is justified by the original approximately 19 GB observed native peak and must be admitted normally.

```json
{
  "observed_at": "2026-10-30T08:53:57.621632+00:00",
  "old_session": 31115,
  "old_capacity_root": 55932,
  "old_grant": "81JvA8QUI2Aq3x560xLXug",
  "known_owned_pids_absent": [
    10524,
    10552,
    11852,
    28440,
    34052,
    37440,
    37448,
    42404,
    44708,
    45884,
    49484,
    53040,
    55932,
    56796
  ],
  "known_owned_command_labels_absent": [
    "final-native-excel",
    "nc-excel",
    "ns-excel-"
  ],
  "doctor_facts_at_ms": 1793350437592,
  "survivors": [],
  "preserved_directories": [
    "final-native-excel",
    "native-batch-excel",
    "nc-excel",
    "scenes-excel-all-en-de",
    "native-batch-excel-Beat1_Anatomy",
    "ns-excel-Beat1_Anatomy",
    "scenes-excel-Beat1_Anatomy-en"
  ],
  "worker_log_last_modified": "2026-10-30T08:50:18.458764+00:00",
  "worker_log_sha256": "8e0de8ca64a41328c6783ffd1593cf9cc393aed8bed31edd6cf57fa2f147f4fe",
  "worker_log_tail": "ast phase started: boot.loaded\n[repo] still working after 4820 s; last phase started: boot.loaded\n[repo] still working after 4850 s; last phase started: boot.loaded\n[repo] still working after 4880 s; last phase started: boot.loaded\n[repo] still working after 4910 s; last phase started: boot.loaded\n[repo] still working after 4940 s; last phase started: boot.loaded\n[repo] still working after 4970 s; last phase started: boot.loaded\n[repo] still working after 5001 s; last phase started: boot.loaded\n",
  "contract_result_sha256": "65803db26ccec83de8fa7c142a93b63c257d6a323b2556b08f6ee7f3c5967b97",
  "native_pair_result_exists": false,
  "batch_result_exists": false
}
```

## Complete First-Party Process Facts At Absence Verification

```json
{
  "version": 1,
  "atMs": 1793350437592,
  "platform": "win32",
  "probes": [
    {
      "area": "leases",
      "status": "ok",
      "durationMs": 12
    },
    {
      "area": "capacity",
      "status": "ok",
      "durationMs": 8
    },
    {
      "area": "processes",
      "status": "ok",
      "durationMs": 390
    }
  ],
  "leases": {
    "holders": [],
    "waiters": [],
    "cycles": [],
    "violations": [],
    "registry": {
      "folders": 2,
      "orphanFolders": 0,
      "deadEntries": 0,
      "liveEntries": 0
    },
    "deadTickets": 0,
    "deadHolders": []
  },
  "host": {
    "cores": 16,
    "totalMegabytes": 65447,
    "availableMegabytes": 32584,
    "floorMegabytes": 4096,
    "pressure": "normal",
    "swapUsedMegabytes": 3011,
    "swapOutMegabytesPerSecond": 0,
    "load": null,
    "openFiles": null,
    "openFilesMaximum": null,
    "attachedImages": null,
    "governor": "on"
  },
  "capacity": {
    "headroomMegabytes": 22344,
    "grants": [
      {
        "operation": "command",
        "pid": 12384,
        "jobs": 1,
        "reservedMegabytes": 16384,
        "observedMegabytes": 2120,
        "heldSeconds": 5737
      },
      {
        "operation": "command",
        "pid": 40200,
        "jobs": 1,
        "reservedMegabytes": 16384,
        "observedMegabytes": 11712,
        "heldSeconds": 5639
      },
      {
        "operation": "dependencies",
        "pid": 27468,
        "jobs": 1,
        "reservedMegabytes": 6144,
        "observedMegabytes": null,
        "heldSeconds": 0
      }
    ],
    "waiters": []
  },
  "processes": [
    {
      "pid": 1072,
      "parent": 25376,
      "ageSeconds": 5640,
      "rssMegabytes": 15,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 2696,
      "parent": 31860,
      "ageSeconds": 5640,
      "rssMegabytes": 36,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 3444,
      "parent": 46876,
      "ageSeconds": 5738,
      "rssMegabytes": 3,
      "cpuPercent": 0,
      "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 4740,
      "parent": 35384,
      "ageSeconds": 0,
      "rssMegabytes": 61,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo doctor --only=processes --facts"
    },
    {
      "pid": 4844,
      "parent": 26980,
      "ageSeconds": 853,
      "rssMegabytes": 11,
      "cpuPercent": 0,
      "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
    },
    {
      "pid": 10440,
      "parent": 31972,
      "ageSeconds": 0,
      "rssMegabytes": 8,
      "cpuPercent": 0,
      "command": "\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4"
    },
    {
      "pid": 12128,
      "parent": 51036,
      "ageSeconds": 0,
      "rssMegabytes": 16,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -X utf8 -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/preserve-excel-interruption.py"
    },
    {
      "pid": 12384,
      "parent": 13936,
      "ageSeconds": 5738,
      "rssMegabytes": 40,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo capacity run --memory=16384 --cpu=2 -- repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 13936,
      "parent": 10796,
      "ageSeconds": 5738,
      "rssMegabytes": 9,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.exe\" capacity run --memory=16384 --cpu=2 -- repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 15588,
      "parent": 29040,
      "ageSeconds": 5392,
      "rssMegabytes": 3,
      "cpuPercent": 0,
      "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 15884,
      "parent": 51524,
      "ageSeconds": 852,
      "rssMegabytes": 1963,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 23684,
      "parent": 33300,
      "ageSeconds": 5393,
      "rssMegabytes": 36,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 25328,
      "parent": 38368,
      "ageSeconds": 853,
      "rssMegabytes": 7,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 25376,
      "parent": 2696,
      "ageSeconds": 5640,
      "rssMegabytes": 6,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 26980,
      "parent": 43056,
      "ageSeconds": 853,
      "rssMegabytes": 12,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.EXE tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 27468,
      "parent": 58096,
      "ageSeconds": 0,
      "rssMegabytes": 67,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec --light uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -X utf8 -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/preserve-excel-interruption.py"
    },
    {
      "pid": 28572,
      "parent": 25328,
      "ageSeconds": 853,
      "rssMegabytes": 25,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 29040,
      "parent": 36004,
      "ageSeconds": 5392,
      "rssMegabytes": 15,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 29528,
      "parent": 27468,
      "ageSeconds": 0,
      "rssMegabytes": 7,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -X utf8 -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/preserve-excel-interruption.py"
    },
    {
      "pid": 31860,
      "parent": 40200,
      "ageSeconds": 5640,
      "rssMegabytes": 9,
      "cpuPercent": 0,
      "command": "repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 31972,
      "parent": 4740,
      "ageSeconds": 0,
      "rssMegabytes": 73,
      "cpuPercent": 0,
      "command": "powershell -NoProfile -NonInteractive -Command \"Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,CreationDate,WorkingSetSize,CommandLine,Name | ConvertTo-Json -Compress\""
    },
    {
      "pid": 33300,
      "parent": 47880,
      "ageSeconds": 5393,
      "rssMegabytes": 9,
      "cpuPercent": 0,
      "command": "repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 35384,
      "parent": 12128,
      "ageSeconds": 0,
      "rssMegabytes": 12,
      "cpuPercent": 0,
      "command": "repo doctor --only=processes --facts"
    },
    {
      "pid": 36004,
      "parent": 23684,
      "ageSeconds": 5392,
      "rssMegabytes": 6,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 38368,
      "parent": 26980,
      "ageSeconds": 853,
      "rssMegabytes": 56,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe --no-project python -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 39916,
      "parent": 12384,
      "ageSeconds": 5738,
      "rssMegabytes": 9,
      "cpuPercent": 0,
      "command": "repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 40200,
      "parent": 43388,
      "ageSeconds": 5640,
      "rssMegabytes": 40,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo capacity run --memory=16384 --cpu=2 -- repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 41932,
      "parent": 15588,
      "ageSeconds": 5392,
      "rssMegabytes": 11778,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\replay-native-pairs.py --label ns-grasshopper-Beat1_Anatomy --series grasshopper --worker-scene Beat1_Anatomy"
    },
    {
      "pid": 43056,
      "parent": 3444,
      "ageSeconds": 5738,
      "rssMegabytes": 123,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 43388,
      "parent": 39920,
      "ageSeconds": 5640,
      "rssMegabytes": 9,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Local\\Semio\\repo\\aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79\\repo.exe\" capacity run --memory=16384 --cpu=2 -- repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 45928,
      "parent": 39916,
      "ageSeconds": 5738,
      "rssMegabytes": 36,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\AppData\\Local\\semio\\runtimes\\bun\\78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6\\bun.exe C:\\git\\semio\\???framework\\???products\\???repo\\???modules\\???library\\???build\\???door\\???script.ts repo tools exec uv -- run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 46276,
      "parent": 1072,
      "ageSeconds": 5640,
      "rssMegabytes": 3,
      "cpuPercent": 0,
      "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 46876,
      "parent": 55692,
      "ageSeconds": 5738,
      "rssMegabytes": 15,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    },
    {
      "pid": 47880,
      "parent": 46276,
      "ageSeconds": 5640,
      "rssMegabytes": 31,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\AppData\\Roaming\\uv\\python\\cpython-3.12-windows-x86_64-none\\python.exe\" -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/replay-native-pairs.py --label final-native-grasshopper --series grasshopper"
    },
    {
      "pid": 48884,
      "parent": 29528,
      "ageSeconds": 0,
      "rssMegabytes": 25,
      "cpuPercent": 0,
      "command": "\"C:\\Users\\Ueli\\scoop\\apps\\uv\\current\\uv.exe\"  run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -X utf8 -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/preserve-excel-interruption.py"
    },
    {
      "pid": 51036,
      "parent": 48884,
      "ageSeconds": 0,
      "rssMegabytes": 4,
      "cpuPercent": 0,
      "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -X utf8 -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/preserve-excel-interruption.py"
    },
    {
      "pid": 51524,
      "parent": 28572,
      "ageSeconds": 852,
      "rssMegabytes": 4,
      "cpuPercent": 0,
      "command": "\"C:\\git\\semio\\.??semio\\???teaching\\??cache\\python\\Scripts\\python.exe\" -B C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\verify.py .??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\capture-worker.py --label fc-direction1 --target visual-audit-gallery.py --baseline C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\start.json --record C:\\git\\semio\\.??semio\\???repo\\???tickets\\???26\\???10\\??28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\???generated\\final-visual-capture-fit\\child-direction1 -- --direction-index 1"
    },
    {
      "pid": 55692,
      "parent": 45928,
      "ageSeconds": 5738,
      "rssMegabytes": 6,
      "cpuPercent": 0,
      "command": "C:\\Users\\Ueli\\scoop\\shims\\uv.exe run --python C:/git/semio/.??semio/???teaching/??cache/python/Scripts/python.exe --no-project python -B C:/git/semio/.??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify.py .??semio/???repo/???tickets/???26/???10/??28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-final-visuals.py --label final-visual-capture-fit --output-name final-visual-capture-fit --mode capture"
    }
  ]
}
```

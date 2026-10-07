# Runtime Output Ownership Diagnosis

Read-only current snapshot:10,483,724KiB free (~10GiB); no Cargo/rustc/native test process visible. Three unrelated native owner wrappers JPG/BMP/PPTX remain alive and are preserved. No Actor or producer command was active during diagnosis.

Exact failed Actor LLVM receipt references compiler unit `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-os-kernel/c01f20880628b79a/out` in current-runtime-member-open-native-qualified-final.log. This is the canonical shared compiler-unit root, not ticket-owned scratch. Other kernel tests and registry builds use the same native owner and dependencies, and a command receipt establishes participation rather than exclusive ownership. Neither that unit nor shared dependency outputs are safe deletion candidates on our evidence. Successful Stdio receipt identifies no separate exclusively owned output beyond its already retained ticket logs.

Lease implementation retains immutable resource identity in SQLite and holds live transaction locks; queue tickets encode PID+owner. Merely finding a lease file or timestamp cannot establish a unit is unused or exclusively ours. Diagnosis was read-only; no lease initialization, cleanup, cache deletion, profile change or unrelated process stop occurred.

Actionable safe outputscope beyond previously acknowledged ticket generated trees: NONE established. Root observed recovered headroom and directed no further reclamation. Next authorized operations: canonical Actor exactlaw, then producer6laws, serialized, with500MiB guard.

Actual read-only preparation queue snapshot:

```json
{
  "resource": "cargo-preparation:/Users/ueli/Documents/semio",
  "queue": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/agents/resource-leases/d66997d14952d15930b33359e79fa513dc13f03f2c1ee6edeb22c3afedac951b.queue",
  "tickets": [
    {
      "ticket": "001791316824013-0000024738-3c158868-9cc3-46bf-bb60-9dcf241dcea5",
      "pid": 24738,
      "alive": true
    },
    {
      "ticket": "001791316832580-0000024850-c309f792-b7bf-425f-b573-b0767c5aece2",
      "pid": 24850,
      "alive": true
    },
    {
      "ticket": "001791316832686-0000024853-23ac934a-aba2-4beb-b98a-3bea061d72f8",
      "pid": 24853,
      "alive": true
    },
    {
      "ticket": "001791316833399-0000024860-71aa683b-b56e-4907-80ab-d746c18db8db",
      "pid": 24860,
      "alive": true
    },
    {
      "ticket": "001791316837177-0000024969-b9bc167b-e8ad-409e-b5c7-3ad9e210c9c3",
      "pid": 24969,
      "alive": true
    },
    {
      "ticket": "001791316845834-0000025080-9a1f40d0-9ae1-408f-bd31-460d5acf4f8a",
      "pid": 25080,
      "alive": true
    },
    {
      "ticket": "001791316918507-0000025694-00bb0c31-9005-4403-97ba-130bfd9f8211",
      "pid": 25694,
      "alive": true
    },
    {
      "ticket": "001791316927062-0000025778-6a8fc792-8696-4c8e-808d-55432fd9ebc0",
      "pid": 25778,
      "alive": true
    },
    {
      "ticket": "001791316927339-0000025779-6c13c821-52e6-4f64-85a5-a852017917de",
      "pid": 25779,
      "alive": true
    },
    {
      "ticket": "001791316928339-0000025790-350a21a0-7d7e-4345-9d1e-142fdde788fd",
      "pid": 25790,
      "alive": true
    },
    {
      "ticket": "001791316928379-0000025791-01b5bf70-2bf5-41c1-b7c6-bf90c6564497",
      "pid": 25791,
      "alive": true
    },
    {
      "ticket": "001791316928629-0000025797-05862475-a8b4-45fc-a979-91835aa71280",
      "pid": 25797,
      "alive": true
    },
    {
      "ticket": "001791316928673-0000025799-00fbaaea-6ccd-4134-8add-5f06067499c4",
      "pid": 25799,
      "alive": true
    },
    {
      "ticket": "001791316928704-0000025795-c3d0199c-fa53-4980-82eb-34267cdb1288",
      "pid": 25795,
      "alive": true
    },
    {
      "ticket": "001791316928855-0000025800-23a6a4ff-eaad-451c-a7a0-2c654cb5ecfc",
      "pid": 25800,
      "alive": true
    },
    {
      "ticket": "001791316928975-0000025801-21a64b08-f03d-408d-8c65-ec9b67e7ccc7",
      "pid": 25801,
      "alive": true
    },
    {
      "ticket": "001791316929015-0000025802-9c8fc6f3-c4f4-40f4-ae85-2639cc357ce8",
      "pid": 25802,
      "alive": true
    },
    {
      "ticket": "001791316929076-0000025803-02df3a1f-c448-4e1a-8bea-8e891ea62475",
      "pid": 25803,
      "alive": true
    },
    {
      "ticket": "001791316937293-0000025882-db3cd064-832c-440c-bdca-c49bd2d59b65",
      "pid": 25882,
      "alive": true
    },
    {
      "ticket": "001791316969216-0000026168-07706738-fcb5-4e44-bdf6-9e8ee95dc1ca",
      "pid": 26168,
      "alive": true
    },
    {
      "ticket": "001791316978638-0000026232-83100934-0eaa-4400-8b2b-3d9b0259e28b",
      "pid": 26232,
      "alive": true
    }
  ]
}
```
No queue ticket or SQLite lease was changed.

Corrected-source producer retry cancelled under500MiB guard: free headroomKiB5753116,5643012,5522012,5409360,4399976,935684,952056,636328,621476,568596,568844,538576,169400. At169400KiB sent only owned terminal SIGINT; exit130 after6m30s, remained in preparation, no native compiler/assertion verdict emitted. Post-cancellation no owned Nx/kernel/preparation descendants, headroom983640KiB. Full terminal receipt retained at generated/current-runtime-dsl-producer-native-retry.log. Canonical profile/source checks unchanged; prior3reported errors already repaired, but no producer6law native pass is claimed. Other owners were not stopped and no shared output/cache was deleted. Queue terminal; further native launch awaits root headroom/order.

Producer confirmation attempt also cancelled under500MiB guard: canonical source/profile unchanged; fresh atlaunch1505972KiB after priorsettling2563908KiB snapshot, then1751504,1168320,223796KiB. At223796KiB sent only owned terminal SIGINT; exit130 after2m24s preparation, no native compiler/assertion verdict. Postsignal no owned Nx/kernel/preparation descendants; headroom150568KiB. Full terminal receipt retained at generated/current-runtime-dsl-producer-native-confirm.log. Otherowners untouched, no shared Cargo/Nx/output pruning; inputs/preimages/reports preserved. Corrected producer6laws still unexecuted. Heavyqueue idle pending rootorder and recoveredheadroom.

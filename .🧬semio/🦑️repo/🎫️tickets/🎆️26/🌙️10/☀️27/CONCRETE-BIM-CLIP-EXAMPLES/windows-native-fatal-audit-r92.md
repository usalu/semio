# Windows Native Lifecycle Fatal Audit

Read-only evidence folder: generated/native-r92/p/20261030003938561. No experiments, process controls, dependency changes or production edits.

## What the retained evidence actually proves

Outer raw status5, signalnull, duration240.735s; faulthandler explicitly records Windows fatal exception: access violation. This is not a unittest assertion or declared900s/5.5GiB limit. {"events":199,"kinds":["start","sample"],"phases":["starting"],"max_rss":1002680320,"max_private":2028310528,"last_elapsed":239.64100000000326} . All observed phases remain starting; no beat-start event exists. Therefore no evidence places this failure in the later interpolation/Cairo capture loops.

The fourth60s timeout dump is interleaved with the fatal dump. Timeout sample points at installed Manim mobject/mobject.py _insert_submobjects with line???; fatal dump ends at installed manim/utils/color/core.py _internal_from_rgb with line???. It does NOT retain the complete fatal caller chain. Do not fuse the two fragments into a proven function-to-function callchain or claim an exact source object from them.

## Native leaf implementation

Current installed Manim0.22.0 core.py:363ff allocates `np.asarray(rgb,dtype=ManimColorDType).copy()`, calls `value.resize(4,refcheck=False)`, assigns alpha and returns. This is NumPy-native allocation/resize reached through Python Manim; the copy is a new owned array, so refcheck=False alone is not proof of resizing shared/borrowed storage. An access violation there can reveal an earlier native failure, but no earlier corrupting write is identified by this trace. Current NumPy2.5.4 satisfies installed Manim numpy>=2.1 metadata and Python3.12 satisfies both; package compatibility by constraints is not a native-safety certificate. These versions are current disk inspection, not an archived per-run loaded-module binary manifest.

Current installed mobject.py:482 _insert_submobjects validates children, deduplicates a Python sequence and performs Python list replacement/pop/insert. It has no ctypes raw-memory write. Its timeout frame is not itself proof of a corrupting list operation.

## Recoverable first-party callers and timing limits

Retained timed snapshots recover test_native_lifecycle→scene.construct→ClipScene.present→present_objects→_facts→building_geometry; geometry runs first and its earlier sampled work included _filling_structures JSON serialization and _service occupied-volume integration. After _facts, present_objects creates text then invokes _schema. Before the first step(schema), _schema builds class cards/text, a complete production villa_axon, count/GUID labels, recoloured copied overlays and animation objects. All are possible pre-step factories. The fatal dump does not distinguish which one.

Actual factory branch reviewed: villa_axon→_Painter.render→_poly→drafting.polygon/region→VMobject/set_points/set_fill/set_stroke. Polygon helpers allocate fresh point arrays through P→np.array. Axon caches pure geometry primitives, while painter allocates new VGroups/VMobjects per call. `_overlay` explicitly copies before recolouring. No concrete cached-Manim mutation or NumPy borrowed-buffer resize was found in these reviewed branches. Drawing groups share child references for selection/layer aliases in one scene; that intentional graph alias is not a demonstrated native dangling pointer.

## Memory observer audit

Ticket native-lifecycle-probe-r92.py memory() defines PROCESS_MEMORY_COUNTERS_EX with2DWORD+9SIZE_T fields: Win64 layout80bytes, cb80, correct field order through PrivateUsage. GetCurrentProcess has HANDLE restype; GetProcessMemoryInfo receives HANDLE,voidpointer,DWORDcb and default C int return matches BOOL. Buffer uses ctypes.byref live structure. No structure-size/return-width mismatch or write-past-buffer is established. Explicit restype/argtypes for every call would improve interface clarity but is not an evidence-based crash fix.

Observer thread only waits, samples process counters, writes JSON events and checks its own bounds/cancel marker. Its retained stack is waiting, not mutating Manim objects. faulthandler timeout is a native watchdog, so additional internal threads are possible; the supplied stack is not a full process native-thread census. No shared first-party renderer thread was identified.

## Bounded next action for implementation owner

Preserve fatal as unresolved native failure, not a passed lifecycle or memory-limit cancellation. If the owner adds diagnostics, instrument pre-first-step factories and retain loaded binary/version hashes and current operation/shape cardinalities before native allocations; do not change geometry or waive animation/Cairo acceptance. Existing evidence cannot justify blaming observer ctypes, shared primitive cache, hardware, or any one native extension. No new reproduction was initiated in this audit lane.

PeakWorkingSet counter max=1028210688bytes; max sampled currentWorkingSet=1002680320bytes. These are different counters; neither reaches declared own-process limit. LastPrivateUsage=2004807680bytes.

#!/usr/bin/env python3
"""🔁️ Baseline toggle for the `boxed_fixed_slots` lane — reverses/re-applies the converted slot
tables by exact string replacement, never a file snapshot, so a concurrent peer's unrelated edits in
the same files survive.

`off` restores the pre-fix inline/stack-materialised form so the per-crate guards can be shown to
fail (and the crate's other failures compared against a true baseline); `on` restores the heap-first
form. Diagnostic only — nothing here ships.

Usage: boxed-slots-baseline-toggle.py on|off [ui|renderer|infinite|all]
"""
import io, sys

ENGINE = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/⚙️engine.rs"
RENDERER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
SHELL = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
CANVAS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"
INTERPRETER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
WORLD = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs"
BOARD = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs"
ICONS = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs"
INFINITE_CANVAS = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs"

HEAP = "semio_framework_async::boxed_fixed_slots"
LANES = {
    "ui": [
        (ENGINE, "    slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>,",
                 "    slots: [Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS],"),
        (ENGINE, f"Self {{ slots: {HEAP}(|| None), generations: [0; UI_LAYOUT_SURFACE_SLOTS] }}",
                 "Self { slots: std::array::from_fn(|_| None), generations: [0; UI_LAYOUT_SURFACE_SLOTS] }"),
    ],
    "renderer": [
        (RENDERER, "        slots: Box<[Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY]>,",
                   "        slots: [Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY],"),
        (RENDERER, f"            Self {{ slots: {HEAP}(|| None), next_generation: 0, generation_exhausted: false, close_cursor: 0 }}",
                   "            Self { slots: std::array::from_fn(|_| None), next_generation: 0, generation_exhausted: false, close_cursor: 0 }"),
        (RENDERER, "        slots: Box<[Option<CommandDocumentRetirementState>; COMMAND_DOCUMENT_RETIREMENT_CAPACITY]>,",
                   "        slots: [Option<CommandDocumentRetirementState>; COMMAND_DOCUMENT_RETIREMENT_CAPACITY],"),
        (RENDERER, f"            Self {{ slots: {HEAP}(|| None), epochs: [0; COMMAND_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }}",
                   "            Self { slots: std::array::from_fn(|_| None), epochs: [0; COMMAND_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }"),
        (RENDERER, "        pages: Mutex<Box<[Option<MountedTypedOperationResultPage>; MOUNTED_TYPED_OPERATION_RESULT_PAGES]>>,",
                   "        pages: Mutex<[Option<MountedTypedOperationResultPage>; MOUNTED_TYPED_OPERATION_RESULT_PAGES]>,"),
        (RENDERER, f"            Self {{ pages: Mutex::new({HEAP}(|| None)) }}",
                   "            Self { pages: Mutex::new(std::array::from_fn(|_| None)) }"),
        (RENDERER, "        slots: Box<[Option<PendingSurfaceRejection>; RETAINED_SURFACE_CAPACITY]>,",
                   "        slots: [Option<PendingSurfaceRejection>; RETAINED_SURFACE_CAPACITY],"),
        (RENDERER, f"            Self {{ slots: {HEAP}(|| None), close_cursor: 0 }}",
                   "            Self { slots: std::array::from_fn(|_| None), close_cursor: 0 }"),
        (RENDERER, "        slots: Box<[Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY]>,",
                   "        slots: [Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY],"),
        (SHELL, "    slots: Box<[Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY]>,",
                "    slots: [Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY],"),
        (SHELL, f"        Self {{ slots: {HEAP}(|| None), epochs: [0; SHELL_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }}",
                "        Self { slots: std::array::from_fn(|_| None), epochs: [0; SHELL_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }"),
        (CANVAS, f"Self {{ slots: {HEAP}(|| None), len: 0, faulted: false }}",
                 "Self { slots: Box::new([const { None }; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]), len: 0, faulted: false }"),
        (INTERPRETER, f"Self {{ slots: {HEAP}(|| None), head: 0, len: 0, next_generation: 1, retiring: None }}",
                      "Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, next_generation: 1, retiring: None }"),
        (RENDERER, f"Self {{ slots: {HEAP}(|| None), len: 0 }}",
                   "Self { slots: Box::new([const { None }; FRAME_ENGINE_PACKET_CAPACITY]), len: 0 }"),
    ],
    "infinite": [
        (WORLD, f"Self {{ slots: {HEAP}(|| None), epochs: {HEAP}(|| 0), faulted: false }}",
                "Self { slots: Box::new([None; WORLD_INTERACTION_MESH_CAPACITY]), epochs: Box::new([0; WORLD_INTERACTION_MESH_CAPACITY]), faulted: false }"),
        (BOARD, f"Self {{ slots: {HEAP}(|| None), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }}",
                "Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }"),
        (ICONS, f"Self {{ slots: {HEAP}(|| IconPaintSlot {{ key: None, epoch: 0, generation: 0, value: None }}), epoch: 1, faulted: false }}",
                "Self { slots: Box::new(std::array::from_fn(|_| IconPaintSlot { key: None, epoch: 0, generation: 0, value: None })), epoch: 1, faulted: false }"),
        (INFINITE_CANVAS, f"Self {{ slots: {HEAP}(|| OpaqueSceneRetirementSlot {{ generation: 0, occupied: false, credited_bytes: 0, scene: None, command: None, command_backing_bytes: 0, command_credited_bytes: 0 }}), faulted: false }}",
                          "Self { slots: Box::new(std::array::from_fn(|_| OpaqueSceneRetirementSlot { generation: 0, occupied: false, credited_bytes: 0, scene: None, command: None, command_backing_bytes: 0, command_credited_bytes: 0 })), faulted: false }"),
    ],
}

mode = sys.argv[1]
lanes = LANES if len(sys.argv) < 3 or sys.argv[2] == "all" else {sys.argv[2]: LANES[sys.argv[2]]}
for lane, pairs in lanes.items():
    for path, fixed, legacy in pairs:
        text = io.open(path, encoding="utf-8").read()
        source, target = (fixed, legacy) if mode == "off" else (legacy, fixed)
        assert text.count(source) == 1, (lane, path, source[:70], text.count(source))
        io.open(path, "w", encoding="utf-8").write(text.replace(source, target))
print(f"toggled {mode}: {', '.join(lanes)}")

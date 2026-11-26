# Presentation Member WASI Compile Admission

The canonical Demonstrator and Process run `wUc5aA5` compiled the presentation member at 15:50 PST on November 25. Cargo reported a fresh compilation of `libsemio_s_artifact_stdio_semio_presentation_member-82e44e772eed812a.rlib` for `wasm32-wasip2` release. The compiler owner recorded 1,611 MiB peak RSS and 39.477 seconds. Admission had forecast 2,944 MB, with observed host availability of 4,927–5,666 MB before the 2,458 MB margin and other active charges. The longest recorded wait was 247 seconds.

The admitted Cargo unit graph records these shortest paths:

- Demonstrator WASI component → Demonstrator router → Process 3D editor → Stdio Semio artifact → Presentation member.
- Process WASI component → Process hub library → Stdio Semio artifact → Presentation member.

The Stdio Semio artifact is the only immediate consumer. Its member enum holds the presentation subset's typed store. The member crate consists of one 14-line `space_members!` invocation that instantiates the presentation snapshot, mutation and store producer. It has no crate features or direct registry dependencies. Its selected dependencies are the framework artifact reference, job and value crates, the OS kernel, the Stdio Semio base and the presentation schema. Cargo unifies the OS kernel's default, deflate and mutation-testing features. The transitive registry closure supplies serialization, WIT processing and macro tooling. These observations identify no standalone presentation-specific registry feature to remove.

The same run has progressed past the presentation member. At 40.1 minutes it had compiled 1,087 units, and the neighboring BREP member was still waiting. The BREP member's earlier recorded peak was 1,526 MiB, with 2,769 MB estimated for admission. The guarded native queue is already the working local route. This investigation made no compiler or source changes.

The run's evidence is in `.cache/b/r/m/components/runs/wUc5aA5/l/unit-graph-release-00.json`, adjacent `cargo-release-00.jsonl`, and compiler-owner timing and peak records under `.cache/b/r/m/u/c/`. The ticket-generated pair log records the admission waits.

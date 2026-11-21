# House world scene reconciliation

The native harness reads the published House DSL. It compiles the current FEM 3D scene producer and links the archived scene producer separately so both execute against the same native model, world-scene carrier, UI arena and production reconciliation cursor. It assembles the Model body and the pending Results caption and scene using the same builders, default cameras, interaction state and surface keys as the window code. The pending case has no ready numerical frame or visual lease.

The harness records the serialized mesh and instance byte lengths, the renderer's charged items and bytes, the surface limits and the time for scene construction and each reconciliation. An optional Demo input exercises the same surface shadow across two generations, first publishing Demo and then replacing it with House.

In the archived browser run, Demo-to-House accepted the typed operation and switched the picker to House. Both reconciliation cursors refused the first charge above the 4,097-item ceiling. At 4,098 items, Model had charged 7,024,433 bytes and Results had charged 7,061,525 bytes. Those measurements were taken at refusal. [Browser activation verification](./📓️activation-verification.md)

## Native cold House

On the same native cursor, indexed House passed both surfaces and the archived producer reproduced their item-credit refusal. The default limits were 128 nodes, 4,097 items and 8,388,608 bytes per surface. Archived charges are the first rejected charge.

| Scene producer | Surface | Outcome | Nodes | Items | Charged bytes | Cursor turns | Cursor time |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| Indexed | Model | Complete | 23 | 1,715 | 3,355,543 | 4,156 | 6.669 ms |
| Indexed | Pending Results | Complete | 26 | 1,755 | 3,494,031 | 4,372 | 5.488 ms |
| Archived | Model | Item refusal | 57 | 4,098 | 7,100,817 | 5,334 | 3.616 ms |
| Archived | Pending Results | Item refusal | 60 | 4,098 | 7,143,617 | 5,232 | 4.355 ms |

The indexed mesh JSON used 230,843 bytes and built in 0.523 s. The archive produced 799,928 mesh bytes in 1.323 s. Both produced 28,199 instance JSON bytes. The cursor's detached whole-tree item estimates were 1,945 and 1,992 for indexed Model and Results and 5,927 and 5,974 for their archived counterparts. The native link finished and the measurement process exited successfully.

## Native Demo-to-House transition

Both indexed surfaces sealed Demo's first shadow, accepted House as the next generation, and remained below the same limits. The archived path first sealed Demo, then reproduced the item-limit refusal when replacing it with House.

| Scene producer | Surface | Outcome | Nodes | Items | Charged bytes | Cursor turns | Cursor time |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| Indexed | Model | Complete | 23 | 1,710 | 3,319,911 | 4,179 | 7.665 ms |
| Indexed | Pending Results | Complete | 26 | 1,747 | 3,436,967 | 4,395 | 10.253 ms |
| Archived | Model | Item refusal | 57 | 4,098 | 7,100,817 | 5,338 | 3.668 ms |
| Archived | Pending Results | Item refusal | 60 | 4,098 | 7,139,533 | 5,234 | 3.572 ms |

In this transition run the indexed House scene built in 0.447 s and the archived scene built in 1.451 s. The recorded cursor times start after world-scene construction and cover only reconciliation, through completion or the first rejected charge. Indexed surfaces then sealed and retired successfully.

Run the [native build input](./house-scene-native.py) through `repo tools exec python -- <ticket>/house-scene-native.py` from the workspace root. The source is [house-scene-native.rs](./house-scene-native.rs). The archived native libraries default to `E:/b/fem3d-rcpt/probe-flat/lib` and can be overridden with `FEM3D_HOUSE_ARCHIVED_LIBRARIES`. The corresponding full metadata defaults to the ticket's generated `m` folder and can be overridden with `FEM3D_HOUSE_ARCHIVED_METADATA`. The generated executable accepts the published House DSL path followed by the optional Demo DSL path.

# Wave 6 consumer rename inventory

| File | Symbol usage |
|---|---|
| ✏️s/.../📐️brep/⚙️engine/🖥️host/🦀️component.rs | BrepkitKernel type |
| ✏️s/.../📐️brep/🧰️kernel/🦀️component.rs | struct BrepkitKernel |
| 🧰️framework/.../flow/.../📐️brep/🦀️component.rs | BrepkitKernel::new |
| ✏️s/🔌️plugins/📐️cad/.../⚙️engine/🦀️component.rs | cad_brep_kernel Mutex<BrepkitKernel> |
| ✏️s/🔌️plugins/🏭️process/.../process3d/⚙️engine/🦀️component.rs | host.kernel() |
| 🧰️framework/.../os/🦀️component.rs | SolidExporter/Importer on BrepkitKernel |
| 🧰️framework/.../os/🖥️host/🦀️component.rs | same |
| ✏️s/🔌️plugins/🎪️demonstrator/.../koordinator/🦀️component.rs | register codecs |
| ✏️s/🔌️plugins/💠️lowpoly/.../media test | BrepkitKernel |
| ✏️s/.../3d/.../benches/kernel.rs | BrepkitKernel |

Rename type to `Brep` (or keep `BrepkitKernel` as type alias deprecated — prefer clean rename `Brep`).

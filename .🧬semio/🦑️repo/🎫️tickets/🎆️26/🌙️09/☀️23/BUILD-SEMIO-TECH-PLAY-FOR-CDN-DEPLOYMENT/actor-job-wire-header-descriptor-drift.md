# Actor Job wire header descriptor drift

Descriptor generation `wuONgiG` reported E0599 for the `MAGIC` and `HEADER_BYTES` accesses in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/💼️job.rs`. Both the Actor frame source and Reactor input changed during that compile.

The live consumer’s Cargo path selects `semio-framework-actor`. The Actor library shim exposes its owner, which reexports `OriginalJobSourceWireHeader` from its direct `semio-framework-actor-retained-turn` dependency. The retained-turn source mounts the wire transport and the transport unconditionally reexports the canonical type from `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📦️transport/📥️source/📦️frame/🦀️.rs`.

The canonical type exposes public inherent constants for the four `SJS1` magic bytes and 40-byte header. Reactor uses those constants for branch selection, the retained-copy charge, and the prefix read. The independent Buffer encoder matched all three neutral corpus frames, including the largest u64 job identifier, non-ASCII text, and present-empty fields. Static checks verified the manifest and module path at every hop.

The concurrent source repair was present on inspection. A fresh descriptor compile remains the integration check.

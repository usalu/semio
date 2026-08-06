# Wave 3 OS State Authority Breaches (SEMIO_OS_STATE_AUTHORITY=1)

Total policy breaches: 595
OS-state-authority breaches: 102

## By kind
- `os-state-authority/item-scope-global`: 23
- `os-state-authority/id-minting`: 32
- `os-state-authority/authority-struct-map`: 38
- `os-state-authority/document-app-shape`: 9

## Samples (first 40)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs:564` — "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs:564" declares item-scope Mutex<…> outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:27` — "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:27" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:36` — "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:36" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎥️video/🦀️component.rs:13` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎥️video/🦀️component.rs:13" PartialMovieCache holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:4791` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:4791" BasicScene holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:5250` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:5250" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:5252` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:5252" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6409` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6409" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6410` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6410" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6762` — "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs:6762" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs:8` — "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs:8" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs:11` — "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs:11" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧱️kernel/🦀️component.rs:9` — "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧱️kernel/🦀️component.rs:9" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧱️kernel/🦀️component.rs:12` — "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧱️kernel/🦀️component.rs:12" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs:133` — "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs:133" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:16` — "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:16" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:60` — "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:60" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:314` — "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:314" ProcessKernelSession holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:324` — "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs:324" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs:44` — "✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs:44" PaintTextureCache holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/⚙️engine/🦀️component.rs:17` — "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/⚙️engine/🦀️component.rs:17" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/⚙️engine/🦀️component.rs:133` — "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/⚙️engine/🦀️component.rs:133" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/⚙️engine/🎬️scene/🦀️component.rs:151` — "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/⚙️engine/🎬️scene/🦀️component.rs:151" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:22` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:22" CadEngagementSession holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:106` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:106" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:116` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs:116" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs:36` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs:36" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs:62` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs:62" declares item-scope OnceLock<…> outside the OS product
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs:310` — "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs:310" CadScene holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/⚙️engine/🦀️component.rs:16` — "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/⚙️engine/🦀️component.rs:16" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/⚙️engine/🦀️component.rs:37` — "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/⚙️engine/🦀️component.rs:37" mints ids via AtomicU32/64 outside the OS product
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:934` — "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:934" RemodelScene holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️kernel.rs:146` — "✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️kernel.rs:146" SimulationState holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️kernel.rs:147` — "✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️kernel.rs:147" SimulationState holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️meters.rs:71` — "✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️meters.rs:71" MeterStore holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️output.rs:96` — "✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️output.rs:96" TimeSeriesStore holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:285` — "✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:285" TrinityHost holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:286` — "✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:286" TrinityHost holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:287` — "✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs:287" TrinityHost holds a HashMap/BTreeMap field outside the OS product
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs:79` — "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs:79" RewriteRuleState holds a HashMap/BTreeMap field outside the OS product

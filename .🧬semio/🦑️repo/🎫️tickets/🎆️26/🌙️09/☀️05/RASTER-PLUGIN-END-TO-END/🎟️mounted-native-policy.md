# Raster Mounted And Native Event Policy

## Actual Custody Path

The actual Raster cdylib installer is `🌎️hub/🧩️compositions/🖨️raster/🧩️component/🦀️.rs`. It previously passed INTERACTIVE_COMPONENT directly to plugin_exports!, establishing the real16MiB allocation/1MiB release clamp. It now receives Raster's explicit resource-policy function, reexported by the Hub owner. Only the Raster component changes: all three mounted phases allow64MiB capacity/release, while32KiB copy,32 items and4096 depth remain unchanged. Mounted turns still intersect these ceilings with their actual original caller and cumulative recipient; the domain cannot refill a spent source.

The host compute owner previously allocated flood masks, seen flags and queue in one96MiB-plus-header phase. Queue reservation now has its own retained phase. The first phase reserves2×pixel-count plus its actual image Arc header; the queue phase reserves4×pixel-count. At16MP both fit64MiB. These are reservations of uninitialized Vec capacity. Source/output and flood initialization continue in bounded write/work pages; the policy does not establish a hard bound on the system allocator's synchronous latency. No claim of an8ms allocator proof is made.

Winit previously ignored launch policy for UI-event sources and called NativeFrameRootPolicyV1::initiating_event(). The existing original launch declaration now requires frameEvent alongside startup,hostIo,compilerTask in JSON schema, TypeScript and Rust. NativeLaunchEnvironment transfers the chosen complete declaration together with the genuine Script owner and nullable original absolute deadline. Winit obtains NativeFrameEventIssuer through that installed admission. Each event computes its finite event deadline intersected with the already converted Script absolute monotone deadline. An existing source is never replaced or extended by continuation input. An expired Script deadline refuses a new source before any root birth.

## Domain Selection And Finite Stock

The generic playground contract now carries an optional nativeLaunchPolicy repository-relative JSON path. Discovery, typed projection and projected readback preserve it. The native launcher reads and validates that declared source before process creation; invalid/absolute/traversal paths refuse before reading. Raster's component Cargo metadata alone names its owned policy JSON. Unrelated playgrounds retain the explicit generic declaration unchanged. Current generated deployment rows contain no Raster entry because its compiled channel is withheld; no fabricated deployment row was added. The actual source discovery/projection/readback chain was executed independently and preserves the Raster path.

Raster's owned frameEvent declaration supplies1GiB cumulative allocation,3GiB copy,2GiB release,134217728 items, depth256 and71632078344 work units for a finite300000000µs event lifetime. An initial cleanup reserve of64MiB allocation/1GiB release and8388608 work units remains part of that original stock. These are genuine program-authored source ceilings installed before event issuance; they are neither per-callback grants nor a later replenishment.

The closed fixture accounts for the known worst stroke loop as16MP×(4096+1) pixel units plus16384×4096 row-selection units. Each segment unit is the implementation's bounded segment-coverage primitive, including its fixed antialias samples. Separate work buckets reserve two declared maximum semantic-wire passes, sixteen RGBA-frame work allowances, coordination and cleanup. The latter buckets are finite admission choices, not a proof that every inherited auxiliary image or generic framework path fits them. Generic Semio images have no universal metadata/frame-count bound. Oversized preserved auxiliary content and arbitrary resident document structure still need an explicit bounded aggregate admission/refusal proof; private source exhaustion/expiry is not presented as successful maximum-image completion. Paging the actual image storage would additionally remove the remaining contiguous64MiB allocator/release obligation.

## Tests And Evidence

Before production schema changes, the new neutral frameEvent fixture and native/TypeScript laws were authored. The independent Ajv comparison actually failed because the old schema rejected frameEvent as an additional property. After the required schema/transport changes, the ticket frame-policy-oracle passed6 admission/deadline checks,10 declared work/physical/caller-intersection checks,8 exact policy-owner/path checks and3 actual catalogue custody checks:27 total. Its independent oracles are Ajv, SQLite arithmetic, Bun TOML and the TypeScript parser. It also executes the real first-party launch reader/selector and real catalogue discovery/projection/readback. These are source-level TypeScript executions; they do not constitute native/Winit runtime evidence.

All nine changed Rust production/law modules passed pinned Rustfmt syntax parsing, and all eight changed TypeScript modules passed Bun parser scanning. The original native-caller law additionally asserts that an expired continuation preserves the same deadline/stock/fuel, and that expired new-event birth allocates nothing. Those native laws are authored and unexecuted. No compiler or application build was started by this continuation.

The later authorized focused repo MCP Cargo request was observed through terminal completion under handle g19:1042: exit1 after4m2.923s, underlying Cargo status101. It stopped in semio-framework-os-store-codec-registry with five E0277/E0599 prerequisite compiler errors before the host-plumbing law compiled or ran. The visible history pair receiving seam still calls map_err directly on a RetainedWorkOutcome from advance/take_ready_into. The parent owns that work-transport prerequisite. No duplicate test request was issued; no native deadline-custody verdict is claimed.

## Exact Rust And TypeScript Census

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🔲️pixel-compute/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️resource-policy/🦀️.rs`
- `🌎️hub/🧩️compositions/🖨️raster/🦀️.rs`
- `🌎️hub/🧩️compositions/🖨️raster/🧩️component/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/📋️contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts`

Additional authored/updated inputs are the native launch JSON schema/default JSON, shared neutral launch fixture, frame-event/selection matrices, Raster resource-policy fixture, Raster owned launch declaration/fixture and the Raster component Cargo metadata. The ticket scripts frame-policy-oracle.ts and source-find.ts are retained read-only verification inputs. No temporary compiler logs were created.

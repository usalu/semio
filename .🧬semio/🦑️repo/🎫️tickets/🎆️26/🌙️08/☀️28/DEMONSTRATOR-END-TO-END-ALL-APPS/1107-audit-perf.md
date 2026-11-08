# 1107 Audit Perf: quadratic quotes, per-turn caps, rescans (lane aud-perf)

STATUS: DONE (read-only). No runs, no edits outside this folder. Paths relative to `/Users/ueli/Documents/semio/`.

Scope: `🧰️framework/🔨️modules/🌱️value`, `🧰️framework/🔨️modules/🧵️job`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, the eight demonstrator pane crates under `✏️s/🔌️plugins/` and `✏️s/🔨️modules/🏗️fem/⚙️engine`, plus the three memory notes.

Note on paths: the os module lives under `🧰️framework/🛍️products/💻️os/`, not at the top level. The fem engine lives under `✏️s/🔨️modules/🏗️fem/⚙️engine`. The UI fixed list moved to `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔢️fixed-list/🦀️.rs` (the memory note cites an older path).

Size context: the largest JSON under `♻️mit-bestand/🧺️demonstrator` is 31 KB (`🔨️modules/🌐️locale/🔣️.json`). Pane fixtures are therefore small. The only realistic large inputs seen in the material are the fem house (6408 tets, 1939 nodes, from the fem memory note) and the 100k-to-2M element law cases in the retirement tests. Items below are rated against those.

## 1. Testdebt-c shapes: current state

| Shape (testdebt-c) | Location now | State | Complexity now |
| --- | --- | --- | --- |
| Forced `maximum_items: 1` per controlled child step | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:246` passes `maximum_items: grant.maximum_items` | FIXED in tree (verified). The bulk Collection close now gets the parent's items and chunks by copy bytes via `📋️collection/🦀️.rs` `close_step` (`count = min(demand, grant.maximum_items)`). | O(n / chunk) turns |
| `Vec/VecDeque::retirement_ready` O(n) per quote | `♻️retirement/🦀️.rs:44` (hook `retirement_always_ready`, default false), `:199` (leaf macro sets true), `♻️retirement/📋️collection/🦀️.rs:17,31`, `:557` | MITIGATED. The per-type hook already exists and short-circuits every leaf element (u8…u128, usize, Number, bool, &str, …). The scan remains only for non-leaf element types, and only on turns where the owner is still unadmitted (`ControlledRetirement.value` is `Some`): `🎮️controlled/🦀️.rs:122`, `:124`, `:189`. Once `admit_retirement_cursor` takes the value, the scan stops. | O(n) once per admission for non-leaf T, O(1) for leaves |

Remaining caveat for the same function: if an unadmitted owner is refused every turn (capacity grant below birth size, `🎮️controlled/🦀️.rs` `step_original` early returns), the O(n) readiness scan repeats every turn. Fix: check the cheap capacity/birth grant before calling `next_demand`, or cache readiness once at seed time.

Testdebt-c statement "every turn rescans `Vec::retirement_ready` 4-5 times" is therefore stale for the current tree. It was true for the pre-fix path.

## 2. Findings

Rating: realism means exposure at demonstrator-scale inputs. Owner uses `1107-lanes.md`.

### A. Quadratic or O(n) per turn

| ID | Location | Shape | Complexity | Realism | Suggested fix | Owner |
| --- | --- | --- | --- | --- | --- | --- |
| A1 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🦀️.rs:530` (`node_exists`), used at `:533`, `:539`, `:549` | Validation closure `model.nodes.iter().any(...)` runs per element node, per support and per load | O(E·k·N): 6408 × ~4 × 1939 ≈ 5·10^7 string compares per validation | Medium-high for the fem house. Low elsewhere. | Build one `HashSet<&str>` (or reuse the existing `node_slot` map) before the loop | statik (fem3d pane) |
| A2 | `✏️s/🔨️modules/🏗️fem/⚙️engine/⚖️static/🦀️.rs:108-116` (`build_dof_map`) and `:140-154` (`node_exists`) | Per node × per element scan, with `active.contains` linear per dof | O(N·E·k) ≈ 5·10^7, plus linear `contains` | Low. Reference oracle (`solve_linear_static`), used by `🧮️analyses/🧪️tests/🔬️unit` and `🧮️analyses/🗂️model` imports, not by the production analyses path, which has the hash-indexed map. | Keep as oracle; if it is ever used at scale, switch to the `HashMap` form already in `🧮️analyses/🗂️model/🦀️.rs` `build_dof_map` | statik |
| A3 | `⚖️static/🦀️.rs:164` (`positions_of`) called per element at `:207` and `:275` | `model.nodes.iter().find` per node id | O(E·k·N) in the oracle | Low (same as A2) | Use the `node_slot` index as the production `positions_of(&node_slot, ...)` does | statik |
| A4 | `🧮️analyses/🦀️.rs:525` (`recover_elements` closure), `:635` (`ref_result.displacements.iter().find` per node) | Per-element and per-node linear `find` | A4a: O(E·L), L = member loads (small). A4b: O(N²) if on a production path, about 3.7 M compares at N=1939 | Low; A4b unverified whether production or comparison-only | Index displacements by node id once | statik |
| A5 | `🌱️value/📸️snapshot/👁️read/🦀️.rs:343` (`owned_backing_work_demand`) | `state.slots.pages.iter().any(...)` and `lease_generations.pages.iter().any(...)` on every quote | O(pages) per quote | Low (page count is a registry constant) | Maintain a live-page counter | port-a (snapshot-read) |
| A6 | `🌱️value/♻️retirement/🦀️.rs:540` (`FixedArray::next_demand`), `:545` (`close_step` uses `rev().find`), `:547` (`terminal_is_empty` uses `all`), `:548` (`next_birth_bytes` uses `rev().flatten()`) | Reverse scan over trailing `None`s, then full-array `all` each turn | O(N) per turn, O(N²) overall when closed from the end | Low (N is a compile-time array length) | Keep a count of live slots; close from the front or track the last live index | runtime (retirement core) |
| A7 | `🌱️value/♻️retirement/🏭️factory/🦀️.rs:88` (`requires_original_birth`), `:94-95` (close steps), `:96` (`demands`), `:97` (`terminal_is_empty`), each `slots.iter().find` or `all` | `[FactoryChildSlot; N]` scanned on every demand, step and requires-check | O(N) per turn with N fixed and small | Low | Acceptable. Only fix if N grows with the document. | runtime |
| A8 | `🌱️value/♻️retirement/📦️allocation-return/🦀️.rs:66` (`next_close_byte_demand`: `slots.iter().find_map`) | Scan from front every quote while closing from the front | O(slots) per turn, O(slots²) overall | Low (fixed array) | Track the next index | runtime |
| A9 | `🌱️value/♻️retirement/🎮️controlled/🦀️.rs:161` and `🌱️value/♻️retirement/🦀️.rs:710-721` (`allows_admitted_narrow_work`, `next_birth_bytes`) | `owner.get(last).retirement_birth_bytes()` per turn calls the element's `retirement_ready` | O(size of that element) per turn | Low. It is element-sized, not collection-sized. | Cache the element's birth bytes at push time | runtime |

### B. Per-turn caps at one item (no chunking)

| ID | Location | Shape | Complexity | Realism | Suggested fix | Owner |
| --- | --- | --- | --- | --- | --- | --- |
| B1 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs:131` | `RetainedCloneGrant{maximum_items: cx.retained_grant().maximum_items.min(1), ..}` on the decoder recipient close. This is the same shape as the fixed 246 bug, one layer up. `close_retirement_recipient` (`🌱️value/🛬️decode/🦀️.rs:102`) then closes one element per job turn when the recipient is a plain Vec. | O(n) job turns, each with a demand quote and turn overhead (fuel, yield check). Not quadratic in the quote itself. | Medium. A decoded collection of 10^4 elements means 10^4 turns. Unmeasured wall time. | Pass `cx.retained_grant().maximum_items` as 246 does, and let copy bytes and `retirement_should_yield()` bound each turn. Probe first: time a 100k-element decode close before and after. | jobsplit (owns `job/⏱️context/` while claimed), runtime (decision) |
| B2 | `🧰️framework/🔨️modules/🧵️job/🏃️headless/🎟️caller/🦀️.rs:55` (`retirement_grant`: `left.maximum_items.min(1)`) | One item per headless step. Used for the caller's retained source. | O(n) turns | Low. Headless is the test and one-shot host. | Same as B1 if the stock is a collection. | runtime (job side) |
| B3 | `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs:323` (`retire_step`) | `maximum_items: 1` per job step, quoting `retirement_demands()` each turn | O(n) job steps | Medium for puzzle job completion. Each step re-quotes the parent. | Pass the parent's items once, as in 246 | aggregator (puzzle3d) |
| B4 | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🔨️modules/🏠️host/🧰️owned/🦀️.rs:34-40` (`OwnedCadStore::close`) | Synchronous `while` loop, `maximum_items: 1`, quoting `close_owned_demands(0)` each iteration | O(n) iterations with a full quote each. Not quadratic if the quote is O(1). Synchronous, so no yield. | Medium at CAD document scale. Unverified quote cost. | Use the quoted item count; keep the loop | koordinator (cad) |
| B5 | `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🧬️generation/🪶️sqlite/🚦️native/🌱️value/🦀️.rs:22` (`retire_map`) | Drop-time loop, `maximum_items: 1`, two quotes per entry | O(N log N): ordered-map `advance` is AVL-bounded (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:451-500`, quotes O(1)). | Low. Drop-only, synchronous, no yield. | Raise items to the map's own bound | proc (generator) |
| B6 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🌉️apply/🦀️.rs:44-52` (`generation2d_retire_displaced_cold`) | Cold-boundary loop, `maximum_items: 1`, four quotes per iteration | O(n) iterations; per-iteration quotes are O(1) for Vec-like owners | Low (cold boundary) | Same as B5 | proc (generator) |
| B7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:742` (`group_grant`) and `🪆️child/🦀️.rs:66` | Group closes one child per turn | O(children) turns, children bounded by `PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN` | Low | Accept, the bound is a constant | runtime-b (publication) |

### C. Rescans from start (resumed cursor, first-pending search)

| ID | Location | Shape | Complexity | Realism | Suggested fix | Owner |
| --- | --- | --- | --- | --- | --- | --- |
| C1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧭️owner-registry/🦀️.rs:122-124` (`preview_retirement_demand`, `preview_retirement_step`), `:255-256` (`direct_ingress_demands`) | `self.owners.values().find(pending)` on every quote and step. Finished owners are skipped from the start every turn. | O(owners) per turn, O(owners²) overall | Low-medium. Owners are windows or partitions; the demonstrator has about 8 panes. | Keep a pending queue (VecDeque of keys) and pop completed owners | port-b (window-config) |
| C2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🦀️.rs:439` | Same shape over `partitions` | O(partitions) per turn | Low | Same as C1 | port-b |
| C3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:265` (`next_owned_job`) | Linear scan of the JOBS slot array per lookup | O(slots) per lookup; capacity not verified | Low | Index by (operation, generation) | port-b or reacthost (unverified owner) |
| C4 | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/…` (`next_depth_demand`, `advance`) | Not a rescan: quotes are O(1) | O(1) quote, O(log n) advance | Not an issue | None | none |
| C5 | `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:810-812` (`next_release_copy_byte_demand`, `release_empty_page`) | Descend from root per turn with `rfind`/`rposition` over children | O(depth × fanout) per turn, not O(n) | Not an issue at current fanout | None | none |

### D. Loops re-pricing whole documents

| ID | Location | Shape | Complexity | Realism | Suggested fix | Owner |
| --- | --- | --- | --- | --- | --- | --- |
| D1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/…` (group publication) and the hover path | Memory note `project-hover-latency-anatomy-and-fixes`: whole-surface re-admission and `paged_text_carrier` re-chunking. Not in the listed directories. | Not re-verified here | Not re-verified | Content-addressed page elision (memory note) | reacthost (React host, UI) and runtime-b (publication). Out of this audit's scope. |
| D2 | `🧰️framework/🔨️modules/🧵️job/📝️text/🧳️owned/🦀️.rs:15-30` (`FundedText`) | Validates the source by pointer and length, and chunks at most 4096 bytes per turn | O(1) quote, bounded chunk. Not an issue. | None | None | none |

## 3. Memory-note verdicts

- `project-fem-engine-quadratic-lookups-and-envelope-ldlt`: the production analyses path is still hash- or slot-indexed (`🧮️analyses/🦀️.rs:417,568,630` use `node_slot`). The envelope LDLT is in place. Still open: the linear node-existence validation (A1), and the oracle's quadratic dof map and positions (A2, A3). The note's "assembly stepping is the next hotspot" was not re-measured here.
- `project-hover-latency-anatomy-and-fixes`: the framework-side structural items (`paged_text_carrier` re-chunking, whole-surface re-admission) are outside the listed directories and were not re-verified (D1).
- `project-ui-fixed-list-cold-push-priced-full-capacity`: FIXED. `🖱️ui/🧬️contract/🌱️values/🔢️fixed-list/🦀️.rs:107-108` `try_push` now uses `reserve_next_slot` (page-exact, `:95`). `reserve_full()` is still called at `:51` by another method, not by `try_push`. The memory note's path is stale. Owner: port-c (ui-contract).

## 4. Summary by owner lane

- runtime: A6, A7, A8, A9, B2 (decision with jobsplit), B7 (with runtime-b), retirement core semantics for B1.
- jobsplit: B1 (job-context file, exclusive while claimed).
- statik: A1 (realistic), A2, A3, A4 (oracle and per-node finds).
- aggregator (puzzle3d): B3.
- koordinator (cad): B4.
- proc (generator): B5, B6.
- port-a (snapshot-read): A5.
- port-b (window-config, store-owners, reactor jobs): C1, C2, C3.
- runtime-b (publication, group): B7.
- value (ordered, list): no action required (C4, C5 are fine).
- testdebt-c: item 246 done. Nothing further on the forced one-item cap.

## 5. Priority

1. A1 (fem validation, fem-house scale, straightforward index). Owner statik.
2. B1 (job decode one element per turn). Probe a 100k decode close first. Owner jobsplit with runtime.
3. B3 and B4 (pane job and CAD close, same shape as 246). Owners aggregator and koordinator.
4. C1 and C2 (rescan of finished owners). Low until the owner count grows.
5. Everything else: accept, or fix when touched.

## 6. Not verified

- Wall-clock cost for B1, B3, B4 (no runs, per brief).
- Whether A4b is on a production path.
- JOBS slot capacity (C3).
- D1 (hover) in its own directories.

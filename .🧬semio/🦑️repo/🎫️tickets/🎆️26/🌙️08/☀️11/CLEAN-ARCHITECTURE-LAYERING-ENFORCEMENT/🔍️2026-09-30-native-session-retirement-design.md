# Native Session Retirement Design Evidence

Read-only design audit; no implementation or runtime claims. Inspected the current Session handoff after the port-admission lock repair. Earlier registry findings are being repaired by the execution owner and are not repeated here.

## Exact Ownership Frontiers

Session source is `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs`. Lines 729–738 contain Session's Arc<SessionState>; each state owns one jobs mutex, authority id, and closed flag, while sibling ports share kernel Arc<RwLock<Box<Brep>>>, mesh-cache Arc<Mutex<HashMap<(String,u64),MeshData>>>, claims Arc<Mutex<HashMap<u64,HashSet<String>>>>, and next-authority counter. Root clones share an authority; port creates a new authority/state. There is no unified kernel-family last-reader retirement authority yet. Port now acquires claims before the parent terminal read and construction, repairing the earlier admission race.

RetainedTessellation and TessellationJobRegistry at lines 523–532 contain the job, last_step, jobs HashMap, and clock. The 32-job count cap limits count, not each job's memory. Cached MeshData owns variable-size geometry arrays. Claims have variable-size authority/handle maps and strings. Removing an entire authority HashSet or clearing jobs/cache in one item hides nested destruction.

Native BREP engine path: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs`. Brep at lines 438–441 owns only Body and live HashMap<String,Entity>. Entity at lines 425–435 owns scalar arena references plus Wire, Compound Vec<SolidId>, standalone Curve3, and Surface. Full-family retirement can consume these fields without reachability GC: every reader is gone, so every entity is dead. Peer-authority closure cannot consume this family Body while peers remain live.

## Narrow Internal Extraction Mechanism

Body topology `🧬️schema/📸️snapshot/🕸️topology/🦀️.rs:107–120` publicly owns ten arena stores: vertices, edges, coedges, loops, faces, shells, solids, curves3, curves2, surfaces, plus the scalar label source. An internal engine retirement constructor can move/mem::take Body and live into a retained cursor in constant structural work; it must not invoke retain, dispose, compact_unreachable, reachable_from, codecs, or clone.

Arena source `🧬️schema/📸️snapshot/🏟️arena/🦀️.rs:66–82` hides slots Vec<Slot<T>> and free Vec<u32>. Existing remove at line 131 can transfer one known value, but discovering all ids through ids().collect is a whole-store scan; len/is_empty at lines 143–149 also scan. The narrow new consuming API belongs inside Store: pop one tail Slot per granted structural unit, transferring Option<T> to the domain cursor, charging vacant slots as work, and separately retire free-list backing. Since the whole arena is terminal, tail popping needs no id remap or generation preservation. No such existing extraction API currently exists.

The domain cursor must dismantle one nested field at a time. Face.inners, Shell.faces, Solid.inners are Vecs (topology lines 69–97). Curve3::Nurbs owns knot/control/weight Vecs (`➰️curve/🦀️.rs:34–43`); Curve2 has the same ownership shape. Surface::Nurbs owns two knot Vecs plus controls/weights Vec<Vec<...>> (`🏄️surface/🦀️.rs:28–46`), so each row must be transferred separately before the outer buffer is released. KnotVector exposes its knots Vec (`➰️curve/🪢️bspline/🦀️.rs:15–16`). Dropping one surface directly recurses over all its rows.

TessellationJob is private-field owned in `🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:191–203`: edge_order, edge_cache HashMap<EdgeId,Vec<(f64,Pnt3)>>, faces, transfer, and report. Its own consuming retirement must detach these buffers; cancel alone does not dismantle them. A job removal cursor in Session cannot bound ordinary job Drop without that internal API.

## Reusable First-Party Semantics

Neural `⚙️engine/📔️registry/🦀️.rs:17–39` is the exact last-reader pattern: SharedRegistry hides raw Arc roots; every reader Drop calls Arc::into_inner, and the final reader transfers ownership into a retained OnceLock<ManuallyDrop<Registry>> controlled by the unique RegistryRetirement. Retirement waits rather than forcing surviving readers closed. Geometry needs an equivalent family ownership handoff, not an Arc::strong_count snapshot: readers can change after a count check, and ordinary final Arc destruction recursively releases live resources.

Neural `⚙️engine/🧵️retirement/🦀️.rs:12–19,72–102` provides the established payload drawdown law: retain a buffer while charging min(positive byte grant, remaining payload), free it only when charged fully, and separately charge structural transitions. ValueRetirement.text can accept detached handle strings. Its bytes frontier is not public, and it does not accept typed geometry vectors; extending a first-party typed POD-buffer cursor is narrower than converting geometry to Value/JSON. This is bounded ownership accounting, not a wall-clock allocator guarantee; preserve that distinction.

Standard HashMap iteration may scan vacant buckets before yielding the next entry. Repeated keys().next/remove also restarts scanning. If strict per-frontier traversal matters for sparse retained maps, use an ordered removable frontier at construction (as the registry uses BTreeMap::pop_first) or an explicit representation supporting charged bucket/slot traversal. Do not claim each discovered HashMap entry is worst-case constant geometric work.

## Viable Close Order And Peer Constraint

The app owner first seals its producing authority under the same claims lock used for admission. It detaches/drains its own jobs and claim strings into persistent cursors, releases supplied registry readers after temporary hosts/work finish, and drives RegistryRetirement so registered operators release their Session clones. Its retained family retirement authority then waits for the real final reader handoff, dismantles kernel live/Body, shared cache, and remaining maps, and reports terminal-empty only after every nested cursor is empty. A busy worker or remaining reader is zero-progress Pending; it is not permission to drop the authority. Pausing cancellation of retirement must retain ownership/cursor state rather than reopen a sealed authority or discard resources.

Closing one sibling while peers continue must preserve their Body and cached/job resources. The current immediate merged-retain GC provides peer safety but is synchronous. A bounded replacement can separate logical authority release from deferred family-terminal physical reclamation; if immediate reclamation during continuing peer use is required, it needs a separate retained reachability/compaction algorithm with transaction/version protection. That is a larger mechanism and cannot be approximated by calling the existing GC once inside close_step. The coordinator and Session execution owner received these exact ownership and extraction findings.

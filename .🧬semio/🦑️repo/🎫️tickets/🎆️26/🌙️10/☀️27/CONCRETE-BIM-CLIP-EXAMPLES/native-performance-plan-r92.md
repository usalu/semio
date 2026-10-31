# Native Drawing Performance Repair

The completed safe-observer lifecycle produced real schema/class Cairo frames but hit its own 900-second cooperative bound before all nineteen beats. Facts took 310.047 seconds; first axon construction took 143.375 seconds. Schema/class animation intervals were approximately 203.218/118.094 seconds. This is diagnostic evidence, not a frozen full pass.

The current source registers a rendered VGroup itself as the first selector bucket. Registering later pieces under a shared alias therefore mutates that original rendered group and leaks its membership into other selectors. Root/layer insertion also adds one piece at a time, causing repeated native identity scans. The repair will use independent selector containers and ordered bulk insertion; native actors will remain scene-local. Portable assembly cases specify exact root paint order, aliases, discipline memberships, no shape mutation and registration-call bounds. A real Cairo comparison will independently validate the resulting render order and pixels.

The model geometry cache producer tuple will be factored into one canonical inventory and expose a process-local integer revision bound to canonical source plus real producer identities. Public native values remain isolated copies. New ground support/difference/orientation and original ramp finish dependencies must be retained. The focused native RED job 22262 is still waiting normal memory admission; no runtime RED/GREEN is claimed yet. No admission bypass or peer-process control has been applied.

Visual pure-geometry/primitive caching must consume the revision once per outer construction, preserve producer invalidation and avoid serializing the entire model per primitive membership lookup. Actor sharing across scenes is not proposed. Full scene interpolation and Cairo times will be measured after batching before any full matrix launch.

## Implemented Checkpoint

Factored the native model producer inventory into _geometry_producers and _geometry_memo, with process-local geometry_revision. The revision is not a persisted render/cache key. All prior identities and original wearing-reference interfaces remain, plus support/difference/orientation and query wrapper identities. Runtime memo copies remain isolated.

Drawing.part and AxonDrawing.layer_part now own independent selector containers. AxonDrawing._assemble inserts root actors, discipline layers and selector buckets in batches; villa and element axons use it, flushing before the existing ground insertion. Primitive geometry and painter styles are unchanged. Standard test discovery includes portable revision and assembly contracts; the latter also compares actual native Cairo pixels with an ordered reference group and checks copied actors cannot mutate the source.

Presentation lifecycle tests now flush timing-only DEBUG records for native animation compilation, setup, interpolation, cleanup and both Cairo captures. All original controls/assertions remain. Focused jobs 22262 and 61301 are still normal-admission queued. Static RED artifacts are revision-interface-red.json and assembly-interface-red.json under generated/native-r92; no native runtime RED/GREEN is represented by those static interface checks.

## Installed Native Insertion Evidence

Read the installed Manim mobject/mobject.py through the filesystem interface (SHA256 fe5659cdeec7a2ee30f3c419f2292b729332ff9fddb68c739cb4cc4621500b44). Its _insert_submobjects uses a direct assignment for an empty container. For one later child it searches the existing child list with list.index; bulk append instead performs a list difference and one extension. This independently supports replacing one-at-a-time root/layer registration with the exact same ordered bulk actor list. No library source or monkeypatch was changed. Actual speed and pixel parity still require the queued focused/native gates.

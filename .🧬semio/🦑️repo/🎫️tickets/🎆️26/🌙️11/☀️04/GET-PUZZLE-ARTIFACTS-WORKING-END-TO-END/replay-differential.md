# Replay Fixture Differential

Source-only independent reference computation, using retained scalar Hamilton/vector/domain kernel and separate existing Three quaternion/vector pipeline on actual authored base/log/edit. No Rust execution, fixture writing, tolerance change or regenerated golden. Inputs retained in replay-math-reference.mjs; numerical output generated/replay-math-reference.json.

Actual unique native3d-store-3.log stops at drag-offset preview. Its eight observed changed values exactly match the scalar calculation (including -6.661338147750941e-16 origin, 1.1102230246251568e-16 quaternion and 1.2325951644078315e-32 cross residual). Remaining stages are independent predictions, not native observations; require actual strict owner rerun before claiming agreement.

Current source semantics explicitly preserve zero drag and missing-target refusal, scale selected unlocked objects only without follower propagation or attraction rederive, and sequentially fold edited prefix then suffix. All99 detected differences are numeric existing fields; no structural/string/boolean/cardinality differences. Three corroborates geometrical operations within4.263256414560601e-14 at maximum angle component; reference tolerances are diagnostic only, never proposed for native acceptance.

| Case | Preview Fields | Replayed Fields |
|---|---:|---:|
|drag-offset|8|17|
|drag-targets|2|10|
|drag-no-op|0|0|
|rotate-angle|10|13|
|rotate-targets|12|15|
|scale-factors|0|6|
|scale-targets|0|6|
|downstream-missing|0|0|

## Exact Field Evidence

New is independent scalar prediction; old is authored expectation. In generated output inherited field names reference=new and native=old refer to this comparison, not an unexecuted native observation.

| Case | Stage | Field | Old | New |
|---|---|---|---:|---:|
|drag-offset|preview|/attractions/1/rotation|-90|-90.00000000000001|
|drag-offset|preview|/attractions/1/shift|0|-6.661338147750939e-16|
|drag-offset|preview|/objects/1/origin/1|0|-6.661338147750941e-16|
|drag-offset|preview|/objects/1/orientation/0|0|1.1102230246251568e-16|
|drag-offset|preview|/objects/3/origin/1|1.9999999999999996|1.9999999999999993|
|drag-offset|preview|/objects/3/orientation/0|0|1.1102230246251568e-16|
|drag-offset|preview|/objects/3/orientation/1|0|1.2325951644078315e-32|
|drag-offset|preview|/objects/3/orientation/2|0|1.1102230246251568e-16|
|drag-offset|replayed|/attractions/0/gap|-0.9999999999999998|-1.0000000000000002|
|drag-offset|replayed|/attractions/0/rise|4.440892098500625e-16|0|
|drag-offset|replayed|/attractions/0/rotation|90|89.99999999999999|
|drag-offset|replayed|/attractions/0/shift|1|0.9999999999999991|
|drag-offset|replayed|/attractions/0/tilt|89.99999829245272|89.99999809090409|
|drag-offset|replayed|/attractions/1/shift|-2.9999999999999996|-3|
|drag-offset|replayed|/attractions/1/tilt|-89.99999829245272|-89.99999790869022|
|drag-offset|replayed|/attractions/2/shift|-0.9999999999999993|-0.9999999999999984|
|drag-offset|replayed|/attractions/2/turn|90|89.99999999999999|
|drag-offset|replayed|/objects/1/origin/1|0|-6.661338147750941e-16|
|drag-offset|replayed|/objects/1/orientation/0|0|7.850462293418877e-17|
|drag-offset|replayed|/objects/1/orientation/1|0|7.850462293418876e-17|
|drag-offset|replayed|/objects/1/orientation/2|0.7071067811865476|0.7071067811865475|
|drag-offset|replayed|/objects/3/origin/1|1.9999999999999996|1.9999999999999993|
|drag-offset|replayed|/objects/3/orientation/0|0|7.850462293418876e-17|
|drag-offset|replayed|/objects/3/orientation/1|0|7.850462293418877e-17|
|drag-offset|replayed|/objects/3/orientation/3|0.7071067811865476|0.7071067811865475|
|drag-targets|preview|/objects/3/origin/1|1.9999999999999996|2|
|drag-targets|preview|/objects/3/orientation/2|0|1.1102230246251568e-16|
|drag-targets|replayed|/attractions/0/gap|-0.9999999999999998|-0.9999999999999996|
|drag-targets|replayed|/attractions/0/rise|1|0.9999999999999998|
|drag-targets|replayed|/attractions/0/shift|1|0.9999999999999998|
|drag-targets|replayed|/attractions/1/shift|-2.9999999999999996|-2.999999999999999|
|drag-targets|replayed|/attractions/2/gap|4.440892098500625e-16|8.88178419700125e-16|
|drag-targets|replayed|/attractions/2/shift|-0.9999999999999993|-0.9999999999999984|
|drag-targets|replayed|/attractions/2/turn|90|89.99999999999999|
|drag-targets|replayed|/objects/1/orientation/2|0.7071067811865476|0.7071067811865475|
|drag-targets|replayed|/objects/3/origin/1|1.9999999999999996|2|
|drag-targets|replayed|/objects/3/orientation/3|0.7071067811865476|0.7071067811865475|
|rotate-angle|preview|/attractions/1/rise|0|4.710277376051325e-16|
|rotate-angle|preview|/attractions/1/rotation|-90|-90.00000000000001|
|rotate-angle|preview|/attractions/1/tilt|-44.99999999999995|-44.999999999999964|
|rotate-angle|preview|/attractions/1/turn|0|4.497983566394947e-15|
|rotate-angle|preview|/objects/1/orientation/0|0|1.0257123289739567e-16|
|rotate-angle|preview|/objects/1/orientation/1|0|-4.248639577543066e-17|
|rotate-angle|preview|/objects/3/origin/1|1.9999999999999996|2|
|rotate-angle|preview|/objects/3/orientation/0|0|1.0257123289739567e-16|
|rotate-angle|preview|/objects/3/orientation/1|0|-4.248639577543065e-17|
|rotate-angle|preview|/objects/3/orientation/2|0.3826834323650898|0.3826834323650899|
|rotate-angle|replayed|/attractions/0/rise|0|-2.220446049250313e-16|
|rotate-angle|replayed|/attractions/0/rotation|-90|-89.99999999999999|
|rotate-angle|replayed|/attractions/1/rise|0|4.710277376051325e-16|
|rotate-angle|replayed|/attractions/1/rotation|-90|-90.00000000000001|
|rotate-angle|replayed|/attractions/1/tilt|-44.99999999999995|-44.999999999999964|
|rotate-angle|replayed|/attractions/1/turn|0|4.497983566394947e-15|
|rotate-angle|replayed|/objects/1/orientation/0|0|1.0257123289739567e-16|
|rotate-angle|replayed|/objects/1/orientation/1|0|-4.248639577543066e-17|
|rotate-angle|replayed|/objects/3/origin/0|2.0000000000000004|1.9999999999999998|
|rotate-angle|replayed|/objects/3/origin/1|2.9999999999999996|3|
|rotate-angle|replayed|/objects/3/orientation/0|0|1.0257123289739567e-16|
|rotate-angle|replayed|/objects/3/orientation/1|0|-4.248639577543065e-17|
|rotate-angle|replayed|/objects/3/orientation/2|0.3826834323650898|0.3826834323650899|
|rotate-targets|preview|/attractions/1/rise|0|6.661338147750937e-16|
|rotate-targets|preview|/attractions/1/rotation|-90|-90.00000000000001|
|rotate-targets|preview|/attractions/1/shift|-2.9999999999999996|-2.999999999999999|
|rotate-targets|preview|/attractions/1/tilt|-89.99999829245272|-89.99999809090409|
|rotate-targets|preview|/objects/0/orientation/2|0.7071067811865476|0.7071067811865475|
|rotate-targets|preview|/objects/1/orientation/0|0|7.850462293418877e-17|
|rotate-targets|preview|/objects/1/orientation/1|0|-7.850462293418876e-17|
|rotate-targets|preview|/objects/1/orientation/2|0.7071067811865476|0.7071067811865475|
|rotate-targets|preview|/objects/3/origin/1|1.9999999999999996|2|
|rotate-targets|preview|/objects/3/orientation/0|0|7.850462293418878e-17|
|rotate-targets|preview|/objects/3/orientation/1|0|-7.850462293418875e-17|
|rotate-targets|preview|/objects/3/orientation/3|0.7071067811865476|0.7071067811865475|
|rotate-targets|replayed|/attractions/0/rise|0|-2.2204460492503126e-16|
|rotate-targets|replayed|/attractions/0/rotation|-90|-89.99999999999999|
|rotate-targets|replayed|/attractions/1/rise|0|6.661338147750937e-16|
|rotate-targets|replayed|/attractions/1/rotation|-90|-90.00000000000001|
|rotate-targets|replayed|/attractions/1/shift|-2|-1.9999999999999996|
|rotate-targets|replayed|/attractions/1/tilt|-89.99999829245272|-89.99999809090409|
|rotate-targets|replayed|/objects/0/orientation/2|0.7071067811865476|0.7071067811865475|
|rotate-targets|replayed|/objects/1/orientation/0|0|7.850462293418877e-17|
|rotate-targets|replayed|/objects/1/orientation/1|0|-7.850462293418876e-17|
|rotate-targets|replayed|/objects/1/orientation/2|0.7071067811865476|0.7071067811865475|
|rotate-targets|replayed|/objects/3/origin/0|2.0000000000000004|1.9999999999999998|
|rotate-targets|replayed|/objects/3/origin/1|2.9999999999999996|3|
|rotate-targets|replayed|/objects/3/orientation/0|0|7.850462293418878e-17|
|rotate-targets|replayed|/objects/3/orientation/1|0|-7.850462293418875e-17|
|rotate-targets|replayed|/objects/3/orientation/3|0.7071067811865476|0.7071067811865475|
|scale-factors|replayed|/attractions/1/rotation|-90|-90.00000000000001|
|scale-factors|replayed|/objects/1/orientation/0|0|1.1102230246251568e-16|
|scale-factors|replayed|/objects/3/origin/1|1.9999999999999996|2|
|scale-factors|replayed|/objects/3/orientation/0|0|1.1102230246251568e-16|
|scale-factors|replayed|/objects/3/orientation/1|0|1.2325951644078315e-32|
|scale-factors|replayed|/objects/3/orientation/2|0|1.1102230246251568e-16|
|scale-targets|replayed|/attractions/1/rotation|-90|-90.00000000000001|
|scale-targets|replayed|/objects/1/orientation/0|0|1.1102230246251568e-16|
|scale-targets|replayed|/objects/3/origin/1|1.9999999999999996|2|
|scale-targets|replayed|/objects/3/orientation/0|0|1.1102230246251568e-16|
|scale-targets|replayed|/objects/3/orientation/1|0|1.2325951644078315e-32|
|scale-targets|replayed|/objects/3/orientation/2|0|1.1102230246251568e-16|

## Handcrafted patch review

Independent read-only comparison of current complete JSON to retained scalar reference finds all16 preview/replayed snapshots exactly equal. Read-only HEAD comparison finds exactly99 changed values, all existing numeric fields under preview/replayed; no base/log/edit/outcome/finalize/schema/cardinality changes. Native strict rerun remains necessary and pending.

## Actual strict replay validation

Parent and integration report canonical host30588 completed4/4unfiltered tests,zeroignored/filtered. All8cases/16states and99corrected numeric expectations now pass strictnative equality, including Pack/DSL/envelope roundtrips. Unique native3d-store-5.log retained.

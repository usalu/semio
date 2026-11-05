# Selection Fixture Differential Audit

Read-only independent audit; no fixtures edited/regenerated and no Rust started. Exact native diagnostic25113 remains queued at this observation, so final field-by-field classification of all10after/diff files is pending actual values.

Scope is authored3d selection fixtures: drag-selection/follow, drag-selection/cross, rotate-selection/skips-locked-ghost, rotate-selection/orbits and rotate-selection/turns-two-objects. Their before/mutation/outcome expectations remain authoritative inputs; no broad expected-output regeneration is authorized by this audit.

## Independent Math And Source Method

Read current RotateSelection diff and schema pose-math, plus selection_follow breadth-first graph placement. Direct addressed unlocked object rotation is axis-angle quaternion pre-multiplied by its base orientation, about its own unchanged origin. It does NOT normalize the direct result. Followed children use attraction_child_pose, which explicitly normalizes derived orientations; locked objects never follow, and other touched attractions rederive six parameters.

Executed existing third-party Three Quaternion.setFromAxisAngle(normalized axis, authored angle).multiply(identity) through repo tools light Bun (no new dependency/source package). Independently checked system Python math sin/cos of half the same authored f64 angle. Both give quarter-turn quaternion active component0.7071067811865475,w0.7071067811865476, norm1. The three rotate scenarios committed after snapshots spell BOTH active component and w as0.7071067811865476 for directly selected objects. This is a one-ULP (1.1102230246251565e-16) direct-quaternion fixture discrepancy from current authored operation math. Do not blanket-normalize or edit follower quaternion fields: they pass a different explicit normalization branch.

Expected follow/cross object-d positions already carry rounding tails2.0000000000000004/1.9999999999999996, and orbits contains tilt -89.99999829245272 near the asin endpoint. Such appearance alone does not establish the native discrepancy magnitude or semantic correctness; exact actual values and inverse/forward pose checks are still required before deciding other fields. The runtime gate also compares serde_json values, whose non-float_roundtrip decoder can differ from framework JSON parsing; distinguish parser rounding from geometry by inspecting actual emitted decimal numbers, not codec agreement alone.

## Current Conclusion

One independently established numerical issue: directly selected quarter-turn expected quaternion active components disagree oneULP with the current leaf math and independent Three/systemmath. No evidence yet of a selection/lock/follow semantic defect. Remaining actual per-field and sparse-diff shape differences await native diagnostic output; this report intentionally does not approve all10fixture changes from a blanket tolerance or a blind generator.

Diagnostic25113 follow-through: integration reports compiler-only failure in newly added diagnostics (text moved into optional std::fs::write, then borrowed for print). Current source now writes &text, correctly preserving custody for later diagnostic without any fixture writes. One additional diagnostic-quality issue sent integration: printing parsed serde_json::Value roundtrips the exact numeric strings through the suspected non-roundtrip parser; use raw committed/text string fields for exact native decimal evidence. No gate tolerance/assertion change requested. Actual field-level native output remains pending.

Exact-decimal regression review: new8case corpus stores decimal strings plus binary64 hex strings, so corpus loading does not round numeric inputs. Python struct.pack(>d,float(text)).hex independently verifies all8expected bit patterns including adjacent quaternion doubles and minimum subnormal. Current Rust regression compares native f64 parse, owned JSON f64 and serde f64 to these bits; it is a precise oracle qualification, not a fixture tolerance. New fixture absolute path181UTF16units. No actual Rust execution result yet.

## Final Exact Native Differential Conclusion

Actual unfiltered native72548 outcome reported by integration:26tests,25passed, one strict committed-fixture failure. The exact8vector native/owned/serde binary64 bit test PASSED without dependency/feature changes. Therefore the suspected serde parser bug is disproved for these inputs; do not change serde features to mask the fixture discrepancy.

Independently decoded the raw quoted committed/actual strings from native3d.log with Python JSON, then recursively compared structures:10files/58differences, exactly29snapshot differences repeated by the5sparse diff outputs. Every difference is numeric or absent-to-numeric sparse patch field. No changed IDs, targets, topology, array cardinalities, strings, booleans or locked-object poses occur. Native selected quarterturn quaternions match independent existing Three math and Python systemmath exactly.

Largest absolute delta outside tilt is1.4210854715202004e-14 (the adjacent binary64 values -90 and -90.00000000000001). Quaternion residuals are about1.11e-16 or smaller; positions differ at binary64 rounding scale. Orbits tilt changes by2.015486302742778e-7degrees because asin is ill-conditioned at the endpoint: independent Python sin maps the committed/actual tilts to -0.9999999999999996 and -0.9999999999999994 (oneULP). Current derive_attraction_params explicitly uses clamped asin(col_y.z), so this is the expected magnification of a oneULP matrix residual, not a changed rotation/selection intent.

Source audit explains these residuals: direct selected axis-angle keeps sin/cos components; followed poses multiply align/connection quaternions and normalize; rederived attraction parameters project those poses and use atan2/asin. The authored fixtures had rounded/symmetrized these outputs inconsistently. Sparse patch presence uses exact inequality against the original attraction fields, so formerly absent tiny shift/rise or adjacent rotation values legitimately become explicit patch fields. Do not loosen exact gate comparisons or introduce a numeric tolerance. A narrowly reviewed correction of only these10after/diff expected files to the actual deterministic values is supported; no before/mutation/outcome/schema/topology change is supported. This audit did not modify any fixture.

### Exact Snapshot Differences

Each listed snapshot has a paired diff file carrying the same changed numerical state; only follow shift/rotation and orbits rise/rotation also change sparse-field absence to explicit value.

#### ✋️drag-selection / ⛓️follow

| JSON pointer | Committed | Actual native |
|---|---:|---:|
| `/attractions/1/rotation` | -90.0 | -90.00000000000001 |
| `/attractions/1/shift` | 0.0 | -2.220446049250313e-16 |
| `/objects/1/orientation/0` | 0.0 | 1.1102230246251568e-16 |
| `/objects/1/origin/1` | 0.0 | -2.2204460492503136e-16 |
| `/objects/3/orientation/0` | 0.0 | 1.1102230246251568e-16 |
| `/objects/3/orientation/1` | 0.0 | 1.2325951644078315e-32 |
| `/objects/3/orientation/2` | 0.0 | 1.1102230246251568e-16 |
| `/objects/3/origin/1` | 1.9999999999999996 | 1.9999999999999998 |

#### ✋️drag-selection / 🪢️cross

| JSON pointer | Committed | Actual native |
|---|---:|---:|
| `/objects/3/orientation/2` | 0.0 | 1.1102230246251568e-16 |
| `/objects/3/origin/0` | 2.0000000000000004 | 1.9999999999999998 |
| `/objects/3/origin/1` | 2.9999999999999996 | 3.0 |

#### 🔄️rotate-selection / ⚠️skips-locked-ghost

| JSON pointer | Committed | Actual native |
|---|---:|---:|
| `/attractions/0/gap` | -2.9999999999999996 | -2.999999999999999 |
| `/attractions/0/shift` | -0.9999999999999998 | -0.9999999999999993 |
| `/attractions/0/turn` | 90.0 | 90.00000000000001 |
| `/objects/1/orientation/1` | 0.7071067811865476 | 0.7071067811865475 |

#### 🔄️rotate-selection / ⛓️orbits

| JSON pointer | Committed | Actual native |
|---|---:|---:|
| `/attractions/1/tilt` | -89.99999829245272 | -89.99999809090409 |
| `/attractions/1/rise` | 0.0 | 6.661338147750937e-16 |
| `/attractions/1/rotation` | -90.0 | -90.00000000000001 |
| `/attractions/1/shift` | -2.9999999999999996 | -2.999999999999999 |
| `/objects/0/orientation/2` | 0.7071067811865476 | 0.7071067811865475 |
| `/objects/1/orientation/0` | 0.0 | 7.850462293418877e-17 |
| `/objects/1/orientation/1` | 0.0 | -7.850462293418876e-17 |
| `/objects/1/orientation/2` | 0.7071067811865476 | 0.7071067811865475 |
| `/objects/3/orientation/0` | 0.0 | 7.850462293418878e-17 |
| `/objects/3/orientation/1` | 0.0 | -7.850462293418875e-17 |
| `/objects/3/orientation/3` | 0.7071067811865476 | 0.7071067811865475 |
| `/objects/3/origin/1` | 1.9999999999999996 | 2.0 |

#### 🔄️rotate-selection / 🔄️turns-two-objects

| JSON pointer | Committed | Actual native |
|---|---:|---:|
| `/objects/0/orientation/2` | 0.7071067811865476 | 0.7071067811865475 |
| `/objects/1/orientation/2` | 0.7071067811865476 | 0.7071067811865475 |


## Full Source-Formula And Third-Party Pipeline Derivation

Executed retained audit input fixture-math-reference.mjs through repo tools light Bun. It reads the actual before+mutation inputs and preserved raw native actual strings. It computes selection survivor order, breadth-first unlocked follower placement and attraction parameter rederivation with two math backends: (1) explicit scalar Hamilton/vector formulas matching current authored source operations and evaluation order; (2) existing external Three Quaternion/Vector3 multiplication, normalization, vector rotation and vector operations. No Rust/production implementation was invoked and no fixture written. Generated detailed result stays under generated/fixture-math-reference.json.

| Scenario | Scalar source formula vs complete actual snapshot | Independent Three vs actual |
|---|---|---|
| ⛓️follow | 0 differences | 5 rounding differences; max 2.842170943040401e-14 |
| 🪢️cross | 0 differences | 5 rounding differences; max 2.842170943040401e-14 |
| ⚠️skips-locked-ghost | 0 differences | 3 rounding differences; max 2.842170943040401e-14 |
| ⛓️orbits | 0 differences | 6 rounding differences; max 2.842170943040401e-14 |
| 🔄️turns-two-objects | 0 differences | 1 rounding differences; max 4.440892098500626e-16 |

All five COMPLETE actual after snapshots match the scalar source-formula reference exactly, not just the29changed fields. The independent Three pipeline exactly corroborates all changed quaternion components, including followed-object tiny residuals, and the changed orbits tilt. Its remaining spatial differences are<=8.881784197001252e-16 and remaining angle differences<=2.842170943040401e-14degrees, from its alternative algebraic vector-rotation evaluation. No semantic discrepancy is hidden by tolerance: the source-formula comparison is exact, and Three is a separate corroborating numerical implementation.

The paired sparse diff changes carry the same independently derived numbers. New shift/rotation fields for follow and rise/rotation fields for orbits are required by current exact patch tests (derived value != before value); their absence in old expectations is stale numerical output shape. No production rounding/snap or comparator relaxation is justified. Every one of the58native field differences is now explained by exact source-formula derivation plus independent math corroboration. Narrow hand correction of the10after/diff expectation files is supported; input/operation/outcome/locking/topology laws stay unchanged.

Handcrafted expectation follow-through: independently read all10current edited after/diff files and compared each complete JSON value with preserved raw native actual output:10/10exact matches. Read-only git diff names under authored3d fixture root list exactly those10files, no before/mutation/outcome/schema/other fixture changed. Existing scalar+Three derivation therefore covers the entire literal correction, with no regenerated collateral. Actual unfiltered green rerun remains pending owner lane.

## Diagnostic retention limitation

The independent scalar/Three audit and complete10-output comparison were performed while the raw original native3d.log diagnostics existed. A later owner rerun reused that log path. This auditor retained numerical differential output and the reproducible reference script, not a separate raw native log copy. Historical committed fixture text remains available through read-only git HEAD; current literals and generated fielddiff preserve numerical evidence, but are not an original raw diagnostic archive. No claim of a separately retained raw native diagnostic is made.

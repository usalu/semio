# FEM Owned Snapshot Scope

This is read-only preparation for the executor's already assigned next family while Grid3d/Flow native verification is blocked on a shared dependency. No FEM provider, schema, capability, executable command or runtime GREEN is claimed. The plugin AGENTS was read. Applicable owners are `🏗️fem/🗿️artifacts/◻️2d` and `🏗️fem/🗿️artifacts/🧊️3d`; their wildcard facet directory is `🌐️any`, distinct from WFC's `✳️any`.

## Exact Persisted Roots

Fem2dSnapshot owns nodes, elements, regions, materials, sections, supports, load_cases, combinations and analysis. Fem3dSnapshot owns nodes, elements, materials, sections, solids, supports, load_cases, combinations and analysis. Both root declarations derive genuine DslRecord and use typed Record Text/Pack ordinary codecs. Neither inspected root ArtifactPack implementation declares SQLite capability. This source observation requires an actual owning capability RED before implementation; adjacent file-token absence alone does not establish registered capability absence.

Fem2d fields are authored in its artifact root: node(id,x,y); elements Bar/Beam(id,start,end,material_id,section_id); material(id,name,e,nu,rho); section(id,name,area,iy); support(id,node_id,fixed ordered DOF list); loads Nodal(id,node_id,dof,value), MemberUdl(id,element_id,wx,wy), Area(id,region_id,pressure); load_case(id,name,ordered loads,self_weight); region(id,name,ordered outline Vec<[f64;2]>,ordered holes Vec<Vec<[f64;2]>>,thickness,material_id,mesh_size); combination(id,name,ordered terms); term(case_id,factor); analysis(modal_count:usize,buckling_count:usize,deformation_scale).

Fem3d differs explicitly: node adds z; elements Bar/Frame, with Frame.roll; material adds g; section adds iz,j; MemberUdl adds wz; Area targets solid_id. Solid owns id,name,ordered outline,ordered holes,base_z,height,layers:usize,mesh_size,material_id,axis. Axis is X/Y/Z, with wire x/y/z. Fem3d combination.terms is BTreeMap<String,f64>, unlike Fem2d's ordered Vec<FemCombinationTerm>. The actual owned DOF type and analysis settings are reused from Fem2d; six DOF variants are Tx/Ty/Tz/Rx/Ry/Rz. No dimensional entity schema should be inferred from a generic object or silently copied across differing fields.

## Handwritten Relational Design Obligations

Each owner needs its own static domain DDL, typed Rust/Source projection and reconstruction, explicit registry/declaration hook and genuine controlled native bridges. Use positive SQL surrogates, root/parent ownership FKs and dense ordinal relationships. Node/element/material/section/support/load/case/combination IDs are literal native text fields; source inspection has not established global native construction uniqueness or reference-resolution requirements. Do not impose a new semantic ID FK/uniqueness restriction merely for SQL convenience. Any genuine owner validation must be inspected and tested separately from model construction.

The geometric region/solid outline and hole loops need explicit coordinate columns, separate ordered hole entities (including empty holes), and ordered vertex rows, with every binary64 component's independently queryable numeric value plus exact signed IEEE word/class companions. All other native binary64 fields need the same preservation. usize counts need the complete actual native width and independently checked scalar storage. No tuple/blob geometry, serialized JSON/native payload or generic property bag is appropriate.

Support DOF relationships preserve order and duplicates because the actual Vec owns both. Fem2d combination term order/duplicates are also owned. Fem3d BTreeMap terms have a genuine unique-key/sorted-order native invariant; any source mirror must use the actual UTF-8 key ordering rather than locale sorting or JavaScript UTF-16 assumptions. Root and relationship widths, every variant shape, optional presence and IEEE companion consistency require explicit reconstruction checks before owned allocation.

The manual intrinsic Value implementations for FemDof and FemAxis currently have only ordinary conversion methods in the inspected source. Genuine controlled native construction/output must author their scalar hooks if the selected typed path actually reaches those Value conversions; no ordinary fallback. Both owners otherwise contain finite typed scalar/vector/map structures, requiring cumulative admission and real collection/UTF callbacks; no retained host scene should be inserted into persisted model tables.

## Required Evidence Before Completion

Measure actual native/source/public missing capability first; handwrite neutral complete fixtures covering all collections/variants and empty/duplicate/unresolved legal literals, all IEEE words and integer boundaries. Independent SQLite table queries/edits, strict schema validation, both declaration-owned erased native encodings, interior cancellation/cumulative bounds and full owning package/type/asset consumers must run. Commands belong to the existing artifact-owned scripts with both launch registrations. Existing solver/editor behavior and published typed facets must remain coherent; snapshot relational tables must not substitute a host solve result or viewport for persisted model fields.


## Complete Authored Domain Stage (2026-10-02)

FEM2d owns eighteen static tables and FEM3d nineteen. Each DDL and neutral fixture is adjacent to its actual snapshot under `🪶️sqlite` and `🧫️fixtures/🪶️sqlite`. Relationships use positive SQL entity aliases only; authored identifiers remain literal, including empty duplicates and unresolved targets. Empty holes have their own ordered entity. All f64 fields have independently queryable REAL plus exact signed bit-word and numeric-class companions. Counts are canonical unsigned decimal TEXT preserving the native unsigned width. FEM3d combination terms use unique UTF-8 sorted keys matching its native BTreeMap, unlike FEM2d's ordered duplicate-preserving term vector. No snapshot JSON/native carrier or inferred schema exists.

The Source providers have separately named exact owned scalar models (`Fem2dSqliteSnapshot`, `Fem3dSqliteSnapshot`) using Binary64 and bigint. Existing finite JSON wire models cannot retain signaling NaN payloads or every native count. These are explicit semantic models, with hand-written per-field projections, rather than compatibility unions.

FEM2d's initial registered Source baseline passed its independent Bun SQLite DDL law and failed the missing provider capability. Its implementation then reached five laws, with the sixth exposing an incorrectly configured test oracle integer mode. The fixed registered Source gate passed six laws / 71 assertions (874ms, 18.4s Nx). The owning public package subsequently passed strict declaration-consumer and suite typechecks, built eight outputs, checked thirteen exports, and passed all six laws / 71 assertions (967ms, 5.7s Nx). A prior strict check correctly exposed readonly test mutation types; the malicious fixture now owns an explicitly mutable clone.

FEM3d initial Source baseline likewise passed independent nineteen-table SQL and failed missing capability. Full handwritten Source provider is now mounted. Its first six-law run passed five laws and exposed that the test used a callback return value instead of the Source API's AbortSignal. The fixture now aborts the actual signal at the interior frontier; fresh verification is pending.

Both native capability baselines executed: FEM2d 0/1 (45ms, 1055 filtered), FEM3d 0/1 (59ms, 975 filtered), each actual declaration missing the semantic provider. FEM2d provider is now mounted after that RED, with controlled derived Text/Pack methods and eight complete native laws. Runtime implementation verification is pending in the single warm executor lane. FEM3d Rust provider remains unmounted/unimplemented. No native implementation completion is claimed.

Owned Rust command routes and both launch files contain combined/native/source: FEM2d 408.671–673, FEM3d 408.677–679. New FEM2d public build/check/test 408.674–676 invoke its package-owned script. FEM3d public package is not yet authored; 408.681–682 were concurrently taken by Puzzle5d and will not be overwritten.

Generated evidence: `fem2d-semantic-capability-source-authentic-red.log`, `fem3d-semantic-capability-source-authentic-red.log`, `fem2d-semantic-source-independent-integer-current.log`, `fem2d-owning-public-readonly-fixture-current.log`, `fem3d-complete-semantic-source-current.log`, `wfc-flow-fem-native-canonical-ui-current.log`.


## Verified Complete Source and First Native Implementation

FEM2d native implementation passed eight selected laws (828ms, 1055 filtered), including all scalar words through actual declaration-owned erased Text/Binary, independent Bun SQLite editing, invalid graph/storage, and real interior ownership/cancellation. Nextest evidence is `fem2d-flow-native-controlled-first-repair.log`. FEM3d Source implementation passed six laws / 76 assertions (1.89s, 26.9s registered Nx). Owning FEM3d public package passed strict declaration-consumer and suite checks, built eight outputs, checked three runtime exports, and passed six laws / 76 assertions (1.92s, 13.0s uncached).

After correcting fixture AbortSignal use, FEM3d's known-node-progress law genuinely failed because the shared inserted-row stream has no known collection total. The authored node loop now publishes its actual input collection total every 256 nodes; the exact same law passed. Source evidence `fem3d-source-signal-cancellation-current.log` → `fem3d-known-node-progress-current.log`.

FEM3d native nineteen-table provider, exact wildcard guard, controlled Text/Pack hooks and eight native laws are now mounted after the executed capability RED. Both Rust providers share only borrowed row validation/admission primitives in plugin-owned `🧩️sqlite/🦀️.rs`; all domain fields/table names/column positions remain individually authored. FEM2d's eight-law green preceded this extraction, so both providers' fresh post-extraction gate is pending; no fresh-green inference is made. Public FEM3d build/check/test launch orders 408.685–687 were confirmed with parent and added to both catalogs, preserving concurrent Puzzle entries.
# Fresh Shared-Helper Native Verification

The registered uncached two-owner run completed successfully after the bounded relationship helpers were shared: FEM2d passed all eight selected native laws; FEM3d passed all eight selected native laws (Nextest `8ebab742-a713-415a-8f5f-e24e3dfeeba0`, 1.326 seconds assertions). Combined Nx duration was 8 minutes 11 seconds. The retained log is `🗑️generated/fem-two-native-shared-row-admission-current.log`. Whole owning Rust suites remain pending and are separate from these selected results.

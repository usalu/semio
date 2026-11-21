# GIS WASI Link Architecture

GIS already uses the existing definition-only stdio catalog. Its catalog source states that the projected commitment matches the full catalog while IFC standards and semio conversions are omitted. The GIS plugin calls that definition-only projection for the dependency and catalog topic. Changing the full catalog to this projection is therefore not an available reduction for this build.

The separate `SemioMembers` generic remains in both GIS map app types. Its closed `dsl::space_members!` expands eighteen typed snapshot, mutation, and producer variants, including B-Rep. Removing variants changes which members a map document can materialize. The semio artifact also keeps executable declarations and registrations alongside its schema-owned definition. Isolating the projection alone does not replace this dispatch.

The GIS component manifest describes one cdylib, and its entry point exports the whole `GisApps` enum containing map and terrain editors and viewers. Existing stdio family components separate their apps, but no alternate GIS component or complete substitute member dispatcher was identified. A narrower GIS plugin or member enum would alter the declared app or document semantics and requires deliberate design, not a build selection switch.

The managed component pipeline applies `wasm-opt -Oz` after linking and transpilation, so that existing stage cannot prevent the linker encoder's pre-publication function-count refusal. No alternate Cargo profile was tested, and no claim is made that a profile change would fit the encoder limit. No builds or production edits were made in this audit.

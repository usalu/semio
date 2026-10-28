# IFC2x3 Normative Catalog

Complete declarations use the exact unchanged pinned buildingSMART IFC2x3 TC1 source. Manifest/hash/provenance are retained next to production source, preserving original notice and repository-final-IFC CC BY-ND4.0 notice. Independent source/license audit: ifc2x3-catalog-audit.md.

Schema-first neutral vectors and tests preceded facade/data implementation; missing-facade red was recorded. Shared first-party extractor and Rust renderer produced653entities327types(164enum46select117defined),38functions,2globalRULE. Both languagefacades delegate the existing schema-neutral declaration/query implementation; no copied validator or runtimeparserdependency was introduced. Nativebase model mounts its owncatalog; publicTypeScriptAPI uses ifc2x3-prefixedexports.

ActualTypeScript3groups pass with zero diagnostics: unchangedinputchecksum, completecatalog/reproduciblenativeoutput, independentAJVschema, neutralinheritance/attribute/enum/select/derivedmarkerdomains and actualIfcOpenShellmetadata facts. All653entitynames327typenames1402documentedattributes88completepredefinedenumdomains agree with existingindependentpackage metadata. CompiledIfcOpenShellschema introspection is not executed here.

Preserved IFC2x3 distinctions include IfcWall8attributes with no IFC4PredefinedType, absentIfcSpatialZone, IfcProject subtypeIfcObject and unrestrictedIfcLabel STRING instead of IFC4STRING255. Retainedrules/functions are NOT evaluated; everyvalidationcapability remainsfalse. No fullEXPRESS/MVDconformance claim.

Nativecatalog tests are authored; authentic nativecompilation/tests remain behind the recorded repositorycachecase-sensitivityrefusal. IndependentRustgrammar validation is pending below. Inputgenerator/harness retained in this ticket; own generated outputs will be removed after finalchecks.

Releaseversion2.3.0.1 verified against the official specifications database: https://technical.buildingsmart.org/standards/ifc/ifc-schema-specifications/ and officialrelease notes https://technical.buildingsmart.org/standards/ifc/ifc-schema-specifications/ifc-release-notes/.

IndependentRustgrammar checked facade/generateddata/nativevectorstest/ownedmodelmount:4files,zeroerror/missingnodes. This is syntaxevidence, notnativecompilation/execution.

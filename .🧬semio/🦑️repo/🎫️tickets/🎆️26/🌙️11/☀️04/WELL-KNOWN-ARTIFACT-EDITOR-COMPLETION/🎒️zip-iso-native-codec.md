# ISO ZIP Natural Codec

Audit's native source roster test reproduced the missing ISO ZIP capability (3 pass,1 fail) before implementation. The ISO editor now declares the .zip/application/zip binary source under s.stdio.zip@2.0, keeping its exact ISO21320 dialect and stdio.zip artifact schema.

The editor shares the existing base ZIP encoder/decoder and enforces existing ISO snapshot conformance on output plus ISO wire/snapshot conformance on input. It does not publish generated descriptors.

A language-neutral JSON fixture and schema pin owner, schema, source metadata and independently decoded member content. A real trait regression in the mounted ISO editor unit source checks metadata, decodes the existing 119-byte rename fixture, roundtrips its exact snapshot, rejects a valid BZIP2 ZIP (method12), and refuses encrypted snapshot output. Python zipfile independently creates/reads the BZIP2 archive's Hello payload; the existing fflate library independently reproduces the valid archive's new.txt/Hello member output.

The ZIP workspace is in the live sequential compile plan; actual Rust regression execution is pending.

Audit's source lifecycle roster is green after the ISO implementation:6tests,0failures,274assertions,167ms. See source-lifecycle roster report for its exact six authored test/schema/fixture files. Independent AJV+fflate oracle also passed. Rust trait regression remains queued.

Corrected my earlier ZIP-editor testkit dev dependency: the artifact workspace has no inherited semio-framework-value entry. The dev declaration now uses the exact existing normal first-party path plus testkit, preserving the source identity. This manifest error blocked Semio/DOCX dependency resolution before Rust; both SDK verdicts remain pending. No external fetch/dependency addition is needed.

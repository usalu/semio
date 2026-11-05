# Playbook Receiving Owner Preflight

Actual5d40764 stopped before tests on shared Playbook derives; no5d test verdict. Read-only audit only.

FormGeneration and GenerationPlayState at Playbook artifact root361/371 derive NativeReceive with explicit value(native_owned=semio_framework_pack_json). Current derive resolver at Value derive expansion376 takes explicit native_owned first, then explicit value crate, otherwise canonical semio_framework_value_native_owned. Generated receiving paths legitimately require native_decoding, NativeRecordStorage and OriginalReceivingPrimary from that actual native owner. Pack JSON is the canonical JSON tree owner, not native receiving; missing paths are cascading symptoms of the two incorrect attributes.

Repair only native receiving ownership: remove wrong override so canonical native-owned default applies, or explicitly name actual semio_framework_value_native_owned, and declare its existing repository package as a direct dependency. Preserve canonical_json(owner=semio_framework_pack_json), fields and all original controls/receipts. Do not restore obsolete Pack JSON native exports, compatibility facade or weaken derive. Other Playbook CanonicalJsonTree-only derives are not this error.

Actual native-owned package glue exports native_decoding and its contents. Root manifest search found no existing direct native-owned dependency before repair. Guidance sent integration; actual patch review pending. No source/compiler changes by auditor.

## Repair review

Current two type declarations remove both incorrect native_owned overrides and therefore use current derive default. CanonicalJsonTree retains pack-json owner; fields/rename/default/skip attributes untouched. Manifest adds existing repository semio-framework-value-native-owned path, independently verified physically present. NativeReceive/RetainedClone requirements remain instead of deleting derives or restoring obsolete Pack JSON exports. No receipt/control or law change. No further concrete mismatch found in this bounded repair; actual5d/native compilation still pending.

## Current Playbook Explicit Owner Repair

Integrationauthoritativelyconfirms83992terminalCargo101andcurrentbadreceivingattrsactuallypresentbeforeedit, not staleSource/defaultfault. Independentfinalread verifiesFormGeneration362 andGenerationPlayState372 explicitlynative_owned=semio_framework_value_native_owned, separatecanonical_json(pack_json)363/373unchanged. Actualderive container_native_owned_path377–379 respects explicitreceiver, elsevaluecrate, elsealreadycanonicalNativeOwned; no macro/defaultrepairneeded. Existingdirectdependencyrelativepathresolvesexactcanonicalnative-ownedCargoowner. Payloadfields/derives/receipts/laws/capsunchanged; currentpatchsource-reviewedreadyforcoordinatednativephysical2dretry, not runtimesuccess.

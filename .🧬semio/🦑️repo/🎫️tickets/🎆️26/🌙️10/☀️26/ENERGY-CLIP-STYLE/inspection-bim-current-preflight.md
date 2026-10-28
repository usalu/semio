# Current BIM Read-Only Preflight

Current source bytes were read afresh and checked stable before/after. No native scene, source encoder, browser capture, producer or product test was started. The existing Ajv8.20.0 dependency validates the full current draft2020-12 model schema through the repo Node tool port.

## Authored Narration And Providers

- bim_information: 21 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.
- bim_objects: 19 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.
- bim_representations: 18 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.
- bim_models: 19 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.
- bim_exchange: 16 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.
- bim_consistency: 20 current clauses; exact portable key order, explicit EN/DE text, handler present, all14required providers in actual literal source closure.

Total113clauses is freshly derived from the six current leaves, not assumed from history. Current renderer and panel calls bind to the actual provider signatures without error.

## Concrete Schema Blocker

The only failing check is current model data against its current schema. At /translation/losses/8/applies_to/class, the authored value IfcSpace is rejected by $defs/ifc_class.enum. Its current loss rule explicitly says Room: spaces have no structural meaning in English/German. The schema enum includes twelve other IFC classes but omits IfcSpace, even though the model has rooms as canonical IfcSpace objects.

No schema/data/provider file was edited during this read-only assignment. The legitimate room-loss rule should be preserved; its canonical class enum needs coordinated schema correction and a regression fixture rather than deleting the authored content or adding a permissive fallback.

## Evidence

Inputs: inspection-bim-current-preflight.py and inspection-bim-schema-current.mjs. Generated source hashes, current keys, closure results and Ajv errors: generated/ip/result.json and schema-validation.json. Diagnostic invocations use unique owned logs; no new dependency was installed.

## Resolved Selector Blocker

The coordinated dedicated-selector schema repair now passes eight focused repository/Ajv cases and full-model Ajv validation. See inspection-bim-selector-schema.md. Current six-leaf narration/provider checks remain freshly derived; no native source producer was started by preflight. The owned subset's separate draft2020 prefixItems limitation is explicitly documented rather than hidden.

## Final Read-Only Rerun

The post-fix current preflight passes with stable before/after input hashes: freshly derived clause counts21/19/18/19/16/20; exact portable key order and both locales; all six leaf handlers; all14required providers in every actual closure; current renderer/panel signature binding; complete current model accepted by Ajv8.20.0. This source-contract check does not claim native lifecycle acceptance; the source115 FadeOut runtime failure is separately owned by the verifier.

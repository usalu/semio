# Checkpoint Domain Fixture Join

The root verifier referenced an unavailable ticket-local JSON input. Its original bytes were not available and were not recreated. This repair replaces that permanent ticket dependency with the already-existing native contract fixture at `plugin/🧵️retained-command/🧪️fixtures/🔣️artifact-command-checkpoint.json` and its adjacent schema. The native test `checkpoint_binary_matches_schema_fixture_and_owned_oracle` already consumes those five vectors.

The new source test uses strict Ajv 2020 validation, independently encodes accepted vectors through DataView and Node Buffer, checks exact capacity outcomes, and rejects byte 256, incorrectly accepted maximum-plus-one, incorrectly rejected exact maximum, wrong header size, and unknown fields. It executes 15 checks, replacing three old ticket-vector checks. No native production checkpoint code changed.

Actual canonical RED is retained in `🧪️checkpoint-domain-schema-red-r1-2026-08-27.txt`: the existing schema required a capacity-error work length of 473, but the authoritative version-3 fixture and native 48-byte header require 465 (512 - 48 + 1). This was a genuine stale schema boundary, not a missing-input bypass. The corrected domain schema uses 464/465 and a version-3 identity; it keeps all closed object, byte, integer, and conditional outcome constraints.

Actual canonical GREEN: `🧪️checkpoint-domain-schema-green-r2-2026-08-27.txt`, exit 0, **1,009 source self-tests**, exact factory owners 33 / custom rows 255 / generic rows 25. This is source/schema evidence only, not a new native checkpoint or all-app runtime result. Lost ticket evidence is not claimed recovered.

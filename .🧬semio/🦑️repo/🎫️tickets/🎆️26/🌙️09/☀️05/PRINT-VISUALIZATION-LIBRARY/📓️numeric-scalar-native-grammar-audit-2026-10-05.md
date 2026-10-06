# Numeric Scalar Native Grammar Audit — 2026-10-05

Read-only source-backed handoff. No product/schema source was changed by this audit.

Current schema SHA: 
2c19256f93dbd22997dd2084ff672450c7cf5515ad440948ea083cd95cd0cc4d
. Current actual authored-transport snapshot SHA: 
3dbb84e4cdbddd88e5ed4eb5a6419e19f06b1a3f6195657192e56909ca953895
.

All 905 currently Scalar numeric family descriptors have an exact key in an actual authored reachable native scope with an explicit fp_set:N or int_set:N setter. The retained contracts record each family/key, its current descriptor, source file/hash/line/variable and exact source scope. This extends the independently retained 603 direct-key candidate list through the current authored transport scopes; it does not infer handlers from variable names or storage types.

The first scanner pass retained 807 matches and 98 unresolved mark cases because repeated key declarations replaced earlier entries in its map. The corrected read-only scanner merges declarations for the same exact native namespace; all 98 remaining entries are explicit mark fp_set:N declarations. No runtime or source classification was changed by that harness correction.

Proposed owned schema action: change only these exact descriptor syntax fields to Expression, preserving primitive types, defaults, minimum, enum, bilingual meanings and all other descriptor/schema bytes. Scalar remains literal number/boolean grammar. A native samples=2+2 compiler control is required in the descriptor admission proof.

Input: 📥️authored-inputs/native-canonical-option-syntax/numeric-handler-source-contracts.json. Existing source-owned scanner: 📜️encoder-edit/📜️script.ts numeric-handler-audit. The source scopes are taken from printed-api-current-snapshot.json, refreshed at Catalogue session 89555; that snapshot is not a final schema/API freeze.

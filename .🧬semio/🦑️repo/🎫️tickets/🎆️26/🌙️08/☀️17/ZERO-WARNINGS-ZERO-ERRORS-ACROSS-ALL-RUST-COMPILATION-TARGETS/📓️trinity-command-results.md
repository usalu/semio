# Trinity Command Results

Pass 451 removes Result wrappers from 17 Jack command helpers whose successful result already represents every branch. Parse/query errors remain represented exactly as before: empty emits or query result payloads. The plugin trait handler wraps its final match once; the retained config reducer preserves its real route-mismatch Fault and wraps only the eight accepted commands. Internal parsing Result matches remain unchanged. Compiler and existing Jack dispatch tests are pending.


Pass 464 applies the same signature cleanup to eleven Rewriting command helpers. Each helper already produces an Emit for every branch, including rejected parsing. The ArtifactApp handler wraps the final dispatch match once, preserving its required Result contract. Runtime dispatch tests and compiler validation remain pending.


Pass 471 removes unnecessary Option wrappers from the Jack and Rewriting LOD serializers. Both automatic and forced modes always produce JSON. Scene construction explicitly sets Some(json), preserving the existing optional scene-field contract. The configuration values and rendered JSON are unchanged. Compiler and existing scene checks remain pending.


Pass 475 removes impossible missing-string/error wrappers from Jack's private retirement helpers. A required string always yields a retirement step; the owner advance helper has no Err branch or error propagation. The public erased-retirement contract still returns Result, and optional owner absence remains Option. Item and byte budgets, phase transitions, active child retirement, and owner releases remain unchanged. Compiler and existing retirement tests remain pending.

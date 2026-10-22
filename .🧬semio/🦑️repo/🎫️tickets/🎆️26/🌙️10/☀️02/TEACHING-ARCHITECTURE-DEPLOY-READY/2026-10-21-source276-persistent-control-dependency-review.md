# Source276 Persistent Control Dependency Review

Read-only static inventory of the83 persistent control paths from existing prefix inputs, plus six prefix-owned and eight guest-dependency paths (97 unique paths). Excluded the seven already-reviewed updated metadata files from the search for additional assumptions; reports and generated history were not scanned. No Source/helper evaluation or Native invocation.

No additional runtime roster or combined parent/worker/host budget constant needs changing outside those seven metadata files. Existing historical inputs seals still need the already-planned full dependent regeneration; this is not new runnable clearance.

Concrete runtime dataflow:

- combined.ts19/33/37 reads phases and parentMilliseconds from combined.schema.json, issues that parent grant, recomputes phase sum/reserve and compares cases DATA. Receipt45 derives phaseCount and Source count from actual arrays/maps.
- guest entry19/21/38 uses policy.workerMilliseconds for wire budget, host grant +120000 and transport cap +30000. guest.py182/197 projects wire milliseconds to fresh native budget and enforces the same wall cap. No older worker number is embedded.
- guest core28/36 and guest.py136/144 compare copied exact policy.sourcePaths and current phase array/order; receipt counts derive from current Source projection and phase list. Tests likewise derive their source/phases counts.
- prefix core24/38 and entry16 derive late critical roster and prefix grant from current policy; production derive.ts25 compares against current derive-schema criticalPaths. No historical252/260/274 roster constant controls these paths.
- Independent fixed owner-local ceilings (capture900000, Git300000/330000, provisioning/transport bounds, cleanup reserves, existing controller21) retain their own roles. They are not stale neutral-parent or worker substitutes and require no276-dependent update. Fixed minimum capture critical-path DATA remains a minimum union, not the complete production roster.

This bounded check found no extra dependency beyond the prior Source276 field-role/seal checklist. Actual final physical seal checks and future captured-manifest projection remain separate.

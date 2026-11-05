# Shooting Schema Verification

The public artifact JSON Schema declares additionalProperties=false at the document root. The TypeScript parseShootingArtifact only calls the object guard there, unlike the closed nested parsers. It silently strips unknown root fields. Added neutral parsing scenarios and AJV comparison to the declared shooting-js test route before changing the guard. Cases also cover nested unknown properties, fixed geometry tuples, uint32 shot dimensions and optional material stroke default. Awaiting queued test admission for red evidence.

After adding fixture/tests first, changed the root guard to the closed object guard already used by nested values. Runtime red evidence was not available because target remained capacity queued. Verification is pending; no pass claim is made. Registered the dynamically dispatched parsing, sun, camera and existing example tests as explicit runtimeSourceEntries, with deterministic test inputs/outputs/cache metadata.

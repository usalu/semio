# Renderer Close Correction

The revised take_probe moves the probe unconditionally before the mounted session begins close, removing the previously reported checked-out handoff deadlock. The direct job close now returns Pending with the actual retained-progress delta after a productive PNG close, so the worker's nested-progress reconciliation receives the effects it expects. The worker is driven via its same-original-caller path; a genuine boundary law remains needed for full validation.

ValueError::invariant is a const call to ValueError::literal, which stores a borrowed Cow<&'static str>; it does not allocate. This is source review only; no native test was run.

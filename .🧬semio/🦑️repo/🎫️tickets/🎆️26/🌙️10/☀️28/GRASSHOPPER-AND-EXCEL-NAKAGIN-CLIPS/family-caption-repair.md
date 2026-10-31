# Family Gallery Caption Repair

A fresh partial closed-family image written2026-10-30T06:32:56UTC was inspected directly at1920x1080 while the family worker was still running. All six captions touched or overlaid the lowest capsule corner; the fixed row offset y-1.7 did not measure the model bounds. The partial image is not an accepted complete capture. Its rawSHA256 is cc29392548b776e576f5a28f2033264356021803a9f4e55f0c179376163393b6.

The ticket capture fixture now positions each caption below its actual complete drawing bounds with0.16world-unit clear space. Assertions require at least0.15units between all model ink bounds and caption, plus0.05units inside the lower frame. No model geometry, rendering kernel, material or production source changed. The live capture checkpoints will reject the changed fixture and terminate only their owned worker; fresh capture uses a new immutable baseline. The separate Visual22 suite does not consume this ticket fixture and continues.

A preceding fixture-only typo expected bolt0-head instead of the canonical bolt-0-head identifiers. It was corrected without changing the physical model. Both failures remain non-passes; neither image count nor an old same-name PNG is promoted to current visual acceptance.

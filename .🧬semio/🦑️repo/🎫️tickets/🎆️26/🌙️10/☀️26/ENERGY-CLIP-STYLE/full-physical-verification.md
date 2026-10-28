# Full Physical Fundamentals Verification

Status: all independent media assertions passed. Visual review of the generated coverage and navigation sheets is recorded separately in full-physical-visual-audit.md.

The canonical manifest contains the complete PhysicalFundamentals_FullSection in exactly the eight declared beats, without an institute introduction or clause excerpts. PyAV successfully decoded every source, authored content and delivery frame. Pillow assembled the independent coverage sheet.

| Source | Frames | Duration | Body Start | SHA-256 |
| --- | ---: | ---: | ---: | --- |
| Beat1_Leistung.mp4 | 1034 | 172.329s | 0.000s | b1ed88e42f0f973ca876c95c45716cfd324fedaa7422d0773157b3ed527bad5c |
| Beat2_EnergieImAlltag.mp4 | 1090 | 181.664s | 172.333s | 8525c83343eaf6389c4c463b4daaccc84418a912955ad550c8b25b056dd182ff |
| Beat3_Energieerhaltung.mp4 | 612 | 102.000s | 354.000s | 7898063a56dc3d11720f76c231524d35ac5be39d4826216c39e655c697f0645a |
| Beat4_Waermepumpe.mp4 | 838 | 139.665s | 456.000s | 36f058932eaefcc2bbe9c42e87a14395fb41c9b5a0612fa7043ed28138932c04 |
| Beat5_Strahlung.mp4 | 585 | 97.500s | 595.667s | 891119a7286d9e308b08ef46962f5ab63d714ac5905be01a8d4d92d1c62fd3d9 |
| Beat6_ThermischeMasse.mp4 | 563 | 93.832s | 693.167s | 557d12c5a8856b4a47068d98e10953b2a256c0910a8f2722e82842cceb6a0b04 |
| Beat7_SensibelLatent.mp4 | 956 | 159.332s | 787.000s | 648a8da206b4621ddd0802e14e28099601f974cb0a7752ca0ac55008b7250d2a |
| Beat8_Zusammenfassung.mp4 | 483 | 80.499s | 946.333s | f972368853aa1dac0b5367e01a4e974b8257ce0e99f6d2ee38613b90c3059782 |

Source durations use container average-rate metadata; minor millisecond differences reflect timestamp quantization. Body starts use the nominal 6fps frame budget.

The authored body has 6161 frames (1026.833s). Each 3.6s navigation sequence is rounded to 22 frames (3.667s) at 6fps. Delivery has 6205 frames, lasts 1034.167s, and is 640 × 360 at 6fps. Frame timestamps are strictly increasing with a regular 1/6s cadence.

Every body frame was compared in order against both the concatenated content and the corresponding canonical source. Maximum source-to-content mean/edge RGB error: 0.00000000; maximum body mean RGB error: 0.00000000; maximum body outer-edge mean RGB error: 0.00000000. This checks source ordering, all eight complete segments and unobstructed full-frame playback across every pixel.

Opening-camera seam mean RGB error: 0.67037037; closing-camera seam: 0.84289641. The opening and closing overview root frames are pixel-identical. The exact endpoint images and source midpoint coverage sheet are in the owned generated verification directory during review.

All eight canonical rendered sources contain no audio stream. The delivery contains one decoded 48000Hz audio stream with 1034.166s decoded samples, peak 0.0000000000 and RMS 0.0000000000; this is explicit silence rather than narration.

## Evidence Identities

- Manifest SHA-256: 01407e4ad8ab2a665496f1af1ec5e414a1586dc47921c5f8896aee592ac6dcfe
- Presentation catalog SHA-256: a9b3e5e2bf251cd4d5c1b9400afeb0b55d52398ed5409f3d8b63a5ac8ec8b91b
- Authored content SHA-256: 7ceac2d762e400169c25d7edf545bc6e158f9eed60ef0ddb75de479203435c2e
- Delivery SHA-256: b7985afec56b9f97a3d880b3c235ba757861e1f14ab14d875c28dc0321982752
- Delivery path: .🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️26\ENERGY-CLIP-STYLE\🗑️generated\full-physical\de\spectator\rendered\Full_Physical_Fundamentals_spectator_de_640x360p6.mp4

The verifier is retained as full-physical-verification.py. Detailed transient measurements and PNGs belong to 🗑️generated/full-physical-verification and may be removed after review.

## Retained Delivery

[Complete video](./full-physical-updated.mp4) is byte-identical to the verified publication. [Coverage](./full-physical-coverage.png), [opening navigation](./full-physical-opening.png), and [closing navigation](./full-physical-closing.png) remain available after generated diagnostics are cleaned. The independent [visual audit](./full-physical-visual-audit.md) passed.

# BMP Preview and Conversion Fidelity Follow-up

Current BMP canonical bytes preserve original source representation and the first editor-assembly run passed37tests. The following source findings still affect end-user behavior and have been assigned to the BMP execution lane.

1. The main image window currently catches encoding/layout failures with empty bytes and zero dimensions. It sends image/bmp to the browser for every admitted profile. Unsupported preview must instead render an explicit localized unavailable state while leaving Details/source editing mounted. The TIFF lane owns a shared ImageWindowKit fallback for both formats; supported profiles should use an ephemeral first-party PNG projection.
2. The new Semio image exporter calls bmp_direct_rgb24_from_rgba8 unconditionally. That constructor copies RGB and ignores each RGBA alpha byte. For transparent input this silently loses alpha. Output must choose a lossless alpha-capable supported profile, or explicitly refuse an unavailable conversion instead of flattening. Required neutral/independent witnesses include alpha0, partial alpha and255.
3. Metadata, DPI, palette and source-profile controls still need an end-user usability review beyond raw bytes and region-paint actions. Native codec tests alone do not establish that these controls exist or are practical.

Evidence: BMP v3 any main image window and io/🦀️.rs constructor around340–380; Semio image BMP serializer invokes that constructor. These are source findings, not fresh browser failures. Existing canonical import/export fidelity and the typed constructor conversion are distinct boundaries.


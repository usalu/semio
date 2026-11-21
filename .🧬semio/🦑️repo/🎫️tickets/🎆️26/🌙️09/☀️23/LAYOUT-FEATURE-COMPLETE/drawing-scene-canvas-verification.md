# Layout Drawing Scene and Canvas Verification

The Scene companion no longer binds the removed inline Drawing codec. Its existing fallback law still checks the linked kind label and stroke marker. The new native law puts two Drawing links on the same active page and checks that the frame targeting the authored Drawing receives its content placement bounds. The frame targeting a different Drawing keeps the stroke marker and has no Drawing placement bounds. The existing frame open action supplies the image link’s artifact reference as the document ID and constructs the dialect URI separately.

The Canvas companion checks that the authored member reference remains the original document member and that synchronous layer assembly yields no inline PNG. The retained render law uses the existing neutral Drawing corpus, the actual Draw snapshot admitter and vector provider, and bounded work and retirement. It expects one fitted path and one image at each destination. The two published PNGs decode to the original opaque red pixel.

| Portable expectation | Result |
| --- | --- |
| Frame targets the original Drawing | Placement bounds `[136,435,60,40,0]` |
| Frame targets a different Drawing | No Drawing placement bounds and the stroke fallback |
| Unit square fitted onto the page | Affine `[400,0,0,400,0,50]` |
| Unit square fitted inside the matching frame | Affine `[40,0,0,40,146,435]` |
| Draw raster sample | One pixel with RGBA `[255,0,0,255]` |

An independent Node check read the default Layout geometry and Draw corpus, computed the two aspect-preserving fits, and used a direct artifact-ID join for matching and foreign frames. Both matrices, both identity outcomes, and the raster sample matched the table. The pinned Rust parser accepted all three modified test sources. The whitespace check passed.

The first focused native command selected the repository root Cargo workspace and stopped at package resolution. The explicit Layout workspace retry passed package resolution and source admission, then exited 101 during dependency compilation, before any native test executed. The last visible unit was `semio-framework-pack-record-encode-cursor`.

# Paged PNG Decode Regression

The neutral PNG fixture reshapes the existing 3 × 5 Adam7 indexed image into a 16,386-byte source using four valid ancillary chunks. The Palette, Transparency, Image Data, and End checksum reads each cross a separate 4096-byte page boundary. The TypeScript law independently generates the padding and confirms the fixture CRCs, field bytes, and full RGBA raster with pngjs.

The Rust companion law checks the physical page splits, exact field bytes, and original page addresses before and after admission and decode. A short copy grant, a short work grant, and an invalid byte limit preserve the source owner and perform no allocation or release. Exact admission moves the original pages without allocating. Every exact decoder turn compares progress and heap receipts with the vector source, and short grants near each seam or allocation leave the decoder unchanged. The published image matches both the vector decoder and the independent `png` crate. Closing the paged job and raster reconciles physical allocation and release totals.

The complete TypeScript PNG suite passed all 76 tests through the repo tool registry. The Rust companion law is included in the parent agent’s warmed verification lane.

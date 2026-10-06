# Current Geographic Gallery Layout Independent Audit

Current ordinary titlewave13588 owner10 is selected by exact publication/receipt manifest. Disk PDFs light320CD39D64741EABE9E1603A1BECD233373E8AE1F8999BB6FCD7D1D9C1228A40 and darkD1F592ED47802C1B7FFB2E7695ED5288EFAE130204276FCDFC2DD541EE6BBCCD were independently hash checked. Changed bytes require new views; prior page1 baseline was not transferred.

Fresh Poppler views: page1 both palettes130dpi; pages3/5 both palettes110dpi. Page1 basemap/physical/street content fits reviewed frames, with small topographic geometry relative to its grid. Page3 choropleth/dasymetric content renders, while palette swatches sit below the figure frame. Page5 spatial-density contour strokes cross the top caption/header and frame; hexbin marks also cross their caption/top frame. Bivariate/multivariate polygon content is visible, with legend swatches below the frame. These are actual current stock gallery layout defects, not an inference from compiler warnings. Root and Catalogue were notified before further source decisions.

This bounded six-page review does not claim every owner10 page is reviewed. Existing numerical/palette/registry proof remains separately valid; it does not admit current default layout cleanliness. No product/test edit, compiler restart or duplicate job by this lane.

- light p1 SHA E0C3A86ADE7738D62432D730B7B446AD9C58343E4E9694B0E041DDC583269073
- light p3 SHA 632644F5527290544119570828F300D215B0A10A7C88148A4D1E7B6E5C1EF743
- light p5 SHA D049D992DD86088C517DD761AE24D3BF80837036D74A89D1AA0F443070061CEF
- dark p1 SHA DE049420B5D8604B9318D270FC2DDD7FC51A23EB5E767FC59F8B00250776FC1E
- dark p3 SHA 0BF748D9BB673C0606C018DB3C084F5AFB1D61857213A592A6876468F285C919
- dark p5 SHA 5F1C8803F919A4D3E021781C7A19C03655B3B872C02D49509C94A7387EB6EA02

Source-bound cause: actual gallery VizFigures are80×40. The nativegeo-hexbin dispatch applies base/own options but has no frame transform; density computation hardcodes16×11 samples/origin0,0/cell5 and ring drawing multiplies grid coordinates by5, yielding vertical extent around50 plus boundary outside40. Hex vertices use raw bin centres plus0.92radius with no canvas fitting. These are native family rendering coordinates, not a changed test-only carrier transformation. Existing choropleth legend uses y−5 to−2.4, proving its below-frame placement. Raw bin/density algorithms can retain numerical semantics while a separate source-owned canvas projection/ink margin handles rendering; Native owns concrete design and neutral proof.
- 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🗺️viz-10.tex SHA 925D67DAB26E7A5DBA000CB66C5422D5B8F923A6D6FFF0FD7C2B9FB75D5C0177
- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-symbols.sty SHA 72E4EDFE02E5C85C0087C862231E156C84C9ABC2B46B8869BCF2D86A914CE0EF
- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-choropleth.sty SHA B27BFFF38A8910BB0516818C09262C57B52D4C960EFED47849400933CDACE949

Raw dataset corroboration: semio-viz-spatial.sty1233 authors24 demo-points-dense points, including67,51. Existing raw hexbin centres/vertices therefore extend above a40-unit canvas even without the density grid. The authored data and D3-equivalent numerical kernels should remain independently testable; the production render projection is the failing boundary, rather than changing stock coordinates to conceal it.

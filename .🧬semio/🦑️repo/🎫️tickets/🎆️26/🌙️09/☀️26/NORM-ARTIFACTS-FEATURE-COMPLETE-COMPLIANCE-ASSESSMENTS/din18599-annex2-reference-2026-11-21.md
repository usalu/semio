# GModG non-residential reference envelope

The reference building retains the supplied geometry, operating setpoints and shading factor. It applies the separate Annex 2 U, solar and visible transmittances for each component and temperature band. The energy balance uses the U and solar transmittance values and shows the visible transmittances in the catalogue. Thermal-bridge allowances are applied by actual area and setpoint for the components listed in item 1.11, excluding curtain walls. The residential reference uses Annex 1. [GModG §18](https://www.gesetze-im-internet.de/geg/__18.html) · [Annex 2](https://www.gesetze-im-internet.de/geg/anlage_2.html)

The non-residential primary-energy comparison remains an indicative result. The reference plant, daylighting calculation and replacement glazing properties for an explicit solar-control glazing specification require further model inputs and normative implementation. The transmitted-solar-gain calculation accepts only a glazed component, including the new roof-window and rooflight-strip types.

Fifty independent SQLite comparisons covered the reference rows, both temperature boundaries and out-of-scope entries. The two bridge-area cases also passed. The SQLite TypeScript suite passed 40 tests with 1,238 assertions after the additional element types were added. The native scene build remains subject to disk capacity.

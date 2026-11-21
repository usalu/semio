# Retained cylindrical Shell

`brep.solid.shell` now accepts a live certified finite Cylinder, including a tilted and translated source. With no selection it creates a closed cylindrical cavity. Selecting the bottom or top cap opens that end, and selecting both creates a through tube. The wall and any retained end caps have the requested normal thickness. Removing both caps permits a short tube even if its height is smaller than twice the wall thickness.

The existing funded selection validates each source face and uses its outward native plane normal to identify the cylinder's cap. Curved side and foreign faces are refused, along with zero wall thickness and radial or axial collapse. The circular trims, cylindrical surfaces, planar caps, inner shell, preview and exact material measures use the existing annular geometry certificate. The finish uses the existing private solid staging and cancellation path.

## Evidence

- The neutral `🔣️original-cylinder-shell.json` covers five positive geometries and six refusals. Manifold independently subtracts the inset cylindrical cutter, transforms the result and compares volume, area, exterior bounds, Euler characteristic and point classifications. The combined cylinder and box Shell oracle passed nine tests and 86 assertions.
- The compact production-source Rust proof `🐍️cylinder-shell-source-check.py` passed 29 tests. It exercises the native stock and opening recipe, annular incidence, exact three-dimensional edge-to-trim coincidence, point classification, analytic area and volume, independently integrated preview and refusal paths.
- Registered `original-brep-booleans` exercises the real retained Shell operator over rotated and translated stock. It selects live cap handles, checks foreigners and the side wall, verifies complete rollback after a cancellation once private faces exist, and checks the final topology, preview and exact registered Measure and Classify results. The managed linked target remains pending available build capacity.
- Canonical source bounds passed for Native Registry at 3,985 lines and 217,162 bytes, Solid Transaction at 3,821 and 198,100, Flow BRep at 2,796 and 151,488, and original-booleans at 1,308 and 159,359.

This scope covers the certified finite cylindrical stock family. Existing box and closed spherical shell behavior continues through the same operation.

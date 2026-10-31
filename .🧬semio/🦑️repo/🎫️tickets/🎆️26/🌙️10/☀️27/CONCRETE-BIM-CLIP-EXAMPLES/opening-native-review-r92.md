# Native Opening Reference/Host Audit R92

All-current-opening capture is running through the actual api.opening_geometry and api.step_excerpt interfaces. Reference voids are semantic subtraction products, not additional occupied materials. The specialized SDK input tests exact9attribute IfcOpeningElement records, Reference representation identifier, authored IfcRelVoidsElement GUID and endpoints, actual native host class/GUID, host containment, and absence of opening material and spatial containment. Reference volume/XYZ and closed-part topology are independently checked.

No result claimed yet. Scripts opening-native-r92.py/.mjs and generated/opening-native-r92.json/output are ticket-scoped. This is separate from physical52footing/railing acceptance.


## First All-Source Capture Failure

OP-D-PARENTS has geometry host W-PARENT-BATH-W but no authored IfcRelVoidsElement references this occurrence. Every other current opening has exactly one relation matching its geometry host. Capture stops at missing source relation; exporter would reject it too. This is a source-semantic gap, not an SDK-oracle issue. Parent and parents owner notified for schema-first correction before fresh capture.


## Canonical Opening Reference Protocol PASS

All59current openings ×ENDE118 real native SDK snippets PASS. Raw sourceb54353ce1417fe19736eadebf1e88b59e99657c3c5852d3a0c8d158106793c13 and both producer hashes stable throughout78.493s capture. Model8d3e551c264c9a11e89899c610ed8b0dfbc42b48d5ca995212329428a8c5b4a1; exchange21fa6a0d7f6bbc538675bd2f7c5dc2da414cd90279666bc0035f2dd14afd478f. All9attribute opening occurrences, Reference representations, authored void GUID/endpoints, actual native host class/GUID, host containment, and absence of opening material/spatial containment verified.148native reference parts have closed welded-edge topology and correct XYZ/volume against independent Shapely prism oracle. Parents missing relation is repaired.

## Stronger Native Host Closure FAIL

Further actual HOST mesh closure check fails at OP-S host W-S component105 (geometry1091). This is not a tiny sliver or a welding-tolerance artifact: exact rawFloat32bit welding and world-coordinate1e-10,1e-9,1e-8,1e-7,1e-6 all expose the same4 incidence1 edges. Bounding boxX[-.15000002384,1.05000002384],Y[.230000000168,.244999999832],Z[5.210000000056,5.214999999944]. Mesh contains10triangles, missing the entire X=1.05 endcap of.015×.005m=7.5e-5m². IFC geometric context precision is1.E-05. The cap coincides with the OP-S opening boundary.

Inference: the SDK may be reapplying an authored void to an already-cut host and deleting a touching/coplanar cap. This causal hypothesis needs direct comparison against a no-void-relation excerpt and native-host source faces; it is not yet proof of exporter versus SDK responsibility. Do not silently drop a part, increase welding tolerance, or call all host meshes closed. Native cavity occupancy extension remains incomplete because closed-host prerequisite fails. Diagnostic generated/opening-host-sliver-r92.json contains exact vertices/indices/floatbits/multiscale results; retained scripts opening-host-sliver-r92.mjs and opening-host-cavity-r92.mjs. Parent notified.

# Exact Eccentric Cylinder Shoulder Fuse

The retained `brep.bool.fuse` dispatcher now accepts two parallel cylinders when the smaller circle lies strictly inside the larger footprint and its axial extension reaches past one or both stock caps. Overlap and contact across a shared cap both produce one solid. The smaller cylinder's authored lateral axis remains unchanged in the projecting necks, including when operands are reversed or their axial frames are antiparallel.

Both coaxial and eccentric unions use the same axial shoulder certificate. Its broad wall and shoulders use the stock axis. The small walls and cap disks use the original narrow axis. The paired angular seam keeps the respective rings nested across each shoulder, and the existing funded solid and preview cursors emit the native planar and cylindrical faces with their original component labels. Center of mass weights the lateral displacement by the material volume of the projecting necks. Area and volume use the original exact cylindrical and annular integrals.

Three neutral cases cover overlap above the stock, shared-cap contact below with reversed operands, and two unequal projecting ends. Manifold-3D matched the expected area, volume, centroid, and bounds. The full independent Boolean suite passed three tests with 78 assertions.

The disk-light source witness first reproduced the unsupported route, then passed all 19 tests, including exact trim correspondence, reciprocal incidences, both shoulder positions, an antiparallel horizontal pair, near-tangent ring nesting at eight samples, and refusal for separated caps or radial tangency. The cylindrical classification regression suite passed all 25 tests. The registered Flow fixture also covers one-item publication and cancellation, separate native wall axes, preview labels, exact measures, and funded previews of the selected annular shoulders. The linked run was left to the shared Cargo lane.

Nightly Rust syntax, scoped whitespace and path lengths passed. Source closures have 2,705 active Representation lines, 3,986 Boolean, and 3,701 Solid Transaction.

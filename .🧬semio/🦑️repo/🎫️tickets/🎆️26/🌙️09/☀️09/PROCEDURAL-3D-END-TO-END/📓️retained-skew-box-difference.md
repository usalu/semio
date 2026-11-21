# Retained exact Cut for skew Boxes

\`brep.bool.cut\` can now subtract certified skew rectangular stock in two domains. A single active tool plane keeps the complementary exact ClippedBox half. A strictly enclosed tool creates an inward six-face cavity shell inside the six-face exterior. The cavity is one solid with 16 vertices, 24 edges, 12 planar faces and two distinct native shells. Each face retains its authored box plane, and the material boundary traverses the inner shell inward.

Separation keeps the complete stock result. Complete removal returns the typed empty result. A non-enclosed recess requiring several active cutter planes returns UnsupportedOwner. Publishing the recess as independent convex pieces would expose artificial interior faces, so this domain still requires a sewn nonconvex boundary. The complementary result supports the existing clipped-box Classify and Section paths. The cavity uses the funded native planar preview and measurement paths.

## Evidence

- The neutral \`🔣️original-skew-box-difference.json\` covers low and high complementary cutter supports, a strictly contained skew cavity, disjoint stock, complete removal and a positive multi-support recess. Manifold independently subtracts both transformed boxes and checks area, volume, bounds and Euler characteristic, including 16 distinct vertices for the cavity. The combined skew and shared-axis oracle run passed 20 tests and 188 assertions.
- The compact skew Rust proof passed 32 tests. It checks the exact complementary cap orientation, closed reciprocal rings, the two native shell allocations, signed inward cavity volume, coplanar trim/edge agreement, area, volume, bounds, point classification and typed failures. The adjacent shared-axis Rust proof passed 22 tests.
- Registered \`original-brep-booleans\` exercises the authored result through the normal retained operation, cancels after both private cavity shells exist, verifies rollback and atomic publication, and checks topology, exact inward tool supports, preview, Measure and clipped Classify. Its managed linked run remains pending shared build capacity.
- Canonical source closure passed Intersect at 3,047 lines and 147,075 bytes, Construction at 3,988 and 200,604, Solid Transaction at 4,000 and 207,271, and original-booleans at 2,219 and 261,272.

This extends the [multi-support skew Intersect](📓️retained-multisupport-skew-box-intersection.md) with representable Cut topology.

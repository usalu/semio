# Retained circle and ellipse contacts with a plane

The registered Curve–Surface intersection now computes contacts of a standalone analytic circle or ellipse with a plane. A secant creates one original open wire with the two exact contacts as endpoints. A tangent creates one native vertex. This follows the existing Curve–Surface output contract. Disjoint and coplanar cases report no finite geometry. The existing supporting line case uses the same entry point.

The solver borrows the curves and plane frame and uses a fixed two-slot result. The original job publishes native vertices and edges under the existing funded owner. A half-built secant has no live handle and cancellation releases its unpublished topology. The cold Circle–Plane and Ellipse–Plane paths share the analytic result. Testing separation before clamping the trigonometric ratio also corrects false tangent hits on disjoint inputs of the existing circle solver.

## Verification

- The neutral intersection fixture covers two ordered secant pairs, a tangent, a separated coplanar-direction ellipse, a plane missing the circle entirely, and infinite intersection. All eight Three.js tests passed. The independent ellipse oracle checks line segment contacts against the plane.
- The isolated Rust contact check passed with the actual bounded root and line equations. It verifies both analytic supports, ordered coordinates and all refusal cases. The isolated topology harness passed seven checks, including fixed segment publication under one-item grants, zero-item progress and complete cancellation rollback.
- The linked original-brep-queries fixture drives the registered Circle, Ellipse and Plane constructors through Curve–Surface. It checks native preview points and labels, refusal without publication, and cancellation after a secant starts building. Linked execution remains pending the shared native build.
- Rust AST and scoped whitespace verification passed. Current source closure is 3,998 active Engine lines, 3,881 native registry lines, and 1,951 active query fixture lines with 239,177 active bytes.

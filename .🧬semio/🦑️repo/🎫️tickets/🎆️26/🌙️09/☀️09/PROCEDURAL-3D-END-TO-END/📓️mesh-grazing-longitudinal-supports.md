# Grazing longitudinal Mesh supports

A longitudinal support can cut one portion of a certified concave prism while touching another portion at a convex corner or along an original sidewall. The clipping cursor classifies these contacts by the direction of original material. It keeps their original corner and face indices on the material side. The contact contributes no crossing interval or cutter face.

The point-contact fixture preserves the original apex in Difference and excludes it from both Intersect operand orders. Manifold gives one closed component per operation. Intersect has area 10 and volume 2. Difference has area 26.23606797749979 and volume 6.5.

The edge-contact fixture has a short original sidewall coincident with the cutting support. Each edge endpoint is classified with its neighboring source corners. The sidewall appears exactly once under its original stock material in Difference. A separate interval along the same support creates the reversed cutter face. Manifold gives area 10 and volume 2 for Intersect, and area 27.06155281280883 and volume 6.75 for Difference. Each result has one component.

Both native red cases reproduced the previous grazing refusals. The latest 10 native Clipping tests passed. They cover closed reciprocal orientations, live corner counts, original sidewall ownership and exact cutter face counts. The Manifold and Clipping/Packaging suite passed 18 tests and 5,009 assertions. The linked authored façade test passed and checks raw and named UVs, both Intersect orders and Difference and closed reimport.

The follow-up covers pinched components at reflex contacts on one longitudinal support. Coincident edges with incompatible adjacent material require additional topology construction. [Reflex contact verification](./📓️mesh-reflex-longitudinal-support.md)

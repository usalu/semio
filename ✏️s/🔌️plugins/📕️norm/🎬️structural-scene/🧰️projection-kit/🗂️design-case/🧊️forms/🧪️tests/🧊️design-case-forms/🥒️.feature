@capability-design-case-forms
@oracle-python
@comparison-closed-form
Feature: Profile forms of the design-case drawing
  The shared forms draw extruded profile prisms, laminated plates, charred sections, mode-shape overlays and composite assemblies from the declared case inputs and the extras a catalogue hands in. Their key dimensions must equal an independent Python implementation (shapely for the areas).

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A tapered member is a profile prism: Given a member of 300 mm by 6 m by 0.5 m with a taper angle of 30 degrees When it is drawn Then its body is the prism of the elevation, extruded across the width, whose far end rises by l tan(alpha) and whose area is l (h + top) / 2
  - A curved member has an apex zone: Given a member with an apex angle of 30 degrees, an inner radius of 8 m and a depth of 0.5 m When it is drawn Then the apex zone is the annular sector between the radii r and r + h of the polyline of 12 chords and the angle and radius grips write degrees and metres
  - A ply stack follows the plies: Given a plate with the plies 30, 40 and 30 mm oriented 1, 0 and 1 When it is drawn Then it is a laminate with one layer per ply, grain along for orientation 0.5 or more and across otherwise, over the plan, with a grip on the top of every ply after the first
  - A charred section is the reduced outline: Given a section 300 mm by 500 mm with the top side protected and char depths handed in as data When it is drawn Then the char band is the section minus the section reduced on the exposed sides, the residual section is the section reduced by its depth and a slab is a box from the exposed face
  - A mode overlay ghosts the body: Given a mode vector, an amplitude and a plate grid with a peak value When it is drawn Then the body carries the displaced shape over its undeformed ghost and the plate is a surface coloured by peak times the absolute value of the mode
  - An assembly is a list of sub-forms with transforms: Given sub-forms with local positions and a case turned by a yaw When the assembly is drawn Then the first sub-form is the body, every sub-form stands at the yawed position and the extent of the case is the extent of the assembly
  - An annotation role draws a dimension: Given a member with an end distance of 40 mm, an edge distance and a load angle of 30 degrees declared as annotation roles When it is drawn Then each role is a dimension line with a label, the distance a marker at the measured point and the angle a sector between the grain and the load direction, each with a grip that writes the input, and 1.25 times the input scales the line (lengths) or turns the angle by 1.25
  - Cases never overlap: Given the cases of a catalogue and rectangles the layout must avoid When the default places are packed Then no two plan rectangles overlap and the shelves stay within the pack width

  Scenario Outline: The Rust forms agree with the Python oracle on <form>
    Given the case inputs of the <form> example
    When the Rust form draws it and the Python oracle recomputes the dimensions from the inputs alone
    Then every key dimension of the drawing equals the oracle value within 1e-9

    Examples:
      | form      |
      | taper     |
      | apex      |
      | plies     |
      | char      |
      | mode      |
      | assembly  |
      | dimension |
      | angle     |

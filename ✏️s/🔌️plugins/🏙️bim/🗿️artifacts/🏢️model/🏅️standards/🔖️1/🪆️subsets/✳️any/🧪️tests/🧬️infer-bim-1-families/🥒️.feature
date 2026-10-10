@capability-bim-1-infer
@oracle-bim-1-numpy-expression-families
@comparison-floating-point-v1
Feature: Infer the parameters, issues and solids of a parametric family from its formulas and audit them with an independent evaluator and numpy
  `s.bim.model@1` stores a family as authored data: its category, its parameters (a kind and a formula in the expression language, such as `height - top_thickness` or
  `if wide then 15 deg else 0 deg`) and its solids (an extrusion, revolution, sweep or cuboid whose dimensions, material and visibility are formulas). It stores no value. `🧬️families`
  derives, per family, the value of every parameter in dependency order, the issues of every formula and solid (a text that does not parse, a mix of kinds, a circle of references, an unknown
  name, a dimension that is not positive, a division by zero, a parameter that depends on one that failed), the solids as meshes (visibility, material, volume, area, triangles, bounds) and the
  outline of a profile family. The oracle is `🐍️.py` in this directory. It shares no line with the subject: a Python recursive-descent translator reads the formulas, the independent Python
  interpreter of the expression framework module (`ast` dimension checking, `networkx` strongly connected components and topological generations) evaluates them, `numpy` rebuilds every solid as
  explicit facets (divergence-theorem volume, cross-product area, mitered sweeps, revolutions in rings) and `shapely` measures the section polygons. It also audits the closed forms (a prism is
  its section area times its height, a pipe is pi r^2 h, a revolution obeys Pappus' theorem) and the parametric laws (the order of the parameters changes nothing, widening a table by 0.1 m
  widens its top by 0.1 m x depth x thickness). The committed expectation is written by that file, never by hand.

  @id-families-table
  @level-quick
  @mode-differential
  Scenario: A table, three profiles and a family full of faults resolve to their values, issues and solid measures
    Given the committed family model shared://💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json
    When 🧬️families is inferred for it
    Then every parameter, issue and solid of every family equals the table shared://💡️inferences/🧬️families/🪑️table/💡️inference/📏️families/🔣️.json

  @id-families-frame
  @level-quick
  @mode-differential
  Scenario: The profile families used by the columns, beams and railings of a frame resolve to their outlines
    Given the committed family model shared://💡️inferences/🧬️families/🏛️frame/📸️snapshot/🔣️.json
    When 🧬️families is inferred for it
    Then every parameter, issue and solid of every family equals the table shared://💡️inferences/🧬️families/🏛️frame/💡️inference/📏️families/🔣️.json

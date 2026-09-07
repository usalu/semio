@capability-fem3d-1-mutate
@oracle-skfem-fem3d-solid
@comparison-semantic-fem3d-solid-v1
Feature: Hold fem3d's meshed-solid answer to an independent 3D continuum library

  A `FemSolid` is the one analysis path a frame solver structurally cannot see:
  `crate::fem3d_engine::meshing::resolve_geometry` triangulates its footprint, extrudes it, splits
  the prisms into `Tet4` elements and solves the result as a three-dimensional elastic continuum.
  Its sibling cases judge the frame answers (`../🧮️solves-fem3d-1-benchmarks`, PyNite) and the
  eigenvalue answers (`../🎵️solves-fem3d-1-eigen`, SciPy); this one judges the continuum.

  THE THIRD PARTY. `scikit-fem` (BSD-3-Clause), an independent continuum finite-element library. It
  builds its OWN hexahedral mesh of the same prism, assembles its own isotropic linear-elasticity
  operator from the Lamé parameters, and condenses and solves its own system. It never sees this
  artifact's triangulation, its extrusion, its tetrahedra or its solver — which is exactly what makes
  the comparison worth making: two different meshes of one continuum must still agree on the
  continuum's answer. The reference is `🐍️.py` beside this file; `🦀️.rs` registers the SUBJECT only.

  WHY THIS FIXTURE, AND WHY THE PRESSURE CASE CAN BE EXACT. `🧱️prismatic-solid-column.snapshot.json`
  gives its solid a `meshSize` larger than its own footprint diagonal, so the footprint triangulates
  into exactly its four corners — every base node is then a DOCUMENT node the fixture can name and
  restrain, which no ordinary solid fixture can do. The restraint set it declares (`Tz` at all four
  corners, `Tx`+`Ty` at the origin, `Ty` at the +x corner, `Tx` at the +y corner) is precisely the
  one the uniform-stress solution already satisfies, so it removes the six rigid-body freedoms
  without fighting the Poisson contraction. Under a uniform 0.5 MPa top pressure the continuum answer
  is therefore exactly `σ = −p` and `δ_top = −pH/E`, which a constant-strain tetrahedron reproduces
  to machine precision and which scikit-fem reproduces to `8e-16` on its own mesh. Under self weight
  the answer is `−ρgH²/2E`, which a hexahedral mesh and a tetrahedral one approach from their own
  sides rather than hit — scikit-fem lands 0.26 % below it — so that case is a five-percent verdict
  and nothing tighter would be honest about either discretisation.

  WHAT IS COMPARED. The pressure case's top-face shortening at six significant figures, which both
  implementations reach exactly; the self-weight case as the shared five-percent verdict against the
  closed form. The displacements themselves are asserted numerically by the subject against
  `local://📊️expected.results.json`, which this case's reference produced with scikit-fem.

  @id-solid
  @level-exhaustive
  @mode-differential
  Scenario Outline: The <id> under top pressure and its own weight answers as the continuum does
    Given the committed model local://<fixture>
    And the committed reference local://📊️expected.results.json
    When both implementations solve it as a three-dimensional elastic continuum
    Then they agree on the pressure case's top shortening to six significant figures, both land on −pH/E and within five percent of −ρgH²/2E, and the subject is within 5e-2 relative of the reference's own self-weight answer
    Examples:
    | id              | fixture                                    |
    | prismatic-column| 🧱️prismatic-solid-column.snapshot.json      |

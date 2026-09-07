@capability-fem3d-1-mutate
@oracle-scipy-fem3d-eigen
@comparison-semantic-fem3d-eigen-v1
Feature: Hold fem3d's modal and buckling answers to an independent eigensolver and to the textbook

  `crate::fem3d_engine::modal_buckling` answers two eigenvalue questions: at what frequencies does a
  frame vibrate freely, and at what load factor does an axially loaded one buckle. It answers both
  with this repository's OWN hand-rolled subspace iteration (`crate::sparse::subspace_iteration`),
  which until this case had no external reference of any kind — the kernel's own benchmarks compared
  it with a closed form at a ten-percent tolerance and with nothing else at all.

  THE THIRD PARTY. `scipy.linalg.eigh` (SciPy, BSD-3-Clause), a dense symmetric generalised
  eigensolver over LAPACK. The reference is `🐍️.py` beside this file; it hands SciPy a `K`, `M` and
  `Kg` it assembles itself from the textbook Euler-Bernoulli frame matrices, in one stated sign
  convention, and reads the answer back. `🦀️.rs` registers the SUBJECT half only.

  ONE SIGN CONVENTION, STATED ONCE, BECAUSE IT IS WHERE THIS GOES WRONG. In the local
  `[u, v, w, θx, θy, θz]` ordering both this artifact and every textbook use, the y-bending plane
  measures its rotation as `θy = −∂w/∂x`. That choice shows up as a sign flip on the off-diagonal
  `L` terms of every matrix in that plane — formally `S·M·S` with `S = diag(1, −1, 1, −1)` — and the
  stiffness, the consistent mass AND the consistent geometric stiffness must all carry it, or the
  pair's eigenvalues are not the structure's. The reference writes all three in that one convention.

  WHAT IS COMPARED. Not the eigenvalues themselves: a subspace iteration and a LAPACK factorisation
  of the same pencil agree to their own convergence tolerance, not to a digit count, and a projection
  that demanded digits would be measuring convergence settings rather than correctness. What the two
  implementations are held to jointly is the thing that genuinely is shared — the CLOSED FORM, and
  whether each of them independently lands within two percent of it over an eight-element
  discretisation. SciPy's own agreement (better than 0.3 % on every scenario below) is the evidence
  that two percent is a fair demand of the discretisation rather than of the solver. The eigenvalues
  themselves are asserted numerically by the subject against `local://📊️expected.results.json`, which
  this case's reference produced and which is committed beside these models.

  THE CLOSED FORMS. A cantilever's natural frequencies are `βₙ²/(2πL²)·√(EI/ρA)` with
  `β = 1.8751, 4.6941, 7.8548`; the fixture's section has `iy ≠ iz` so the two bending planes give
  two distinct frequency families and no mode is a double root an eigensolver could return in either
  order. A column's critical load is `π²EI/(KL)²` with `K = 2.0` fixed-free, `1.0` pinned-pinned,
  `0.7` fixed-pinned and `0.5` fixed-fixed; the reference load is exactly 1 N, so the lowest load
  factor IS the critical load in newtons, and the section's `iz < iy` makes the first mode the
  weak-axis one.

  @id-modal
  @level-exhaustive
  @mode-differential
  Scenario Outline: The <id>'s natural frequencies are the textbook's, to two percent
    Given the committed model local://<fixture>
    And the committed reference local://📊️expected.results.json
    When both implementations solve its free-vibration eigenproblem
    Then each lands within two percent of βₙ²/(2πL²)·√(EI/ρA) for the first four modes, they agree on that verdict mode by mode, and the subject is within 1e-3 relative of the reference's own frequencies
    Examples:
    | id         | fixture                          |
    | cantilever | 🎵️modal-cantilever.snapshot.json  |

  @id-buckling
  @level-exhaustive
  @mode-differential
  Scenario Outline: The <id> column buckles at π²EI/(KL)² with K = <k>, to two percent
    Given the committed model local://<fixture>
    And the committed reference local://📊️expected.results.json
    When both implementations solve its linear-buckling eigenproblem under the unit axial reference load
    Then each lands within two percent of π²EI/(K·L)², they agree on that verdict, and the subject is within 1e-3 relative of the reference's own lowest load factor
    Examples:
    | id                   | k   | fixture                                          |
    | column-fixed-free    | 2.0 | 🏛️buckling-column-fixed-free.snapshot.json         |
    | column-pinned-pinned | 1.0 | 🏛️buckling-column-pinned-pinned.snapshot.json      |
    | column-fixed-pinned  | 0.7 | 🏛️buckling-column-fixed-pinned.snapshot.json       |
    | column-fixed-fixed   | 0.5 | 🏛️buckling-column-fixed-fixed.snapshot.json        |

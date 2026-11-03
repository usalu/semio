@capability-diagram-kit
@oracle-numpy-shapely
@comparison-floating-point-v1
Feature: Diagram kit builders
  Pure builders for interaction polygons, spectra, S-N curves, stress blocks, buckling and fire curves
  must agree with independent numpy and shapely computations.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Interaction polygon and utilisation ray: Given capacity points of an M-N interaction diagram When the convex interaction polygon and the ray from the origin through the demand are built Then the polygon area, vertex count, boundary hit and utilisation equal the shapely result
  - Spectrum corner periods are exact: Given spectrum parameters ag, S, TB, TC, TD, eta, q When the elastic and design spectra are sampled Then every sample equals the numpy piecewise result and TB, TC, TD are sample abscissae
  - Detail category S-N curve: Given detail category 80 MPa When the strength at 1e4 to 5e8 cycles is evaluated Then it equals the log-log interpolation through the 2e6, 5e6 and 1e8 knees
  - Stress block force and lever arm: Given a 300 by 450 section with x = 150 and fck = 60 When the rectangular stress block is built Then the compression force and lever arm equal the closed form
  - Buckling and fire curves: Given imperfection factor 0.34 and standard fire curves When they are sampled Then they equal the numpy evaluation of the EN 1993-1-1 6.49 and EN 1991-1-2 3.2 formulas

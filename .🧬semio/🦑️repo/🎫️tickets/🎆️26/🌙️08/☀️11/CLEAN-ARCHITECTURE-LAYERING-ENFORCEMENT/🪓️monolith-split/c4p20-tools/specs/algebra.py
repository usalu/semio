from specs.lib import *

SPEC = dict(
    orig='algebra.rs.txt',
    file=X + '/🧬️schema/➕️algebra-internals/🦀️.rs',
    nest=False,
    external=[orig('algebraunit'), orig('algebraexact')],
    children=[
        ch('🧱', 'dense-types', 'dense_types', [(5, 303)],
           'Dense types of the linear algebra: the 2x2 matrix, the dynamic vector and matrix, the 3x3 matrix and the 3-vector helpers.', glob=False),
        ch('🔗', 'lattice-sparse-and-errors', 'lattice_sparse_and_errors', [(307, 469)],
           'Lattice reduction, sparse matrices and errors of the linear algebra: the LLL basis reduction, the compressed sparse row matrix and the algebra error.'),
        ch('📉', 'factorizations-and-eigen', 'factorizations_and_eigen', [(473, 735)],
           'Factorizations and symmetric eigenproblems of the linear algebra: the Cholesky and Householder QR factorizations and the Jacobi and Lanczos eigen solvers.'),
        ch('🔁', 'iterative-and-svd', 'iterative_and_svd', [(738, 1033)],
           'Iterative solvers and the SVD of the linear algebra: the power iteration, the conjugate gradient, the Pade matrix exponential, the singular value decomposition and the least-squares solvers built on it.'),
        ch('🌀', 'schur-and-roots', 'schur_and_roots', [(1037, 1265)],
           'Hessenberg, real Schur and polynomial roots of the linear algebra: the Hessenberg reduction, the Francis double-shift Schur form, the real eigenvalues and the companion-matrix roots.'),
    ],
)

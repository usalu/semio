"""🧮 Three-dimensional vectors and matrices for the red-fox edit scripts (no third-party package on this host)."""
import math


class Arr:
    """🔢 A 3-vector or a 3x3 matrix with the few operations the edit scripts use."""

    def __init__(self, data):
        self.d = [list(r) if isinstance(r, (list, tuple)) else float(r) for r in data]
        self.matrix = isinstance(self.d[0], list)

    def __getitem__(self, key):
        return self.d[key[0]][key[1]] if isinstance(key, tuple) else self.d[key]

    def __iter__(self):
        return iter(self.d)

    def __len__(self):
        return len(self.d)

    @property
    def T(self):
        return Arr([[self.d[j][i] for j in range(3)] for i in range(3)])

    def __matmul__(self, other):
        other = other if isinstance(other, Arr) else Arr(other)
        if other.matrix:
            return Arr([[sum(self.d[i][k] * other.d[k][j] for k in range(3)) for j in range(3)] for i in range(3)])
        return Arr([sum(self.d[i][k] * other.d[k] for k in range(3)) for i in range(3)])

    def __add__(self, other):
        return Arr([a + b for a, b in zip(self.d, other)])

    def __sub__(self, other):
        return Arr([a - b for a, b in zip(self.d, other)])

    def __truediv__(self, k):
        return Arr([a / k for a in self.d])

    def flat(self):
        return [v for r in self.d for v in r] if self.matrix else list(self.d)


def array(data, dtype=None):
    return data if isinstance(data, Arr) else Arr(data)


def norm(v):
    return math.sqrt(sum(a * a for a in v))


def cross(a, b):
    a, b = list(a), list(b)
    return Arr([a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]])


def column_stack(columns):
    columns = [list(c) for c in columns]
    return Arr([[columns[j][i] for j in range(3)] for i in range(3)])


def gap(a, b):
    return max(abs(x - y) for x, y in zip(a.flat(), b.flat()))

"""🧰 Shared inputs of the clause-form oracles: the NDP registry read, the Table 3.1 columns and the design strengths built from `structuralcodes.codes.ec2_2004`; every figure SI (Pa, N, m, N·m) once it leaves a handler."""

import importlib.util
import math
from pathlib import Path

import numpy as np
from structuralcodes.codes import ec2_2004 as ec2

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors_clause_forms", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)

MM = 1e-3
MPA = 1e6
GRADES = [12, 16, 20, 25, 30, 35, 40, 45, 50, 55, 60, 70, 80, 90]
CUBE = [15, 20, 25, 30, 37, 45, 50, 55, 60, 67, 75, 85, 95, 105]
ECM_GPA = [27, 29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 41, 42, 44]
FCTM = [1.6, 1.9, 2.2, 2.6, 2.9, 3.2, 3.5, 3.8, 4.1, 4.2, 4.4, 4.6, 4.8, 5.0]
FCTK05 = [1.1, 1.3, 1.5, 1.8, 2.0, 2.2, 2.5, 2.7, 2.9, 3.0, 3.1, 3.2, 3.4, 3.5]
FCTK95 = [2.0, 2.5, 2.9, 3.3, 3.8, 4.2, 4.6, 4.9, 5.3, 5.5, 5.7, 6.0, 6.3, 6.6]


def ndp(row, annex):
    return FACTORS.ndp(row, annex)


def row_of(fck, column):
    return float(np.interp(fck, GRADES, column))


def fcd_of(annex, fck, accidental=False):
    suffix = "accidental" if accidental else "persistent"
    return ec2.fcd(fck, ndp(f"ndp.en1992.m-01.alpha-cc-{suffix}", annex), ndp(f"ndp.en1992.m-01.gamma-c-{suffix}", annex))


def fctd_of(annex, fck):
    return ec2.fctd(row_of(fck, FCTK05), ndp("ndp.en1992.m-01.alpha-ct", annex), ndp("ndp.en1992.m-01.gamma-c-persistent", annex))


def fyd_of(annex, fyk, accidental=False):
    return ec2.fyd(fyk, ndp("ndp.en1992.gamma-s-acc" if accidental else "ndp.en1992.gamma-s", annex))


def nu_of(fck):
    return 0.6 * (1 - fck / 250)

# #region 📊Header

# 2026 Ueli Saluz <ueli@compose-tech.de>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# #endregion 📊Header

import functools
import importlib.util
import inspect
import pathlib
import sys
import typing

# Patch typing._eval_type for Pydantic compatibility with Python 3.14 betas.
# Pydantic passes `prefer_fwd_module` which was removed from the CPython API.
_original_eval_type = typing._eval_type
_eval_type_params = inspect.signature(_original_eval_type).parameters
if "prefer_fwd_module" not in _eval_type_params:

    @functools.wraps(_original_eval_type)
    def _patched_eval_type(*args, **kwargs):
        kwargs.pop("prefer_fwd_module", None)
        return _original_eval_type(*args, **kwargs)

    typing._eval_type = _patched_eval_type

root = pathlib.Path(__file__).parent

# Load compose/py/main.py as 'main' (the compose core library)
_compose_py_path = str(root / "compose" / "py" / "main.py")
_compose_spec = importlib.util.spec_from_file_location("main", _compose_py_path)
_compose_mod = importlib.util.module_from_spec(_compose_spec)
sys.modules["main"] = _compose_mod
_compose_spec.loader.exec_module(_compose_mod)

compose = _compose_mod  # noqa: F841
compose.__path__ = [str(root / "compose")]

# Load compose/engine/main.py as 'engine'
_engine_path = str(root / "compose" / "engine" / "main.py")
_engine_spec = importlib.util.spec_from_file_location("engine", _engine_path)
engine = importlib.util.module_from_spec(_engine_spec)  # noqa: F841
sys.modules["engine"] = engine
_engine_spec.loader.exec_module(engine)

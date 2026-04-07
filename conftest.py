# #region 📊Header

# 2026 Ueli Saluz <ueli@semio-tech.de>

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

import importlib.util
import pathlib
import sys

root = pathlib.Path(__file__).parent

# Load semio/py/main.py as 'main' (the semio core library)
_semio_py_path = str(root / "semio" / "py" / "main.py")
_semio_spec = importlib.util.spec_from_file_location("main", _semio_py_path)
_semio_mod = importlib.util.module_from_spec(_semio_spec)
sys.modules["main"] = _semio_mod
_semio_spec.loader.exec_module(_semio_mod)

semio = _semio_mod  # noqa: F841
semio.__path__ = [str(root / "semio")]

# Load semio/engine/main.py as 'engine'
_engine_path = str(root / "semio" / "engine" / "main.py")
_engine_spec = importlib.util.spec_from_file_location("engine", _engine_path)
engine = importlib.util.module_from_spec(_engine_spec)  # noqa: F841
sys.modules["engine"] = engine
_engine_spec.loader.exec_module(engine)

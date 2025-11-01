// #region Header

// FreezeButton.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { Download } from "lucide-react";
import { ButtonGroupItem } from "../../elements/input/ButtonGroup";
import { useSketchpadStore } from "../store";

export function FreezeButton() {
  const store = useSketchpadStore();

  return (
    <ButtonGroupItem id="semio.sketchpad.navbar.freeze" value="freeze" onClick={() => store.execute("semio.sketchpad.freeze", "semio.sketchpad.navbar.freeze")}>
      <Download size={16} />
    </ButtonGroupItem>
  );
}

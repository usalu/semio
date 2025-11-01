// #region Header

// TimetravelButton.tsx

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

import { Upload } from "lucide-react";
import { ButtonGroupItem } from "../../elements/input/ButtonGroup";
import { useSketchpadStore } from "../store";

export function TimetravelButton() {
  const store = useSketchpadStore();

  return (
    <ButtonGroupItem id="semio.sketchpad.navbar.timetravel" value="timetravel" onClick={() => store.execute("semio.sketchpad.timetravel", "semio.sketchpad.navbar.timetravel")}>
      <Upload size={16} />
    </ButtonGroupItem>
  );
}

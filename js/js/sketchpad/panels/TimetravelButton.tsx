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
import { useEffect } from "react";
import { Action } from "../../elements/input/Action";
import { useAddFooterItem, useRemoveFooterItem } from "../Footer";
import { Mode, useMode, useSketchpadStore } from "../store";

export function TimetravelButton() {
  const mode = useMode();
  const store = useSketchpadStore();
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();

  useEffect(() => {
    if (mode === Mode.DEV) {
      addFooterItem({
        id: "semio.sketchpad.footer.timetravel",
        order: -99,
        content: (
          <Action
            id="semio.sketchpad.footer.timetravel.action"
            onClick={() => store.execute("semio.sketchpad.timetravel", "semio.sketchpad.footer.timetravel.action")}
          >
            <Upload className="w-4 h-4" />
          </Action>
        ),
      });

      return () => {
        removeFooterItem("semio.sketchpad.footer.timetravel");
      };
    }
  }, [mode, store, addFooterItem, removeFooterItem]);

  return null;
}

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
import { FC, useEffect } from "react";
import { useAddFooterItem, useRemoveFooterItem } from "../Footer";
import { Mode, useMode, useSketchpadStore } from "../store";

export const TimetravelButton: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const mode = useMode();
  const store = useSketchpadStore();

  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("semio.sketchpad.footer.timetravel");
      return;
    }

    addFooterItem({
      id: "semio.sketchpad.footer.timetravel",
      icon: Upload,
      onClick: () => store.execute("semio.sketchpad.timetravel", "semio.sketchpad.footer.timetravel"),
      order: 1001,
    });

    return () => {
      removeFooterItem("semio.sketchpad.footer.timetravel");
    };
  }, [mode, store, addFooterItem, removeFooterItem]);

  return null;
};

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
import { FC, useEffect } from "react";
import { Button } from "../../elements/input/Button";
import { useAddFooterItem, useRemoveFooterItem } from "../Footer";
import { Mode, useMode, useSketchpadStore } from "../store";

export const FreezeButton: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const mode = useMode();
  const store = useSketchpadStore();

  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("freeze-button");
      return;
    }

    addFooterItem({
      id: "freeze-button",
      content: (
        <Button id="footer-freeze-button" variant="ghost" onClick={() => store.execute("semio.sketchpad.freeze", "semio.sketchpad.footer.freeze")} className="h-5 w-8 p-0">
          <Download className="h-3 w-3" />
        </Button>
      ),
      order: 1000,
    });

    return () => {
      removeFooterItem("freeze-button");
    };
  }, [mode, store, addFooterItem, removeFooterItem]);

  return null;
};

// #region Header

// LassoTool.tsx

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

import { Lasso, Square } from "lucide-react";
import { Tool, ToolRenderContext } from "../../../Tool";
import { ToolType } from "../../../store";
import { DesignAppState } from "../store";

export const LassoRectangularTool: Tool<DesignAppState> = {
  id: ToolType.LASSO_RECTANGULAR,
  label: "tools.lasso.rectangular",
  icon: <Square className="h-4 w-4" />,
  tooltip: "tools.lasso.rectangular.extensive",
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const LassoFreeformTool: Tool<DesignAppState> = {
  id: ToolType.LASSO_FREEFORM,
  label: "tools.lasso.freeform",
  icon: <Lasso className="h-4 w-4" />,
  tooltip: "tools.lasso.freeform.extensive",
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

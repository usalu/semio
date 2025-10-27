// #region Header

// index.tsx

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

import { Tool } from "../../../Tool";
import { DesignAppState } from "../store";

const toolModules = import.meta.glob<Record<string, Tool<DesignAppState>>>("./*Tool.tsx", { eager: true });

export const DesignAppTools: Tool<DesignAppState>[] = Object.values(toolModules)
  .flatMap((module) => Object.values(module))
  .filter((exp): exp is Tool<DesignAppState> => exp != null && typeof exp === "object" && "id" in exp && "render" in exp);

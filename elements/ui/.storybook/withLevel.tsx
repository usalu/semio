// #region 🧲Header

// 💻 .elements/ui/.storybook/withLevel.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import type { Decorator } from "@storybook/react";
import { Level, LevelProvider, getLevelBgClass } from "..";

export const withLevel: Decorator = (Story, context) => {
  const level = context.globals.level as Level;
  return (
    <LevelProvider level={level}>
      <div className={`p-4 ${getLevelBgClass(level)}`}>
        <Story />
      </div>
    </LevelProvider>
  );
};

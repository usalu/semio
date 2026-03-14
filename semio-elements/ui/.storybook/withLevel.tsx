// #region 🔖Header

// 💻 semio-elements/ui/.storybook/withLevel.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import type { Decorator } from "@storybook/react";
import React from "react";
import { Level, LevelProvider, getLevelBgClass } from "../elements";

export const LevelShowcase: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const levels: Level[] = ["base", "window", "panel", "overlay", "temporary"];
  return (
    <div className="flex flex-col gap-4">
      {levels.map((level) => (
        <div key={level} className={`p-4 ${getLevelBgClass(level)} border`}>
          <div className="text-xs text-muted-foreground mb-2 capitalize">{level}</div>
          <LevelProvider level={level}>{children}</LevelProvider>
        </div>
      ))}
    </div>
  );
};

export const LevelWrapper: React.FC<{ level: Level; children: React.ReactNode }> = ({ level, children }) => {
  const bgClass = getLevelBgClass(level);
  return (
    <div className={`p-4 ${bgClass} border min-w-[200px]`}>
      <LevelProvider level={level}>{children}</LevelProvider>
    </div>
  );
};

export const withLevel: Decorator = (Story, context) => {
  const level = context.args?.level as Level | undefined;
  if (level) {
    return (
      <LevelWrapper level={level}>
        <Story />
      </LevelWrapper>
    );
  }
  return <Story />;
};

// #region 🔖Header
// 💻 semio/algorithms/.storybook/withLevel.tsx
// Specs: Keep level decorator behavior consistent with .elements/ui.
// Summary: Wraps stories with optional level-aware backgrounds.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { type Level, LevelProvider, getLevelBgClass } from "@elements/ui";
import type { Decorator } from "@storybook/react";
import React from "react";

export const LevelWrapper: React.FC<{ level: Level; children: React.ReactNode }> = ({ level, children }) => {
  return (
    <div className={`p-4 ${getLevelBgClass(level)} border min-w-[200px]`}>
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

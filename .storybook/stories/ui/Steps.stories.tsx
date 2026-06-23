// #region 🧲Header

// .elements/ui/.storybook/story/elements/display/Steps.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Steps } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🪬Steps

const meta = {
  title: "🖱️ui⚛️react/Steps",
  component: Steps,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Steps>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: (
      <>
        <li>Install the dependencies</li>
        <li>Configure the project</li>
        <li>Start the development server</li>
        <li>Open the browser</li>
      </>
    ),
  },
};

// #endregion 🪬Steps

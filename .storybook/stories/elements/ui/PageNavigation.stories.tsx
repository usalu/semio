// #region 🧲Header

// .elements/ui/.storybook/stories/elements/navigation/PageNavigation.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { PageNavigation } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter } from "react-router";

// #region 🪩PageNavigation

const meta = {
  title: "elements/react/PageNavigation",
  component: PageNavigation,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
  decorators: [
    (Story) => (
      <MemoryRouter>
        <Story />
      </MemoryRouter>
    ),
  ],
} satisfies Meta<typeof PageNavigation>;

export default meta;

type Story = StoryObj<typeof meta>;

export const BothLinks: Story = {
  args: {
    prev: { title: "Getting Started", path: "getting-started" },
    next: { title: "Advanced Topics", path: "advanced" },
  },
};

export const PreviousOnly: Story = {
  args: {
    prev: { title: "Introduction", path: "intro" },
  },
};

export const NextOnly: Story = {
  args: {
    next: { title: "Next Chapter", path: "next" },
  },
};

export const WithSections: Story = {
  args: {
    prev: { title: "Installation", path: "installation", section: "Getting Started" },
    next: { title: "Configuration", path: "configuration", section: "Setup" },
  },
};

// #endregion 🪩PageNavigation

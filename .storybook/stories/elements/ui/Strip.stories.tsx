// #region 🧲Header

// .elements/ui/.storybook/stories/elements/Strip.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Band, Strip } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// #region 📢Strip

const stripMeta = {
  title: "elements/react/Strip",
  component: Strip,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Strip>;

export default stripMeta;

type StripStory = StoryObj<typeof stripMeta>;

export const Default: StripStory = {
  args: {
    items: [
      { id: "item-1", content: <span>Item 1</span>, order: 0 },
      { id: "item-2", content: <span>Item 2</span>, order: 1 },
      { id: "item-3", content: <span>Item 3</span>, order: 2 },
    ],
  },
};

export const WithFlexItems: StripStory = {
  args: {
    items: [
      { id: "left", content: <span className="font-bold">Logo</span>, order: 0 },
      { id: "center", content: <input type="text" placeholder="Search..." className="px-2 py-1 bg-panel border rounded w-full" />, order: 1, className: "flex-1" },
      { id: "right", content: <span>Profile</span>, order: 2 },
    ],
  },
};

// #endregion 📢Strip

// #region 🥁Band

export const BandDefault: StripStory = {
  name: "Band",
  args: {
    items: [
      { id: "b1", content: <span>Band Item 1</span>, order: 0 },
      { id: "b2", content: <span>Band Item 2</span>, order: 1 },
    ],
  },
  render: () => (
    <Band
      items={[
        { id: "b1", content: <span>Band Item 1</span>, order: 0 },
        { id: "b2", content: <span>Band Item 2</span>, order: 1 },
        { id: "b3", content: <span>Band Item 3</span>, order: 2 },
      ]}
    />
  ),
};

// #endregion 🥁Band

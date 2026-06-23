// #region 🧲Header

// .elements/ui/.storybook/story/elements/window/Page.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Page } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🌈Page

const meta = {
  title: "🖱️ui⚛️react/Page",
  component: Page,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Page>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    frontmatter: {
      title: "Getting Started",
      description: "Learn how to use the platform.",
    },
    children: (
      <>
        <h2>Introduction</h2>
        <p>This is the introduction section with some example content.</p>
        <h2>Prerequisites</h2>
        <p>You need a modern web browser and basic understanding of the concepts.</p>
        <h3>Software Requirements</h3>
        <ul>
          <li>Node.js 18+</li>
          <li>npm or yarn</li>
        </ul>
      </>
    ),
  },
  render: (args) => (
    <div className="h-[500px] w-[700px] border">
      <Page {...args} />
    </div>
  ),
};

export const WithFooter: Story = {
  args: {
    frontmatter: {
      title: "Tutorial: Hello World",
      description: "Your first project step by step.",
    },
    children: (
      <>
        <h2>Step 1: Setup</h2>
        <p>Install the required dependencies.</p>
        <h2>Step 2: Create a project</h2>
        <p>Initialize a new project with the CLI.</p>
      </>
    ),
    footer: <div className="text-sm text-muted-foreground border-t pt-4 mt-4">Last updated: March 2026</div>,
  },
  render: (args) => (
    <div className="h-[500px] w-[700px] border">
      <Page {...args} />
    </div>
  ),
};

export const LongContent: Story = {
  args: {
    frontmatter: {
      title: "Reference Guide",
      description: "Complete API reference documentation.",
    },
    children: (
      <>
        {Array.from({ length: 10 }, (_, i) => (
          <div key={i}>
            <h2>Section {i + 1}</h2>
            <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
          </div>
        ))}
      </>
    ),
  },
  render: (args) => (
    <div className="h-[500px] w-[700px] border">
      <Page {...args} />
    </div>
  ),
};

// #endregion 🌈Page

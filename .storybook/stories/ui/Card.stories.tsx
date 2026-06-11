// #region 🧲Header

// .elements/ui/.storybook/story/elements/display/Card.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Aside, Card, CardGrid, LoadingRow, NotFound, Section, Spinner } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter } from "react-router";

// #region 🎬Card

const meta = {
  title: "🖱️ui⚛️react/Card",
  component: Card,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Card>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    title: "Example Card",
    children: <p>A brief description of this card item.</p>,
  },
};

export const WithIcon: Story = {
  args: {
    title: "Card with Emoji Icon",
    icon: "📦",
    children: <p>This card has an emoji icon.</p>,
  },
};

export const Grid: Story = {
  args: { title: "", children: null },
  render: () => (
    <CardGrid>
      {Array.from({ length: 6 }, (_, i) => (
        <Card key={i} title={`Card ${i + 1}`}>
          <p>Content for card {i + 1}.</p>
        </Card>
      ))}
    </CardGrid>
  ),
};

// #endregion 🎬Card

// #region 🎹Spinner

export const SpinnerStory: Story = {
  name: "Spinner",
  args: { title: "", children: null },
  render: () => (
    <div className="flex items-center gap-8">
      <Spinner size="small" />
      <Spinner size="medium" />
      <Spinner size="large" />
    </div>
  ),
};

// #endregion 🎹Spinner

// #region 🎍NotFound

export const NotFoundStory: Story = {
  name: "Not Found",
  args: { title: "", children: null },
  decorators: [
    (Story) => (
      <MemoryRouter>
        <Story />
      </MemoryRouter>
    ),
  ],
  render: () => (
    <div className="h-80">
      <NotFound title="Page Not Found" description="The page you're looking for doesn't exist." />
    </div>
  ),
};

// #endregion 🎍NotFound

// #region 🎺LoadingRow

export const LoadingRowStory: Story = {
  name: "Loading Row",
  args: { title: "", children: null },
  render: () => (
    <div className="space-y-2 w-75">
      <LoadingRow name="Loading item 1..." />
      <LoadingRow name="Loading item 2..." />
      <LoadingRow name="Loading item 3..." />
    </div>
  ),
};

// #endregion 🎺LoadingRow

// #region 🖲️Section

export const SectionStory: Story = {
  name: "Section",
  args: { title: "", children: null },
  render: () => (
    <div className="w-100 space-y-4">
      <Section title="Configuration">
        <div className="p-2 text-sm">Section content here.</div>
      </Section>
      <Section title="Advanced">
        <div className="p-2 text-sm">Advanced settings content.</div>
      </Section>
    </div>
  ),
};

// #endregion 🖲️Section

// #region 📣Aside

export const AsideStory: Story = {
  name: "Aside",
  args: { title: "", children: null },
  render: () => (
    <div className="w-100 space-y-4">
      <Aside title="Note" kind="note">
        This is an informational aside with important context.
      </Aside>
      <Aside title="Tip" kind="tip">
        This is a helpful tip for the reader.
      </Aside>
      <Aside title="Caution" kind="caution">
        Be careful with this operation.
      </Aside>
      <Aside title="Danger" kind="danger">
        This action cannot be undone.
      </Aside>
    </div>
  ),
};

// #endregion 📣Aside

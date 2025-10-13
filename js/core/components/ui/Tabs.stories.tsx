// #region Header

// Tabs.stories.tsx

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

import type { Meta, StoryObj } from "@storybook/react";
import { Home, Settings, User } from "lucide-react";
import { Button } from "./Button";
import { Input } from "./Input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./Tabs";

const meta = {
  title: "Elements/Tabs",
  component: Tabs,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tabs>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Tabs defaultValue="properties" className="w-[600px]">
      <TabsList>
        <TabsTrigger value="properties">Properties</TabsTrigger>
        <TabsTrigger value="connections">Connections</TabsTrigger>
        <TabsTrigger value="statistics">Statistics</TabsTrigger>
      </TabsList>
      <TabsContent value="properties" className="space-y-4">
        <div className="border p-4 space-y-2">
          <div className="flex justify-between">
            <span className="text-sm font-medium">Volume</span>
            <span className="text-sm">25.0 m³</span>
          </div>
          <div className="flex justify-between">
            <span className="text-sm font-medium">Area</span>
            <span className="text-sm">10.0 m²</span>
          </div>
          <div className="flex justify-between">
            <span className="text-sm font-medium">Mass</span>
            <span className="text-sm">3500 kg</span>
          </div>
        </div>
      </TabsContent>
      <TabsContent value="connections" className="space-y-4">
        <div className="border p-4 space-y-2">
          <p className="text-sm">3 active connections</p>
          <p className="text-sm text-muted-foreground">Base → Capsule J (Standard)</p>
          <p className="text-sm text-muted-foreground">Capsule J → Tambour A</p>
          <p className="text-sm text-muted-foreground">Tambour A → Capital</p>
        </div>
      </TabsContent>
      <TabsContent value="statistics" className="space-y-4">
        <div className="border p-4 space-y-2">
          <p className="text-sm">Total pieces: 140</p>
          <p className="text-sm">Capsule count: 132</p>
          <p className="text-sm">Total height: 52.4m</p>
        </div>
      </TabsContent>
    </Tabs>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Default</p>
        <Tabs defaultValue="model" className="w-96">
          <TabsList>
            <TabsTrigger value="model">Model</TabsTrigger>
            <TabsTrigger value="diagram">Diagram</TabsTrigger>
            <TabsTrigger value="details">Details</TabsTrigger>
          </TabsList>
          <TabsContent value="model">
            <div className="border p-4 rounded-md">3D model view</div>
          </TabsContent>
          <TabsContent value="diagram">
            <div className="border p-4 rounded-md">Connection diagram</div>
          </TabsContent>
          <TabsContent value="details">
            <div className="border p-4 rounded-md">Design properties</div>
          </TabsContent>
        </Tabs>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Vertical</p>
        <Tabs defaultValue="types" orientation="vertical" className="flex w-96">
          <TabsList className="flex-col h-auto">
            <TabsTrigger value="types">Types</TabsTrigger>
            <TabsTrigger value="designs">Designs</TabsTrigger>
          </TabsList>
          <TabsContent value="types" className="flex-1 ml-4">
            <div className="border p-4 rounded-md">Type library</div>
          </TabsContent>
          <TabsContent value="designs" className="flex-1 ml-4">
            <div className="border p-4 rounded-md">Design library</div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  ),
};

export const Basic: Story = {
  render: () => (
    <Tabs defaultValue="tab1" className="w-96">
      <TabsList>
        <TabsTrigger value="tab1">Tab 1</TabsTrigger>
        <TabsTrigger value="tab2">Tab 2</TabsTrigger>
        <TabsTrigger value="tab3">Tab 3</TabsTrigger>
      </TabsList>
      <TabsContent value="tab1">
        <div className="border p-4 rounded-md">Content for Tab 1</div>
      </TabsContent>
      <TabsContent value="tab2">
        <div className="border p-4 rounded-md">Content for Tab 2</div>
      </TabsContent>
      <TabsContent value="tab3">
        <div className="border p-4 rounded-md">Content for Tab 3</div>
      </TabsContent>
    </Tabs>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <Tabs defaultValue="home" className="w-96">
      <TabsList>
        <TabsTrigger value="home">
          <Home />
          Home
        </TabsTrigger>
        <TabsTrigger value="profile">
          <User />
          Profile
        </TabsTrigger>
        <TabsTrigger value="settings">
          <Settings />
          Settings
        </TabsTrigger>
      </TabsList>
      <TabsContent value="home">
        <div className="border p-4 rounded-md">Home content goes here</div>
      </TabsContent>
      <TabsContent value="profile">
        <div className="border p-4 rounded-md">Profile content goes here</div>
      </TabsContent>
      <TabsContent value="settings">
        <div className="border p-4 rounded-md">Settings content goes here</div>
      </TabsContent>
    </Tabs>
  ),
};

export const WithForm: Story = {
  render: () => (
    <Tabs defaultValue="account" className="w-96">
      <TabsList>
        <TabsTrigger value="account">Account</TabsTrigger>
        <TabsTrigger value="password">Password</TabsTrigger>
      </TabsList>
      <TabsContent value="account">
        <div className="border p-4 space-y-4">
          <div>
            <h3 className="font-medium mb-2">Account Settings</h3>
            <p className="text-sm text-muted-foreground">Make changes to your account here.</p>
          </div>
          <Input label="Name" defaultValue="John Doe" />
          <Input label="Email" type="email" defaultValue="john@example.com" />
          <Button>Save Changes</Button>
        </div>
      </TabsContent>
      <TabsContent value="password">
        <div className="border p-4 space-y-4">
          <div>
            <h3 className="font-medium mb-2">Password</h3>
            <p className="text-sm text-muted-foreground">Change your password here.</p>
          </div>
          <Input label="Current Password" type="password" />
          <Input label="New Password" type="password" />
          <Input label="Confirm Password" type="password" />
          <Button>Update Password</Button>
        </div>
      </TabsContent>
    </Tabs>
  ),
};

export const ManyTabs: Story = {
  render: () => (
    <Tabs defaultValue="tab1" className="w-full">
      <TabsList>
        <TabsTrigger value="tab1">Overview</TabsTrigger>
        <TabsTrigger value="tab2">Analytics</TabsTrigger>
        <TabsTrigger value="tab3">Reports</TabsTrigger>
        <TabsTrigger value="tab4">Notifications</TabsTrigger>
        <TabsTrigger value="tab5">Settings</TabsTrigger>
      </TabsList>
      <TabsContent value="tab1">
        <div className="border p-4 rounded-md">Overview content</div>
      </TabsContent>
      <TabsContent value="tab2">
        <div className="border p-4 rounded-md">Analytics content</div>
      </TabsContent>
      <TabsContent value="tab3">
        <div className="border p-4 rounded-md">Reports content</div>
      </TabsContent>
      <TabsContent value="tab4">
        <div className="border p-4 rounded-md">Notifications content</div>
      </TabsContent>
      <TabsContent value="tab5">
        <div className="border p-4 rounded-md">Settings content</div>
      </TabsContent>
    </Tabs>
  ),
};

export const Disabled: Story = {
  render: () => (
    <Tabs defaultValue="tab1" className="w-96">
      <TabsList>
        <TabsTrigger value="tab1">Active</TabsTrigger>
        <TabsTrigger value="tab2" disabled>
          Disabled
        </TabsTrigger>
        <TabsTrigger value="tab3">Another Active</TabsTrigger>
      </TabsList>
      <TabsContent value="tab1">
        <div className="border p-4 rounded-md">Content for active tab</div>
      </TabsContent>
      <TabsContent value="tab2">
        <div className="border p-4 rounded-md">Content for disabled tab</div>
      </TabsContent>
      <TabsContent value="tab3">
        <div className="border p-4 rounded-md">Content for another active tab</div>
      </TabsContent>
    </Tabs>
  ),
};

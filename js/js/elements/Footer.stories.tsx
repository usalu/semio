// #region Header

// Footer.stories.tsx

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
import { AlertCircle, CheckCircle2, Clock } from "lucide-react";
import Footer from "./Footer";

const meta = {
  title: "Elements/Footer",
  component: Footer,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Footer>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    items: [
      { id: "status", content: "Ready", tooltip: "Application status", order: 0 },
      { id: "cursor", content: "Ln 1, Col 1", tooltip: "Cursor position", order: 1 },
      { id: "encoding", content: "UTF-8", tooltip: "File encoding", order: 2 },
      { id: "language", content: "TypeScript", tooltip: "Language mode", order: 3 },
    ],
    height: 20,
    isVisible: true,
  },
};

export const WithIcons: Story = {
  args: {
    items: [
      { 
        id: "success", 
        content: <div className="flex items-center gap-1"><CheckCircle2 size={14} className="text-green-500" /><span>Success</span></div>, 
        tooltip: "Operation completed", 
        order: 0 
      },
      { 
        id: "warning", 
        content: <div className="flex items-center gap-1"><AlertCircle size={14} className="text-yellow-500" /><span>2 warnings</span></div>, 
        tooltip: "Click to view warnings", 
        order: 1 
      },
      { 
        id: "time", 
        content: <div className="flex items-center gap-1"><Clock size={14} /><span>2:30 PM</span></div>, 
        order: 2 
      },
    ],
    height: 24,
  },
};

export const Minimal: Story = {
  args: {
    items: [
      { id: "info", content: "Application v1.0.0", order: 0 },
    ],
    height: 20,
  },
};

export const Hidden: Story = {
  args: {
    items: [
      { id: "status", content: "Hidden footer", order: 0 },
    ],
    height: 20,
    isVisible: false,
  },
};

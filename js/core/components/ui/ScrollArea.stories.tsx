// #region Header

// ScrollArea.stories.tsx

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
import { ScrollArea } from "./ScrollArea";

const meta = {
  title: "Elements/ScrollArea",
  component: ScrollArea,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ScrollArea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Vertical: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <div className="p-4 space-y-4">
        <h4 className="text-sm font-medium">Scroll down to see more content</h4>
        {Array.from({ length: 50 }, (_, i) => (
          <div key={i} className="text-sm">
            Item {i + 1}
          </div>
        ))}
      </div>
    </ScrollArea>
  ),
};

export const LongText: Story = {
  render: () => (
    <ScrollArea className="h-64 w-96 border rounded-md">
      <div className="p-4">
        <h4 className="mb-4 text-sm font-medium">Terms and Conditions</h4>
        <div className="text-sm space-y-4">
          <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
          <p>Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
          <p>Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
          <p>Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
          <p>Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium.</p>
          <p>Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.</p>
          <p>Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.</p>
        </div>
      </div>
    </ScrollArea>
  ),
};

export const Horizontal: Story = {
  render: () => (
    <ScrollArea className="w-96 border rounded-md whitespace-nowrap">
      <div className="p-4">
        <div className="flex gap-4">
          {Array.from({ length: 20 }, (_, i) => (
            <div key={i} className="inline-flex h-20 w-40 items-center justify-center rounded-md border bg-muted">
              Card {i + 1}
            </div>
          ))}
        </div>
      </div>
    </ScrollArea>
  ),
};

export const CodeBlock: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <pre className="p-4 text-sm">
        {`function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

const result = fibonacci(10);
console.log(result); // 55

// More code...
function factorial(n) {
  if (n === 0) return 1;
  return n * factorial(n - 1);
}

const fact = factorial(5);
console.log(fact); // 120

// Additional functions
function isPrime(num) {
  if (num <= 1) return false;
  for (let i = 2; i <= Math.sqrt(num); i++) {
    if (num % i === 0) return false;
  }
  return true;
}

function findPrimes(limit) {
  const primes = [];
  for (let i = 2; i <= limit; i++) {
    if (isPrime(i)) primes.push(i);
  }
  return primes;
}

console.log(findPrimes(50));`}
      </pre>
    </ScrollArea>
  ),
};

export const List: Story = {
  render: () => (
    <ScrollArea className="h-80 w-96 border rounded-md">
      <div className="p-4">
        <h4 className="mb-4 text-sm font-medium">File List</h4>
        <div className="space-y-2">
          {[
            "package.json",
            "tsconfig.json",
            "vite.config.ts",
            "index.html",
            "README.md",
            "src/main.tsx",
            "src/App.tsx",
            "src/components/Button.tsx",
            "src/components/Input.tsx",
            "src/components/Dialog.tsx",
            "src/components/Select.tsx",
            "src/components/Tabs.tsx",
            "src/lib/utils.ts",
            "src/styles/globals.css",
            "public/favicon.ico",
            "public/logo.svg",
          ].map((file) => (
            <div key={file} className="flex items-center gap-2 p-2 hover:bg-accent rounded-sm cursor-pointer text-sm">
              <span className="font-mono">{file}</span>
            </div>
          ))}
        </div>
      </div>
    </ScrollArea>
  ),
};

export const BothDirections: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <div className="p-4" style={{ width: "800px" }}>
        <h4 className="mb-4 text-sm font-medium">Wide Table</h4>
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b">
              <th className="text-left p-2">Column 1</th>
              <th className="text-left p-2">Column 2</th>
              <th className="text-left p-2">Column 3</th>
              <th className="text-left p-2">Column 4</th>
              <th className="text-left p-2">Column 5</th>
              <th className="text-left p-2">Column 6</th>
            </tr>
          </thead>
          <tbody>
            {Array.from({ length: 20 }, (_, i) => (
              <tr key={i} className="border-b">
                <td className="p-2">Row {i + 1} Col 1</td>
                <td className="p-2">Data {i + 1}.2</td>
                <td className="p-2">Data {i + 1}.3</td>
                <td className="p-2">Data {i + 1}.4</td>
                <td className="p-2">Data {i + 1}.5</td>
                <td className="p-2">Data {i + 1}.6</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </ScrollArea>
  ),
};

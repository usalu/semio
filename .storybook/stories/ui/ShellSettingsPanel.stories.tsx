// #region 🧲Header

// 🥼︎ .storybook/stories/ui/ShellSettingsPanel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { Expertise, ShellSettingsPanel, type ElementsSurfaceAppearance, type UiLocale } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌Adapters

// 🧭#region 🧭ShellSettingsPanel
const locales: { readonly id: UiLocale; readonly label: string }[] = [
  { id: "en", label: "English" },
  { id: "de", label: "Deutsch" },
];

const meta = {
  title: "🖱️ui⚛️react/ShellSettingsPanel",
  component: ShellSettingsPanel,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ShellSettingsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [locale, setLocale] = useState<UiLocale>("en");
    const [theme, setTheme] = useState<ElementsSurfaceAppearance>("system");
    const [expertise, setExpertise] = useState<Expertise>(Expertise.NORMAL);
    return (
      <div className="w-80 border bg-panel">
        <ShellSettingsPanel locale={locale} locales={locales} onLocaleChange={setLocale} theme={theme} onThemeChange={setTheme} expertise={expertise} onExpertiseChange={setExpertise} />
      </div>
    );
  },
};

export const ExpertMode: Story = {
  render: () => {
    const [locale, setLocale] = useState<UiLocale>("de");
    const [theme, setTheme] = useState<ElementsSurfaceAppearance>("dark");
    const [expertise, setExpertise] = useState<Expertise>(Expertise.EXPERT);
    return (
      <div className="w-80 border bg-panel">
        <ShellSettingsPanel locale={locale} locales={locales} onLocaleChange={setLocale} theme={theme} onThemeChange={setTheme} expertise={expertise} onExpertiseChange={setExpertise} />
      </div>
    );
  },
};
// #endregion 🧭ShellSettingsPanel

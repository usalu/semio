// #region Header

// Hud.tsx

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

import { FC } from "react";
import { useTranslation } from "react-i18next";
import Panel from "../Panel";
import { ResizablePanelProps } from "../Sketchpad";

interface HudProps extends ResizablePanelProps {}

const Hud: FC<HudProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();

  return <Panel panelId="hud" visible={visible} onWidthChange={onWidthChange} width={width} resizeSide="right" zIndex={30} showBackground={false} emptyMessage={t("panels.hud.noSections")} />;
};

export default Hud;
